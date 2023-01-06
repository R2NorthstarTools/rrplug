#![allow(clippy::not_unsafe_ptr_arg_deref)]

use super::northstar::CREATE_OBJECT_FUNC;
use std::ffi::CStr;
use std::os::raw::c_void;
use std::ptr;

use crate::bindings::command::{CCommand, ConCommand, ConCommandConstructorType};
use crate::bindings::plugin_abi::ObjectType_CONCOMMANDS;
use crate::to_sq_string;

use super::errors::RegisterError;

#[derive(Debug, Default)]
pub struct CCommandResult {
    pub args: String,
}

impl From<*const CCommand> for CCommandResult {
    fn from(value: *const CCommand) -> Self {
        let ccommand = match unsafe { value.as_ref() } {
            Some(c) => c,
            None => return Self::default(),
        };
        let ccommand = *ccommand;

        let args = unsafe {

            log::info!( "ccommand.m_nArgv0Size {}", ccommand.m_nArgv0Size );

            if ccommand.m_nArgv0Size == 0 {
                "".to_string()
            } else {
                let buffer = ccommand.m_pArgSBuffer.to_vec().as_ptr();
                CStr::from_ptr( buffer ).to_string_lossy().into()
            }
        };

        Self { args }
    }
}

pub struct RegisterConCommands {
    reg_func: ConCommandConstructorType,
}

impl RegisterConCommands {
    pub(crate) unsafe fn new(ptr: *const c_void) -> Self {
        let reg_func: *const ConCommandConstructorType = std::mem::transmute(ptr);

        Self {
            reg_func: *reg_func,
        }
    }

    pub fn register_concommand(
        &self,
        name: String,
        callback: extern "C" fn(arg1: *const CCommand),
        help_string: String,
        flags: i32,
    ) -> Result<(), RegisterError> {
        let name = to_sq_string!(name);
        let help_string = to_sq_string!(help_string);
        let command: *mut ConCommand = unsafe {
            let obj = (CREATE_OBJECT_FUNC
                .wait()
                .ok_or(RegisterError::NoneFunction)?)(ObjectType_CONCOMMANDS);

            std::mem::transmute(obj)
        };

        let help_string_ptr = help_string.as_ptr();
        let name_ptr = name.as_ptr();

        let func = (self.reg_func).unwrap();

        unsafe {
            func(
                command,
                name_ptr,
                Some(callback),
                help_string_ptr,
                flags,
                ptr::null_mut(),
            )
        };

        Ok(())
    }
}
