#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{bindings::server::cai_combatant::CAI_Combatant, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CNPC_Soldier {
    pub base: CAI_Combatant,
    pub m_lastRunAwayCheckTime: f32, // +0x2500 size: 0x4 (0x1 * 0x4) type 1
    pub m_burstsBeforeRunAway: i8,   // +0x2504 size: 0x1 (0x1 * 0x1) type 8
}

size_assert!(CNPC_SOLDIER where CNPC_Soldier == 0x2508);
field_assert!(SIZE_LASTRUNAWAYCHECKTIME where CNPC_Soldier, m_lastRunAwayCheckTime == 0x24f8);
field_assert!(SIZE_BURSTSBEFORERUNAWAY where CNPC_Soldier, m_burstsBeforeRunAway == 0x24fc);

impl DerefMut for CNPC_Soldier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CNPC_Soldier {
    type Target = CAI_Combatant;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
