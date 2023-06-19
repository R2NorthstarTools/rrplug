//! engine related stuff with minimal abstractions
use once_cell::sync::OnceCell;

use crate::high::engine::EngineData;

use super::{concommands::RegisterConCommands, convars::ConVarClasses};

pub static mut ENGINE_DATA: OnceCell<EngineData> = OnceCell::new();

impl EngineData {
    pub fn get_convar_ptrs(&self) -> &ConVarClasses {
        &self.convar
    }
}

impl EngineData {
    pub fn get_concommand_func(&self) -> &RegisterConCommands {
        &self.concommands
    }
}

pub fn get_engine_data() -> Option<&'static EngineData> {
    unsafe { ENGINE_DATA.get() }
}
