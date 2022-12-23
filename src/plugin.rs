use crate::{
    bindings::{squirreldatatypes::CSquirrelVM, plugin_abi::PluginGameStatePresence},
    wrappers::northstar::{EngineLoadType, PluginData, ScriptVmType},
};

pub trait Plugin {
    fn new() -> Self;

    fn initialize(&mut self, plugin_data: &PluginData);

    fn main(&self);

    fn on_engine_load(&self, _engine: EngineLoadType) {}
    
    /// maybe [`Pin<Option<&'static CSquirrelVM>>`] would be better for when a sqvm si destroyed hmm
    fn on_sqvm_created(&self, _context: ScriptVmType, _sqvm: &'static CSquirrelVM) {}

    fn on_sqvm_destroyed(&self, _context: ScriptVmType) {}

    fn on_presence_updated(&self, _presence: PluginGameStatePresence) {}
}
