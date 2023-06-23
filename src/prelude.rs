pub use crate::{
    bindings::{squirreldatatypes::HSquirrelVM,unwraped::SquirrelFunctionsUnwraped},
    high::{
        concommands::CCommandResult,
        northstar::{EngineLoadType, PluginData, ScriptVmType},
        squirrel::{CSquirrelVMHandle},
    },
    mid::squirrel::SQFUNCTIONS,
    plugin::Plugin,
    entry,
};
pub use log;

// consider adding more stuff ^

/// puts a thread on sleep in milliseconds
pub fn wait(milliseconds: u64) {
    std::thread::sleep(std::time::Duration::from_millis(milliseconds))
}
