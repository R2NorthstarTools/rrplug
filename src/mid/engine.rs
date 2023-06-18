//! engine related stuff with minimal abstractions
use once_cell::sync::OnceCell;

use crate::{bindings::plugin_abi::PluginEngineData, high::engine::EngineData};

pub static mut ENGINE_DATA: OnceCell<EngineData> = OnceCell::new();

impl EngineData {
    pub fn get_convar_ptrs(&self) -> &PluginEngineData {
        &self.low
    }
}

pub fn get_engine_data() -> Option<&'static EngineData> {
    unsafe { ENGINE_DATA.get() }
}
