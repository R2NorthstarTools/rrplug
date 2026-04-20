#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{bindings::server::cai_base_npc::CAI_BaseNPC, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_BaseActorBase {
    pub base: CAI_BaseNPC,
    pub m_bCalledBehaviorSelectSchedule: bool, // +0x20b0 size: 0x1 (0x1 * 0x1) type 6
}

size_assert!(SIZE where CAI_BaseActorBase == 0x20b8);
field_assert!(+ SIZE2_FISH where CAI_BaseActorBase, m_bCalledBehaviorSelectSchedule == 0x20a8);

impl DerefMut for CAI_BaseActorBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_BaseActorBase {
    type Target = CAI_BaseNPC;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
