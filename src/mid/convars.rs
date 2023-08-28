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

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConVarClasses {
    pub convar_vtable: *mut c_void,
    pub convar_register: ConVarRegisterType,
    pub iconvar_vtable: *mut ConCommandBase,
    pub convar_malloc: ConVarMallocType,
}

impl ConVarClasses {
    pub(crate) unsafe fn new(raw: &PluginEngineData) -> Self {
        let convar_malloc: ConVarMallocType = mem::transmute(raw.conVarMalloc);
        let iconvar_vtable = mem::transmute(raw.IConVar_Vtable);
        let convar_register: ConVarRegisterType = mem::transmute(raw.conVarRegister);
        Self {
            convar_vtable: raw.ConVar_Vtable,
            iconvar_vtable,
            convar_register,
            convar_malloc,
        }
    }
}

pub fn find_convar_with_cvar(name: &str, cvar: &RawCVar) -> Option<&'static mut ConVar> {
    let name = to_c_string!(name);
    unsafe { cvar.find_convar(name.as_ptr()).as_mut() }
}

pub fn find_convar(name: &str) -> Option<&'static mut ConVar> {
    find_convar_with_cvar(name, &get_engine_data()?.cvar)
}
