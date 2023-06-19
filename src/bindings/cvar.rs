use crate::impl_vmethods;
use std::ffi::{c_char, c_void};

use super::{
    command::{ConCommand, ConCommandBase},
    convar::ConVar,
};

type SizeHint = extern "C" fn();

pub struct RawCVar {
    vtable_adr: *const SizeHint,
}

#[doc(hidden)]
impl From<*const c_void> for RawCVar {
    fn from(vtable_adr: *const c_void) -> Self {
        Self {
            vtable_adr: vtable_adr as *const SizeHint,
        }
    }
}

impl_vmethods! {
    pub fn find_command_base( command_name: *const c_char ) -> *const ConCommandBase, for RawCVar, offset 14;
    pub fn find_convar( convar_name: *const c_char ) -> *const ConVar, for RawCVar, offset 16;
    pub fn find_concommand( concommand_name: *const c_char ) -> *const ConCommand, for RawCVar, offset 18;
    pub fn get_cvar_raw_interator() -> *const ConCommandBase, for RawCVar, offset 41;
}

pub struct RawCvarIterator {
    vtable_adr: *const SizeHint,
}

#[doc(hidden)]
impl From<*const c_void> for RawCvarIterator {
    fn from(vtable_adr: *const c_void) -> Self {
        Self {
            vtable_adr: vtable_adr as *const SizeHint,
        }
    }
}

impl_vmethods! {
    pub fn set_first() -> (), for RawCvarIterator, offset 0;
    pub fn next() -> (), for RawCvarIterator, offset 1;
    pub fn is_valid() -> (), for RawCvarIterator, offset 2;
    pub fn get() -> *const ConCommandBase, for RawCvarIterator, offset 3;
}
