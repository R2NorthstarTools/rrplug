//! engine related abstractions and functions

use parking_lot::Mutex;

#[cfg(doc)]
use crate::high::convars::ConVarStruct;
use crate::{
    bindings::{
        cvar::{
            command::{CCommand, ConCommand},
            RawCVar,
        },
        plugin_abi::PluginEngineData,
    },
    errors::RegisterError,
    mid::{concommands::RegisterConCommands, convars::ConVarClasses, engine::PluginLoadDLL},
};

/// internal vec to not call on_dll_load
#[doc(hidden)]
pub static CALLED_DLLS: Mutex<Vec<PluginLoadDLL>> = Mutex::new(Vec::new());

/// Use this struct to register convars and concommands
///
/// only usefull when the convars or concommands features are enabled
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EngineData {
    pub(crate) concommands: RegisterConCommands,
    pub(crate) convar: ConVarClasses,
    pub(crate) low: PluginEngineData,
    pub(crate) cvar: RawCVar,
}

unsafe impl Send for EngineData {}
unsafe impl Sync for EngineData {}

// don't forget about CVar class

impl EngineData {
    /// # Safety
    ///
    /// hoping that the void ptrs point to the right stuff
    pub unsafe fn new(raw: &PluginEngineData) -> Self {
        unsafe {
            Self {
                concommands: RegisterConCommands::new(raw.ConCommandConstructor),
                convar: ConVarClasses::new(raw),
                cvar: RawCVar::from(raw.g_pCVar.cast_const()),
                low: *raw,
            }
        }
    }

    /// registers a command
    ///
    /// returns a pointer to [`ConCommand`] which is unsafe to access and has a static lifetime
    ///
    ///  # Example
    /// ```no_run
    /// # use rrplug::mid::engine::get_engine_data;
    /// # use rrplug::errors::RegisterError;
    /// # use rrplug::prelude::*;
    /// # let engine = get_engine_data().unwrap();
    /// engine.register_concommand("cool_command", cool_command, "this is cool_command", 0).expect("failed to register cool_command");
    ///
    /// #[rrplug::concommand]
    /// fn cool_command() {
    ///     println!("cool_command");
    /// }
    /// ```
    pub fn register_concommand(
        &self,
        name: impl Into<String>,
        callback: unsafe extern "C" fn(arg1: *const CCommand),
        help_string: impl Into<String>,
        flags: i32,
    ) -> Result<*mut ConCommand, RegisterError> {
        let name = name.into();
        log::info!("Registering ConCommand {}", name);

        self.concommands
            .mid_register_concommand(name, callback, help_string.into(), flags)
    }

    /// registers a convar without any complex steps and without giving back the convar pointer
    ///
    /// it's better to use [`ConVarStruct`] instead since you usually would want to get values out of the convar
    pub fn register_convar(
        &self,
        name: impl Into<String>,
        default_value: impl Into<String>,
        help_string: &'static str,
        flags: i32,
    ) -> Result<(), RegisterError> {
        use super::convars::{ConVarRegister, ConVarStruct};

        let mut convar = ConVarStruct::try_new().ok_or(RegisterError::NoneResult)?;
        let register_info = ConVarRegister::new(name, default_value, flags, help_string);
        convar.private_register(register_info, self)
    }
}
