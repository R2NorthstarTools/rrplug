#![allow(non_camel_case_types, non_snake_case)]
use crate::{bindings::server::EHandle, field_assert, high::vector::Vector3, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CSound {
    pub m_hOwner: EHandle,        // +0x0 size: 0x4 (0x1 * 0x4) type 13
    pub m_hTarget: EHandle,       // +0x4 size: 0x4 (0x1 * 0x4) type 13
    pub m_audibleRadius: f32,     // +0x8 size: 0x4 (0x1 * 0x4) type 1
    pub m_expireTime: f32,        // +0xc size: 0x4 (0x1 * 0x4) type 16
    pub m_ownerChannelIndex: i32, // +0x10 size: 0x4 (0x1 * 0x4) type 5
    pub m_iType: i32,             // +0x14 size: 0x4 (0x1 * 0x4) type 5
    pub m_vecOrigin: Vector3,     // +0x18 size: 0xc (0x1 * 0xc) type 15
    pub m_iNext: i16,             // +0x24 size: 0x2 (0x1 * 0x2) type 7
    pub m_bReserved: bool,        // +0x26 size: 0x1 (0x1 * 0x1) type 6
    pub m_ownerMustExist: bool,   // +0x27 size: 0x1 (0x1 * 0x1) type 6
}

size_assert!(CSOUND where CSound == 0x28);
field_assert!(M_HOWNER where CSound, m_hOwner == 0x0);
field_assert!(M_HTARGET where CSound, m_hTarget == 0x4);
field_assert!(M_AUDIBLERADIUS where CSound, m_audibleRadius == 0x8);
field_assert!(M_EXPIRETIME where CSound, m_expireTime == 0xc);
field_assert!(M_OWNERCHANNELINDEX where CSound, m_ownerChannelIndex == 0x10);
field_assert!(M_ITYPE where CSound, m_iType == 0x14);
field_assert!(M_VECORIGIN where CSound, m_vecOrigin == 0x18);
field_assert!(M_INEXT where CSound, m_iNext == 0x24);
field_assert!(M_BRESERVED where CSound, m_bReserved == 0x26);
field_assert!(M_OWNERMUSTEXIST where CSound, m_ownerMustExist == 0x27);
