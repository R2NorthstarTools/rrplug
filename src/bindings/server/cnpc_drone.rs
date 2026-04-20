#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::{cai_baseflyingbot::CAI_BaseFlyingBot, EHandle},
    field_assert, size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CNPC_Drone {
    pub base: CAI_BaseFlyingBot,
    pub m_bAttackTarget: bool, // +0x2458 size: 0x1 (0x1 * 0x1) type 6
    pub gap_2459: [u8; 3],
    pub m_beamSpread: f32,     // +0x245c size: 0x4 (0x1 * 0x4) type 1
    pub m_scanTarget: EHandle, // +0x2460 size: 0x4 (0x1 * 0x4) type 13
}

size_assert!(CNPC_DRONE where CNPC_Drone == 0x2468);
field_assert!(+ WHRE_BATTACKTARGET where CNPC_Drone, m_bAttackTarget == 0x2450);
field_assert!(+ WHRE_BEAMSPREAD where CNPC_Drone, m_beamSpread == 0x2454);
field_assert!(+ WHRE_SCANTARGET where CNPC_Drone, m_scanTarget == 0x2458);

impl DerefMut for CNPC_Drone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CNPC_Drone {
    type Target = CAI_BaseFlyingBot;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
