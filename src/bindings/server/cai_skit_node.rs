#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{bindings::server::cbaseentity::CBaseEntity, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_SkitNode {
    pub base: CBaseEntity,
    pub m_distanceFromRef: f32, // +0x9e0 size: 0x4 (0x1 * 0x4) type 1
    pub m_reserved: bool,       // +0x9e4 size: 0x1 (0x1 * 0x1) type 6
}

size_assert!(CAI_SKITNODE where CAI_SkitNode == 0x9e8);
field_assert!(M_DISTANCEFROMREF where CAI_SkitNode, m_distanceFromRef == 0x9d8);
field_assert!(M_RESERVED where CAI_SkitNode, m_reserved == 0x9dc);

impl DerefMut for CAI_SkitNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_SkitNode {
    type Target = CBaseEntity;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
