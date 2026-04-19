#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{bindings::server::cai_base_npc::CAI_BaseNPC, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_TestHull {
    pub base: CAI_BaseNPC,
    pub bInUse: bool, // +0x20b0 size: 0x1 (0x1 * 0x1) type 6
}

size_assert!(A where CAI_TestHull == 0x20b8);
field_assert!(B where CAI_TestHull, bInUse == 0x20a8);

impl DerefMut for CAI_TestHull {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_TestHull {
    type Target = CAI_BaseNPC;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
