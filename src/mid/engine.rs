//! engine related stuff with minimal abstractions
use once_cell::sync::OnceCell;

use crate::{bindings::cvar::RawCVar, high::engine::EngineData};

use super::{concommands::RegisterConCommands, convars::ConVarClasses};

pub static ENGINE_DATA: OnceCell<EngineData> = OnceCell::new();

impl EngineData {
    pub fn get_convar_ptrs(&self) -> &ConVarClasses {
        &self.convar
    }

    pub fn get_concommand_func(&self) -> &RegisterConCommands {
        &self.concommands
    }

    pub fn get_cvar(&self) -> &RawCVar {
        &self.cvar
    }
}

pub fn get_engine_data() -> Option<&'static EngineData> {
    ENGINE_DATA.get()
}
