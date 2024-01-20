//! minimal abstraction for convars

use std::ffi::c_void;

use crate::errors::CVarQueryError;
use crate::{
    bindings::cvar::{
        command::ConCommandBase,
        convar::{ConVar, ConVarMallocType, ConVarRegisterType},
        RawCVar,
    },
    offset_functions,
};

use super::engine::get_engine_data;
use super::utils::try_cstring;

offset_functions! {
    CVAR_GLOBALS + CvarGlobals for WhichDll::Engine => {
        convar_vtable = *mut c_void where offset(0x415C20);
        convar_register = ConVarRegisterType where offset(0x417230);
        iconvar_vtable = *mut ConCommandBase where offset(0x67FD28);
        convar_malloc = ConVarMallocType where offset(0x415C20);
    }
}

/// finds a convar by name
///
/// # Example
/// ```no_run
/// # use rrplug::mid::engine::get_engine_data;
/// # use rrplug::mid::convars::find_convar_with_cvar;
/// # fn sub() -> Option<()> {
/// let convar = find_convar_with_cvar("spewlog_enable", &get_engine_data()?.get_cvar()).ok()?;
/// # Some(())
/// # }
/// ```
pub fn find_convar_with_cvar(
    name: &str,
    cvar: &RawCVar,
) -> Result<&'static mut ConVar, CVarQueryError> {
    let name = try_cstring(name)?;
    unsafe {
        cvar.find_convar(name.as_ptr())
            .as_mut()
            .ok_or(CVarQueryError::NotFound)
    }
}

/// finds a convar by name
///
/// # Example
/// ```no_run
/// # use rrplug::mid::convars::find_convar;
/// # fn sub() -> Option<()> {
/// let convar = find_convar("spewlog_enable").ok()?;
/// # Some(())
/// # }
/// ```
pub fn find_convar(name: &str) -> Result<&'static mut ConVar, CVarQueryError> {
    find_convar_with_cvar(
        name,
        get_engine_data()
            .ok_or(CVarQueryError::NoCVarInterface)?
            .cvar,
    )
}
