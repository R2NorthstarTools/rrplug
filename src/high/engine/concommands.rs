#![allow(clippy::not_unsafe_ptr_arg_deref)]

//! concommands are console commands. yes.
//!
//! registering a concommand
//! ```no_run
//! use rrplug::prelude::*;
//!
//! // inside Plugin impl
//! fn on_engine_load(engine_data: Option<&EngineData>, _dll_ptr: &DLLPointer, engine_token: EngineToken) {
//!     let Some(engine_data) = engine_data else {
//!         return;
//!     };
//!
//!     engine_data.register_concommand("boom", explode, "displays a explosion in the console", 0, engine_token); // register the concommand
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

use std::ffi::{c_char, CStr};

use crate::{
    bindings::cvar::command::{
        CCommand, COMMAND_COMPLETION_ITEM_LENGTH, COMMAND_COMPLETION_MAXITEMS,
    },
    errors::CompletionError,
    mid::utils::set_c_char_array,
};

/// [`CCommandResult`] gets all the usefull stuff from [`*const CCommand`] and puts in this struct
#[derive(Debug, Default)]
pub struct CCommandResult {
    args: Vec<String>,
    command: String,
}

/// used to track the completion suggestions and then to also output the amount
pub struct CommandCompletion<'a> {
    suggestions: &'a mut [[i8; COMMAND_COMPLETION_ITEM_LENGTH as usize]],
    suggestions_left: u32,
}

/// holds imformation about the current completion request
pub struct CurrentCommand<'a> {
    /// the concommand that is being completed
    pub cmd: &'a str,

    /// the arguments passed to the concommand
    pub partial: &'a str,
}

// should maybe make this lazy?
impl CCommandResult {
    /// this function shouldn't be used by the crate users
    ///
    /// # Safety
    ///
    /// should be safe if the input is correct
    pub unsafe fn new(ccommand: *const CCommand) -> Self {
        let ccommand = match unsafe { ccommand.as_ref() } {
            Some(c) => c,
            None => return Self::default(),
        };

        if ccommand.m_nArgv0Size == 0 {
            Self::default()
        } else {
            let buffer = ccommand.m_pArgSBuffer.as_ptr();
            let whole_command = unsafe { CStr::from_ptr(buffer).to_string_lossy().to_string() };
            let mut whole_command = whole_command.split_whitespace();

            let command = whole_command.next().unwrap_or_default().into();
            let args = whole_command.map(|a| a.to_string()).collect();

            Self { args, command }
        }
    }

    /// pops an arg from args [`Vec`]
    pub fn pop_arg(&mut self) -> Option<String> {
        self.args.pop()
    }

    /// gets an arg at a index
    pub fn get_arg(&self, index: usize) -> Option<&str> {
        self.args.get(index).map(|s| &**s)
    }

    /// returns the whole [`Vec`] as a slice
    pub const fn get_args(&self) -> &[String] {
        &self.args
    }

    /// returns the command's name
    pub const fn get_command(&self) -> &str {
        &self.command
    }
}

impl From<*mut [c_char; COMMAND_COMPLETION_ITEM_LENGTH as usize]> for CommandCompletion<'_> {
    fn from(commands: *mut [c_char; COMMAND_COMPLETION_ITEM_LENGTH as usize]) -> Self {
        Self {
            suggestions: unsafe {
                std::slice::from_raw_parts_mut(commands, COMMAND_COMPLETION_MAXITEMS as usize)
            },
            suggestions_left: COMMAND_COMPLETION_MAXITEMS,
        }
    }
}
impl CommandCompletion<'_> {
    /// tries to add a new completion string
    ///
    /// # Errors
    ///
    /// This function will return an error if all completion slots are exhausted
    pub fn push(&mut self, new: &str) -> Result<(), CompletionError> {
        if self.suggestions_left == 0 {
            return Err(CompletionError::NoCompletionSlotsLeft);
        }

        set_c_char_array(
            &mut self.suggestions[(COMMAND_COMPLETION_MAXITEMS - self.suggestions_left) as usize],
            new,
        );
        self.suggestions_left -= 1;

        Ok(())
    }

    /// Returns the commands used of this [`CommandCompletion`].
    ///
    /// is called by the macro completion so you don't need to care about this.
    pub const fn commands_used(&self) -> i32 {
        (COMMAND_COMPLETION_MAXITEMS - self.suggestions_left) as i32
    }
}

impl CurrentCommand<'_> {
    /// creates a new CurrenCommand from the partial string spliting it into command and args
    pub fn new(partial: *const c_char) -> Option<Self> {
        let partial = unsafe { CStr::from_ptr(partial).to_str() }.ok()?;
        let (name, cmd) = partial.split_once(' ').unwrap_or((partial, ""));

        Some(Self {
            cmd: name,
            partial: cmd,
        })
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

#[cfg(test)]
mod test {
    // TODO: test some completion structs

    use crate::rrplug;

    #[rrplug::completion]
    fn completion_test(current: CurrentCommand, suggestions: CommandCompletion) {
        suggestions
            .push(format!("{} {}", current.cmd, "test").as_str())
            .unwrap();
    }
}
