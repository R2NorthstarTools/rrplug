//! minimal abstraction for concommands

use std::ffi::c_void;

use crate::{
    bindings::cvar::{
        command::{CCommand, ConCommand, ConCommandBase, ConCommandConstructorType},
        RawCVar,
    },
    errors::RegisterError,
    offset_functions, to_c_string,
};

use super::engine::get_engine_data;

offset_functions! {
    REGISTER_CONCOMNMADS + RegisterConCommands for WhichDll::Engine => {
        reg_func = ConCommandConstructorType where offset(0x415F60);
    }
}

impl RegisterConCommands {
    pub(crate) fn mid_register_concommand(
        &self,
        name: String,
        callback: unsafe extern "C" fn(arg1: *const CCommand),
        help_string: String,
        flags: i32,
    ) -> Result<*mut ConCommand, RegisterError> {
        let name_ptr = to_c_string!(name).into_raw();

        let help_string_ptr = to_c_string!(help_string).into_raw();

        // let command: *mut ConCommand = unsafe {
        //     std::mem::transmute((CREATE_OBJECT_FUNC
        //         .get()
        //         .ok_or(RegisterError::NoneFunction)?
        //         .ok_or(RegisterError::NoneFunction))?(
        //         ObjectType::CONCOMMANDS
        //     ))
        // };

        let command = unsafe { std::alloc::alloc(std::alloc::Layout::new::<ConCommand>()) }
            as *mut ConCommand; // TODO: this is not good since if the source allocator decides to drop this concommand bad things will happen

        unsafe {
            self.reg_func.ok_or(RegisterError::NoneFunction)?(
                command,
                name_ptr,
                Some(callback),
                help_string_ptr,
                flags,
                std::ptr::null_mut(),
            )
        };
        Ok(command)
    }
}

/// finds a concommand by name
///
/// # Example
/// ```no_run
/// # use rrplug::mid::engine::get_engine_data;
/// # use rrplug::mid::concommands::find_concommand_with_cvar;
/// # fn sub() -> Option<()> {
/// let concommand = find_concommand_with_cvar("force_newgame", &get_engine_data()?.get_cvar())?;
/// # Some(())
/// # }
/// ```
pub fn find_concommand_with_cvar(name: &str, cvar: &RawCVar) -> Option<&'static mut ConCommand> {
    let name = to_c_string!(name);
    unsafe { cvar.find_concommand(name.as_ptr()).as_mut() }
}

/// finds a concommand by name
///
/// # Example
/// ```no_run
/// # use rrplug::mid::concommands::find_concommand;
/// # fn sub() -> Option<()> {
/// let concommand = find_concommand("force_newgame")?;
/// # Some(())
/// # }
/// ```
pub fn find_concommand(name: &str) -> Option<&'static mut ConCommand> {
    find_concommand_with_cvar(name, &get_engine_data()?.cvar)
}

/// finds a concommand base by name
///
/// # Example
/// ```no_run
/// # use rrplug::mid::engine::get_engine_data;
/// # use rrplug::mid::concommands::find_concommand_base_with_cvar;
/// # fn sub() -> Option<()> {
/// let base = find_concommand_base_with_cvar("spewlog_enable", &get_engine_data()?.get_cvar())?;
/// # Some(())
/// # }
/// ```
pub fn find_concommand_base_with_cvar(
    name: &str,
    cvar: &RawCVar,
) -> Option<&'static mut ConCommandBase> {
    let name = to_c_string!(name);
    unsafe { cvar.find_command_base(name.as_ptr()).as_mut() }
}

/// finds a concommand base by name
///
/// # Example
/// ```no_run
/// # use rrplug::mid::concommands::find_concommand_base;
/// # fn sub() -> Option<()> {
/// let base = find_concommand_base("spewlog_enable")?;
/// # Some(())
/// # }
/// ```
pub fn find_concommand_base(name: &str) -> Option<&'static mut ConCommandBase> {
    find_concommand_base_with_cvar(name, &get_engine_data()?.cvar)
}
