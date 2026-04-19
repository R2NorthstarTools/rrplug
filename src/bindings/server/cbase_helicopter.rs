#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::{
        cai_assault_behavior::CAI_AssaultBehavior, cai_followbehavior::CAI_FollowBehavior,
        cai_patrol_behavior::CAI_PatrolBehavior, cai_search_behavior::CAI_SearchBehavior,
        cbase_helicopter_behavior_host::CBaseHelicopterBehaviorHost, EHandle,
    },
    field_assert,
    prelude::Vector3,
    size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CBaseHelicopter {
    pub base: CBaseHelicopterBehaviorHost,
    pub m_flForce: f32,                // +0x2198 size: 0x4 (0x1 * 0x4) type 1
    pub m_vecAngAcceleration: Vector3, // +0x219c size: 0xc (0x1 * 0xc) type 3
    pub m_fMaxAngAcceleration: f32,    // +0x21a8 size: 0x4 (0x1 * 0x4) type 1
    pub m_vMaxAngVelocity: Vector3,    // +0x21ac size: 0xc (0x1 * 0xc) type 3
    pub m_vecDesiredFaceDir: Vector3,  // +0x21b8 size: 0xc (0x1 * 0xc) type 3
    pub m_distToNext: f32,             // +0x21c4 size: 0x4 (0x1 * 0x4) type 1
    pub m_flLastSeen: f32,             // +0x21c8 size: 0x4 (0x1 * 0x4) type 16
    pub m_flPrevSeen: f32,             // +0x21cc size: 0x4 (0x1 * 0x4) type 16
    pub m_vecTargetPosition: Vector3,  // +0x21d0 size: 0xc (0x1 * 0xc) type 15
    pub m_flGoalSpeed: f32,            // +0x21dc size: 0x4 (0x1 * 0x4) type 1
    pub m_flRandomOffsetTime: f32,     // +0x21e0 size: 0x4 (0x1 * 0x4) type 16
    pub m_vecRandomOffset: Vector3,    // +0x21e4 size: 0xc (0x1 * 0xc) type 3
    pub m_facingEntity: EHandle,       // +0x21f0 size: 0x4 (0x1 * 0x4) type 13
    pub gap_21f4: [u8; 4],
    pub m_AssaultBehavior: CAI_AssaultBehavior, // +0x21f8 size: 0xe8 (0x1 * 0xe8) type 10
    pub m_FollowBehavior: CAI_FollowBehavior,   // +0x22e0 size: 0x148 (0x1 * 0x148) type 10
    pub m_SearchBehavior: CAI_SearchBehavior,   // +0x2428 size: 0x50 (0x1 * 0x50) type 10
    pub m_PatrolBehavior: CAI_PatrolBehavior,   // +0x2478 size: 0x50 (0x1 * 0x50) type 10
    pub m_cullBoxMins: Vector3,                 // +0x24c8 size: 0xc (0x1 * 0xc) type 3
    pub m_cullBoxMaxs: Vector3,                 // +0x24d4 size: 0xc (0x1 * 0xc) type 3
}

size_assert!(CBASEHELICOPTER where CBaseHelicopter == 0x24e0);
field_assert!(M_FLFORCE where CBaseHelicopter, m_flForce == 0x2190);
field_assert!(M_VECANGACCELERATION where CBaseHelicopter, m_vecAngAcceleration == 0x2194);
field_assert!(M_FMAXANGACCELERATION where CBaseHelicopter, m_fMaxAngAcceleration == 0x21a0);
field_assert!(M_VMAXANGVELOCITY where CBaseHelicopter, m_vMaxAngVelocity == 0x21a4);
field_assert!(M_VECDESIREDFACEDIR where CBaseHelicopter, m_vecDesiredFaceDir == 0x21b0);
field_assert!(M_DISTTONEXT where CBaseHelicopter, m_distToNext == 0x21bc);
field_assert!(M_FLLASTSEEN where CBaseHelicopter, m_flLastSeen == 0x21c0);
field_assert!(M_FLPREVSEEN where CBaseHelicopter, m_flPrevSeen == 0x21c4);
field_assert!(M_VECTARGETPOSITION where CBaseHelicopter, m_vecTargetPosition == 0x21c8);
field_assert!(M_FLGOALSPEED where CBaseHelicopter, m_flGoalSpeed == 0x21d4);
field_assert!(M_FLRANDOMOFFSETTIME where CBaseHelicopter, m_flRandomOffsetTime == 0x21d8);
field_assert!(M_VECRANDOMOFFSET where CBaseHelicopter, m_vecRandomOffset == 0x21dc);
field_assert!(M_FACINGENTITY where CBaseHelicopter, m_facingEntity == 0x21e8);
field_assert!(M_ASSAULTBEHAVIOR where CBaseHelicopter, m_AssaultBehavior == 0x21f0);
field_assert!(M_FOLLOWBEHAVIOR where CBaseHelicopter, m_FollowBehavior == 0x22d8);
field_assert!(M_SEARCHBEHAVIOR where CBaseHelicopter, m_SearchBehavior == 0x2420);
field_assert!(M_PATROLBEHAVIOR where CBaseHelicopter, m_PatrolBehavior == 0x2470);
field_assert!(M_CULLBOXMINS where CBaseHelicopter, m_cullBoxMins == 0x24c0);
field_assert!(M_CULLBOXMAXS where CBaseHelicopter, m_cullBoxMaxs == 0x24cc);

impl DerefMut for CBaseHelicopter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CBaseHelicopter {
    type Target = CBaseHelicopterBehaviorHost;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
