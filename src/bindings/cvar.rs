use crate::impl_vmethods;
use std::ffi::{c_char, c_void};

use super::{
    command::{ConCommand, ConCommandBase},
    convar::ConVar,
};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawCVar {
    vtable_adr: *const c_void,
}

#[doc(hidden)]
impl From<*const c_void> for RawCVar {
    /// unsafe since this can take any ptr
    fn from(vtable_adr: *const c_void) -> Self {
        Self { vtable_adr }
    }
}

impl_vmethods! {
    impl RawCVar {
        pub fn find_command_base( command_name: *const c_char ) -> *mut ConCommandBase where offset(14);
        pub fn find_convar( convar_name: *const c_char ) -> *mut ConVar where offset(16);
        pub fn find_concommand( concommand_name: *const c_char ) -> *mut ConCommand where offset(18);
        pub fn get_cvar_raw_interator() -> *const ConCommandBase where offset(41);
    }
}

#[derive(Debug, Clone)]
pub struct RawCvarIterator {
    vtable_adr: *const c_void,
}

#[doc(hidden)]
impl From<*const c_void> for RawCvarIterator {
    /// unsafe since this can take any ptr
    fn from(vtable_adr: *const c_void) -> Self {
        Self { vtable_adr }
    }
}

impl_vmethods! {
    impl RawCvarIterator {
        pub fn set_first() -> ()  where offset(0);
        pub fn next() -> ()  where offset(1);
        pub fn is_valid() -> ()  where offset(2);
        pub fn get() -> *const ConCommandBase  where offset(3);
    }
}
