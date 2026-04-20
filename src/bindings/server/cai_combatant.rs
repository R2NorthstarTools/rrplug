#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::{
        cai_assault_behavior::CAI_AssaultBehavior, cai_base_actor::CAI_BaseActor,
        cai_followbehavior::CAI_FollowBehavior, cai_patrol_behavior::CAI_PatrolBehavior,
        cai_search_behavior::CAI_SearchBehavior,
    },
    field_assert,
    high::vector::Vector3,
    size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_Combatant {
    pub base: CAI_BaseActor,
    pub m_flNextPainSoundTime: f32, // +0x21f0 size: 0x4 (0x1 * 0x4) type 16
    pub m_flNextAlertSoundTime: f32, // +0x21f4 size: 0x4 (0x1 * 0x4) type 16
    pub m_flNextLostSoundTime: f32, // +0x21f8 size: 0x4 (0x1 * 0x4) type 16
    pub m_flAlertPatrolTime: f32,   // +0x21fc size: 0x4 (0x1 * 0x4) type 16
    pub m_flNextPieOffTime: f32,    // +0x2200 size: 0x4 (0x1 * 0x4) type 16
    pub gap_2204: [u8; 4],
    pub m_AssaultBehavior: CAI_AssaultBehavior, // +0x2208 size: 0xe8 (0x1 * 0xe8) type 10
    pub m_FollowBehavior: CAI_FollowBehavior,   // +0x22f0 size: 0x148 (0x1 * 0x148) type 10
    pub m_SearchBehavior: CAI_SearchBehavior,   // +0x2438 size: 0x50 (0x1 * 0x50) type 10
    pub m_PatrolBehavior: CAI_PatrolBehavior,   // +0x2488 size: 0x50 (0x1 * 0x50) type 10
    pub m_snipingState: i32,                    // +0x24d8 size: 0x4 (0x1 * 0x4) type 5
    pub m_snipingRetry: i32,                    // +0x24dc size: 0x4 (0x1 * 0x4) type 5
    pub m_sniperShots: i32,                     // +0x24e0 size: 0x4 (0x1 * 0x4) type 5
    pub m_maxSnipingRetry: i32,                 // +0x24e4 size: 0x4 (0x1 * 0x4) type 5
    pub m_maxSniperShots: i32,                  // +0x24e8 size: 0x4 (0x1 * 0x4) type 5
    pub m_vecAltFireTarget: Vector3,            // +0x24ec size: 0xc (0x1 * 0xc) type 3
    pub m_iTacticalVariant: i32,                // +0x24f8 size: 0x4 (0x1 * 0x4) type 5
    pub m_iPathfindingVariant: i32,             // +0x24fc size: 0x4 (0x1 * 0x4) type 5
}

size_assert!(CAI_COMBATANT  where CAI_Combatant == 0x2500);
field_assert!(+ SIZE_FLNEXTPAINSOUNDTIME  where CAI_Combatant, m_flNextPainSoundTime == 0x21e8);
field_assert!(+ SIZE_FLNEXTALERTSOUNDTIME  where CAI_Combatant, m_flNextAlertSoundTime == 0x21ec);
field_assert!(+ SIZE_FLNEXTLOSTSOUNDTIME  where CAI_Combatant, m_flNextLostSoundTime == 0x21f0);
field_assert!(+ SIZE_FLALERTPATROLTIME  where CAI_Combatant, m_flAlertPatrolTime == 0x21f4);
field_assert!(+ SIZE_FLNEXTPIEOFFTIME  where CAI_Combatant, m_flNextPieOffTime == 0x21f8);
field_assert!(+ SIZE_ASSAULTBEHAVIOR  where CAI_Combatant, m_AssaultBehavior == 0x2200);
field_assert!(+ SIZE_FOLLOWBEHAVIOR  where CAI_Combatant, m_FollowBehavior == 0x22e8);
field_assert!(+ SIZE_SEARCHBEHAVIOR  where CAI_Combatant, m_SearchBehavior == 0x2430);
field_assert!(+ SIZE_PATROLBEHAVIOR  where CAI_Combatant, m_PatrolBehavior == 0x2480);
field_assert!(+ SIZE_SNIPINGSTATE  where CAI_Combatant, m_snipingState == 0x24d0);
field_assert!(+ SIZE_SNIPINGRETRY  where CAI_Combatant, m_snipingRetry == 0x24d4);
field_assert!(+ SIZE_SNIPERSHOTS  where CAI_Combatant, m_sniperShots == 0x24d8);
field_assert!(+ SIZE_MAXSNIPINGRETRY  where CAI_Combatant, m_maxSnipingRetry == 0x24dc);
field_assert!(+ SIZE_MAXSNIPERSHOTS  where CAI_Combatant, m_maxSniperShots == 0x24e0);
field_assert!(+ SIZE_VECALTFIRETARGET  where CAI_Combatant, m_vecAltFireTarget == 0x24e4);
field_assert!(+ SIZE_ITACTICALVARIANT  where CAI_Combatant, m_iTacticalVariant == 0x24f0);
field_assert!(+ SIZE_IPATHFINDINGVARIANT  where CAI_Combatant, m_iPathfindingVariant == 0x24f4);

impl DerefMut for CAI_Combatant {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_Combatant {
    type Target = CAI_BaseActor;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
