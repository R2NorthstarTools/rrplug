#![allow(non_camel_case_types, non_snake_case)]
use std::{
    ops::{Deref, DerefMut},
    os::raw::c_void,
};

use crate::{bindings::server::cai_component::CAI_Component, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_BehaviorBase {
    pub base: CAI_Component,
    pub IAI_BehaviorBridge: *mut c_void, // +0x10 size: 0x8 (0x0 * 0x8) type 0
    pub m_ScheduleChannels: [u8; 32],    // +0x18 size: 0x20 (0x1 * 0x20) type 11
    pub m_fOverrode: bool,               // +0x38 size: 0x1 (0x1 * 0x1) type 6
    pub gap_39: [u8; 7],
    pub m_pBackBridge: *mut c_void, // +0x40 size: 0x8 (0x1 * 0x8) type 31
    pub m_bAllocated: bool,         // +0x48 size: 0x1 (0x1 * 0x1) type 6
    pub m_bBehaviorEnabled: bool,   // +0x49 size: 0x1 (0x1 * 0x1) type 6
    pub gap_4a: [u8; 2],
    pub m_behaviorType: i32, // +0x4c size: 0x4 (0x1 * 0x4) type 5
}

size_assert!(CAI_BEHAVIORBASE where  CAI_BehaviorBase == 0x50);
field_assert!(+ IAI_BEHAVIORBRIDGE where  CAI_BehaviorBase, IAI_BehaviorBridge == 0x8);
field_assert!(+ M_SCHEDULECHANNELS where  CAI_BehaviorBase, m_ScheduleChannels == 0x10);
field_assert!(+ M_FOVERRODE where  CAI_BehaviorBase, m_fOverrode == 0x30);
field_assert!(+ M_PBACKBRIDGE where  CAI_BehaviorBase, m_pBackBridge == 0x38);
field_assert!(+ M_BALLOCATED where  CAI_BehaviorBase, m_bAllocated == 0x40);
field_assert!(+ M_BBEHAVIORENABLED where  CAI_BehaviorBase, m_bBehaviorEnabled == 0x41);
field_assert!(+ M_BEHAVIORTYPE where  CAI_BehaviorBase, m_behaviorType == 0x44);

impl DerefMut for CAI_BehaviorBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_BehaviorBase {
    type Target = CAI_Component;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
