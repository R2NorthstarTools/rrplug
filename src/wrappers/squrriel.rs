use std::sync::Mutex;

use crate::bindings::{plugin_abi::SquirrelFunctions, squirrelclasstypes::SQFuncRegistration};

pub(crate) static mut FUNCTION_SQ_REGISTER: Mutex<Vec<SQFuncRegistration>> = Mutex::new(Vec::new());
pub(crate) static mut SQFUNCTIONS: Mutex<SqFunctions> = Mutex::new(SqFunctions{client: None, server: None});
// pub(crate) static mut SQFUNCTIONS: Mutex<SqFunctions> = Mutex::new(SqFunctions{client: None, server: None});

pub(crate) struct SqFunctions {
    pub(crate) client: Option<&'static SquirrelFunctions>,
    pub(crate) server: Option<&'static SquirrelFunctions>
}