#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{bindings::server::cbaseentity::CBaseEntity, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_DynamicLinkController {
    pub base: CBaseEntity,
    pub m_ControlledLinks: [u8; 32], // +0x9e0 size: 0x20 (0x1 * 0x20) type 0
    pub m_nLinkState: i32,           // +0xa00 size: 0x4 (0x1 * 0x4) type 5
    pub gap_a04: [u8; 4],
    pub m_strAllowUse: *mut char,  // +0xa08 size: 0x8 (0x1 * 0x8) type 2
    pub m_nPriority: i32,          // +0xa10 size: 0x4 (0x1 * 0x4) type 5
    pub m_bInvertAllow: bool,      // +0xa14 size: 0x1 (0x1 * 0x1) type 6
    pub m_bUseAirLinkRadius: bool, // +0xa15 size: 0x1 (0x1 * 0x1) type 6
}

size_assert!(CAI_DYNAMICLINKCONTROLLER where CAI_DynamicLinkController == 0xa18);
field_assert!(SIZE_CONTROLLEDLINKS where CAI_DynamicLinkController, m_ControlledLinks == 0x9d8);
field_assert!(SIZE_NLINKSTATE where CAI_DynamicLinkController, m_nLinkState == 0x9f8);
field_assert!(SIZE_STRALLOWUSE where CAI_DynamicLinkController, m_strAllowUse == 0xa00);
field_assert!(SIZE_NPRIORITY where CAI_DynamicLinkController, m_nPriority == 0xa08);
field_assert!(SIZE_BINVERTALLOW where CAI_DynamicLinkController, m_bInvertAllow == 0xa0c);
field_assert!(SIZE_BUSEAIRLINKRADIUS where CAI_DynamicLinkController, m_bUseAirLinkRadius == 0xa0d);

impl DerefMut for CAI_DynamicLinkController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_DynamicLinkController {
    type Target = CBaseEntity;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
