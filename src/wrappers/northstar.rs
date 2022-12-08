//! wrappers for structs that are passed to the plugin

use log::SetLoggerError;
use std::fmt::Display;
use std::sync::Mutex;

use super::engine::EngineCallbacks;
use super::squrriel::SquirrelBuilder;
use super::squrrielvm::SquirrelVMCallbacks;
use crate::bindings::plugin_abi::{PluginEngineData, PluginInitFuncs, PluginNorthstarData};
use crate::bindings::squirrelclasstypes::{
    ScriptContext_CLIENT, ScriptContext_SERVER, ScriptContext_UI,
};
use crate::nslog;

#[derive(Debug, Clone, Copy,PartialEq)]
pub enum ScriptVmType {
    Server,
    Client,
    Ui,
}

impl Display for ScriptVmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({self:?})")
    }
}

impl ScriptVmType {
    pub fn to_int(&self) -> i32 {
        match self {
            ScriptVmType::Server => ScriptContext_SERVER,
            ScriptVmType::Client => ScriptContext_CLIENT,
            ScriptVmType::Ui => ScriptContext_UI,
        }
    }
}

pub struct PluginData {
    plugin_init_funcs: PluginInitFuncs,
    plugin_northstar_data: PluginNorthstarData,
    engine_callbacks: &'static mut Option<Mutex<EngineCallbacks>>,
    sqvm_callbacks: &'static mut Option<Mutex<SquirrelVMCallbacks>>,
}

impl PluginData {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new(
        plugin_init_funcs: *const PluginInitFuncs,
        plugin_northstar_data: *const PluginNorthstarData,
        engine_callbacks: &'static mut Option<Mutex<EngineCallbacks>>,
        sqvm_callbacks: &'static mut Option<Mutex<SquirrelVMCallbacks>>,
    ) -> Self {
        Self {
            plugin_init_funcs: *plugin_init_funcs,
            plugin_northstar_data: *plugin_northstar_data,
            engine_callbacks,
            sqvm_callbacks,
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
        let mut engine_callbacks = match self.engine_callbacks.as_ref().unwrap().try_lock() {
            Ok(engine_callbacks) => engine_callbacks,
            Err(err) => {
                log::error!("failed to add engine callbacks because of {err:?}");
                return;
            }
        };
        engine_callbacks.add_callback(callback);
    }

    pub fn add_sqvm_created_callback(
        &self,
        sqvm_type: ScriptVmType,
        callback: Box<dyn Fn(SquirrelBuilder)>,
    ) {
        let mut sqvm_callbacks = match self.sqvm_callbacks.as_ref().unwrap().try_lock() {
            Ok(sqvm_callbacks) => sqvm_callbacks,
            Err(err) => {
                log::error!("failed to add engine callbacks because of {err:?}");
                return;
            }
        };

        match sqvm_type {
            ScriptVmType::Server => sqvm_callbacks.add_callback_server(callback),
            ScriptVmType::Client => sqvm_callbacks.add_callback_client(callback),
            ScriptVmType::Ui => sqvm_callbacks.add_callback_ui(callback),
        }
    }

    pub fn add_sqvm_init_callback(
        &self,
        sqvm_type: ScriptVmType,
        callback: Box<dyn Fn(SquirrelBuilder)>,
    ) {
        let mut sqvm_callbacks = match self.sqvm_callbacks.as_ref().unwrap().try_lock() {
            Ok(sqvm_callbacks) => sqvm_callbacks,
            Err(err) => {
                log::error!("failed to add engine callbacks because of {err:?}");
                return;
            }
        };

        match sqvm_type {
            ScriptVmType::Server => sqvm_callbacks.add_callback_server_init(callback),
            ScriptVmType::Client => sqvm_callbacks.add_callback_client_init(callback),
            ScriptVmType::Ui => {
                log::warn!("UI functions and Client functions are the same");
                sqvm_callbacks.add_callback_client_init(callback);
            }
        }
    }
}
