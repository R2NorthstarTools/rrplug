use crate::impl_vmethods;
use std::ffi::{c_char, c_void};

pub mod command;
pub mod convar;

use self::{
    command::{ConCommand, ConCommandBase},
    convar::ConVar,
};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawCVar {
    class: *const c_void,
}

#[doc(hidden)]
impl From<*const c_void> for RawCVar {
    /// unsafe since this can take any ptr
    fn from(class: *const c_void) -> Self {
        Self { class }
    }
}

impl_vmethods! {
    impl WRAPPER RawCVar {
        pub fn find_command_base( command_name: *const c_char ) -> *mut ConCommandBase where offset(14);
        pub fn find_convar( convar_name: *const c_char ) -> *mut ConVar where offset(16);
        pub fn find_concommand( concommand_name: *const c_char ) -> *mut ConCommand where offset(18);
        pub fn get_cvar_raw_interator() -> *const ConCommandBase where offset(41);
    }
}

#[derive(Debug, Clone)]
pub struct RawCvarIterator {
    class: *const c_void,
}

#[doc(hidden)]
impl From<*const c_void> for RawCvarIterator {
    /// unsafe since this can take any ptr
    fn from(class: *const c_void) -> Self {
        Self { class }
    }
}

impl_vmethods! {
    impl WRAPPER RawCvarIterator {
        pub fn set_first() -> ()  where offset(0);
        pub fn next() -> ()  where offset(1);
        pub fn is_valid() -> ()  where offset(2);
        pub fn get() -> *const ConCommandBase  where offset(3);
    }
}
