#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::cbaseentity::CBaseEntity, field_assert, high::vector::Vector3, size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_RadialLinkController {
    pub base: CBaseEntity,
    pub m_flRadius: f32,            // +0x9e0 size: 0x4 (0x1 * 0x4) type 1
    pub m_vecAtRestOrigin: Vector3, // +0x9e4 size: 0xc (0x1 * 0xc) type 15
    pub m_bAtRest: bool,            // +0x9f0 size: 0x1 (0x1 * 0x1) type 6
}

size_assert!(CAI_RADIALLINKCONTROLLER where CAI_RadialLinkController == 0x9f8);
field_assert!(+ M_FLRADIUS where CAI_RadialLinkController, m_flRadius == 0x9d8);
field_assert!(+ M_VECATRESTORIGIN where CAI_RadialLinkController, m_vecAtRestOrigin == 0x9dc);
field_assert!(+ M_BATREST where CAI_RadialLinkController, m_bAtRest == 0x9e8);

impl DerefMut for CAI_RadialLinkController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_RadialLinkController {
    type Target = CBaseEntity;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
