use crate::{wrappers::northstar::PluginData, bindings::plugin_abi::PluginEngineData};

pub trait Plugin {
    fn new() -> Self;

    fn initialize(&mut self, plugin_data: &PluginData);

    fn main(&self);

    fn on_engine_load(&self, _data: PluginEngineData) {}
}
