use super::concommands::RegisterConCommands;
#[cfg(feature = "concommand")]
pub use crate::bindings::cxx_binds::__concommand::CCommand;
use crate::bindings::plugin_abi::PluginEngineData;

pub struct EngineData {
    #[cfg(feature = "concommand")]
    concommands: RegisterConCommands,
    pub raw: PluginEngineData,
}

impl EngineData {
    pub fn new(raw: PluginEngineData) -> Self {
        Self {
            #[cfg(feature = "concommand")]
            concommands: unsafe { RegisterConCommands::new(raw.conVarRegister) },
            raw,
        }
    }

    #[cfg(feature = "concommand")]
    pub fn register_concommand(
        &self,
        name: String,
        callback: extern "C" fn(arg1: &CCommand),
        help_string: String,
        flags: i32,
    ) {
        self.concommands
            .register_concommand(name, callback, help_string, flags)
    }
}
