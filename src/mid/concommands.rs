use std::ffi::c_void;

use crate::{
    bindings::{
        command::{CCommand, ConCommand, ConCommandConstructorType},
        plugin_abi::ObjectType,
    },
    errors::RegisterError,
    mid::northstar::CREATE_OBJECT_FUNC,
    to_sq_string,
};

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
        let name_ptr = to_sq_string!(name).into_raw();

        let help_string_ptr = to_sq_string!(help_string).into_raw();

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
