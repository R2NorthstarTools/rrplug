use std::sync::Mutex;

use super::northstar::FuncSQFuncInfo;
use crate::bindings::unwraped::SquirrelFunctionsUnwraped;

pub static mut FUNCTION_SQ_REGISTER: Mutex<Vec<FuncSQFuncInfo>> = Mutex::new(Vec::new());
pub static mut SQFUNCTIONS: SqFunctions = SqFunctions {
    client: None,
    server: None,
};

pub struct SqFunctions {
    pub client: Option<SquirrelFunctionsUnwraped>,
    pub server: Option<SquirrelFunctionsUnwraped>,
}
