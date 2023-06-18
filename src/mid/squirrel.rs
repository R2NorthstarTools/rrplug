#![allow(clippy::not_unsafe_ptr_arg_deref)] // maybe remove later

use std::mem::MaybeUninit;

use once_cell::sync::OnceCell;

use crate::{
    bindings::{
        squirreldatatypes::{HSquirrelVM, SQObject},
        unwraped::SquirrelFunctionsUnwraped,
    },
    errors::CallError,
    high::{squirrel_traits::PushToSquirrelVm, vector::Vector3},
    to_sq_string,
};

/// functions that used to interact with the sqvm
///
/// client functions are both for ui and client vms
pub static SQFUNCTIONS: SqFunctions = SqFunctions {
    client: OnceCell::new(),
    server: OnceCell::new(),
};

/// functions that used to interact with the sqvm
///
/// client functions are both for ui and client vms
pub struct SqFunctions {
    pub client: OnceCell<SquirrelFunctionsUnwraped>,
    pub server: OnceCell<SquirrelFunctionsUnwraped>,
}

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

pub fn push_sq_float(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    float: impl Into<f32>,
) {
    unsafe { (sqfunctions.sq_pushfloat)(sqvm, float.into()) };
}

pub fn push_sq_int(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    int: impl Into<i32>,
) {
    unsafe { (sqfunctions.sq_pushinteger)(sqvm, int.into()) };
}

pub fn push_sq_bool(sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped, boolen: bool) {
    unsafe { (sqfunctions.sq_pushbool)(sqvm, boolen as u32) };
}

pub fn push_sq_string(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    string: impl Into<String>,
) {
    // boxing this would be a good idea and leaking; altough we don't need to?
    let cstring = to_sq_string!(string.into());
    // its impossble for it to crash since we replace null with space if it does it must be reported
    unsafe { (sqfunctions.sq_pushstring)(sqvm, cstring.as_ptr(), -1) }; // why -1?
}

pub fn push_sq_vector(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    vector: Vector3,
) {
    unsafe { (sqfunctions.sq_pushvector)(sqvm, (&vector).into()) };
}

pub fn push_sq_object(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    mut object: MaybeUninit<SQObject>,
) {
    unsafe { (sqfunctions.sq_pushobject)(sqvm, object.as_mut_ptr()) };
}

pub fn get_sq_function_object(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    function_name: impl Into<String>,
) -> Result<Box<MaybeUninit<SQObject>>, CallError> {
    let mut obj = Box::new(MaybeUninit::<SQObject>::zeroed());
    let ptr = obj.as_mut_ptr();

    let function_name = to_sq_string!(function_name.into());

    let result = unsafe {
        (sqfunctions.sq_getfunction)(sqvm, function_name.as_ptr(), ptr, std::ptr::null())
    };

    if result != 0 {
        Err(CallError::FunctionNotFound(
            function_name.to_string_lossy().into(),
        )) // totaly safe :clueless:
    } else {
        Ok(obj)
    }
}
