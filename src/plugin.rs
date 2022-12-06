use crate::wrappers::northstar::{PluginData};

pub trait Plugin {
    fn new() -> Self;

    fn initialize(&mut self, plugin_data: &PluginData);  

    fn main(&self) {
        super::prelude::wait_forever();
    }
}
