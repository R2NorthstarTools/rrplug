#![allow(non_camel_case_types, non_snake_case)]
use std::{
    ops::{Deref, DerefMut},
    os::raw::c_char,
};

use crate::{bindings::server::cbaseentity::CBaseEntity, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_ChangeTarget {
    pub base: CBaseEntity,
    pub m_iszNewTarget: *mut c_char, // +0x9e0 size: 0x8 (0x1 * 0x8) type 2
}

size_assert!(A where CAI_ChangeTarget == 0x9e8);
field_assert!(B where CAI_ChangeTarget, m_iszNewTarget == 0x9d8);

impl DerefMut for CAI_ChangeTarget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_ChangeTarget {
    type Target = CBaseEntity;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
