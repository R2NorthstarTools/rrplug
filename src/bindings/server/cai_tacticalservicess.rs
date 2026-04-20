#![allow(non_camel_case_types, non_snake_case)]
use std::{
    ops::{Deref, DerefMut},
    os::raw::c_void,
};

use crate::{
    bindings::server::cai_component::CAI_Component, field_assert, prelude::Vector3, size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_TacticalServices {
    pub base: CAI_Component,
    pub m_LOSSearchDataIndex: i32, // +0x10 size: 0x4 (0x1 * 0x4) type 5
    pub m_strafeActivity: i32,     // +0x14 size: 0x4 (0x1 * 0x4) type 5
    pub m_firstIterationOfFindLos: bool, // +0x18 size: 0x1 (0x1 * 0x1) type 6
    pub gap_19: [u8; 7],
    pub m_pNetwork: *mut c_void,    // +0x20 size: 0x8 (0x1 * 0x8) type 31
    pub m_pPathfinder: *mut c_void, // +0x28 size: 0x8 (0x1 * 0x8) type 31
    pub m_prevLOSCheckSuccess: bool, // +0x30 size: 0x1 (0x1 * 0x1) type 6
    pub gap_31: [u8; 3],
    pub m_prevLOSCheckTime: f32,  // +0x34 size: 0x4 (0x1 * 0x4) type 1
    pub m_prevThreatPos: Vector3, // +0x38 size: 0xc (0x1 * 0xc) type 3
    pub m_prevSearchPos: Vector3, // +0x44 size: 0xc (0x1 * 0xc) type 3
}

size_assert!(CAI_TACTICALSERVICES where CAI_TacticalServices == 0x50);
field_assert!(+ LOSSEARCHDATAINDEX where CAI_TacticalServices, m_LOSSearchDataIndex == 0x8);
field_assert!(+ STRAFEACTIVITY where CAI_TacticalServices, m_strafeActivity == 0xc);
field_assert!(+ FIRSTITERATIONOFFINDLOS where CAI_TacticalServices, m_firstIterationOfFindLos == 0x10);
field_assert!(+ PNETWORK where CAI_TacticalServices, m_pNetwork == 0x18);
field_assert!(+ PPATHFINDER where CAI_TacticalServices, m_pPathfinder == 0x20);
field_assert!(+ PREVLOSCHECKSUCCESS where CAI_TacticalServices, m_prevLOSCheckSuccess == 0x28);
field_assert!(+ PREVLOSCHECKTIME where CAI_TacticalServices, m_prevLOSCheckTime == 0x2c);
field_assert!(+ PREVTHREATPOS where CAI_TacticalServices, m_prevThreatPos == 0x30);
field_assert!(+ PREVSEARCHPOS where CAI_TacticalServices, m_prevSearchPos == 0x3c);

impl DerefMut for CAI_TacticalServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_TacticalServices {
    type Target = CAI_Component;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
