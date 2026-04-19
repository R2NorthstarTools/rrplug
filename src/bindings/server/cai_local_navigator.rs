#![allow(non_camel_case_types, non_snake_case)]
use std::{
    ops::{Deref, DerefMut},
    os::raw::c_void,
};

use crate::{bindings::server::cai_component::CAI_Component, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_LocalNavigator {
    pub base: CAI_Component,
    pub CAI_ProxyMovementSink: [u8; 16], // +0x10 size: 0x10 (0x0 * 0x10) type 0
    pub m_bLastWasClear: bool,           // +0x20 size: 0x1 (0x1 * 0x1) type 6
    pub gap_21: [u8; 7],
    pub m_LastMoveGoal: [u8; 216], // +0x28 size: 0xd8 (0x1 * 0xd8) type 10
    pub m_FullDirectTimer: [u8; 4], // +0x100 size: 0x4 (0x1 * 0x4) type 10
    pub gap_104: [u8; 4],
    pub m_pPlaneSolver: *mut c_void, // +0x108 size: 0x8 (0x1 * 0x8) type 31
    pub m_pMoveProbe: *mut c_void,   // +0x110 size: 0x8 (0x1 * 0x8) type 31
}

size_assert!(CAI_LOCALNAVIGATOR where CAI_LocalNavigator == 0x118);
field_assert!(CAI_PROXYMOVEMENTSINK where CAI_LocalNavigator, CAI_ProxyMovementSink == 0x8);
field_assert!(SIZE_BLASTWASCLEAR where CAI_LocalNavigator, m_bLastWasClear == 0x18);
field_assert!(SIZE_LASTMOVEGOAL where CAI_LocalNavigator, m_LastMoveGoal == 0x20);
field_assert!(SIZE_FULLDIRECTTIMER where CAI_LocalNavigator, m_FullDirectTimer == 0xf8);
field_assert!(SIZE_PPLANESOLVER where CAI_LocalNavigator, m_pPlaneSolver == 0x100);
field_assert!(SIZE_PMOVEPROBE where CAI_LocalNavigator, m_pMoveProbe == 0x108);

impl DerefMut for CAI_LocalNavigator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_LocalNavigator {
    type Target = CAI_Component;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
