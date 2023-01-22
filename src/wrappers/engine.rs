#[cfg(feature = "concommand")]
pub use crate::bindings::command::CCommand;
use crate::bindings::plugin_abi::PluginEngineData;
#[cfg(any(feature = "concommand", feature = "convar"))]
use crate::wrappers::errors::RegisterError;
use once_cell::sync::OnceCell;

#[doc(hidden)]
pub static mut ENGINE_DATA: OnceCell<EngineData> = OnceCell::new();

pub fn get_engine_data() -> Option<&'static EngineData> {
    unsafe { ENGINE_DATA.get() }
}

pub struct EngineData {
    #[cfg(feature = "concommand")]
    concommands: super::concommands::RegisterConCommands,
    #[cfg(feature = "convar")]
    pub(crate) convar: super::convars::ConVarClasses,

    pub raw: PluginEngineData,
}

impl EngineData {
    pub fn new(raw: PluginEngineData) -> Self {
        Self {
            #[cfg(feature = "concommand")]
            concommands: unsafe {
                super::concommands::RegisterConCommands::new(raw.ConCommandConstructor)
            },
            #[cfg(feature = "convar")]
            convar: super::convars::ConVarClasses::new(&raw),
            raw,
        }
    }

    #[cfg(feature = "concommand")]
    pub fn register_concommand(
        &self,
        name: impl Into<String>,
        callback: extern "C" fn(arg1: *const CCommand),
        help_string: impl Into<String>,
        flags: i32,
    ) -> Result<(), RegisterError> {
        let name = name.into();
        log::info!("Registering ConCommand {}", name);

        self.concommands
            .register_concommand(name, callback, help_string.into(), flags)
    }

    #[cfg(feature = "convar")]
    pub fn register_convar(
        &self,
        name: impl Into<String>,
        default_value: impl Into<String>,
        help_string: impl Into<String>,
        flags: i32,
    ) -> Result<(), RegisterError> {
        use super::convars::{ConVarRegister, ConVarStruct};

        let convar = ConVarStruct::try_new().ok_or(RegisterError::NoneResult)?;
        let register_info = ConVarRegister::new(name, default_value, flags, help_string);
        convar.private_register(register_info, self)
    }
}
