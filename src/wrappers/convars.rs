//! convar related abstractions

use std::{ffi::CStr, mem, os::raw::c_void, ptr::addr_of_mut};

use super::{
    engine::{get_engine_data, EngineData},
    errors::RegisterError,
    northstar::CREATE_OBJECT_FUNC,
};
use crate::{
    bindings::{
        command::ConCommandBase,
        convar::{
            ConVar, ConVarMallocType, ConVarRegisterType, FnChangeCallback_t, FCVAR_NEVER_AS_STRING,
        },
        plugin_abi::{ObjectType_CONVAR, PluginEngineData},
    },
    to_sq_string,
};

/// the state of the convar in all of its possible types
///
/// value should be valid most of the time
pub struct ConVarValues {
    pub value: Option<String>,
    pub value_float: f32,
    pub value_int: i32,
}

/// [`ConVarRegister`] is builder sturct for convars
///
/// consumed by [`ConVarStruct`]`::register`
pub struct ConVarRegister {
    pub name: String,
    pub default_value: String,
    pub flags: i32,
    pub help_string: String,
    pub bmin: bool,
    pub fmin: f32,
    pub bmax: bool,
    pub fmax: f32,
    pub callback: FnChangeCallback_t,
}

impl ConVarRegister {
    pub fn new(
        name: impl Into<String>,
        default_value: impl Into<String>,
        flags: i32,
        help_string: impl Into<String>,
    ) -> Self {
        Self::mandatory(name, default_value, flags, help_string)
    }

    pub fn mandatory(
        name: impl Into<String>,
        default_value: impl Into<String>,
        flags: i32,
        help_string: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            default_value: default_value.into(),
            flags,
            help_string: help_string.into(),
            bmin: bool::default(),
            fmin: f32::default(),
            bmax: bool::default(),
            fmax: f32::default(),
            callback: None,
        }
    }
}

/// [`ConVarStruct`] wraps unsafe code in a safe api
///
/// ### Thread Safety
/// even thought [`Sync`] and [`Send`] are implemented for this struct
///
/// it is not safe to call any of its functions outside of titanfall's engine callbacks to plugins
/// and may result in a crash
///
/// [`Sync`] and [`Send`] will be removed once plugins v3 will be real
pub struct ConVarStruct {
    inner: *mut ConVar,
}

impl ConVarStruct {
    /// Creates an unregistered convar
    ///
    /// Would only fail if something goes wrong with northstar
    pub fn try_new() -> Option<Self> {
        let obj_func = (*CREATE_OBJECT_FUNC.wait())?;

        get_engine_data().map(move |engine| Self::new(engine, obj_func))
    }

    fn new(engine: &EngineData, obj_func: unsafe extern "C" fn(i32) -> *mut c_void) -> Self {
        let convar_classes = &engine.convar;

        let convar = unsafe { mem::transmute::<_, *mut ConVar>(obj_func(ObjectType_CONVAR)) };

        unsafe {
            addr_of_mut!((*convar).m_ConCommandBase.m_pConCommandBaseVTable)
                .write(convar_classes.convar_vtable);

            addr_of_mut!((*convar).m_ConCommandBase.s_pConCommandBases)
                .write(convar_classes.iconvar_vtable);

            #[allow(clippy::crosspointer_transmute)] // its what c++ this->convar_malloc is
            (convar_classes.convar_malloc)(mem::transmute(addr_of_mut!((*convar).m_pMalloc)), 0, 0);
            // Allocate new memory for ConVar.
        }
        Self { inner: convar }
    }

    pub fn register(&self, register_info: ConVarRegister) -> Result<(), RegisterError> {
        let engine_data = get_engine_data().ok_or(RegisterError::NoneFunction)?;

        self.private_register(register_info, engine_data)
    }

    pub(crate) fn private_register(
        &self,
        register_info: ConVarRegister,
        engine_data: &EngineData,
    ) -> Result<(), RegisterError> {
        log::info!("Registering ConVar {}", register_info.name);

        debug_assert!(!register_info.name.is_empty());
        debug_assert!(!register_info.default_value.is_empty());

        // the following stuff may still leak memory
        // has to be investigated
        // I think its safe
        // since it should live until process termination so the os would clean it

        let name_ptr = to_sq_string!(register_info.name).into_raw();

        let default_value_ptr = to_sq_string!(register_info.default_value).into_raw();

        let help_string_ptr = to_sq_string!(register_info.help_string).into_raw();

        unsafe {
            (engine_data
                .convar
                .convar_register
                .ok_or(RegisterError::NoneFunction)?)(
                self.inner,
                name_ptr,
                default_value_ptr,
                register_info.flags,
                help_string_ptr,
                register_info.bmin,
                register_info.fmin,
                register_info.bmax,
                register_info.fmax,
                mem::transmute(register_info.callback),
            )
        }
        Ok(())
    }

    /// get the name of the convar
    ///
    /// only really safe on the titanfall thread
    pub fn get_name(&self) -> String {
        unsafe {
            let cstr = CStr::from_ptr((*self.inner).m_ConCommandBase.m_pszName);
            cstr.to_string_lossy().to_string()
        }
    }

    /// get the value inside the convar
    ///
    /// only safe on the titanfall thread
    pub fn get_value(&self) -> ConVarValues {
        unsafe {
            let value = &(*self.inner).m_Value;

            let string = if !value.m_pszString.is_null()
                && self.has_flag(
                    FCVAR_NEVER_AS_STRING
                        .try_into()
                        .expect("supposed to always work"),
                ) {
                Some(
                    CStr::from_ptr(value.m_pszString)
                        .to_string_lossy()
                        .to_string(),
                )
            } else {
                None
            };

            ConVarValues {
                value: string,
                value_float: value.m_fValue,
                value_int: value.m_nValue,
            }
        }
    }

    /// get the value as a string
    ///
    /// only safe on the titanfall thread
    pub fn get_value_string(&self) -> Option<String> {
        unsafe {
            let value = &(*self.inner).m_Value;

            if value.m_pszString.is_null()
                || !self.has_flag(
                    FCVAR_NEVER_AS_STRING
                        .try_into()
                        .expect("supposed to always work"),
                )
            {
                return None;
            }

            let value = CStr::from_ptr(value.m_pszString)
                .to_string_lossy()
                .to_string();
            Some(value)
        }
    }

    /// get the value as a i32
    ///
    /// only safe on the titanfall thread
    pub fn get_value_i32(&self) -> i32 {
        unsafe {
            let value = &(*self.inner).m_Value;

            value.m_nValue
        }
    }

    /// get the value as a f32
    ///
    /// only safe on the titanfall thread
    pub fn get_value_f32(&self) -> f32 {
        unsafe {
            let value = &(*self.inner).m_Value;

            value.m_fValue
        }
    }
    
    /// set the int value of the convar
    /// 
    /// only safe on the titanfall thread
    /// 
    /// ## Warning
    /// this function only updates the integer and float values of the convar, the string value remains unchanged 
    /// see this [discord message](https://discord.com/channels/920776187884732556/950322078945538058/1084286030858948638)
    /// for more info
    pub fn set_value_i32(&self, new_value: i32) {
        unsafe {
            let value = &mut (*self.inner).m_Value;

            if value.m_nValue == new_value {
                return;
            }
            
            value.m_fValue = new_value as f32;
            value.m_nValue = new_value;
        }
    }

    /// set the int value of the convar
    /// 
    /// only safe on the titanfall thread
    /// 
    /// ## Warning
    /// this function only updates the integer and float values of the convar, the string value remains unchanged 
    /// see this [discord message](https://discord.com/channels/920776187884732556/950322078945538058/1084286030858948638)
    /// for more info
    pub fn set_value_f32(&self, new_value: f32) {
        unsafe {
            let value = &mut (*self.inner).m_Value;

            if value.m_fValue == new_value {
                return;
            }
            
            value.m_nValue = new_value as i32;
            value.m_fValue = new_value;
        }
    }

    /// fr why would you need this?
    ///
    /// only safe on the titanfall thread
    pub fn get_help_text(&self) -> String {
        unsafe {
            let help = (*self.inner).m_ConCommandBase.m_pszHelpString;
            CStr::from_ptr(help).to_string_lossy().to_string()
        }
    }

    /// returns [`true`] if the convar is registered
    ///
    /// only safe on the titanfall thread
    pub fn is_registered(&self) -> bool {
        unsafe { (*self.inner).m_ConCommandBase.m_bRegistered }
    }

    /// returns [`true`] if the given flags are set for this convar
    ///
    /// only safe on the titanfall thread
    pub fn has_flag(&self, flags: i32) -> bool {
        unsafe { (*self.inner).m_ConCommandBase.m_nFlags & flags != 0 }
    }

    /// adds flags to the convar
    ///
    /// only safe on the titanfall thread
    pub fn add_flags(&mut self, flags: i32) {
        unsafe { (*self.inner).m_ConCommandBase.m_nFlags |= flags }
    }

    /// removes flags from the convar
    ///
    /// only safe on the titanfall thread
    pub fn remove_flags(&mut self, flags: i32) {
        unsafe { (*self.inner).m_ConCommandBase.m_nFlags |= !flags }
    }

    /// exposes the raw pointer to the [`ConVar`] class
    ///
    /// # Safety
    /// its safe unless you start iteracting with the raw pointer
    pub unsafe fn get_raw_convar_ptr(&self) -> *mut ConVar {
        self.inner
    }
}

impl From<*mut ConVar> for ConVarStruct {
    fn from(value: *mut ConVar) -> Self {
        Self { inner: value }
    }
}

// this must be revert once plugins v3 is out or not
unsafe impl Sync for ConVarStruct {}
unsafe impl Sync for ConVar {}
unsafe impl Send for ConVarStruct {}
unsafe impl Send for ConVar {}

pub(crate) struct ConVarClasses {
    convar_vtable: *mut c_void,
    convar_register: ConVarRegisterType,
    iconvar_vtable: *mut ConCommandBase,
    convar_malloc: ConVarMallocType,
}

impl ConVarClasses {
    pub fn new(raw: &PluginEngineData) -> Self {
        let convar_malloc: ConVarMallocType = unsafe { mem::transmute(raw.conVarMalloc) };
        let iconvar_vtable = unsafe { mem::transmute(raw.IConVar_Vtable) };
        let convar_register: ConVarRegisterType = unsafe { mem::transmute(raw.conVarRegister) };
        Self {
            convar_vtable: raw.ConVar_Vtable,
            iconvar_vtable,
            convar_register,
            convar_malloc,
        }
    }
}
