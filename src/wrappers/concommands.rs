use cxx::private::c_char;
use std::ffi::CStr;
use std::os::raw::c_void;

use crate::bindings::cxx_binds::__concommand::CCommand;
use crate::to_sq_string;

// WARNING: this is not the right function, 
// this is passed instead
// typedef void (*ConCommandConstructorType)( ConCommand* newCommand, const char* name, FnCommandCallback_t callback, const char* helpString, int flags, void* parent);
// must be reworked
// or ask emma to include the function we want
// or pr it
// the reason we need it is since in rust there is no way to create a c++ class
// >:(
type RegisterConCommandFunction = unsafe extern "C" fn(
    name: *const c_char,
    callback: extern "C" fn(arg1: &CCommand), // must be converted to a ptr
    helpString: *const c_char,
    flags: i32,
);

pub struct CCommandResult {
    pub command: String,
    pub args: String,
}

impl From<&CCommand> for CCommandResult {
    fn from(value: &CCommand) -> Self {
        let command = unsafe {
            CStr::from_ptr(value.GetCommandString())
                .to_string_lossy()
                .to_string()
        };

        let args = unsafe { CStr::from_ptr(value.ArgS()).to_string_lossy().to_string() };

        Self { command, args }
    }
}

pub struct RegisterConCommands {
    reg_func: RegisterConCommandFunction,
}

impl RegisterConCommands {
    pub(crate) unsafe fn new(ptr: *const c_void) -> Self {
        let reg_func: *const RegisterConCommandFunction = std::mem::transmute(ptr);

        Self {
            reg_func: *reg_func,
        }
    }

    pub fn register_concommand(
        &self,
        name: String,
        callback: extern "C" fn(arg1: &CCommand),
        help_string: String,
        flags: i32,
    ) {
        let name = to_sq_string!(name);
        let help_string = to_sq_string!(help_string);

        unsafe { (self.reg_func)(name.as_ptr(), callback, help_string.as_ptr(), flags) };
    }
}
