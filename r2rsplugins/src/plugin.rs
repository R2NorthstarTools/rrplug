use crate::prelude::ExternalPluginData;

pub trait Plugin {
    fn new() -> Self;

    fn initialize(&mut self, external_plugin_data: ExternalPluginData); // 

    fn main(&self);
}
