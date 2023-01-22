//! squrriel vm related function and statics

use once_cell::sync::OnceCell;
use std::sync::Mutex;

use super::northstar::{FuncSQFuncInfo, ScriptVmType};
use crate::{
    bindings::{
        squirreldatatypes::HSquirrelVM,
        squirrelclasstypes::SQFunction,
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
pub fn call_sq_function(contex: ScriptVmType, function_name: impl Into<String>, pop_function: Option<SQFunction>) {
    let sqfunctions = match contex {
        ScriptVmType::Server => SQFUNCTIONS.server.wait(),
        _ => SQFUNCTIONS.client.wait(),
    };

    let pop_function = match pop_function {
        Some(callback) => callback,
        None => __pop_function,
    };

    let function_name = to_sq_string!(function_name.into());

    unsafe { (sqfunctions.sq_schedule_call_external)(contex.into(), function_name.as_ptr(), pop_function) }
}

unsafe extern "C" fn __pop_function(_: *mut HSquirrelVM) -> i32 {
    sq_return_null!()
}
