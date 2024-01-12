//! prelude for rrplug

pub use crate::{
    bindings::{squirreldatatypes::HSquirrelVM, unwraped::SquirrelFunctionsUnwraped},
    entry,
    high::{
        concommands::CCommandResult,
        convars::{ConVarRegister, ConVarStruct},
        engine::EngineData,
        northstar::ScriptVmType,
        squirrel::{register_sq_functions, CSquirrelVMHandle},
        vector::Vector3,
    },
    mid::{
        engine::{DLLPointer, WhichDll},
        squirrel::SQFUNCTIONS,
    },
    plugin::Plugin,
};
pub use log;

// consider adding more stuff ^

/// puts a thread on sleep in milliseconds
pub fn wait(milliseconds: u64) {
    std::thread::sleep(std::time::Duration::from_millis(milliseconds))
}
