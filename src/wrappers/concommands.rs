#![allow(clippy::not_unsafe_ptr_arg_deref)]

use super::northstar::CREATE_OBJECT_FUNC;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::{mem, ptr};

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
        log::info!("CCommandResult");

        let ccommand = match unsafe { value.as_ref() } {
            Some(c) => c,
            None => return Self::default(),
        };
        let ccommand = *ccommand;

        let args = unsafe {
            log::info!("ccommand.m_nArgv0Size {}", ccommand.m_nArgv0Size);

            if ccommand.m_nArgv0Size == 0 {
                "".to_string()
            } else {
                let buffer = ccommand.m_pArgSBuffer.to_vec().as_ptr();
                CStr::from_ptr(buffer).to_string_lossy().into()
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
        let reg_func: ConCommandConstructorType = std::mem::transmute(ptr);

        Self { reg_func }
    }

    pub fn register_concommand(
        &self,
        name: String,
        callback: extern "C" fn(arg1: *const CCommand),
        help_string: String,
        flags: i32,
    ) -> Result<(), RegisterError> {
        
        let name = Box::new(to_sq_string!(name).into_bytes_with_nul())
            .into_raw_parts();
        let name_ptr = name.0 as *mut i8;

        let help_string = Box::new(to_sq_string!(help_string).into_bytes_with_nul())
            .into_raw_parts();
        let help_string_ptr = help_string.0 as *mut i8;

        let command: *mut ConCommand = unsafe {
            std::mem::transmute((CREATE_OBJECT_FUNC
                .wait()
                .ok_or(RegisterError::NoneFunction)?)(
                ObjectType_CONCOMMANDS
            ))
        };

        unsafe {
            self.reg_func.unwrap()(
                command,
                name_ptr,
                Some(callback),
                help_string_ptr,
                flags,
                ptr::null_mut(),
            )
        };

        unsafe {
            (*command).m_pCompletionCallback = Some(completion_callback);
        }

        Ok(())
    }
}

unsafe extern "C" fn completion_callback(whar: *const i8, whar2: *mut [i8; 128]) -> i32 {
    log::info!("called completion_callback");
    unsafe {
        log::info!("whar {}", *whar);

        log::info!("whar2 {:?}", CString::from_raw(mem::transmute_copy(&whar2)));
    }
    0
}
