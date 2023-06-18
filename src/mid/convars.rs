use std::{ffi::c_void, mem};

use crate::bindings::{convar::{ConVarRegisterType, ConVarMallocType}, command::ConCommandBase, plugin_abi::PluginEngineData};

pub struct ConVarClasses {
    pub convar_vtable: *mut c_void,
    pub convar_register: ConVarRegisterType,
    pub iconvar_vtable: *mut ConCommandBase,
    pub convar_malloc: ConVarMallocType,
}

impl ConVarClasses {
    pub(crate) unsafe fn new(raw: &PluginEngineData) -> Self {
        let convar_malloc: ConVarMallocType =  mem::transmute(raw.conVarMalloc);
        let iconvar_vtable =  mem::transmute(raw.IConVar_Vtable);
        let convar_register: ConVarRegisterType =  mem::transmute(raw.conVarRegister);
        Self {
            convar_vtable: raw.ConVar_Vtable,
            iconvar_vtable,
            convar_register,
            convar_malloc,
        }
    }
}
