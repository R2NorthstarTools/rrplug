#![allow(non_camel_case_types, non_snake_case)]
use std::{
    ops::{Deref, DerefMut},
    os::raw::{c_char, c_void},
};

use crate::{bindings::server::cbaseentity::CBaseEntity, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_DynamicLink {
    pub base: CBaseEntity,
    pub m_pNextDynamicLink: *mut c_void, // +0x9e0 size: 0x8 (0x1 * 0x8) type 31
    pub m_nSrcEditID: i32,               // +0x9e8 size: 0x4 (0x1 * 0x4) type 5
    pub m_nDestEditID: i32,              // +0x9ec size: 0x4 (0x1 * 0x4) type 5
    pub m_nSrcID: i32,                   // +0x9f0 size: 0x4 (0x1 * 0x4) type 5
    pub m_nDestID: i32,                  // +0x9f4 size: 0x4 (0x1 * 0x4) type 5
    pub m_nLinkState: i32,               // +0x9f8 size: 0x4 (0x1 * 0x4) type 5
    pub gap_9fc: [u8; 4],
    pub m_strAllowUse: *mut c_char, // +0xa00 size: 0x8 (0x1 * 0x8) type 2
    pub m_bInvertAllow: bool,       // +0xa08 size: 0x1 (0x1 * 0x1) type 6
    pub m_bPreciseMovement: bool,   // +0xa09 size: 0x1 (0x1 * 0x1) type 6
    pub m_bFixedUpIds: bool,        // +0xa0a size: 0x1 (0x1 * 0x1) type 6
    pub m_bNotSaved: bool,          // +0xa0b size: 0x1 (0x1 * 0x1) type 6
    pub m_nLinkType: i32,           // +0xa0c size: 0x4 (0x1 * 0x4) type 5
    pub m_nPriority: i32,           // +0xa10 size: 0x4 (0x1 * 0x4) type 5
}

size_assert!(CAI_DYNAMICLINK where CAI_DynamicLink == 0xa18);
field_assert!(PNEXTDYNAMICLINK where CAI_DynamicLink, m_pNextDynamicLink == 0x9d8);
field_assert!(NSRCEDITID where CAI_DynamicLink, m_nSrcEditID == 0x9e0);
field_assert!(NDESTEDITID where CAI_DynamicLink, m_nDestEditID == 0x9e4);
field_assert!(NSRCID where CAI_DynamicLink, m_nSrcID == 0x9e8);
field_assert!(NDESTID where CAI_DynamicLink, m_nDestID == 0x9ec);
field_assert!(NLINKSTATE where CAI_DynamicLink, m_nLinkState == 0x9f0);
field_assert!(STRALLOWUSE where CAI_DynamicLink, m_strAllowUse == 0x9f8);
field_assert!(BINVERTALLOW where CAI_DynamicLink, m_bInvertAllow == 0xa00);
field_assert!(BPRECISEMOVEMENT where CAI_DynamicLink, m_bPreciseMovement == 0xa01);
field_assert!(BFIXEDUPIDS where CAI_DynamicLink, m_bFixedUpIds == 0xa02);
field_assert!(BNOTSAVED where CAI_DynamicLink, m_bNotSaved == 0xa03);
field_assert!(NLINKTYPE where CAI_DynamicLink, m_nLinkType == 0xa04);
field_assert!(NPRIORITY where CAI_DynamicLink, m_nPriority == 0xa08);

impl DerefMut for CAI_DynamicLink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_DynamicLink {
    type Target = CBaseEntity;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
