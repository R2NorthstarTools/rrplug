//! prelude for rrplug

// TODO: add whole modules instead of normal stuff

pub use crate::{
    bindings::{
        plugin_abi::PluginContext, squirrelclasstypes::ScriptContext,
        squirreldatatypes::HSquirrelVM, squirrelfunctions::SquirrelFunctions,
    },
    entry,
    high::{
        self,
        engine::{
            concommands::CCommandResult,
            convars::{ConVarRegister, ConVarStruct},
            EngineData, EngineGlobal, EngineToken,
        },
        northstar::PluginInfo,
        squirrel::{register_sq_functions, CSquirrelVMHandle},
        vector::Vector3,
    },
    interfaces::{external::SourceInterface, interface::AsInterface},
    mid::{
        self,
        engine::{DLLPointer, WhichDll},
        reloading,
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
