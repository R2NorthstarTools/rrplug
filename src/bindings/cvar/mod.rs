use crate::{create_external_interface, impl_vmethods};
use std::ffi::{c_char, c_void};

pub mod command;
pub mod convar;

use self::{
    command::{ConCommand, ConCommandBase},
    convar::ConVar,
};

create_external_interface! {
    pub RawCVar + Fffsf => {
        pub(self) fn unk0() -> ();
        pub(self) fn unk1() -> ();
        pub(self) fn unk2() -> ();
        pub(self) fn unk3() -> ();
        pub(self) fn unk4() -> ();
        pub(self) fn unk5() -> ();
        pub(self) fn unk6() -> ();
        pub(self) fn unk7() -> ();
        pub(self) fn unk8() -> ();
        pub(self) fn unk9() -> ();
        pub(self) fn unk10() -> ();
        pub(self) fn unk11() -> ();
        pub(self) fn unk12() -> ();
        pub(self) fn unk13() -> ();
        pub fn find_command_base(command_name: *const c_char) -> *mut ConCommandBase; // 0x14
        pub(self) fn unk15() -> ();
        pub fn find_convar(convar_name: *const c_char) -> *mut ConVar; // 0x17
        pub(self) fn unk16() -> ();
        pub fn find_concommand(concommand_name: *const c_char) -> *mut ConCommand; // 0x18
        pub(self) fn unk17() -> ();
        pub(self) fn unk18() -> ();
        pub(self) fn unk19() -> ();
        pub(self) fn unk20() -> ();
        pub(self) fn unk21() -> ();
        pub(self) fn unk22() -> ();
        pub(self) fn unk23() -> ();
        pub(self) fn unk24() -> ();
        pub(self) fn unk25() -> ();
        pub(self) fn unk26() -> ();
        pub(self) fn unk27() -> ();
        pub(self) fn unk28() -> ();
        pub(self) fn unk29() -> ();
        pub(self) fn unk30() -> ();
        pub(self) fn unk31() -> ();
        pub(self) fn unk32() -> ();
        pub(self) fn unk33() -> ();
        pub(self) fn unk34() -> ();
        pub(self) fn unk35() -> ();
        pub(self) fn unk36() -> ();
        pub(self) fn unk37() -> ();
        pub(self) fn unk38() -> ();
        pub(self) fn unk39() -> ();
        pub(self) fn unk40() -> ();
        pub fn get_cvar_raw_interator() -> *const RawCvarIterator; // 0x41
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
        pub fn set_first() -> () where offset(0);
        pub fn next() -> () where offset(1);
        pub fn is_valid() -> () where offset(2);
        pub fn get() -> *const ConCommandBase where offset(3);
    }
}
