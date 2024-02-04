//! minimal abstraction for concommands
use std::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_char,
};

use crate::{
    bindings::cvar::{
        command::{
            CCommand, ConCommand, ConCommandBase, ConCommandConstructorType,
            COMMAND_COMPLETION_ITEM_LENGTH,
        },
        RawCVar,
    },
    errors::{CVarQueryError, RegisterError},
    mid::{source_alloc::SOURCE_ALLOC, utils::try_cstring},
    offset_functions,
};

use super::get_engine_data;

offset_functions! {
    REGISTER_CONCOMNMADS + RegisterConCommands for WhichDll::Engine => {
        reg_func = ConCommandConstructorType where offset(0x415F60);
    }
}

impl RegisterConCommands {
    pub(crate) fn mid_register_concommand(
        &self,
        name: &str,
        callback: unsafe extern "C" fn(arg1: *const CCommand),
        help_string: &str,
        flags: i32,
    ) -> Result<*mut ConCommand, RegisterError> {
        // TODO: use IMemAlloc here
        let name = try_cstring(name)?.into_bytes_with_nul();
        let name_ptr =
            unsafe {
                SOURCE_ALLOC.alloc(Layout::array::<c_char>(name.len()).expect(
                    "the Layout for a char array became too large : string allocation failed",
                ))
            };
        unsafe { name_ptr.copy_from_nonoverlapping(name.as_ptr(), name.len()) };

        let help_string = try_cstring(help_string)?.into_bytes_with_nul();
        let help_string_ptr =
            unsafe {
                SOURCE_ALLOC.alloc(Layout::array::<c_char>(help_string.len()).expect(
                    "the Layout for a char array became too large : string allocation failed",
                ))
            };
        unsafe {
            help_string_ptr.copy_from_nonoverlapping(help_string.as_ptr(), help_string.len())
        };

        let command = unsafe { SOURCE_ALLOC.alloc(std::alloc::Layout::new::<ConCommand>()) }
            as *mut ConCommand; // TODO: this is not good since if the source allocator decides to drop this concommand bad things will happen

        unsafe {
            self.reg_func.ok_or(RegisterError::NoneFunction)?(
                command,
                name_ptr as *const i8,
                Some(callback),
                help_string_ptr as *const i8,
                flags,
                std::ptr::null_mut(),
            )
        };
        Ok(command)
    }

    pub(crate) fn mid_register_concommand_with_completion(
        &self,
        name: &str,
        callback: unsafe extern "C" fn(arg1: *const CCommand),
        help_string: &str,
        flags: i32,
        completion_callback: unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut [::std::os::raw::c_char; COMMAND_COMPLETION_ITEM_LENGTH as usize],
        ) -> ::std::os::raw::c_int,
    ) -> Result<*mut ConCommand, RegisterError> {
        self.mid_register_concommand(name, callback, help_string, flags)
            .map(move |command| {
                unsafe {
                    (*command).m_pCompletionCallback = Some(completion_callback);
                    (*command).m_nCallbackFlags |= 0x3;
                }
                command
            })
    }
}

/// adds a completion function to an existing concommand
pub fn add_completion_callback(
    command: &mut ConCommand,
    completion_callback: unsafe extern "C" fn(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut [::std::os::raw::c_char; COMMAND_COMPLETION_ITEM_LENGTH as usize],
    ) -> ::std::os::raw::c_int,
) {
    command.m_pCompletionCallback = Some(completion_callback);
    command.m_nCallbackFlags |= 0x3;
}

/// finds a concommand by name
///
/// # Example
/// ```no_run
/// # use rrplug::mid::engine::get_engine_data;
/// # use rrplug::mid::engine::concommands::find_concommand_with_cvar;
/// # fn sub() -> Option<()> {
/// let concommand = find_concommand_with_cvar("force_newgame", &get_engine_data()?.get_cvar()).ok()?;
/// # Some(())
/// # }
/// ```
pub fn find_concommand_with_cvar(
    name: &str,
    cvar: &RawCVar,
) -> Result<&'static mut ConCommand, CVarQueryError> {
    let name = try_cstring(name)?;
    unsafe {
        cvar.find_concommand(name.as_ptr())
            .as_mut()
            .ok_or(CVarQueryError::NotFound)
    }
}

/// finds a concommand by name
///
/// # Example
/// ```no_run
/// # use rrplug::mid::engine::concommands::find_concommand;
/// # fn sub() -> Option<()> {
/// let concommand = find_concommand("force_newgame").ok()?;
/// # Some(())
/// # }
/// ```
pub fn find_concommand(name: &str) -> Result<&'static mut ConCommand, CVarQueryError> {
    find_concommand_with_cvar(
        name,
        get_engine_data()
            .ok_or(CVarQueryError::NoCVarInterface)?
            .cvar,
    )
}

/// finds a concommand base by name
///
/// # Example
/// ```no_run
/// # use rrplug::mid::engine::get_engine_data;
/// # use rrplug::mid::engine::concommands::find_concommand_base_with_cvar;
/// # fn sub() -> Option<()> {
/// let base = find_concommand_base_with_cvar("spewlog_enable", &get_engine_data()?.get_cvar()).ok()?;
/// # Some(())
/// # }
/// ```
pub fn find_concommand_base_with_cvar(
    name: &str,
    cvar: &RawCVar,
) -> Result<&'static mut ConCommandBase, CVarQueryError> {
    let name = try_cstring(name)?;
    unsafe {
        cvar.find_command_base(name.as_ptr())
            .as_mut()
            .ok_or(CVarQueryError::NotFound)
    }
}

/// finds a concommand base by name
///
/// # Example
/// ```no_run
/// # use rrplug::mid::engine::concommands::find_concommand_base;
/// # fn sub() -> Option<()> {
/// let base = find_concommand_base("spewlog_enable").ok()?;
/// # Some(())
/// # }
/// ```
pub fn find_concommand_base(name: &str) -> Result<&'static mut ConCommandBase, CVarQueryError> {
    find_concommand_base_with_cvar(
        name,
        get_engine_data()
            .ok_or(CVarQueryError::NoCVarInterface)?
            .cvar,
    )
}
