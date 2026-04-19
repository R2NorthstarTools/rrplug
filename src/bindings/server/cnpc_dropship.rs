#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::{cbase_helicopter::CBaseHelicopter, EHandle},
    field_assert, size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CNPC_Dropship {
    pub base: CBaseHelicopter,
    pub m_flTimeTakeOff: f32,      // +0x24e0 size: 0x4 (0x1 * 0x4) type 16
    pub m_flTimeNextAttack: f32,   // +0x24e4 size: 0x4 (0x1 * 0x4) type 16
    pub m_flLastTime: f32,         // +0x24e8 size: 0x4 (0x1 * 0x4) type 16
    pub m_iDropState: i32,         // +0x24ec size: 0x4 (0x1 * 0x4) type 5
    pub m_iLandState: i32,         // +0x24f0 size: 0x4 (0x1 * 0x4) type 5
    pub m_existPitch: f32,         // +0x24f4 size: 0x4 (0x1 * 0x4) type 1
    pub m_existRoll: f32,          // +0x24f8 size: 0x4 (0x1 * 0x4) type 1
    pub m_bJetWakeFXEnabled: bool, // +0x24fc size: 0x1 (0x1 * 0x1) type 6
    pub gap_24fd: [u8; 3],
    pub m_hLandTarget: EHandle, // +0x2500 size: 0x4 (0x1 * 0x4) type 13
}

size_assert!(CNPC_DROPSHIP where CNPC_Dropship == 0x2508);
field_assert!(M_FLTIMETAKEOFF where CNPC_Dropship, m_flTimeTakeOff == 0x24d8);
field_assert!(M_FLTIMENEXTATTACK where CNPC_Dropship, m_flTimeNextAttack == 0x24dc);
field_assert!(M_FLLASTTIME where CNPC_Dropship, m_flLastTime == 0x24e0);
field_assert!(M_IDROPSTATE where CNPC_Dropship, m_iDropState == 0x24e4);
field_assert!(M_ILANDSTATE where CNPC_Dropship, m_iLandState == 0x24e8);
field_assert!(M_EXISTPITCH where CNPC_Dropship, m_existPitch == 0x24ec);
field_assert!(M_EXISTROLL where CNPC_Dropship, m_existRoll == 0x24f0);
field_assert!(M_BJETWAKEFXENABLED where CNPC_Dropship, m_bJetWakeFXEnabled == 0x24f4);
field_assert!(M_HLANDTARGET where CNPC_Dropship, m_hLandTarget == 0x24f8);

impl DerefMut for CNPC_Dropship {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CNPC_Dropship {
    type Target = CBaseHelicopter;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
