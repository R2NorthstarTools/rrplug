//! thin wrappers around squirrel functions
//!
//! some can produce exepections which cannot be caught
//!
//! good reference for some functions : [objecthandling](https://r2northstar.readthedocs.io/en/latest/squirrel/cpp_api/objecthandling.htmls)

#![allow(clippy::not_unsafe_ptr_arg_deref)] // maybe remove later

use std::{cell::RefCell, ffi::CStr, mem::MaybeUninit};

use once_cell::sync::OnceCell;

use crate::{
    bindings::{
        squirrelclasstypes::{SQFunction, ScriptContext},
        squirreldatatypes::{CSquirrelVM, HSquirrelVM, SQClosure, SQObject},
        squirrelfunctions::{
            ClientSQFunctions, ServerSQFunctions, SquirrelFunctions, SQUIRREL_CLIENT_FUNCS,
            SQUIRREL_SERVER_FUNCS,
        },
    },
    errors::CallError,
    high::{
        engine::EngineGlobal,
        squirrel::SQHandle,
        squirrel_traits::{GetFromSQObject, PushToSquirrelVm},
        vector::Vector3,
    },
    prelude::DLLPointer,
};

use super::utils::{to_cstring, try_cstring};

/// functions that used to interact with the sqvm
///
/// client functions are both for ui and client vms
pub static SQFUNCTIONS: SqFunctions = SqFunctions {
    client: OnceCell::new(),
    server: OnceCell::new(),
};

pub static SQVM_SERVER: EngineGlobal<RefCell<Option<*mut HSquirrelVM>>> =
    EngineGlobal::new(RefCell::new(None));
pub static SQVM_UI: EngineGlobal<RefCell<Option<*mut HSquirrelVM>>> =
    EngineGlobal::new(RefCell::new(None));
pub static SQVM_CLIENT: EngineGlobal<RefCell<Option<*mut HSquirrelVM>>> =
    EngineGlobal::new(RefCell::new(None));

/// functions that are used to interact with the sqvm
///
/// client functions are both for ui and client vms
pub struct SqFunctions {
    /// client squirrel functions
    pub client: OnceCell<SquirrelFunctions>,

    /// server squirrel functions
    pub server: OnceCell<SquirrelFunctions>,
}

impl SqFunctions {
    #[doc(hidden)]
    pub fn fetch_functions(&self, dll: &DLLPointer) {
        unsafe { ClientSQFunctions::try_init(dll, &SQUIRREL_CLIENT_FUNCS) };
        unsafe { ServerSQFunctions::try_init(dll, &SQUIRREL_SERVER_FUNCS) };
    }

    #[doc(hidden)]
    pub fn try_init(&self) -> Option<()> {
        self.client.set(SQUIRREL_CLIENT_FUNCS.wait().into()).ok()?;
        self.server.set(SQUIRREL_SERVER_FUNCS.wait().into()).ok()?;

        None
    }

    pub fn from_sqvm(&'static self, sqvm: *mut HSquirrelVM) -> &'static SquirrelFunctions {
        self.from_cssqvm(unsafe { (*(*sqvm).sharedState).cSquirrelVM })
    }
    pub fn from_cssqvm(&'static self, sqvm: *mut CSquirrelVM) -> &'static SquirrelFunctions {
        const SERVER: i32 = ScriptContext::SERVER as i32;
        const CLIENT: i32 = ScriptContext::CLIENT as i32;
        const UI: i32 = ScriptContext::UI as i32;

        match unsafe { (*(sqvm)).vmContext } {
            SERVER => self.server.wait(),
            CLIENT | UI => self.client.wait(),
            _ => {
                #[cfg(not(debug_assertions))]
                unreachable!();
                #[cfg(debug_assertions)]
                panic!("vmContext was somehow something other than the valid vms");
            }
        }
    }
}

/// function type which is used in `register_sq_functions` to get [`SQFuncInfo`]
pub type FuncSQFuncInfo = fn() -> SQFuncInfo;

/// holds infomation about a sq function for it to be registered corretly
///
/// it creates a native closure btw but sqfunction is also a valid name for it.
/// sqfunction is used in a lot of places with diffrent meanings `¯\_(ツ)_/¯`
#[derive(Debug, PartialEq, Eq)]
pub struct SQFuncInfo {
    /// the name used in source code
    pub cpp_func_name: &'static str,
    /// name of the defined
    pub sq_func_name: &'static str,
    /// the arguments of the function in squirrel form
    ///
    /// # Example
    /// ```
    /// let types = "string name, int id";
    /// ```
    pub types: String,
    /// the return value of the function in squirrel form
    pub return_type: String,
    /// the which vm should be used to register the function on
    pub vm: SQFunctionContext,
    /// the actual function pointer
    pub function: SQFunction,
}

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
    pub struct SQFunctionContext: u32 {
        const SERVER = 0b001;
        const CLIENT = 0b010;
        const UI = 0b100;
    }
}

impl SQFunctionContext {
    pub const fn contains_context(&self, context: ScriptContext) -> bool {
        match context {
            ScriptContext::SERVER => self.contains(Self::SERVER),
            ScriptContext::CLIENT => self.contains(Self::CLIENT),
            ScriptContext::UI => self.contains(Self::UI),
        }
    }
}

/// pushes a `Vec<T>` to the sqvm
#[inline]
pub fn push_sq_array<T>(sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctions, arguments: Vec<T>)
where
    T: PushToSquirrelVm,
{
    unsafe { (sqfunctions.sq_newarray)(sqvm, 0) }

    for e in arguments.into_iter() {
        e.push_to_sqvm(sqvm, sqfunctions);
        unsafe { (sqfunctions.sq_arrayappend)(sqvm, -2) };
    }
}

/// pushes a [`f32`] to the sqvm
#[inline]
pub fn push_sq_float(sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctions, float: f32) {
    unsafe { (sqfunctions.sq_pushfloat)(sqvm, float) };
}

/// pushes a [`i32`] to the sqvm
#[inline]
pub fn push_sq_int(sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctions, int: i32) {
    unsafe { (sqfunctions.sq_pushinteger)(sqvm, int) };
}

/// pushes a [`bool`] to the sqvm
#[inline]
pub fn push_sq_bool(sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctions, boolean: bool) {
    unsafe { (sqfunctions.sq_pushbool)(sqvm, boolean as u32) };
}

/// pushes a `T: Into<String>` to the sqvm
#[inline]
pub fn push_sq_string(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctions,
    string: impl AsRef<str>,
) {
    let cstring = try_cstring(string.as_ref())
        .unwrap_or_else(|_| to_cstring(&string.as_ref().replace('\0', "")));
    // its impossble for it to crash since we replace null with space if it does it must be reported
    unsafe { (sqfunctions.sq_pushstring)(sqvm, cstring.as_ptr(), -1) }; // why -1?
}

/// pushes a [`Vector3`] to the sqvm
#[inline]
pub fn push_sq_vector(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctions,
    vector: Vector3, // this could be a borrow actually but this function is used in places where it would hard to change to a borrow so yeah
) {
    unsafe { (sqfunctions.sq_pushvector)(sqvm, (&vector).into()) };
}

/// pushes a [`SQObject`] to the sqvm
#[inline]
pub fn push_sq_object(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctions,
    mut object: MaybeUninit<SQObject>,
) {
    unsafe { (sqfunctions.sq_pushobject)(sqvm, object.as_mut_ptr()) };
}

/// gets a array of T at a stack pos
///
/// type T must have GetFromSQObject implemented
#[inline]
pub fn get_sq_array<T>(sqvm: *mut HSquirrelVM, stack_pos: i32) -> Vec<T>
where
    T: GetFromSQObject,
{
    unsafe {
        let sqvm_ref = sqvm.as_ref().expect("ok how is this sqvm invalid");

        let array = sqvm_ref
            ._stackOfCurrentFunction
            .add(stack_pos as usize)
            .as_ref()
            .expect("the stack pos may be invalid")
            ._VAL
            .asArray
            .as_ref()
            .expect("the sq object may be invalid");

        (0..array._usedSlots as usize)
            .map(|i| array._values.add(i))
            .filter_map(|obj| obj.as_ref())
            .map(T::get_from_sqobject)
            .collect()
    }
}

/// gets a float at a stack pos
///
/// # Exceptions
/// the sqvm can throw an exceptions if the it's not a float
#[inline]
pub fn get_sq_float(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctions,
    stack_pos: i32,
) -> f32 {
    unsafe { (sqfunctions.sq_getfloat)(sqvm, stack_pos) }
}

/// gets a int at a stack pos
///
/// # Exceptions
/// the sqvm can throw an exceptions if the it's not a int
#[inline]
pub fn get_sq_int(sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctions, stack_pos: i32) -> i32 {
    unsafe { (sqfunctions.sq_getinteger)(sqvm, stack_pos) }
}

/// gets a bool at a stack pos
///
/// # Exceptions
/// the sqvm can throw an exceptions if the it's not a bool
#[inline]
pub fn get_sq_bool(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctions,
    stack_pos: i32,
) -> bool {
    unsafe { (sqfunctions.sq_getbool)(sqvm, stack_pos) != 0 }
}

/// gets a string at a stack pos
///
/// uses `CStr::to_string_lossy` to always get a valid string
///
/// # Exceptions
/// the sqvm can throw an exceptions if the it's not a string
#[inline]
pub fn get_sq_string(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctions,
    stack_pos: i32,
) -> String {
    unsafe {
        CStr::from_ptr((sqfunctions.sq_getstring)(sqvm, stack_pos))
            .to_string_lossy()
            .into()
    }
}

/// gets a vector at a stack pos
///
/// # Exceptions
/// the sqvm can throw an exceptions if the it's not a vector
#[inline]
pub fn get_sq_vector(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctions,
    stack_pos: i32,
) -> Vector3 {
    unsafe { (sqfunctions.sq_getvector)(sqvm, stack_pos).into() }
}

/// gets the [`SQObject`] at a stack pos
#[inline]
pub fn get_sq_object(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctions,
    stack_pos: i32,
) -> MaybeUninit<SQObject> {
    let mut obj: MaybeUninit<SQObject> = MaybeUninit::uninit();
    unsafe {
        (sqfunctions.sq_getobject)(sqvm, stack_pos, obj.as_mut_ptr());
    };

    obj
}

/// gets a function [`SQObject`] from the sqvm
///
/// # Errors
/// fails if it doesn't exist
pub fn get_sq_function_object(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctions,
    function_name: impl AsRef<str>,
) -> Result<SQHandle<SQClosure>, CallError> {
    let mut obj = MaybeUninit::<SQObject>::zeroed();

    let function_name = try_cstring(function_name.as_ref())?;

    let result = unsafe {
        (sqfunctions.sq_getfunction)(
            sqvm,
            function_name.as_ptr(),
            obj.as_mut_ptr(),
            std::ptr::null(),
        )
    };

    if result != 0 {
        Err(CallError::FunctionNotFound(
            function_name.to_string_lossy().into(),
        )) // totaly safe :clueless:
    } else {
        Ok(unsafe { SQHandle::new_unchecked(obj.assume_init()) }) // this is always corret since sq_getfunction can only return SQClosure
    }
}
