//! engine related abstractions and functions

use parking_lot::Mutex;
use std::{cell::UnsafeCell, marker::PhantomData};

#[cfg(doc)]
use crate::high::convars::ConVarStruct;
use crate::{
    bindings::cvar::{
        command::{CCommand, ConCommand},
        convar::COMMAND_COMPLETION_ITEM_LENGTH,
        RawCVar,
    },
    errors::RegisterError,
    mid::{
        concommands::{RegisterConCommands, REGISTER_CONCOMNMADS},
        convars::{CvarGlobals, CVAR_GLOBALS},
    },
};

use super::{concommands::CommandCompletion, UnsafeHandle};

/// internal vec to not call on_dll_load
#[doc(hidden)]
pub static CALLED_DLLS: Mutex<Vec<String>> = Mutex::new(Vec::new());

#[derive(Debug, Clone, Copy)]
pub struct EngineToken(PhantomData<*mut UnsafeCell<()>>);

impl EngineToken {
    /// this is a super expensive operation that checks if this is the engine thread
    pub fn try_new() -> Option<Self> {
        // TODO: finish this
        todo!()
    }

    pub unsafe fn new_unchecked() -> Self {
        Self(PhantomData)
    }
}

pub struct EngineGlobal<T>(UnsafeHandle<T>);

impl<T> EngineGlobal<T> {
    pub fn new(data: T) -> Self {
        Self(UnsafeHandle { inner: data })
    }

    pub fn get(&self, _: EngineToken) -> &T {
        self.0.get()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }

    pub fn take(self) -> T {
        self.0.take()
    }
}

impl<T: Copy> EngineGlobal<T> {
    pub fn copy(&self, _: EngineToken) -> T {
        self.0.copy()
    }
}

/// Use this struct to register convars and concommands
///
/// only usefull when the convars or concommands features are enabled
pub struct EngineData {
    pub(crate) concommands: &'static RegisterConCommands,
    pub(crate) convar: &'static CvarGlobals,
    pub(crate) cvar: &'static RawCVar,
}

unsafe impl Send for EngineData {}
unsafe impl Sync for EngineData {}

impl EngineData {
    /// # Safety
    ///
    /// hopefuly rrplug has correct offsets
    pub unsafe fn new(cvar: &'static RawCVar) -> Self {
        Self {
            concommands: REGISTER_CONCOMNMADS.wait(),
            convar: CVAR_GLOBALS.wait(),
            cvar,
        }
    }

    /// registers a command
    ///
    /// returns a pointer to [`ConCommand`] which is unsafe to access and has a static lifetime
    ///
    ///  # Example
    /// ```no_run
    /// # use rrplug::mid::engine::get_engine_data;
    /// # use rrplug::errors::RegisterError;
    /// # use rrplug::prelude::*;
    /// # let engine = get_engine_data().unwrap();
    /// engine.register_concommand("cool_command", cool_command, "this is cool_command", 0).expect("failed to register cool_command");
    ///
    /// #[rrplug::concommand]
    /// fn cool_command() {
    ///     println!("cool_command");
    /// }
    /// ```
    pub fn register_concommand(
        &self,
        name: impl Into<String>,
        callback: unsafe extern "C" fn(arg1: *const CCommand),
        help_string: impl Into<String>,
        flags: i32,
    ) -> Result<*mut ConCommand, RegisterError> {
        let name = name.into();
        log::info!("Registering ConCommand {}", name);

        self.concommands
            .mid_register_concommand(name, callback, help_string.into(), flags)
    }

    pub fn register_concommand_with_completion(
        &self,
        name: impl Into<String>,
        callback: unsafe extern "C" fn(arg1: *const CCommand),
        help_string: impl Into<String>,
        flags: i32,
        completion_callback: unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut [::std::os::raw::c_char; COMMAND_COMPLETION_ITEM_LENGTH as usize],
        ) -> ::std::os::raw::c_int,
    ) -> Result<*mut ConCommand, RegisterError> {
        let name = name.into();
        log::info!("Registering ConCommand {} with completion", name);

        self.concommands.mid_register_concommand_with_completion(
            name,
            callback,
            help_string.into(),
            flags,
            completion_callback,
        )
    }

    /// registers a convar without any complex steps and without giving back the convar pointer
    ///
    /// it's better to use [`ConVarStruct`] instead since you usually would want to get values out of the convar
    pub fn register_convar(
        &self,
        name: impl Into<String>,
        default_value: impl Into<String>,
        help_string: &'static str,
        flags: i32,
    ) -> Result<(), RegisterError> {
        use super::convars::{ConVarRegister, ConVarStruct};

        let mut convar = ConVarStruct::try_new().ok_or(RegisterError::NoneResult)?;
        let register_info = ConVarRegister::new(name, default_value, flags, help_string);
        convar.private_register(register_info, self)
    }
}
