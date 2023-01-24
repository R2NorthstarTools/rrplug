use std::{any::Any, fmt::Debug};

use crate::{
    bindings::{plugin_abi::PluginGameStatePresence, squirreldatatypes::CSquirrelVM},
    wrappers::northstar::{EngineLoadType, PluginData, ScriptVmType},
};

/// Trait for defining the callbacks and entry point of the plugin
///
/// also provides a thread to run code on (the main function)
///
/// it is unsafe to run any titanfall engine functions on it
pub trait Plugin: Any + Debug + Sync {
    fn new() -> Self;

    fn initialize(&mut self, plugin_data: &PluginData);

    fn main(&self);

    fn on_engine_load(&self, _engine: EngineLoadType) {}

    fn on_sqvm_created(&self, _context: ScriptVmType, _sqvm: &'static CSquirrelVM) {}

    fn on_sqvm_destroyed(&self, _context: ScriptVmType) {}

    fn on_presence_updated(&self, _presence: PluginGameStatePresence) {}
}
