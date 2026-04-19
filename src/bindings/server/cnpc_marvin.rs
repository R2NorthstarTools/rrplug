#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::{
        cai_assault_behavior::CAI_AssaultBehavior, cai_base_actor::CAI_BaseActor,
        cai_followbehavior::CAI_FollowBehavior, cai_patrol_behavior::CAI_PatrolBehavior,
        cai_search_behavior::CAI_SearchBehavior,
    },
    field_assert, size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CNPC_Marvin {
    pub base: CAI_BaseActor,
    pub m_AssaultBehavior: CAI_AssaultBehavior, // +0x21f0 size: 0xe8 (0x1 * 0xe8) type 10
    pub m_FollowBehavior: CAI_FollowBehavior,   // +0x22d8 size: 0x148 (0x1 * 0x148) type 10
    pub m_SearchBehavior: CAI_SearchBehavior,   // +0x2420 size: 0x50 (0x1 * 0x50) type 10
    pub m_PatrolBehavior: CAI_PatrolBehavior,   // +0x2470 size: 0x50 (0x1 * 0x50) type 10
}

size_assert!(CNPC_MARVIN where CNPC_Marvin == 0x24c0);
field_assert!(M_ASSAULTBEHAVIOR where CNPC_Marvin, m_AssaultBehavior == 0x21e8);
field_assert!(M_FOLLOWBEHAVIOR where CNPC_Marvin, m_FollowBehavior == 0x22d0);
field_assert!(M_SEARCHBEHAVIOR where CNPC_Marvin, m_SearchBehavior == 0x2418);
field_assert!(M_PATROLBEHAVIOR where CNPC_Marvin, m_PatrolBehavior == 0x2468);

impl DerefMut for CNPC_Marvin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CNPC_Marvin {
    type Target = CAI_BaseActor;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
