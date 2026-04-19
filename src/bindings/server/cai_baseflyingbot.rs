#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::{
        cai_assault_behavior::CAI_AssaultBehavior, cai_baseactorbase::CAI_BaseActorBase,
        cai_followbehavior::CAI_FollowBehavior, cai_patrol_behavior::CAI_PatrolBehavior,
        cai_search_behavior::CAI_SearchBehavior, cplayer::CTakeDamageInfo,
    },
    field_assert,
    high::vector::Vector3,
    size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_BaseFlyingBot {
    pub base: CAI_BaseActorBase,
    pub m_vCurrentVelocity: Vector3, // +0x20b8 size: 0xc (0x1 * 0xc) type 3
    pub m_vCurrentAngularVelocity: Vector3, // +0x20c4 size: 0xc (0x1 * 0xc) type 3
    pub m_vCurrentBanking: Vector3,  // +0x20d0 size: 0xc (0x1 * 0xc) type 3
    pub m_vNoiseMod: Vector3,        // +0x20dc size: 0xc (0x1 * 0xc) type 3
    pub m_fHeadYaw: f32,             // +0x20e8 size: 0x4 (0x1 * 0x4) type 1
    pub m_fNextYawChangeSoundTime: f32, // +0x20ec size: 0x4 (0x1 * 0x4) type 1
    pub m_vLastPatrolDir: Vector3,   // +0x20f0 size: 0xc (0x1 * 0xc) type 3
    pub m_vecDesiredVel: Vector3,    // +0x20fc size: 0xc (0x1 * 0xc) type 3
    pub m_flAccelTime: f32,          // +0x2108 size: 0x4 (0x1 * 0x4) type 16
    pub m_KilledInfo: CTakeDamageInfo, // +0x210c size: 0x78 (0x1 * 0x78) type 10
    pub gap_2184: [u8; 4],
    pub m_AssaultBehavior: CAI_AssaultBehavior, // +0x2188 size: 0xe8 (0x1 * 0xe8) type 10
    pub m_FollowBehavior: CAI_FollowBehavior,   // +0x2270 size: 0x148 (0x1 * 0x148) type 10
    pub m_SearchBehavior: CAI_SearchBehavior,   // +0x23b8 size: 0x50 (0x1 * 0x50) type 10
    pub m_PatrolBehavior: CAI_PatrolBehavior,   // +0x2408 size: 0x50 (0x1 * 0x50) type 10
}

size_assert!(CAI_BASEFLYINGBOT where  CAI_BaseFlyingBot == 0x2458);
field_assert!(SIZE_VCURRENTVELOCITY where  CAI_BaseFlyingBot, m_vCurrentVelocity == 0x20b0);
field_assert!(SIZE_VCURRENTANGULARVELOCITY where  CAI_BaseFlyingBot, m_vCurrentAngularVelocity == 0x20bc);
field_assert!(SIZE_VCURRENTBANKING where  CAI_BaseFlyingBot, m_vCurrentBanking == 0x20c8);
field_assert!(SIZE_VNOISEMOD where  CAI_BaseFlyingBot, m_vNoiseMod == 0x20d4);
field_assert!(SIZE_FHEADYAW where  CAI_BaseFlyingBot, m_fHeadYaw == 0x20e0);
field_assert!(SIZE_FNEXTYAWCHANGESOUNDTIME where  CAI_BaseFlyingBot, m_fNextYawChangeSoundTime == 0x20e4);
field_assert!(SIZE_VLASTPATROLDIR where  CAI_BaseFlyingBot, m_vLastPatrolDir == 0x20e8);
field_assert!(SIZE_VECDESIREDVEL where  CAI_BaseFlyingBot, m_vecDesiredVel == 0x20f4);
field_assert!(SIZE_FLACCELTIME where  CAI_BaseFlyingBot, m_flAccelTime == 0x2100);
field_assert!(SIZE_KILLEDINFO where  CAI_BaseFlyingBot, m_KilledInfo == 0x2104);
field_assert!(SIZE_ASSAULTBEHAVIOR where  CAI_BaseFlyingBot, m_AssaultBehavior == 0x2180);
field_assert!(SIZE_FOLLOWBEHAVIOR where  CAI_BaseFlyingBot, m_FollowBehavior == 0x2268);
field_assert!(SIZE_SEARCHBEHAVIOR where  CAI_BaseFlyingBot, m_SearchBehavior == 0x23b0);
field_assert!(SIZE_PATROLBEHAVIOR where  CAI_BaseFlyingBot, m_PatrolBehavior == 0x2400);

impl DerefMut for CAI_BaseFlyingBot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_BaseFlyingBot {
    type Target = CAI_BaseActorBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
