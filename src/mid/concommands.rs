use std::ffi::c_void;

use crate::{
    bindings::{
        cvar::{
            command::{CCommand, ConCommand, ConCommandBase, ConCommandConstructorType},
            RawCVar,
        },
        plugin_abi::ObjectType,
    },
    errors::RegisterError,
    mid::northstar::CREATE_OBJECT_FUNC,
    to_c_string,
};

use super::engine::get_engine_data;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RegisterConCommands {
    pub reg_func: ConCommandConstructorType,
}

impl RegisterConCommands {
    pub(crate) unsafe fn new(ptr: *const c_void) -> Self {
        let reg_func: ConCommandConstructorType = std::mem::transmute(ptr);

        Self { reg_func }
    }
}

impl RegisterConCommands {
    pub(crate) fn mid_register_concommand(
        &self,
        name: String,
        callback: unsafe extern "C" fn(arg1: *const CCommand),
        help_string: String,
        flags: i32,
    ) -> Result<(), RegisterError> {
        let name_ptr = to_c_string!(name).into_raw();

        let help_string_ptr = to_c_string!(help_string).into_raw();

        let command: *mut ConCommand = unsafe {
            std::mem::transmute((CREATE_OBJECT_FUNC
                .get()
                .ok_or(RegisterError::NoneFunction)?
                .ok_or(RegisterError::NoneFunction))?(
                ObjectType::CONCOMMANDS
            ))
        };

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
        Ok(())
    }
}

pub fn find_concommand_with_cvar(name: &str, cvar: &RawCVar) -> Option<&'static mut ConCommand> {
    let name = to_c_string!(name);
    unsafe { cvar.find_concommand(name.as_ptr()).as_mut() }
}

pub fn find_concommand(name: &str) -> Option<&'static mut ConCommand> {
    find_concommand_with_cvar(name, &get_engine_data()?.cvar)
}

pub fn find_concommand_base_with_cvar(
    name: &str,
    cvar: &RawCVar,
) -> Option<&'static mut ConCommandBase> {
    let name = to_c_string!(name);
    unsafe { cvar.find_command_base(name.as_ptr()).as_mut() }
}

pub fn find_concommand_base(name: &str) -> Option<&'static mut ConCommandBase> {
    find_concommand_base_with_cvar(name, &get_engine_data()?.cvar)
}
