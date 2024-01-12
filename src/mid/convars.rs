//! minimal abstraction for convars

use std::ffi::c_void;

use crate::to_c_string;
use crate::{
    bindings::cvar::{
        command::ConCommandBase,
        convar::{ConVar, ConVarMallocType, ConVarRegisterType},
        RawCVar,
    },
    offset_functions,
};

use super::engine::get_engine_data;

offset_functions! {
    CVAR_GLOBALS + CvarGlobals for WhichDll::Engine => {
        convar_vtable = *mut c_void where offset(0x67FD28);
        convar_register = ConVarRegisterType where offset(0x67FD28);
        iconvar_vtable = *mut ConCommandBase where offset(0x67FD28);
        convar_malloc = ConVarMallocType where offset(0x67FD28);
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
