#![allow(clippy::not_unsafe_ptr_arg_deref)]

//! squirrel vm related function and statics

use once_cell::sync::OnceCell;
use std::sync::Mutex;

use super::{
    northstar::{FuncSQFuncInfo, ScriptVmType},
    vector::Vector3,
};
use crate::{
    bindings::{
        squirrelclasstypes::SQFunction, squirreldatatypes::HSquirrelVM,
        unwraped::SquirrelFunctionsUnwraped,
    },
    sq_return_null, to_sq_string,
};

#[doc(hidden)]
pub static mut FUNCTION_SQ_REGISTER: Mutex<Vec<FuncSQFuncInfo>> = Mutex::new(Vec::new());

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

/// [`call_sq_function`] "safely" calls any function defined on the sqvm
///
/// they would only run when the sqvm is valid
pub fn call_sq_function(
    contex: ScriptVmType,
    function_name: impl Into<String>,
    pop_function: Option<SQFunction>,
) {
    let sqfunctions = match contex {
        ScriptVmType::Server => SQFUNCTIONS.server.wait(),
        _ => SQFUNCTIONS.client.wait(),
    };

    let pop_function = match pop_function {
        Some(callback) => callback,
        None => __pop_function,
    };

    let function_name = to_sq_string!(function_name.into());

    unsafe {
        (sqfunctions.sq_schedule_call_external)(contex.into(), function_name.as_ptr(), pop_function)
    }
}

pub fn push_sq_array<T>(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    element_append: Vec<T>,
) where
    T: FnOnce(*mut HSquirrelVM, &SquirrelFunctionsUnwraped),
{
    unsafe { (sqfunctions.sq_newarray)(sqvm, 0) }

    for e in element_append.into_iter() {
        e.call_once((sqvm, sqfunctions));
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
    unsafe { (sqfunctions.sq_pushvector)(sqvm, vector.into()) };
}

unsafe extern "C" fn __pop_function(_: *mut HSquirrelVM) -> i32 {
    sq_return_null!()
}
