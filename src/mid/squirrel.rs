//! thin wrappers around squirrel functions
//!
//! some can produce exepections which cannot be caught
//!
//! good reference for some functions : [objecthandling](https://r2northstar.readthedocs.io/en/latest/squirrel/cpp_api/objecthandling.htmls)

#![allow(clippy::not_unsafe_ptr_arg_deref)] // maybe remove later

use std::{ffi::CStr, mem::MaybeUninit};

use once_cell::sync::OnceCell;

#[cfg(doc)]
use crate::high::squirrel_traits::GetFromSQObject;
use crate::{
    bindings::{
        squirreldatatypes::{HSquirrelVM, SQClosure, SQObject},
        unwraped::SquirrelFunctionsUnwraped,
    },
    errors::CallError,
    high::{squirrel::SQHandle, squirrel_traits::PushToSquirrelVm, vector::Vector3},
    to_c_string,
};

/// functions that used to interact with the sqvm
///
/// client functions are both for ui and client vms
pub static SQFUNCTIONS: SqFunctions = SqFunctions {
    client: OnceCell::new(),
    server: OnceCell::new(),
};

/// functions that are used to interact with the sqvm
///
/// client functions are both for ui and client vms
pub struct SqFunctions {
    /// client squirrel functions
    pub client: OnceCell<SquirrelFunctionsUnwraped>,

    /// server squirrel functions
    pub server: OnceCell<SquirrelFunctionsUnwraped>,
}

// maybe this will work in the future
// const fn get_sq_function<const T: i8>() -> &'static OnceCell<SquirrelFunctionsUnwraped> {
//     const SERVER: i8 = ScriptVmType::Server as i8;
//     const CLIENT: i8 = ScriptVmType::Client as i8;
//     const UI: i8 = ScriptVmType::Ui as i8;
//     match T {
//         SERVER => &SQFUNCTIONS.server,
//         CLIENT => &SQFUNCTIONS.client,
//         UI => &SQFUNCTIONS.client,
//         _ => unreachable!(),
//     }
// }

/// pushes a `Vec<T>` to the sqvm
#[inline]
pub fn push_sq_array<T>(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    arguments: Vec<T>,
) where
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
pub fn push_sq_float(sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped, float: f32) {
    unsafe { (sqfunctions.sq_pushfloat)(sqvm, float) };
}

/// pushes a [`i32`] to the sqvm
#[inline]
pub fn push_sq_int(sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped, int: i32) {
    unsafe { (sqfunctions.sq_pushinteger)(sqvm, int) };
}

/// pushes a [`bool`] to the sqvm
#[inline]
pub fn push_sq_bool(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    boolean: bool,
) {
    unsafe { (sqfunctions.sq_pushbool)(sqvm, boolean as u32) };
}

/// pushes a `T: Into<String>` to the sqvm
#[inline]
pub fn push_sq_string(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    string: impl Into<String>,
) {
    // boxing this would be a good idea and leaking; altough we don't need to?
    let cstring = to_c_string!(string.into());
    // its impossble for it to crash since we replace null with space if it does it must be reported
    unsafe { (sqfunctions.sq_pushstring)(sqvm, cstring.as_ptr(), -1) }; // why -1?
}

/// pushes a [`Vector3`] to the sqvm
#[inline]
pub fn push_sq_vector(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    vector: Vector3, // this could be a borrow actually but this function is used in places where it would hard to change to a borrow so yeah
) {
    unsafe { (sqfunctions.sq_pushvector)(sqvm, (&vector).into()) };
}

/// pushes a [`SQObject`] to the sqvm
#[inline]
pub fn push_sq_object(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    mut object: MaybeUninit<SQObject>,
) {
    unsafe { (sqfunctions.sq_pushobject)(sqvm, object.as_mut_ptr()) };
}

/// gets a array of T at a stack pos
///
/// # Caller input
///
/// requires the caller to be able to translate a [`SQObject`] into T.
/// it can be done using [`GetFromSQObject`].
#[inline]
pub fn get_sq_array<T, F>(sqvm: *mut HSquirrelVM, stack_pos: i32, transformer: F) -> Vec<T>
where
    T: PushToSquirrelVm,
    F: Fn(&SQObject) -> Option<T>,
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
            .filter_map(transformer)
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
    sqfunctions: &SquirrelFunctionsUnwraped,
    stack_pos: i32,
) -> f32 {
    unsafe { (sqfunctions.sq_getfloat)(sqvm, stack_pos) }
}

/// gets a int at a stack pos
///
/// # Exceptions
/// the sqvm can throw an exceptions if the it's not a int
#[inline]
pub fn get_sq_int(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    stack_pos: i32,
) -> i32 {
    unsafe { (sqfunctions.sq_getinteger)(sqvm, stack_pos) }
}

/// gets a bool at a stack pos
///
/// # Exceptions
/// the sqvm can throw an exceptions if the it's not a bool
#[inline]
pub fn get_sq_bool(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
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
    sqfunctions: &SquirrelFunctionsUnwraped,
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
    sqfunctions: &SquirrelFunctionsUnwraped,
    stack_pos: i32,
) -> Vector3 {
    unsafe { (sqfunctions.sq_getvector)(sqvm, stack_pos).into() }
}

/// gets the [`SQObject`] at a stack pos
#[inline]
pub fn get_sq_object(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
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
    sqfunctions: &SquirrelFunctionsUnwraped,
    function_name: impl Into<String>,
) -> Result<SQHandle<SQClosure>, CallError> {
    let mut obj = MaybeUninit::<SQObject>::zeroed();
    let ptr = obj.as_mut_ptr();

    let function_name = to_c_string!(function_name.into());

    let result = unsafe {
        (sqfunctions.sq_getfunction)(sqvm, function_name.as_ptr(), ptr, std::ptr::null())
    };

    if result != 0 {
        Err(CallError::FunctionNotFound(
            function_name.to_string_lossy().into(),
        )) // totaly safe :clueless:
    } else {
        Ok(unsafe { SQHandle::new_unchecked(obj.assume_init()) }) // this is always corret since sq_getfunction can only return SQClosure
    }
}
