#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::cbase_helicopter::CBaseHelicopter, field_assert, prelude::Vector3,
    size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CNPC_Gunship {
    pub base: CBaseHelicopter,
    pub m_accel: Vector3,             // +0x24e0 size: 0xc (0x1 * 0xc) type 3
    pub m_vecAttackPosition: Vector3, // +0x24ec size: 0xc (0x1 * 0xc) type 3
    pub m_vecAttackVelocity: Vector3, // +0x24f8 size: 0xc (0x1 * 0xc) type 3
}

size_assert!(CNPC_GUNSHIP where CNPC_Gunship == 0x2508);
field_assert!(+ SIZE_ACCEL where CNPC_Gunship, m_accel == 0x24d8);
field_assert!(+ SIZE_VECATTACKPOSITION where CNPC_Gunship, m_vecAttackPosition == 0x24e4);
field_assert!(+ SIZE_VECATTACKVELOCITY where CNPC_Gunship, m_vecAttackVelocity == 0x24f0);

impl DerefMut for CNPC_Gunship {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CNPC_Gunship {
    type Target = CBaseHelicopter;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
