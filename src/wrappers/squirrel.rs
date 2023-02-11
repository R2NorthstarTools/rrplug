#![allow(clippy::not_unsafe_ptr_arg_deref)]

//! squirrel vm related function and statics

use once_cell::sync::OnceCell;
use std::sync::Mutex;

use super::{
    errors::CallError,
    northstar::{FuncSQFuncInfo, ScriptVmType},
    vector::Vector3,
};
use crate::{
    bindings::{
        squirrelclasstypes::SQFunction,
        squirreldatatypes::{HSquirrelVM, SQObject},
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

/// "safely" calls any function defined on the sqvm
///
/// they would only run when the sqvm is valid
pub fn async_call_sq_function(
    context: ScriptVmType,
    function_name: impl Into<String>,
    pop_function: Option<SQFunction>,
) {
    let sqfunctions = match context {
        ScriptVmType::Server => SQFUNCTIONS.server.wait(),
        _ => SQFUNCTIONS.client.wait(),
    };

    let pop_function = match pop_function {
        Some(callback) => callback,
        None => __pop_function,
    };

    let function_name = to_sq_string!(function_name.into());

    unsafe {
        (sqfunctions.sq_schedule_call_external)(
            context.into(),
            function_name.as_ptr(),
            pop_function,
        )
    }
}

/// "safely" calls any function defined on the sqvm
///
/// this should only be called on the tf2 thread aka when concommands, convars, sqfunctions, runframe
/// 
/// this only allows calls without args use the marco instead if you want args
pub fn call_sq_function<T>(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    function_name: impl Into<String>,
) -> Result<(), CallError>
{
    let obj = Box::new(std::mem::MaybeUninit::<SQObject>::zeroed());
    let ptr = Box::leak(obj).as_mut_ptr();

    let function_name = to_sq_string!(function_name.into());

    let result = unsafe {
        (sqfunctions.sq_getfunction)(sqvm, function_name.as_ptr(), ptr, std::ptr::null())
    };

    if result != 0 {
        Err(CallError::FunctionNotFound(function_name.to_string_lossy().into())) // totaly safe :clueless:
    } else {
        log::info!("got function");
        unsafe {
            (sqfunctions.sq_pushobject)(sqvm, ptr);
            (sqfunctions.sq_pushroottable)(sqvm);

            let result = if (sqfunctions.sq_call)(sqvm, 1, false as u32, false as u32) == -1 {
                Err(CallError::FunctionFailedToExecute)
            } else {
                Ok(())
            };

            _ = *ptr; // drop?

            result
        }
    }
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
    unsafe { (sqfunctions.sq_pushvector)(sqvm, vector.into()) };
}

unsafe extern "C" fn __pop_function(_: *mut HSquirrelVM) -> i32 {
    sq_return_null!()
}

pub trait PushToSquirrelVm {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped);
}

impl PushToSquirrelVm for String {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        push_sq_string(sqvm, sqfunctions, self)
    }
}

impl PushToSquirrelVm for i32 {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        push_sq_int(sqvm, sqfunctions, self)
    }
}

impl PushToSquirrelVm for f32 {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        push_sq_float(sqvm, sqfunctions, self)
    }
}

impl PushToSquirrelVm for bool {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        push_sq_bool(sqvm, sqfunctions, self)
    }
}

impl PushToSquirrelVm for Vector3 {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        push_sq_vector(sqvm, sqfunctions, self)
    }
}

impl PushToSquirrelVm for Box<dyn Iterator<Item = String>> {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        unsafe { (sqfunctions.sq_newarray)(sqvm, 0) }

        for e in self {
            e.push_to_sqvm(sqvm, sqfunctions);
            unsafe { (sqfunctions.sq_arrayappend)(sqvm, -2) };
        }
    }
}

impl PushToSquirrelVm for Box<dyn Iterator<Item = i32>> {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        unsafe { (sqfunctions.sq_newarray)(sqvm, 0) }

        for e in self {
            e.push_to_sqvm(sqvm, sqfunctions);
            unsafe { (sqfunctions.sq_arrayappend)(sqvm, -2) };
        }
    }
}

impl PushToSquirrelVm for Box<dyn Iterator<Item = f32>> {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        unsafe { (sqfunctions.sq_newarray)(sqvm, 0) }

        for e in self {
            e.push_to_sqvm(sqvm, sqfunctions);
            unsafe { (sqfunctions.sq_arrayappend)(sqvm, -2) };
        }
    }
}

impl PushToSquirrelVm for Box<dyn Iterator<Item = bool>> {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        unsafe { (sqfunctions.sq_newarray)(sqvm, 0) }

        for e in self {
            e.push_to_sqvm(sqvm, sqfunctions);
            unsafe { (sqfunctions.sq_arrayappend)(sqvm, -2) };
        }
    }
}

impl PushToSquirrelVm for Box<dyn Iterator<Item = Vector3>> {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        unsafe { (sqfunctions.sq_newarray)(sqvm, 0) }

        for e in self {
            e.push_to_sqvm(sqvm, sqfunctions);
            unsafe { (sqfunctions.sq_arrayappend)(sqvm, -2) };
        }
    }
}
