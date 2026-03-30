//! thin wrappers around squirrel functions
//!
//! some can produce exepections which cannot be caught
//!
//! good reference for some functions : [objecthandling](https://r2northstar.readthedocs.io/en/latest/squirrel/cpp_api/objecthandling.htmls)

#![allow(clippy::not_unsafe_ptr_arg_deref)] // maybe remove later

use std::{
    cell::RefCell, ffi::CStr, mem::MaybeUninit, path::PathBuf, ptr::NonNull,
    sync::atomic::AtomicU32,
};

use once_cell::sync::OnceCell;

use crate::{
    bindings::{
        squirrelclasstypes::{
            eSQReturnType, SQFuncRegistration, SQFunction, ScriptContext, SQRESULT,
        },
        squirreldatatypes::{CSquirrelVM, HSquirrelVM, SQClosure, SQObject},
        squirrelfunctions::{
            ClientSQFunctions, ServerSQFunctions, SquirrelFunctions, SQUIRREL_CLIENT_FUNCS,
            SQUIRREL_SERVER_FUNCS,
        },
    },
    errors::{CallError, SQFunctionRegistrationError},
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

/// the server sqvm
pub static SQVM_SERVER: EngineGlobal<RefCell<Option<NonNull<HSquirrelVM>>>> =
    EngineGlobal::new(RefCell::new(None));
/// the ui sqvm
pub static SQVM_UI: EngineGlobal<RefCell<Option<NonNull<HSquirrelVM>>>> =
    EngineGlobal::new(RefCell::new(None));
/// the client sqvm
pub static SQVM_CLIENT: EngineGlobal<RefCell<Option<NonNull<HSquirrelVM>>>> =
    EngineGlobal::new(RefCell::new(None));

/// the server sqvm generation
pub static SQVM_SERVER_GENERATION: AtomicU32 = AtomicU32::new(0);
/// the ui sqvm generation
pub static SQVM_UI_GENERATION: AtomicU32 = AtomicU32::new(0);
/// the client sqvm generation
pub static SQVM_CLIENT_GENERATION: AtomicU32 = AtomicU32::new(0);

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
        self.server
            .set(
                SQUIRREL_SERVER_FUNCS
                    .get()
                    .expect("server functions have to always be present!")
                    .into(),
            )
            .ok()?;
        self.client.set(SQUIRREL_CLIENT_FUNCS.get()?.into()).ok()?; // client functions can be absent on dedicated servers

        None
    }

    /// returns [`SquirrelFunctions `] that match the squirrel vm context
    pub fn from_sqvm(&'static self, sqvm: NonNull<HSquirrelVM>) -> &'static SquirrelFunctions {
        self.from_cssqvm(unsafe { (*sqvm.as_ref().sharedState).cSquirrelVM })
    }

    /// returns [`SquirrelFunctions `] that match the squirrel vm context
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

/// holds information about a native sqfunction for it to be registered correctly
///
/// it creates a native closure btw but sqfunction is also a valid name for it.
/// sqfunction is used in a lot of places with different meanings `¯\_(ツ)_/¯`
#[derive(Debug, PartialEq, Eq)]
#[allow(unpredictable_function_pointer_comparisons)]
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
    #[doc = "the contexts for which the squirrel function would be registered"]
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
    pub struct SQFunctionContext: u32 {
        #[doc = "server context"]
        const SERVER = 0b001;
        #[doc = "client context"]
        const CLIENT = 0b010;
        #[doc = "ui context"]
        const UI = 0b100;
    }
}

impl SQFunctionContext {
    /// returns if [`SQFunctionContext`] covers a [`ScriptContext`]
    pub const fn contains_context(&self, context: ScriptContext) -> bool {
        match context {
            ScriptContext::SERVER => self.contains(Self::SERVER),
            ScriptContext::CLIENT => self.contains(Self::CLIENT),
            ScriptContext::UI => self.contains(Self::UI),
        }
    }
}

/// returns the context of the sqvm
///
/// # Safety
/// assumes the sqvm is valid and does a bunch of derefs
#[inline]
pub unsafe fn sqvm_to_context(sqvm: NonNull<HSquirrelVM>) -> ScriptContext {
    ScriptContext::try_from(unsafe { (*(*sqvm.as_ref().sharedState).cSquirrelVM).vmContext })
        .expect("sqvm should have a valid vm context")
}

/// allows you to manually register native squirrel functions
///
/// # Panics
///
/// Panics if somehow your functions, types or return types have a null byte, therefore never
///
/// # Errors
///
/// This function will return an error if the function context and the sqvm context have a mismatch or if it couldn't get registered for some reason
///
/// # Safety
///
/// only safe to be called when the sqvm is fully init and that the call is happening from the game thread
pub unsafe fn manually_register_sq_functions(
    csqvm: &mut CSquirrelVM,
    func_info: &SQFuncInfo,
) -> Result<(), SQFunctionRegistrationError> {
    let context: ScriptContext = csqvm
        .vmContext
        .try_into()
        .expect("sqvm was not valid :((((");

    if !func_info.vm.contains_context(context) {
        return Err(SQFunctionRegistrationError::WrongContext(
            func_info.cpp_func_name,
            context,
            func_info
                .vm
                .iter_names()
                .next()
                .map(|(name, _)| name)
                .unwrap_or("NULL"),
        ));
    }

    log::info!(
        "Registering {context} function {} with types: {}",
        func_info.sq_func_name,
        func_info.types
    );

    let enum_return_type = match func_info
        .return_type
        .split_once('<')
        .map(|(ty, _)| ty)
        .unwrap_or(&func_info.return_type)
    {
        "bool" => eSQReturnType::Boolean,
        "float" => eSQReturnType::Float,
        "vector" => eSQReturnType::Vector,
        "int" => eSQReturnType::Integer,
        "entity" => eSQReturnType::Entity,
        "string" => eSQReturnType::String,
        "array" => eSQReturnType::Arrays,
        "asset" => eSQReturnType::Asset,
        "table" => eSQReturnType::Table,
        "void" => eSQReturnType::Default,
        "var" => eSQReturnType::Default,
        _ => eSQReturnType::Default,
    };

    let sq_func_name = try_cstring(func_info.sq_func_name).unwrap();
    let actual_func_name = try_cstring(func_info.cpp_func_name).unwrap();
    let return_type = try_cstring(&func_info.return_type).unwrap();
    let types: &str = &func_info.types;
    let types = try_cstring(types).unwrap();

    let mut reg = SQFuncRegistration {
        squirrelFuncName: sq_func_name.as_ptr(),
        cppFuncName: actual_func_name.as_ptr(),
        helpText: c"default rrplug help message".as_ptr(),
        returnTypeString: return_type.as_ptr(),
        argTypes: types.as_ptr(),
        unknown1: 0,
        devLevel: 0,
        shortNameMaybe: std::ptr::null(),
        unknown2: 0,
        returnType: enum_return_type,
        externalBufferPointer: std::ptr::null_mut(),
        externalBufferSize: 0,
        unknown3: 0,
        unknown4: 0,
        funcPtr: func_info.function,
    };

    // TODO: figure when it fails
    // if unsafe { (SQFUNCTIONS.from_cssqvm(csqvm).register_squirrel_func)(csqvm, &mut reg, 1) }
    //     != SQRESULT::SQRESULT_ERROR as i64
    // {
    //     return Err(SQFunctionRegistrationError::FailedRegistration(
    //         func_info.cpp_func_name,
    //     ));
    // }
    unsafe { (SQFUNCTIONS.from_cssqvm(csqvm).register_squirrel_func)(csqvm, &mut reg, 1) };

    Ok(())
}

/// pushes a `Vec<T>` to the sqvm
#[inline]
pub fn push_sq_array<T>(
    sqvm: NonNull<HSquirrelVM>,
    sqfunctions: &SquirrelFunctions,
    arguments: impl IntoIterator<Item = T>,
) where
    T: PushToSquirrelVm,
{
    unsafe { (sqfunctions.sq_newarray)(sqvm.as_ptr(), 0) }

    for e in arguments.into_iter() {
        e.push_to_sqvm(sqvm, sqfunctions);
        unsafe { (sqfunctions.sq_arrayappend)(sqvm.as_ptr(), -2) };
    }
}

/// pushes a [`f32`] to the sqvm
#[inline]
pub fn push_sq_float(sqvm: NonNull<HSquirrelVM>, sqfunctions: &SquirrelFunctions, float: f32) {
    unsafe { (sqfunctions.sq_pushfloat)(sqvm.as_ptr(), float) };
}

/// pushes a [`i32`] to the sqvm
#[inline]
pub fn push_sq_int(sqvm: NonNull<HSquirrelVM>, sqfunctions: &SquirrelFunctions, int: i32) {
    unsafe { (sqfunctions.sq_pushinteger)(sqvm.as_ptr(), int) };
}

/// pushes a [`bool`] to the sqvm
#[inline]
pub fn push_sq_bool(sqvm: NonNull<HSquirrelVM>, sqfunctions: &SquirrelFunctions, boolean: bool) {
    unsafe { (sqfunctions.sq_pushbool)(sqvm.as_ptr(), boolean as u32) };
}

/// pushes a `T: Into<String>` to the sqvm
#[inline]
pub fn push_sq_string(
    sqvm: NonNull<HSquirrelVM>,
    sqfunctions: &SquirrelFunctions,
    string: impl AsRef<str>,
) {
    let cstring = try_cstring(string.as_ref())
        .unwrap_or_else(|_| to_cstring(&string.as_ref().replace('\0', "")));
    // its impossible for it to crash since we replace null with space if it does it must be reported
    unsafe { (sqfunctions.sq_pushstring)(sqvm.as_ptr(), cstring.as_ptr(), -1) };
    // why -1?
}

/// pushes a [`Vector3`] to the sqvm
#[inline]
pub fn push_sq_vector(
    sqvm: NonNull<HSquirrelVM>,
    sqfunctions: &SquirrelFunctions,
    vector: Vector3, // this could be a borrow actually but this function is used in places where it would hard to change to a borrow so yeah
) {
    unsafe { (sqfunctions.sq_pushvector)(sqvm.as_ptr(), (&vector).into()) };
}

/// pushes a [`SQObject`] to the sqvm
#[inline]
pub fn push_sq_object(
    sqvm: NonNull<HSquirrelVM>,
    sqfunctions: &SquirrelFunctions,
    mut object: SQObject,
) {
    unsafe { (sqfunctions.sq_pushobject)(sqvm.as_ptr(), &mut object) };
}

/// gets a array of T at a stack pos
///
/// type T must have GetFromSQObject implemented
#[inline]
pub fn get_sq_array<T>(sqvm: NonNull<HSquirrelVM>, stack_pos: i32) -> Vec<T>
where
    T: GetFromSQObject,
{
    unsafe {
        let sqvm_ref = sqvm.as_ref();

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
    sqvm: NonNull<HSquirrelVM>,
    sqfunctions: &SquirrelFunctions,
    stack_pos: i32,
) -> f32 {
    unsafe { (sqfunctions.sq_getfloat)(sqvm.as_ptr(), stack_pos) }
}

/// gets a int at a stack pos
///
/// # Exceptions
/// the sqvm can throw an exceptions if the it's not a int
#[inline]
pub fn get_sq_int(
    sqvm: NonNull<HSquirrelVM>,
    sqfunctions: &SquirrelFunctions,
    stack_pos: i32,
) -> i32 {
    unsafe { (sqfunctions.sq_getinteger)(sqvm.as_ptr(), stack_pos) }
}

/// gets a bool at a stack pos
///
/// # Exceptions
/// the sqvm can throw an exceptions if the it's not a bool
#[inline]
pub fn get_sq_bool(
    sqvm: NonNull<HSquirrelVM>,
    sqfunctions: &SquirrelFunctions,
    stack_pos: i32,
) -> bool {
    unsafe { (sqfunctions.sq_getbool)(sqvm.as_ptr(), stack_pos) != 0 }
}

/// gets a string at a stack pos
///
/// uses `CStr::to_string_lossy` to always get a valid string
///
/// # Exceptions
/// the sqvm can throw an exceptions if the it's not a string
#[inline]
pub fn get_sq_string(
    sqvm: NonNull<HSquirrelVM>,
    sqfunctions: &SquirrelFunctions,
    stack_pos: i32,
) -> String {
    unsafe {
        CStr::from_ptr((sqfunctions.sq_getstring)(sqvm.as_ptr(), stack_pos))
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
    sqvm: NonNull<HSquirrelVM>,
    sqfunctions: &SquirrelFunctions,
    stack_pos: i32,
) -> Vector3 {
    unsafe { (sqfunctions.sq_getvector)(sqvm.as_ptr(), stack_pos).into() }
}

/// gets the [`SQObject`] at a stack pos
#[inline]
pub fn get_sq_object(
    sqvm: NonNull<HSquirrelVM>,
    sqfunctions: &SquirrelFunctions,
    stack_pos: i32,
) -> SQObject {
    let mut obj: MaybeUninit<SQObject> = MaybeUninit::uninit();
    unsafe {
        (sqfunctions.sq_getobject)(sqvm.as_ptr(), stack_pos, obj.as_mut_ptr());
    };

    unsafe { obj.assume_init() }
}

/// gets a function [`SQObject`] from the sqvm
///
/// # Errors
/// fails if it doesn't exist
pub fn get_sq_function_object<'a>(
    sqvm: NonNull<HSquirrelVM>,
    sqfunctions: &'a SquirrelFunctions,
    function_name: impl AsRef<str>,
) -> Result<SQHandle<'a, SQClosure>, CallError> {
    let mut obj = MaybeUninit::<SQObject>::zeroed();

    let function_name = try_cstring(function_name.as_ref())?;

    let result = unsafe {
        (sqfunctions.sq_getfunction)(
            sqvm.as_ptr(),
            function_name.as_ptr(),
            obj.as_mut_ptr(),
            std::ptr::null(),
        )
    };

    if result != 0 {
        Err(CallError::FunctionNotFound(
            function_name.to_string_lossy().into(),
        )) // totally safe :clueless:
    } else {
        Ok(unsafe { SQHandle::new_unchecked(obj.assume_init()) }) // this is always correct since sq_getfunction can only return SQClosure
    }
}

/// returns the file path in the virtual filesystem from where the call originated
///
/// # Errors
///
/// if the source of the file is null returns None
pub fn get_calling_file(
    mut sqvm: NonNull<HSquirrelVM>,
    sq_functions: &SquirrelFunctions,
) -> Option<PathBuf> {
    // if 1 >= unsafe { sqvm.as_ref()._callstacksize } {
    //     return None;
    // }

    let stack_info = unsafe {
        let mut stack_info = MaybeUninit::uninit();
        (sq_functions.sq_stackinfos)(
            sqvm.as_mut(),
            1,
            stack_info.as_mut_ptr(),
            sqvm.as_ref()._callstacksize,
        );
        stack_info.assume_init()
    };

    if stack_info._sourceName.is_null() {
        return None;
    }

    let path = PathBuf::from("scripts")
        .join("vscripts")
        .join(PathBuf::from(
            unsafe { CStr::from_ptr(stack_info._sourceName) }
                .to_string_lossy()
                .to_string()
                .replace('/', "\\")
                .to_lowercase(),
        ));
    // Some(path.normalize_lexically().unwrap_or(path))
    Some(path)
}
