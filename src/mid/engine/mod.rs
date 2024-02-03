//! engine related stuff with minimal abstractions
use once_cell::sync::OnceCell;
use std::ffi::c_void;

use self::{concommands::RegisterConCommands, convars::CvarGlobals};
use crate::{bindings::cvar::RawCVar, high::engine::EngineData};

pub mod concommands;
pub mod convars;

/// used to create to ConVars and ConComands
///
/// also allows access to [`RawCVar`] if you need it but beter options are available in [`crate::high::convars`]/[`crate::mid::convars`] and [`crate::mid::concommands`]
pub static ENGINE_DATA: OnceCell<EngineData> = OnceCell::new();

impl EngineData {
    /// just returns everything in it's raw form

    /// returns the functions and statics needed to register ConVars
    ///
    /// if you need to use it for some reason you can refer to rrplug code in [`crate::high::convars`]
    pub const fn get_convar_ptrs(&self) -> &CvarGlobals {
        self.convar
    }

    /// returns the function to register concommands
    ///
    /// if you need to use it for some reason you can refer to rrplug code in [`crate::mid::concommands`]
    pub const fn get_concommand_func(&self) -> &RegisterConCommands {
        self.concommands
    }

    /// # major notes about [`RawCVar`]
    ///
    /// [`RawCVar`] has many many unsafe functions
    /// but the `iterator` function should not be invoked
    /// since the returned `iterator` cannot be dropped by rust's default `allocator` which may produce ub.
    pub const fn get_cvar(&self) -> &RawCVar {
        self.cvar
    }
}

/// returns engine data [`EngineData`]
///
/// refer to [`ENGINE_DATA`] for more docs:tm:
pub fn get_engine_data() -> Option<&'static EngineData> {
    ENGINE_DATA.get()
}

/// specifies what is the current dll without the engine functions payload
///
/// only gets received once because of a internal count of called dlls
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WhichDll<'a> {
    /// engine.dll
    Engine,
    /// client.dll
    Client,
    /// server.dll
    Server,
    /// any other loaded dll
    Other(&'a str),
}

/// only holding the current dll's pointer
pub struct DLLPointer<'a> {
    dll: WhichDll<'a>,
    ptr: *const c_void,
}

impl<'a> DLLPointer<'a> {
    /// not for public use, made public for [`crate::entry`] macro
    #[doc(hidden)]
    pub fn new(dll_str: &'a str, ptr: *const c_void) -> DLLPointer<'a> {
        let which_dll = match dll_str {
            "engine.dll" => WhichDll::Engine,
            "client.dll" => WhichDll::Client,
            "server.dll" => WhichDll::Server,
            dll => WhichDll::Other(dll),
        };

        Self {
            dll: which_dll,
            ptr,
        }
    }

    /// return the address of the dll
    pub const fn get_dll_ptr(&self) -> *const c_void {
        self.ptr
    }

    /// return the name of the dll
    pub const fn which_dll(&self) -> &WhichDll {
        &self.dll
    }

    /// adds a [`isize`] to the base dll address
    ///
    /// # Safety
    ///
    /// a really bad idea if you have a bad offset or if you don't know what you are doing.
    /// this is mainly here to give access to the engines functions without having the crate be the provider of them.
    pub const unsafe fn offset(&self, offset: isize) -> *const c_void {
        unsafe { self.ptr.offset(offset) }
    }
}
