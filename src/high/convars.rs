//! **convars** are simple. You register them then you can read or edit its data; either in the plugin or from scripts.
//!
//! they can either be a
//! 1. `string` (`String`)
//! 2. `int` (`i32`)
//! 3. `float` (`f32`)
//!
//! ## Safety
//!
//! convars are also being accessed by the sqvm or the engine so it is unsafe to read or edit them from threads.
//!
//! so they must read/writen only from **convar callbacks**, **concommands** or **native sqfunction**
//!
//! if you like **taking risks** you can ignore this or **if** the sqvm/engine never read or edit the convar.
//!
//!
//! ## Working with Convars
//!
//! convars can be created at any time after engine load but its better to create them when the engine loads
//!
//! ```no_run
//! use rrplug::prelude::*;
//! use rrplug::exports::OnceCell; // just as a example
//!
//! // inside Plugin impl
//! fn on_engine_load(engine_data: Option<&EngineData>, _dll_ptr: &DLLPointer) {
//!     let Some(_) = engine_data else {
//!         return;
//!     };
//!
//!     let mut convar = ConVarStruct::try_new().unwrap(); // creates the convar struct
//!     let register_info = ConVarRegister { // struct containing info the convar ( there is a lot of stuff )
//!         callback: Some(cool_convar_change_callback),
//!         ..ConVarRegister::mandatory(
//!         "cool_convar",
//!         "cool_convar",
//!         0,
//!         "cool_convar",
//!     )
//!     };
//!
//!     convar.register(register_info).unwrap(); // register the convar
//! }
//!
//! // to access your convar, you will have to save them into a static or in the plugin struct
//!
//! static COOLCONVAR: OnceCell<ConVarStruct> = OnceCell::new();
//!
//! // reading it from a convar change callback
//! #[rrplug::convar]
//! fn cool_convar_change_callback(old_value: String, float_old_value: f32) {
//!     let convar = COOLCONVAR.wait();
//!
//!     log::info!("convar name: {}", convar.get_name());
//!     log::info!("new value: {}", convar.get_value().value.unwrap());
//!     log::info!("old value: {}", old_value)
//! }
//! ```

use std::{
    alloc::{GlobalAlloc, Layout},
    ffi::{c_char, CStr},
    mem,
    ptr::addr_of_mut,
};

use super::engine::EngineData;
use crate::{
    bindings::cvar::convar::{ConVar, FnChangeCallback_t, FCVAR_NEVER_AS_STRING},
    errors::{CStringPtrError, RegisterError},
    mid::{
        engine::{get_engine_data, ENGINE_DATA},
        source_alloc::SOURCE_ALLOC,
        utils::{to_cstring, try_cstring},
    },
};

/// the state of the convar in all of its possible types
///
/// values are always valid
#[derive(Debug, PartialEq)]
pub struct ConVarValues<'a> {
    /// string value
    ///
    /// the strings should always be valid utf8
    pub value: Result<&'a str, CStringPtrError>,
    /// float value
    pub value_float: f32,
    ///
    pub value_int: i32,
}

/// [`ConVarRegister`] is builder sturct for convars
///
/// consumed by [`ConVarStruct`]`::register`
pub struct ConVarRegister {
    /// literally the name of the convar
    ///
    /// This is **required**
    pub name: String,
    /// the default value
    ///
    /// This is **required**
    pub default_value: String,
    /// any flags like [`crate::bindings::cvar::convar::FCVAR_GAMEDLL`]
    ///
    /// This is **required**
    pub flags: i32,
    /// the help string
    pub help_string: &'static str,
    /// should use min or not
    pub bmin: bool,
    /// min value for floats and integers
    pub fmin: f32,
    /// should use max or not
    pub bmax: bool,
    /// max value for floats and integers
    pub fmax: f32,
    /// callbak when the convar is changed should be created with [`crate::convar`]
    pub callback: FnChangeCallback_t,
}

impl ConVarRegister {
    /// creates a new [`ConVarRegister`]
    pub fn new(
        name: impl Into<String>,
        default_value: impl Into<String>,
        flags: i32,
        help_string: &'static str,
    ) -> Self {
        Self::mandatory(name, default_value, flags, help_string)
    }

    /// all the required fields to register a convar
    ///
    /// can be used in this way to inforce it
    /// ```
    /// # use rrplug::prelude::*;
    /// _ = ConVarRegister {
    ///     ..ConVarRegister::mandatory(
    ///     "a_convar",
    ///     "default_value",
    ///     0,
    ///     "this is a convar",
    /// )
    /// };
    /// ```
    pub fn mandatory(
        name: impl Into<String>,
        default_value: impl Into<String>,
        flags: i32,
        help_string: &'static str,
    ) -> Self {
        Self {
            name: name.into(),
            default_value: default_value.into(),
            flags,
            help_string,
            bmin: bool::default(),
            fmin: f32::default(),
            bmax: bool::default(),
            fmax: f32::default(),
            callback: None,
        }
    }
}

// TODO: rewrite convars with thread_local!
// so the convars have to init at mod scope with a macro
// sync and send must be removed from ConVarStruct to forbid unsafe access with race condition
// altought a unsafe way of initing the ConVarStruct should be given
// of course this adds extra overhead with forced atomics for convars but it's required to make everything harder to break by accident
// also concommand and convar macros should give an option of including the plugin struct like a self input :P
// note: not all dlls are loaded on the engine thread

/// [`ConVarStruct`] wraps unsafe code in a safe api for convar access
///
/// ### Thread Safety
/// even thought [`Sync`] and [`Send`] are implemented for this struct
///
/// it is not safe to call any of its functions outside of titanfall's engine callbacks to plugins
/// and may result in race condition (really bad thing)
pub struct ConVarStruct {
    inner: &'static mut ConVar,
}

impl ConVarStruct {
    /// Creates an unregistered convar
    ///
    /// Would only fail if something goes wrong with northstar
    pub fn try_new() -> Option<Self> {
        get_engine_data().map(move |engine| unsafe { Self::new(engine) })
    }

    unsafe fn new(engine: &EngineData) -> Self {
        let convar_classes = &engine.convar;
        unsafe {
            let convar = SOURCE_ALLOC.alloc(Layout::new::<ConVar>()) as *mut ConVar;

            addr_of_mut!((*convar).m_ConCommandBase.m_pConCommandBaseVTable)
                .write(convar_classes.convar_vtable);

            addr_of_mut!((*convar).m_ConCommandBase.s_pConCommandBases)
                .write(convar_classes.iconvar_vtable);

            #[allow(clippy::crosspointer_transmute)] // its what c++ this->convar_malloc is
            (convar_classes.convar_malloc)(mem::transmute(addr_of_mut!((*convar).m_pMalloc)), 0, 0);
            // Allocate new memory for ConVar.

            Self {
                inner: &mut *convar, // no way this is invalid
            }
        }
    }

    /// registers a convar from [`ConVarRegister`]
    ///
    /// this functions leaks the strings since convars live for the lifetime of the game :)
    ///
    /// # Example
    /// ```no_run
    /// # use rrplug::prelude::*;
    ///
    /// let mut convar = ConVarStruct::try_new().unwrap(); // creates the convar struct
    /// let register_info = ConVarRegister { // struct containing info the convar ( there is a lot of stuff )
    ///     ..ConVarRegister::mandatory(
    ///     "a_convar",
    ///     "default_value",
    ///     0,
    ///     "this is a convar",
    /// )
    /// };
    ///
    /// convar.register(register_info).unwrap(); // register the convar
    /// ```
    pub fn register(&mut self, register_info: ConVarRegister) -> Result<(), RegisterError> {
        let engine_data = get_engine_data().ok_or(RegisterError::NoneFunction)?;

        self.private_register(register_info, engine_data)
    }

    pub(crate) fn private_register(
        &mut self,
        register_info: ConVarRegister,
        engine_data: &EngineData,
    ) -> Result<(), RegisterError> {
        log::info!("Registering ConVar {}", register_info.name);

        debug_assert!(!register_info.name.is_empty());

        // the following stuff may still leak memory
        // has to be investigated
        // I think its safe
        // since it should live until process termination so the os would clean it

        let name = try_cstring(&register_info.name)?.into_bytes_with_nul();
        let name_ptr = unsafe { SOURCE_ALLOC.alloc(Layout::for_value(&name)) };
        unsafe { name_ptr.copy_from_nonoverlapping(name.as_ptr(), name.len()) };

        let default_value = try_cstring(&register_info.default_value)?.into_bytes_with_nul();
        let default_value_ptr = unsafe { SOURCE_ALLOC.alloc(Layout::for_value(&default_value)) };
        unsafe {
            default_value_ptr.copy_from_nonoverlapping(default_value.as_ptr(), default_value.len())
        };

        let help_string = try_cstring(&register_info.help_string)?.into_bytes_with_nul();
        let help_string_ptr = unsafe { SOURCE_ALLOC.alloc(Layout::for_value(&help_string)) };
        unsafe {
            help_string_ptr.copy_from_nonoverlapping(help_string.as_ptr(), help_string.len())
        };

        unsafe {
            (engine_data
                .convar
                .convar_register
                .ok_or(RegisterError::NoneFunction)?)(
                self.inner,
                name_ptr as *const i8,
                default_value_ptr as *const i8,
                register_info.flags,
                help_string_ptr as *const i8,
                register_info.bmin,
                register_info.fmin,
                register_info.bmax,
                register_info.fmax,
                mem::transmute(register_info.callback),
            )
        }
        Ok(())
    }

    ///
    pub fn find_convar_by_name(name: &str) -> Option<Self> {
        let name = try_cstring(name).ok()?;

        Some(Self {
            inner: unsafe {
                ENGINE_DATA
                    .get()?
                    .cvar
                    .find_convar(name.as_ptr())
                    .as_mut()?
            },
        })
    }

    /// get the name of the convar
    ///
    /// only really safe on the titanfall thread
    pub fn get_name(&self) -> String {
        unsafe {
            CStr::from_ptr(self.inner.m_ConCommandBase.m_pszName)
                .to_string_lossy()
                .to_string()
        }
    }

    /// get the value inside the convar
    ///
    /// only safe on the titanfall thread
    pub fn get_value(&self) -> ConVarValues {
        unsafe {
            let value = &self.inner.m_Value;

            let string = if !value.m_pszString.is_null()
                && !self.has_flags(
                    FCVAR_NEVER_AS_STRING
                        .try_into()
                        .expect("supposed to always work"),
                ) {
                CStr::from_ptr(value.m_pszString)
                    .to_str()
                    .map_err(|err| err.into())
            } else {
                Err(CStringPtrError::None)
            };

            ConVarValues {
                value: string,
                value_float: value.m_fValue,
                value_int: value.m_nValue,
            }
        }
    }

    // add a real String version

    /// get the value as a string
    ///
    /// only safe on the titanfall thread
    pub fn get_value_string(&self) -> Result<&str, CStringPtrError> {
        unsafe {
            let value = &self.inner.m_Value;

            if value.m_pszString.is_null()
                || self.has_flags(
                    FCVAR_NEVER_AS_STRING
                        .try_into()
                        .expect("supposed to always work"),
                )
            {
                return Err(CStringPtrError::None);
            }

            CStr::from_ptr(value.m_pszString)
                .to_str()
                .map_err(|err| err.into())
        }
    }

    /// get the value as a i32
    ///
    /// only safe on the titanfall thread
    pub fn get_value_i32(&self) -> i32 {
        self.inner.m_Value.m_nValue
    }

    /// get the value as a bool
    ///
    /// only safe on the titanfall thread
    pub fn get_value_bool(&self) -> bool {
        self.inner.m_Value.m_nValue != 0
    }

    /// get the value as a f32
    ///
    /// only safe on the titanfall thread
    pub fn get_value_f32(&self) -> f32 {
        self.inner.m_Value.m_fValue
    }

    // todo: add exclusive access set_value s aka &mut self

    /// set the int value of the convar
    /// also sets float and string
    ///
    /// only safe on the titanfall thread
    pub fn set_value_i32(&self, new_value: i32) {
        unsafe {
            let value = &self.inner.m_Value;

            if value.m_nValue == new_value {
                return;
            }

            let vtable_adr = self.inner.m_ConCommandBase.m_pConCommandBaseVTable as usize;
            let vtable_array = *(vtable_adr as *const [*const std::ffi::c_void; 21]);
            let set_value_int = vtable_array[14];
            // the index for SetValue for ints; weird stuff

            let func = mem::transmute::<_, fn(*const ConVar, i32)>(set_value_int);

            func(self.inner, new_value)
        }
    }

    /// set the float value of the convar
    /// also sets int and string
    ///
    /// only safe on the titanfall thread
    pub fn set_value_f32(&self, new_value: f32) {
        unsafe {
            let value = &self.inner.m_Value;

            if value.m_fValue == new_value {
                return;
            }

            let vtable_adr = self.inner.m_ConCommandBase.m_pConCommandBaseVTable as usize;
            let vtable_array = *(vtable_adr as *const [*const std::ffi::c_void; 21]);
            let set_value_float = vtable_array[13];
            // the index for SetValue for floats; weird stuff

            let func = mem::transmute::<_, fn(*const ConVar, f32)>(set_value_float);

            func(self.inner, new_value)
        }
    }

    /// set the string value of the convar
    ///
    /// only safe on the titanfall thread
    pub fn set_value_string(&self, new_value: impl AsRef<str>) {
        unsafe {
            if self.has_flags(FCVAR_NEVER_AS_STRING.try_into().unwrap()) {
                return;
            }

            let vtable_adr = self.inner.m_ConCommandBase.m_pConCommandBaseVTable as usize;
            let vtable_array = *(vtable_adr as *const [*const std::ffi::c_void; 21]);
            let set_value_string = vtable_array[12];
            // the index for SetValue for strings; weird stuff

            let func = mem::transmute::<_, fn(*const ConVar, *const c_char)>(set_value_string);

            let string_value = to_cstring(new_value.as_ref());
            func(self.inner, string_value.as_ptr())
        }
    }

    /// fr why would you need this?
    ///
    /// only safe on the titanfall thread
    pub fn get_help_text(&self) -> String {
        let help = self.inner.m_ConCommandBase.m_pszHelpString;
        unsafe { CStr::from_ptr(help).to_string_lossy().to_string() }
    }

    /// returns [`true`] if the convar is registered
    ///
    /// only safe on the titanfall thread
    pub fn is_registered(&self) -> bool {
        self.inner.m_ConCommandBase.m_bRegistered
    }

    /// returns [`true`] if the given flags are set for this convar
    ///
    /// only safe on the titanfall thread
    pub fn has_flags(&self, flags: i32) -> bool {
        self.inner.m_ConCommandBase.m_nFlags & flags != 0
    }

    /// adds flags to the convar
    ///
    /// only safe on the titanfall thread
    pub fn add_flags(&mut self, flags: i32) {
        self.inner.m_ConCommandBase.m_nFlags |= flags
    }

    /// removes flags from the convar
    ///
    /// only safe on the titanfall thread
    pub fn remove_flags(&mut self, flags: i32) {
        self.inner.m_ConCommandBase.m_nFlags &= !flags // TODO: figure out if this still needs fixing
    }

    /// exposes the raw pointer to the [`ConVar`] class
    ///
    /// # Safety
    /// accessing the underlying pointer can produce ub
    pub unsafe fn get_raw_convar_ptr(&mut self) -> *mut ConVar {
        self.inner
    }
}

unsafe impl Sync for ConVarStruct {}
unsafe impl Sync for ConVar {}
unsafe impl Send for ConVarStruct {}
unsafe impl Send for ConVar {}
