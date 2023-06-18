//! lowest access stuff to engine

use crate::{bindings::plugin_abi::PluginEngineData, high::engine::EngineData};

impl EngineData {
    pub fn get_raw_ptrs(&self) -> &PluginEngineData {
        &self.low
    }
}
