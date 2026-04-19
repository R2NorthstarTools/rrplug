#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{bindings::server::cai_component::CAI_Component, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_MoveAndShootOverlay {
    pub base: CAI_Component,
    pub m_forcedMovement: i32, // +0x10 size: 0x4 (0x1 * 0x4) type 5
    pub m_forcedMovementExpireTime: f32, // +0x14 size: 0x4 (0x1 * 0x4) type 16
    pub m_isMovingAndShooting: bool, // +0x18 size: 0x1 (0x1 * 0x1) type 6
    pub m_disableMoveAndShoot: bool, // +0x19 size: 0x1 (0x1 * 0x1) type 6
}

size_assert!(CAI_MOVEANDSHOOTOVERLAY where CAI_MoveAndShootOverlay == 0x20);
field_assert!(FORCEDMOVEMENT where CAI_MoveAndShootOverlay, m_forcedMovement == 0x8);
field_assert!(FORCEDMOVEMENTEXPIRETIME where CAI_MoveAndShootOverlay, m_forcedMovementExpireTime == 0xc);
field_assert!(ISMOVINGANDSHOOTING where CAI_MoveAndShootOverlay, m_isMovingAndShooting == 0x10);
field_assert!(DISABLEMOVEANDSHOOT where CAI_MoveAndShootOverlay, m_disableMoveAndShoot == 0x11);

impl DerefMut for CAI_MoveAndShootOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_MoveAndShootOverlay {
    type Target = CAI_Component;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
