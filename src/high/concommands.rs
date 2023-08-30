#![allow(clippy::not_unsafe_ptr_arg_deref)]

//! concommands are console commands. yes.
//!
//! registering a concommand
//! ```no_run
//! use rrplug::prelude::*;
//! 
//! // inside Plugin impl
//! fn on_engine_load(engine: &PluginLoadDLL, _dll_ptr: &DLLPointer) {
//!     let engine = match engine {
//!         PluginLoadDLL::Engine(engine) => engine,
//!         _ => return
//!     };
//!
//!     engine.register_concommand("boom", explode, "displays a explosion in the console", 0); // register the concommand
//! }
//!
//! // concommand use callback
//! #[rrplug::concommand]
//! fn explode() {
//!     log::info!("explode");
//!
//!     const BOOM: &str = r#"
//!           _ ._  _ , _ ._
//!           (_ ' ( `  )_  .__)
//!       ( (  (    )   `)  ) _)
//!       (__ (_   (_ . _) _) ,__)
//!           `~~`\ ' . /`~~`
//!               ;   ;
//!               /   \
//! _____________/_ __ \_____________
//!     "#;
//!
//!     for line in BOOM.split('\n') {
//!         log::info!("{line}")
//!     }
//! }
//! ```

use std::ffi::CStr;

use crate::bindings::cvar::command::CCommand;

/// [`CCommandResult`] gets all the usefull stuff from [`*const CCommand`] and puts in this struct
#[derive(Debug, Default)]
pub struct CCommandResult {
    args: Vec<String>,
    command: String,
}

// should maybe make this lazy?
impl CCommandResult {
    /// this function shouldn't be used by the crate users
    ///
    /// # Safety
    ///
    /// should be safe if the input is correct
    pub unsafe fn new(ccommand: *const CCommand) -> Self {
        let ccommand = match ccommand.as_ref() {
            Some(c) => c,
            None => return Self::default(),
        };

        if ccommand.m_nArgv0Size == 0 {
            Self::default()
        } else {
            let buffer = ccommand.m_pArgSBuffer.as_ptr();
            let whole_command = CStr::from_ptr(buffer).to_string_lossy().to_string();
            let mut whole_command = whole_command.split_whitespace();

            let command = whole_command.next().unwrap_or_default().into();
            let args = whole_command.map(|a| a.to_string()).collect();

            Self { args, command }
        }
    }

    pub fn pop_arg(&mut self) -> Option<String> {
        self.args.pop()
    }

    pub fn get_arg(&self, index: usize) -> Option<&str> {
        self.args.get(index).map(|s| &**s)
    }

    pub fn get_args(&self) -> &[String] {
        &self.args
    }

    pub fn get_command(&self) -> &str {
        &self.command
    }
}

// maybe this will work in the future
/*
impl RegisterConCommands {
    pub(crate) fn register_concommand<T: Fn(CCommandResult)>(
        &self,
        name: String,
        callback: T,
        help_string: String,
        flags: i32,
    ) -> Result<(), RegisterError> {
        let name_ptr = to_c_string!(name).into_raw();

        let help_string_ptr = to_c_string!(help_string).into_raw();

        let command: *mut ConCommand = unsafe {
            std::mem::transmute((CREATE_OBJECT_FUNC
                .wait()
                .ok_or(RegisterError::NoneFunction)?)(
                ObjectType::CONCOMMANDS
            ))
        };

        unsafe {
            self.reg_func.unwrap()(
                command,
                name_ptr,
                Some(ccommand_trampoline::<T>),
                help_string_ptr,
                flags,
                ptr::null_mut(),
            )
        };
        Ok(())
    }
}

extern "C" fn ccommand_trampoline<const T: fn(CCommandResult)>(ccommand: *const CCommand) {
    ccommand_trampoline(ccommand.into())
}
*/
