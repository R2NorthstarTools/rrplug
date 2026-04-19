#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::cai_component::CAI_Component, field_assert, high::vector::Vector3,
    size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_Enemies {
    pub base: CAI_Component,
    pub m_playerEnemyClass: i32, // +0x10 size: 0x4 (0x1 * 0x4) type 5
    pub gap_14: [u8; 4],
    pub m_Map: [u8; 48],                // +0x18 size: 0x30 (0x1 * 0x30) type 11
    pub m_flFreeKnowledgeDuration: f32, // +0x48 size: 0x4 (0x1 * 0x4) type 1
    pub m_flEnemyDiscardTime: f32,      // +0x4c size: 0x4 (0x1 * 0x4) type 1
    pub m_vecDefaultLKP: Vector3,       // +0x50 size: 0xc (0x1 * 0xc) type 15
    pub m_vecDefaultLSP: Vector3,       // +0x5c size: 0xc (0x1 * 0xc) type 15
    pub m_serial: i32,                  // +0x68 size: 0x4 (0x1 * 0x4) type 5
}

size_assert!(CAI_ENEMIES where CAI_Enemies == 0x70);
field_assert!(SIZE_PLAYERENEMYCLASS where CAI_Enemies, m_playerEnemyClass == 0x8);
field_assert!(SIZE_MAP where CAI_Enemies, m_Map == 0x10);
field_assert!(SIZE_FLFREEKNOWLEDGEDURATION where CAI_Enemies, m_flFreeKnowledgeDuration == 0x40);
field_assert!(SIZE_FLENEMYDISCARDTIME where CAI_Enemies, m_flEnemyDiscardTime == 0x44);
field_assert!(SIZE_VECDEFAULTLKP where CAI_Enemies, m_vecDefaultLKP == 0x48);
field_assert!(SIZE_VECDEFAULTLSP where CAI_Enemies, m_vecDefaultLSP == 0x54);
field_assert!(SIZE_SERIAL where CAI_Enemies, m_serial == 0x60);

impl DerefMut for CAI_Enemies {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_Enemies {
    type Target = CAI_Component;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
