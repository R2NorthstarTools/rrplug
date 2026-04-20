#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::{
        cai_assault_behavior::CAI_AssaultBehavior, cai_base_actor::CAI_BaseActor,
        cai_followbehavior::CAI_FollowBehavior,
    },
    field_assert, size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CNPC_MeleeOnly {
    pub base: CAI_BaseActor,
    pub m_AssaultBehavior: CAI_AssaultBehavior, // +0x21f0 size: 0xe8 (0x1 * 0xe8) type 10
    pub m_FollowBehavior: CAI_FollowBehavior,   // +0x22d8 size: 0x148 (0x1 * 0x148) type 10
}

size_assert!(CNPC_MELEEONLY where CNPC_MeleeOnly == 0x2420);
field_assert!(+ M_ASSAULTBEHAVIOR where CNPC_MeleeOnly, m_AssaultBehavior == 0x21e8);
field_assert!(+ M_FOLLOWBEHAVIOR where CNPC_MeleeOnly, m_FollowBehavior == 0x22d0);

impl DerefMut for CNPC_MeleeOnly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CNPC_MeleeOnly {
    type Target = CAI_BaseActor;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
