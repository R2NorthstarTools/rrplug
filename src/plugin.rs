//! the plugin trait module
//!
//! the plugin system will look for a exported function to pass stuff to
//!
//! this exported function and others are created by the `entry` macro
//!
//! it takes your plugin struct and calls specific function for each event
use std::any::Any;

use crate::{
    high::{
        engine::{EngineData, EngineToken},
        northstar::PluginInfo,
        squirrel::CSquirrelVMHandle,
    },
    mid::{engine::DLLPointer, reloading},
};

/// Trait for defining the callbacks and entry point of the plugin
///
/// it is unsafe to run any titanfall engine functions on it
pub trait Plugin: Any + Sync {
    /// info about the plugin
    const PLUGIN_INFO: PluginInfo;

    /// init function
    ///
    /// TODO: redo docs for this about registering sq functions
    fn new(reloaded: bool) -> Self;

    /// called when a dll is loaded with winapi functions by the game (full paths are not provided)
    ///
    /// only calls once for each unique dll
    fn on_dll_load(
        &self,
        _engine_data: Option<&EngineData>,
        _dll_ptr: &DLLPointer,
        _engine_token: EngineToken,
    ) {
    }

    /// called when a sqvm is created
    ///
    /// can be used to store the sqvm for use on the titanfall 2 thread but it is unsafe since the sqvm can be invalided at any point
    fn on_sqvm_created(&self, _sqvm_handle: &CSquirrelVMHandle, _engine_token: EngineToken) {}

    /// called when a sqvm is dropped
    ///
    /// the sqvm will be invalid after this call
    fn on_sqvm_destroyed(&self, _sqvm_handle: &CSquirrelVMHandle, _engine_token: EngineToken) {}

    /// called on the module dll load bassically before new
    ///
    /// using this may be a bad idea since rrplug may not be done initializing everything.
    fn on_module_load() {}

    /// called when all plugins are loaded so this a good place to get interfaces from them
    fn plugins_loaded(&self, _engine_token: EngineToken) {} // probably not engine thread but the engine thread doesn't even exist at his point XD

    /// called on each engine frame (runs on the titanfall 2 thread ofc lol)
    fn runframe(&self, _engine_token: EngineToken) {}

    /// called before the plugin is unloaded
    ///
    /// the plugin can deny to unload itself to prevent ub if reloaded
    fn on_reload_request(&self) -> reloading::ReloadResponse {
        reloading::ReloadResponse::deny_reload()
    }
}
