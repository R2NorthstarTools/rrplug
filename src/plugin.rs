use crate::bindings::plugin_abi::{PluginInitFuncs,PluginNorthstarData};

pub trait Plugin {
    fn new() -> Self;

    fn initialize(&mut self, plugin_init_funcs: *const PluginInitFuncs, plugin_northstar_data: *const PluginNorthstarData); // 

    fn main(&self);
}
