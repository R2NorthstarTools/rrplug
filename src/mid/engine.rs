//! engine related stuff with minimal abstractions
use std::ffi::c_void;

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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PluginLoadDLL {
    Engine(&'static EngineData),
    Client,
    Server,
    Other(String),
}

pub struct DLLPointer<'a> {
    dll: &'a PluginLoadDLL,
    ptr: *const c_void,
}

impl<'a> DLLPointer<'a> {
    /// not for public use, made public for [`crate::entry`] macro
    #[doc(hidden)]
    pub fn new(dll: &'a PluginLoadDLL, ptr: *const c_void) -> DLLPointer<'a> {
        Self { dll, ptr }
    }

    pub fn get_dll_ptr(&self) -> *const c_void {
        self.ptr
    }

    pub fn which_dll(&self) -> &PluginLoadDLL {
        self.dll
    }

    /// # Safety
    ///
    /// a really bad idea if you have a bad offset or if you don't know what you are doing.
    /// this is mainly here to give access to the engines functions without having the crate be the provider of them.
    pub unsafe fn offset(&self, offset: isize) -> *const c_void {
        self.ptr.offset(offset)
    }
}
