use std::{any::Any, fmt::Debug};

use crate::wrappers::{
    northstar::{EngineLoadType, PluginData, ScriptVmType},
    presence::GamePresence,
    squirrel::CSquirrelVMHandle,
};

///! the plugin system will look for a exported function to pass stuff to
///!
///! this exported function and others are created by the `entry` macro
///!
///! it takes your plugin struct and calls specific function for each event

/// Trait for defining the callbacks and entry point of the plugin
///
/// also provides a thread to run code on (the main function)
///
/// it is unsafe to run any titanfall engine functions on it
pub trait Plugin: Any + Debug + Sync {
    type SaveType;

    fn new() -> Self;

    fn initialize(&mut self, plugin_data: &PluginData);

    fn main(&self);

    fn on_engine_load(&self, _engine: &EngineLoadType) {}

    fn on_sqvm_created(&self, _sqvm_handle: &CSquirrelVMHandle<Self::SaveType>) {}

    fn on_sqvm_destroyed(&self, _context: ScriptVmType) {}

    fn on_presence_updated(&self, _presence: &GamePresence) {}
}
