//! wrappers for structs that are passed to the plugin

use log::SetLoggerError;
use once_cell::sync::Lazy;
use std::fmt::Display;
use std::sync::Mutex;

use super::engine::EngineCallbacks;
use super::errors::SqFunctionError;
use super::squrriel::FUNCTION_SQ_REGISTER;
use crate::bindings::plugin_abi::{PluginEngineData, PluginInitFuncs, PluginNorthstarData};
use crate::bindings::squirrelclasstypes::{
    SQFunction, ScriptContext_CLIENT, ScriptContext_SERVER, ScriptContext_UI,
};
use crate::nslog;

pub type SQFuncInfo = fn() -> (&'static str, &'static str, &'static str, SQFunction);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScriptVmType {
    Server,
    Client,
    Ui,
    UiClient,
}

impl Display for ScriptVmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({self:?})")
    }
}

impl ScriptVmType {
    pub fn to_int(&self) -> i32 {
        match self {
            Self::Server => ScriptContext_SERVER,
            Self::Client => ScriptContext_CLIENT,
            Self::Ui => ScriptContext_UI,
            Self::UiClient => ScriptContext_UI,
        }
    }
}

pub struct PluginData {
    plugin_init_funcs: PluginInitFuncs,
    plugin_northstar_data: PluginNorthstarData,
    engine_callbacks: &'static mut Lazy<Mutex<EngineCallbacks>>,
}

impl PluginData {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new(
        plugin_init_funcs: *const PluginInitFuncs,
        plugin_northstar_data: *const PluginNorthstarData,
        engine_callbacks: &'static mut Lazy<Mutex<EngineCallbacks>>,
    ) -> Self {
        Self {
            plugin_init_funcs: *plugin_init_funcs,
            plugin_northstar_data: *plugin_northstar_data,
            engine_callbacks,
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

    pub fn add_engine_load_callback(&self, callback: Box<dyn Fn(PluginEngineData)>) {
        let mut engine_callbacks = match self.engine_callbacks.try_lock() {
            Ok(engine_callbacks) => engine_callbacks,
            Err(err) => {
                log::error!("failed to add engine callbacks because of {err:?}");
                return;
            }
        };
        engine_callbacks.add_callback(callback);
    }

    pub fn register_sq_functions(&self, get_info_func: SQFuncInfo) -> Result<(), SqFunctionError> {
        match unsafe { FUNCTION_SQ_REGISTER.try_lock() } {
            Ok(mut sq_function_vec) => {
                sq_function_vec.push(get_info_func);
                Ok(())
            }
            Err(_) => Err(SqFunctionError::LockedSqFunctionVec),
        }
    }
}
