//! minimal abstraction for convars

use std::{ffi::c_void, mem};

use crate::{
    bindings::{
        cvar::{
            command::ConCommandBase,
            convar::{ConVar, ConVarMallocType, ConVarRegisterType},
            RawCVar,
        },
        plugin_abi::PluginEngineData,
    },
    to_c_string,
};

use super::engine::get_engine_data;

/// holds stuff required to register convar
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConVarClasses {
    /// convar vtable
    ///
    /// quite useful since no source allocator :(
    pub convar_vtable: *mut c_void,

    /// function to register convars
    pub convar_register: ConVarRegisterType,

    /// another vtable
    pub iconvar_vtable: *mut ConCommandBase,

    /// allocator for m_pMalloc in convars
    pub convar_malloc: ConVarMallocType,
}

impl ConVarClasses {
    pub(crate) unsafe fn new(raw: &PluginEngineData) -> Self {
        unsafe {
            let convar_malloc: ConVarMallocType = mem::transmute(raw.conVarMalloc);
            let iconvar_vtable = raw.IConVar_Vtable as *mut ConCommandBase;
            let convar_register: ConVarRegisterType = mem::transmute(raw.conVarRegister);

            Self {
                convar_vtable: raw.ConVar_Vtable,
                iconvar_vtable,
                convar_register,
                convar_malloc,
            }
        }
    }
}

/// finds a convar by name
///
/// # Example
/// ```no_run
/// # use rrplug::mid::engine::get_engine_data;
/// # use rrplug::mid::convars::find_convar_with_cvar;
/// # fn sub() -> Option<()> {
/// let convar = find_convar_with_cvar("spewlog_enable", &get_engine_data()?.get_cvar())?;
/// # Some(())
/// # }
/// ```
pub fn find_convar_with_cvar(name: &str, cvar: &RawCVar) -> Option<&'static mut ConVar> {
    let name = to_c_string!(name);
    unsafe { cvar.find_convar(name.as_ptr()).as_mut() }
}

/// finds a convar by name
///
/// # Example
/// ```no_run
/// # use rrplug::mid::convars::find_convar;
/// # fn sub() -> Option<()> {
/// let convar = find_convar("spewlog_enable")?;
/// # Some(())
/// # }
/// ```
pub fn find_convar(name: &str) -> Option<&'static mut ConVar> {
    find_convar_with_cvar(name, &get_engine_data()?.cvar)
}
