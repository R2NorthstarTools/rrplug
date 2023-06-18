use once_cell::sync::OnceCell;

use crate::bindings::unwraped::SquirrelFunctionsUnwraped;

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
