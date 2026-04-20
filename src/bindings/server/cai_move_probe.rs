#![allow(non_camel_case_types, non_snake_case)]
use std::{
    ops::{Deref, DerefMut},
    os::raw::c_void,
};

use crate::{
    bindings::server::{cai_component::CAI_Component, EHandle},
    field_assert,
    high::vector::Vector3,
    size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_MoveProbe {
    pub base: CAI_Component,
    pub m_bIgnoreTransientEntities: bool, // +0x10 size: 0x1 (0x1 * 0x1) type 6
    pub gap_11: [u8; 3],
    pub m_vecFloorPoint: Vector3, // +0x14 size: 0xc (0x1 * 0xc) type 3
    pub m_floorPointTime: f32,    // +0x20 size: 0x4 (0x1 * 0x4) type 16
    pub m_floorPointStandable: bool, // +0x24 size: 0x1 (0x1 * 0x1) type 6
    pub gap_25: [u8; 3],
    pub m_pTraceListData: *mut c_void, // +0x28 size: 0x8 (0x1 * 0x8) type 31
    pub m_hLastProbeBlockingEnt: EHandle, // +0x30 size: 0x4 (0x1 * 0x4) type 13
}

size_assert!(CAI_MOVEPROBE where CAI_MoveProbe == 0x38);
field_assert!(+ SIZE_BIGNORETRANSIENTENTITIES where CAI_MoveProbe, m_bIgnoreTransientEntities == 0x8);
field_assert!(+ SIZE_VECFLOORPOINT where CAI_MoveProbe, m_vecFloorPoint == 0xc);
field_assert!(+ SIZE_FLOORPOINTTIME where CAI_MoveProbe, m_floorPointTime == 0x18);
field_assert!(+ SIZE_FLOORPOINTSTANDABLE where CAI_MoveProbe, m_floorPointStandable == 0x1c);
field_assert!(+ SIZE_PTRACELISTDATA where CAI_MoveProbe, m_pTraceListData == 0x20);
field_assert!(+ SIZE_HLASTPROBEBLOCKINGENT where CAI_MoveProbe, m_hLastProbeBlockingEnt == 0x28);

impl DerefMut for CAI_MoveProbe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_MoveProbe {
    type Target = CAI_Component;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
