use std::{mem, os::raw::c_void, ptr::addr_of_mut, ffi::CStr};

use super::{
    engine::{get_engine_data, EngineData},
    errors::RegisterError,
    northstar::CREATE_OBJECT_FUNC,
};
use crate::{
    bindings::{
        command::ConCommandBase,
        convar::{ConVar, ConVarMallocType, ConVarRegisterType, FnChangeCallback_t},
        plugin_abi::{ObjectType_CONVAR, PluginEngineData},
    },
    to_sq_string,
};

pub struct ConvarValues {
    pub value: String,
    pub value_float: f32,
    pub value_int:i32
}

pub struct ConvarRegister {
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

impl ConvarRegister {
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

pub struct ConvarStruct {
    inner: *mut ConVar,
}

impl ConvarStruct {
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

    pub fn register(&self, register_info: ConvarRegister) -> Result<(), RegisterError> {
        let engine_data = get_engine_data().ok_or(RegisterError::NoneFunction)?;

        self.private_register(register_info, engine_data)
    }

    pub(crate) fn private_register(
        &self,
        register_info: ConvarRegister,
        engine_data: &EngineData,
    ) -> Result<(), RegisterError> {
        log::info!("Registering ConVar {}", register_info.name);

        debug_assert!(!register_info.name.is_empty());
        debug_assert!(!register_info.default_value.is_empty());

        // the following stuff may still leak memory
        // has to be investigated

        let (name_ptr, _, _) =
            Box::new(to_sq_string!(register_info.name).into_bytes_with_nul()).into_raw_parts();

        let (default_value_ptr, _, _) =
            Box::new(to_sq_string!(register_info.default_value).into_bytes_with_nul())
                .into_raw_parts();

        let (help_string_ptr, _, _) =
            Box::new(to_sq_string!(register_info.help_string).into_bytes_with_nul())
                .into_raw_parts();

        unsafe {
            (engine_data
                .convar
                .convar_register
                .ok_or(RegisterError::NoneFunction)?)(
                self.inner,
                name_ptr as *mut i8,
                default_value_ptr as *mut i8,
                register_info.flags,
                help_string_ptr as *mut i8,
                register_info.bmin,
                register_info.fmin,
                register_info.bmax,
                register_info.fmax,
                mem::transmute(register_info.callback),
            )
        }
        Ok(())
    }
    
    /// # get_name
    /// gets the name of the convar
    /// 
    /// only really safe on the titanfall thread
    pub fn get_name(&self) -> String {
        unsafe {
            let cstr = CStr::from_ptr( (*self.inner).m_ConCommandBase.m_pszName );
            cstr.to_string_lossy().to_string()
        }
    }

    /// # get_value
    /// gets the value inside the convar
    /// 
    /// only safe on the titanfall thread
    pub fn get_value(&self) -> ConvarValues {
        unsafe {
            let value = (*self.inner).m_Value;
            let string = CStr::from_ptr(value.m_pszString).to_string_lossy().to_string();
            
            ConvarValues {
                value: string,
                value_float: value.m_fValue,
                value_int: value.m_nValue,
            }
        }
    }
}

impl From<*mut ConVar> for ConvarStruct {
    fn from(value: *mut ConVar) -> Self {
        Self { inner: value }
    }
}

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
