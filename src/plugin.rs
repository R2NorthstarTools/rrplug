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
        engine::EngineData,
        northstar::{PluginData, ScriptVmType},
        squirrel::CSquirrelVMHandle,
    },
    mid::engine::DLLPointer,
};

/// Trait for defining the callbacks and entry point of the plugin
///
/// also provides a thread to run code on (the main function)
///
/// it is unsafe to run any titanfall engine functions on it
pub trait Plugin: Any + Sync {
    /// init function
    ///
    /// [`PluginData`] can be used to register sqfunctions and get the northstar version
    fn new(plugin_data: &PluginData) -> Self;

    /// provided separate thread
    fn main(&self);

    /// called when a dll is loaded with winapi functions by the game (full paths are not provided)
    ///
    /// only calls once for each unique dll
    fn on_dll_load(&self, _engine_data: Option<&EngineData>, _dll_ptr: &DLLPointer) {}

    /// called when a sqvm is created
    ///
    /// can be used to store the sqvm for use on the titanfall 2 thread but it is unsafe since the sqvm can be invalided at any point
    fn on_sqvm_created(&self, _sqvm_handle: &CSquirrelVMHandle) {}

    /// called when a sqvm is dropped
    fn on_sqvm_destroyed(&self, _context: ScriptVmType) {}

    /// called on each engine frame (runs on the titanfall 2 thread ofc lol)
    fn runframe(&self) {}
}
