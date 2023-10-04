//! wrappers for structs that are passed to the plugin

use log::SetLoggerError;
use std::ffi::CStr;
use std::fmt::Display;

use crate::{
    bindings::plugin_abi::{PluginInitFuncs, PluginNorthstarData},
    bindings::squirrelclasstypes::{SQFunction, ScriptContext},
    high::squirrel::FUNCTION_SQ_REGISTER,
    mid::northstar::CREATE_OBJECT_FUNC,
    nslog,
};

/// function type which is used in `register_sq_functions` to get [`SQFuncInfo`]
pub type FuncSQFuncInfo = fn() -> SQFuncInfo;

/// holds infomation about a sq function for it to be registered corretly
///
/// it creates a native closure btw but sqfunction is also a valid name for it.
/// sqfunction is used in a lot of places with diffrent meanings `¯\_(ツ)_/¯`
#[derive(Debug, PartialEq, Eq)]
pub struct SQFuncInfo {
    /// the name used in source code
    pub cpp_func_name: &'static str,
    /// name of the defined
    pub sq_func_name: &'static str,
    /// the arguments of the function in squirrel form
    ///
    /// # Example
    /// ```
    /// let types = "string name, int id";
    /// ```
    pub types: String,
    /// the return value of the function in squirrel form
    pub return_type: String,
    /// the which vm should be used to register the function on
    pub vm: ScriptVmType,
    /// the actual function pointer
    pub function: SQFunction,
}

/// All the possible vm types titanfall 2 has
///
/// `UiClient` is used for function registration on both vms
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScriptVmType {
    /// server vm
    Server,
    /// client vm
    Client,
    /// ui vm
    Ui,
    /// ui and client used in sqfunction registration
    UiClient,
}

// todo find a better way to do this my brain is just melting rn
impl ScriptVmType {
    /// checks if the other [`ScriptVmType`] matches the current one
    ///
    /// `UiClient` is equal to `Ui` or `Client`
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

/// Provides Usefull Initilization infomation
///
/// Alought I would only count sqfunction registration as usefull
pub struct PluginData {
    plugin_init_funcs: PluginInitFuncs,
    plugin_northstar_data: PluginNorthstarData,
}

impl PluginData {
    /// shouldn't be used outside of [`crate::entry`]
    ///
    /// # Safety
    ///
    /// expects the inputed ptrs to be valid
    #[doc(hidden)]
    pub unsafe fn new(
        plugin_init_funcs: *const PluginInitFuncs,
        plugin_northstar_data: *const PluginNorthstarData,
    ) -> Self {
        let plugin_init_funcs = unsafe { *plugin_init_funcs };
        let plugin_northstar_data = unsafe { *plugin_northstar_data };

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
        FUNCTION_SQ_REGISTER.lock().push(get_info_func());
    }
}
