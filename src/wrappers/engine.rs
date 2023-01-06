#[cfg(feature = "concommand")]
use super::concommands::RegisterConCommands;
#[cfg(feature = "concommand")]
pub use crate::bindings::command::CCommand;
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
            concommands: unsafe { RegisterConCommands::new(raw.ConCommandConstructor ) },
            raw,
        }
    }
    
    /// this function will crash
    #[cfg(feature = "concommand")]
    pub fn register_concommand(
        &self,
        name: impl Into<String>,
        callback: extern "C" fn(arg1: *const CCommand),
        help_string: impl Into<String>,
        flags: i32,
    ) -> Result<(), super::errors::RegisterError> {
        let name = name.into();
        log::info!("Registering ConCommand {}", name);

        self.concommands
            .register_concommand(name, callback, help_string.into(), flags)
    }

    #[cfg(feature = "concommand")]
    pub fn register_conconvar(
        &self,
        name: impl Into<String>,
        callback: extern "C" fn(arg1: *const CCommand),
        help_string: impl Into<String>,
        flags: i32,
    ) -> Result<(), super::errors::RegisterError> {
        let name = name.into();
        log::info!("Registering ConCommand {}", name);

        self.concommands
            .register_concommand(name, callback, help_string.into(), flags)
    }
}
