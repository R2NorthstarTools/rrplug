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

use super::UnsafeHandle;

/// internal vec to not call on_dll_load
#[doc(hidden)]
pub static CALLED_DLLS: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// a ZST marker to provided a invariance to safe and unsafe apis of rrplug of running on engine threads
#[derive(Debug, Clone, Copy)]
pub struct EngineToken(PhantomData<*mut UnsafeCell<()>>);

impl EngineToken {
    /// this is a super expensive operation that checks if this is the engine thread
    #[doc(hidden)]
    pub fn try_new() -> Option<Self> {
        // TODO: finish this
        todo!()
    }

    /// allows you to call engine restricted functions without checking if it's the right thread
    ///
    /// # Safety
    /// please only use this if you are sure that it is the engine thread   
    ///
    /// can lead to crashes or ub since most functions are not thread safe
    ///
    /// this is usually the most sensible solution altought it does at least require you to think from where you call stuff
    pub const unsafe fn new_unchecked() -> Self {
        Self(PhantomData)
    }
}

/// struct that guarantees that a value will only be accessed on the engine thread using [`EngineToken `]
pub struct EngineGlobal<T>(UnsafeHandle<T>);

impl<T> EngineGlobal<T> {
    pub const fn new(data: T) -> Self {
        Self(UnsafeHandle { inner: data })
    }

    pub const fn get(&self, _: EngineToken) -> &T {
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
    pub const fn copy(&self, _: EngineToken) -> T {
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
    /// # let engine_token = unsafe { EngineToken::new_unchecked() };
    /// engine.register_concommand("cool_command", cool_command, "this is cool_command", 0, engine_token).expect("failed to register cool_command");
    ///
    /// #[rrplug::concommand]
    /// fn cool_command() {
    ///     println!("cool_command");
    /// }
    /// ```
    pub fn register_concommand(
        &self,
        name: impl AsRef<str>,
        callback: unsafe extern "C" fn(arg1: *const CCommand),
        help_string: impl AsRef<str>,
        flags: i32,
        _: EngineToken,
    ) -> Result<*mut ConCommand, RegisterError> {
        let name = name.as_ref();
        log::info!("Registering ConCommand {}", name);

        self.concommands
            .mid_register_concommand(name, callback, help_string.as_ref(), flags)
    }

    pub fn register_concommand_with_completion(
        &self,
        name: impl AsRef<str>,
        callback: unsafe extern "C" fn(arg1: *const CCommand),
        help_string: impl AsRef<str>,
        flags: i32,
        completion_callback: unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut [::std::os::raw::c_char; COMMAND_COMPLETION_ITEM_LENGTH as usize],
        ) -> ::std::os::raw::c_int,
        _: EngineToken,
    ) -> Result<*mut ConCommand, RegisterError> {
        let name = name.as_ref();
        log::info!("Registering ConCommand {} with completion", name);

        self.concommands.mid_register_concommand_with_completion(
            name,
            callback,
            help_string.as_ref(),
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
        token: EngineToken,
    ) -> Result<(), RegisterError> {
        use super::convars::{ConVarRegister, ConVarStruct};

        ConVarStruct::try_new(
            &ConVarRegister::new(name, default_value, flags, help_string),
            token,
        )
        .map(|_| ())
    }
}

// hacky way to test compile failure

#[cfg(doctest)]
mod doctest {
    #![allow(unused)]
    use super::*;

    /// ```compile_fail
    /// use super::*;
    /// let engine_token = unsafe { EngineToken::new_unchecked() };
    /// #[allow(unused)]
    /// std::thread::spawn(move || {let e = engine_token; panic!()} );
    /// _ = engine_token;
    /// ```
    const fn test_engine_token() {}
}
