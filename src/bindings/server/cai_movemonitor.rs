#![allow(non_camel_case_types, non_snake_case)]
use crate::{field_assert, high::vector::Vector3, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_MoveMonitor {
    pub m_markPos: Vector3,      // +0x0 size: 0xc (0x1 * 0xc) type 15
    pub m_markToleranceSqr: f32, // +0xc size: 0x4 (0x1 * 0x4) type 1
    pub m_markSerialNumber: i32, // +0x10 size: 0x4 (0x1 * 0x4) type 5
    pub m_markSet: bool,         // +0x14 size: 0x1 (0x1 * 0x1) type 6
}

size_assert!(CAI_MoveMonitor where CAI_MoveMonitor == 0x18);
field_assert!(m_markPos where CAI_MoveMonitor, m_markPos == 0x0);
field_assert!(m_markToleranceSqr where CAI_MoveMonitor, m_markToleranceSqr == 0xc);
field_assert!(m_markSerialNumber where CAI_MoveMonitor, m_markSerialNumber == 0x10);
field_assert!(m_markSet where CAI_MoveMonitor, m_markSet == 0x14);
