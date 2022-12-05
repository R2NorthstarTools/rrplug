//! wrappers for structs that are passed to the plugin

use std::sync::Mutex;
use log::SetLoggerError;

use crate::bindings::plugin_abi::{PluginInitFuncs, PluginNorthstarData, PluginEngineData};
use crate::nslog;
use super::engine::EngineCallbacks;

pub struct PluginData {
    plugin_init_funcs: PluginInitFuncs,
    plugin_northstar_data: PluginNorthstarData,
    engine_callbacks: &'static mut Option<Mutex<EngineCallbacks>>
}

impl PluginData {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new(
        plugin_init_funcs: *const PluginInitFuncs,
        plugin_northstar_data: *const PluginNorthstarData,
        engine_callbacks: &'static mut Option<Mutex<EngineCallbacks>>
    ) -> Self {
        Self {
            plugin_init_funcs: *plugin_init_funcs,
            plugin_northstar_data: *plugin_northstar_data,
            engine_callbacks
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

    pub fn add_engine_load_callback(&self, callback: Box<dyn Fn(PluginEngineData)>) -> Option<()>{
        let mut engine_callbacks = self.engine_callbacks.as_ref().unwrap().try_lock().ok()?;
        engine_callbacks.add_callback(callback);
        Some(())
    }
}