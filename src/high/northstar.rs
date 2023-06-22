//! wrappers for structs that are passed to the plugin

use log::SetLoggerError;
use std::ffi::CStr;
use std::fmt::Display;

use super::engine::EngineData;
use crate::{
    bindings::plugin_abi::{PluginInitFuncs, PluginNorthstarData},
    bindings::squirrelclasstypes::{SQFunction, ScriptContext},
    high::squirrel::FUNCTION_SQ_REGISTER,
    mid::northstar::CREATE_OBJECT_FUNC,
    nslog,
};

pub type FuncSQFuncInfo = fn() -> SQFuncInfo;

/// holds infomation about a sq function for it to be registered corretly
pub struct SQFuncInfo {
    pub cpp_func_name: &'static str,
    pub sq_func_name: &'static str,
    pub types: &'static str,
    pub return_type: &'static str,
    pub vm: ScriptVmType,
    pub function: SQFunction,
}

/// All the possible vm types titanfall 2 has
///
/// `UiClient` is used for function registration on both vms
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScriptVmType {
    Server,
    Client,
    Ui,
    UiClient,
}

// todo find a better way to do this my bain is just melting rn
impl ScriptVmType {
    pub fn is_right_vm(&self, other: &Self) -> bool {
        self == other
            || (self == &Self::UiClient && (other == &Self::Client || other == &Self::Ui))
            || (other == &Self::UiClient && (self == &Self::Client || self == &Self::Ui))
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
            ScriptContext::SERVER => Self::Server,
            ScriptContext::CLIENT => Self::Client,
            ScriptContext::UI => Self::Ui,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<ScriptContext> for ScriptVmType {
    fn into(self) -> ScriptContext {
        match self {
            ScriptVmType::Server => ScriptContext::SERVER,
            ScriptVmType::Client => ScriptContext::CLIENT,
            ScriptVmType::Ui => ScriptContext::UI,
            ScriptVmType::UiClient => {
                #[cfg(debug_assertions)]
                log::warn!("ScriptVmType::UiClient is interpreted as ScriptVmType::Client");
                ScriptContext::CLIENT
            }
        }
    }
}

/// All the engine load states northstar has
///
/// Each one loads a dll at it stage
///
/// `EngineFailed` is an Error
pub enum EngineLoadType {
    Engine(&'static EngineData),
    EngineFailed,
    Server,
    Client,
}

/// Provides Usefull Initilization infomation
///
/// Alought I would only count sqfunction registration as usefull
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
        let plugin_northstar_data = *plugin_northstar_data;

        CREATE_OBJECT_FUNC
            .set(plugin_init_funcs.createObject)
            .unwrap_or(log::error!("failed to set CREATE_OBJECT_FUNC"));

        Self {
            plugin_init_funcs,
            plugin_northstar_data,
        }
    }

    /// logging is already initialized in the entry marco by default
    #[doc(hidden)]
    pub fn try_init_logger(&self) -> Result<(), SetLoggerError> {
        nslog::try_init(
            self.plugin_init_funcs.logger,
            self.plugin_northstar_data.pluginHandle,
        )
    }

    /// logging is already initialized in the entry marco by default\
    #[doc(hidden)]
    pub fn init_logger(&self) {
        self.try_init_logger().unwrap();
    }

    /// returns the current northsar version
    pub fn get_northstar_version(&self) -> String {
        unsafe {
            CStr::from_ptr(self.plugin_northstar_data.version)
                .to_string_lossy()
                .into_owned()
        }
    }

    /// returns the plugin id
    ///
    /// only used for login which is handled by rrplug
    pub fn get_plugin_handle(&self) -> i32 {
        self.plugin_northstar_data.pluginHandle
    }

    /// Adds a sqfunction to the registration list
    ///
    /// The sqfunction will be registered when its vm is loaded
    pub fn register_sq_functions(&self, get_info_func: FuncSQFuncInfo) {
        FUNCTION_SQ_REGISTER.lock().push(get_info_func);
    }
}
