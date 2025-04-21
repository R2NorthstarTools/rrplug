#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use super::{cbaseentity::CBaseEntity, cplayer::EHandle};

use crate::size_assert;

#[repr(C)]
pub struct CWeaponX {
    pub base: CBaseEntity,
    pub gap_9e0: [u8; 1240],
    pub m_weaponOwner: EHandle, // 0xeb8
    pub gap_8: [u8; 1788],
    pub currentModBitfield: i32, // 0x15B8
    pub gap_15BC: [u8; 324],
    // is a struct in reality.
    pub weaponVars: [u8; 0xCA0],
}
size_assert!(SIZE_BASE where CBaseEntity == 0x9E0);

impl DerefMut for CWeaponX {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CWeaponX {
    type Target = CBaseEntity;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
