use std::sync::Mutex;

use super::northstar::FuncSQFuncInfo;
use crate::bindings::{unwraped::SquirrelFunctionsUnwraped, squirreldatatypes::CSquirrelVM};

pub static mut FUNCTION_SQ_REGISTER: Mutex<Vec<FuncSQFuncInfo>> = Mutex::new(Vec::new());
pub static mut SQFUNCTIONS: SqFunctions = SqFunctions {
    client: None,
    server: None,
};

pub struct SqFunctions {
    pub client: Option<SquirrelFunctionsUnwraped>,
    pub server: Option<SquirrelFunctionsUnwraped>,
}

/// yes unsafe
unsafe impl Sync for CSquirrelVM {
    
}