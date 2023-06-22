pub use crate::entry;
pub use crate::plugin::Plugin;
pub use crate::{
    bindings::unwraped::SquirrelFunctionsUnwraped,
    high::{
        concommands::CCommandResult,
        northstar::{EngineLoadType, PluginData, ScriptVmType},
        squirrel::{CSquirrelVMHandle},
    },
    mid::squirrel::SQFUNCTIONS,
};
pub use log;

// consider adding more stuff ^

/// puts a thread on sleep in milliseconds
pub fn wait(milliseconds: u64) {
    std::thread::sleep(std::time::Duration::from_millis(milliseconds))
}
