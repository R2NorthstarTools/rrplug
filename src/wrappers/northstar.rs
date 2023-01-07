//! wrappers for structs that are passed to the plugin

use log::SetLoggerError;
use once_cell::sync::OnceCell;
use std::fmt::Display;

use super::engine::EngineData;
use super::errors::RegisterError;
use super::squrriel::FUNCTION_SQ_REGISTER;
use crate::bindings::plugin_abi::{CreateObjectFunc, PluginInitFuncs, PluginNorthstarData};
use crate::bindings::squirrelclasstypes::{SQFunction, ScriptContext};
use crate::nslog;

pub static CREATE_OBJECT_FUNC: OnceCell<CreateObjectFunc> = OnceCell::new();

// cpp name, sq name, types, return, func
pub type FuncSQFuncInfo = fn() -> SQFuncInfo;
pub type SQFuncInfo = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    ScriptVmType,
    SQFunction,
);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScriptVmType {
    Server,
    Client,
    Ui,
    UiClient,
    UiServer,
    ClientServer,
    All,
}

// todo find a better way to do this my bain is just melting rn
impl ScriptVmType {
    pub fn is_right_vm(&self, other: &Self) -> bool {
        self == other
            || (self == &Self::All || other == &Self::All)
            || (self == &Self::UiClient && (other == &Self::Client || other == &Self::Ui))
            || (other == &Self::UiClient && (self == &Self::Client || self == &Self::Ui))
            || (other == &Self::UiServer && (self == &Self::Server || self == &Self::Ui))
            || (self == &Self::UiServer && (other == &Self::Server || other == &Self::Ui))
            || (self == &Self::ClientServer && (other == &Self::Server || other == &Self::Client))
    }
}

impl Display for ScriptVmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{self:?}").to_uppercase();
        write!(f, "{s}")
    }
}

impl From<ScriptContext> for ScriptVmType {
    fn from(i: ScriptContext) -> Self {
        match i {
            0 => Self::Server,
            1 => Self::Client,
            2 => Self::Ui,
            _ => todo!(),
        }
    }
}

pub enum EngineLoadType {
    Engine(&'static EngineData),
    EngineFailed,
    Server,
    Client,
}

pub struct PluginData {
    plugin_init_funcs: PluginInitFuncs,
    plugin_northstar_data: PluginNorthstarData,
}

impl PluginData {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new(
        plugin_init_funcs: *const PluginInitFuncs,
        plugin_northstar_data: *const PluginNorthstarData,
    ) -> Self {
        let plugin_init_funcs = *plugin_init_funcs;

        CREATE_OBJECT_FUNC
            .set(plugin_init_funcs.createObject)
            .unwrap_or(log::error!("failed to set CREATE_OBJECT_FUNC"));

        Self {
            plugin_init_funcs,
            plugin_northstar_data: *plugin_northstar_data,
        }
    }

    /// logging is already initialized in the entry marco by default
    pub fn try_init_logger(&self) -> Result<(), SetLoggerError> {
        nslog::try_init(
            self.plugin_init_funcs.logger,
            self.plugin_northstar_data.pluginHandle,
        )
    }

    /// logging is already initialized in the entry marco by default
    pub fn init_logger(&self) {
        self.try_init_logger().unwrap();
    }

    pub fn get_northstar_version(&self) -> i8 {
        unsafe { *self.plugin_northstar_data.version }
    }

    pub fn get_plugin_handle(&self) -> i32 {
        self.plugin_northstar_data.pluginHandle
    }

    pub fn register_sq_functions(
        &self,
        get_info_func: FuncSQFuncInfo,
    ) -> Result<(), RegisterError> {
        match unsafe { FUNCTION_SQ_REGISTER.try_lock() } {
            Ok(mut sq_function_vec) => {
                sq_function_vec.push(get_info_func);
                Ok(())
            }
            Err(_) => Err(RegisterError::LockedSqFunctionVec),
        }
    }
}
