#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::{cai_component::CAI_Component, csound::CSound, EHandle},
    field_assert, size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_Senses {
    pub base: CAI_Component,
    pub m_seenEnemiesCount: i32, // +0x10 size: 0x4 (0x1 * 0x4) type 5
    pub m_ClosestHighPriority: EHandle, // +0x14 size: 0x4 (0x1 * 0x4) type 13
    pub m_ClosestEnemyNPC: EHandle, // +0x18 size: 0x4 (0x1 * 0x4) type 13
    pub m_lookDistOverride: f32, // +0x1c size: 0x4 (0x1 * 0x4) type 1
    pub m_LastLookDist: f32,     // +0x20 size: 0x4 (0x1 * 0x4) type 1
    pub m_TimeLastLook: f32,     // +0x24 size: 0x4 (0x1 * 0x4) type 16
    pub m_hearingSensitivity: f32, // +0x28 size: 0x4 (0x1 * 0x4) type 1
    pub m_iAudibleList: i32,     // +0x2c size: 0x4 (0x1 * 0x4) type 5
    pub m_NextAudibleSound: [u8; 512], // +0x30 size: 0x200 (0x1 * 0x200) type 0
    pub m_reactToSound: CSound,  // +0x230 size: 0x28 (0x1 * 0x28) type 10
    pub m_sightProgress: [u8; 32], // +0x258 size: 0x20 (0x1 * 0x20) type 0
    pub m_SeenHighPriority: [u8; 32], // +0x278 size: 0x20 (0x1 * 0x20) type 11
    pub m_SeenNPCs: [u8; 32],    // +0x298 size: 0x20 (0x1 * 0x20) type 11
    pub m_SeenMisc: [u8; 32],    // +0x2b8 size: 0x20 (0x1 * 0x20) type 11
    pub m_TeamSpotted: [u8; 32], // +0x2d8 size: 0x20 (0x1 * 0x20) type 11
    pub m_SeenCorpseIndex: i32,  // +0x2f8 size: 0x4 (0x1 * 0x4) type 5
    pub m_bGatheringSeenEntities: bool, // +0x2fc size: 0x1 (0x1 * 0x1) type 6
    pub gap_2fd: [u8; 3],
    pub m_currentSeenArray: i32, // +0x300 size: 0x4 (0x1 * 0x4) type 5
    pub gap_304: [u8; 4],
    pub m_SeenArrays: [u8; 24], // +0x308 size: 0x18 (0x1 * 0x18) type 0
    pub m_TimeLastLookHighPriority: f32, // +0x320 size: 0x4 (0x1 * 0x4) type 16
    pub m_TimeLastLookNPCs: f32, // +0x324 size: 0x4 (0x1 * 0x4) type 16
    pub m_TimeLastLookMisc: f32, // +0x328 size: 0x4 (0x1 * 0x4) type 16
    pub m_TimeLastLookCorpse: f32, // +0x32c size: 0x4 (0x1 * 0x4) type 16
    pub m_iSensingFlags: i32,   // +0x330 size: 0x4 (0x1 * 0x4) type 5
    pub gap_334: [u8; 4],
    pub m_mutex: [u8; 40], // +0x338 size: 0x28 (0x1 * 0x28) type 0
}

size_assert!(CAI_SENSES where CAI_Senses == 0x360);
field_assert!(M_SEENENEMIESCOUNT where CAI_Senses, m_seenEnemiesCount == 0x8);
field_assert!(M_CLOSESTHIGHPRIORITY where CAI_Senses, m_ClosestHighPriority == 0xc);
field_assert!(M_CLOSESTENEMYNPC where CAI_Senses, m_ClosestEnemyNPC == 0x10);
field_assert!(M_LOOKDISTOVERRIDE where CAI_Senses, m_lookDistOverride == 0x14);
field_assert!(M_LASTLOOKDIST where CAI_Senses, m_LastLookDist == 0x18);
field_assert!(M_TIMELASTLOOK where CAI_Senses, m_TimeLastLook == 0x1c);
field_assert!(M_HEARINGSENSITIVITY where CAI_Senses, m_hearingSensitivity == 0x20);
field_assert!(M_IAUDIBLELIST where CAI_Senses, m_iAudibleList == 0x24);
field_assert!(M_NEXTAUDIBLESOUND where CAI_Senses, m_NextAudibleSound == 0x28);
field_assert!(M_REACTTOSOUND where CAI_Senses, m_reactToSound == 0x228);
field_assert!(M_SIGHTPROGRESS where CAI_Senses, m_sightProgress == 0x250);
field_assert!(M_SEENHIGHPRIORITY where CAI_Senses, m_SeenHighPriority == 0x270);
field_assert!(M_SEENNPCS where CAI_Senses, m_SeenNPCs == 0x290);
field_assert!(M_SEENMISC where CAI_Senses, m_SeenMisc == 0x2b0);
field_assert!(M_TEAMSPOTTED where CAI_Senses, m_TeamSpotted == 0x2d0);
field_assert!(M_SEENCORPSEINDEX where CAI_Senses, m_SeenCorpseIndex == 0x2f0);
field_assert!(M_BGATHERINGSEENENTITIES where CAI_Senses, m_bGatheringSeenEntities == 0x2f4);
field_assert!(M_CURRENTSEENARRAY where CAI_Senses, m_currentSeenArray == 0x2f8);
field_assert!(M_SEENARRAYS where CAI_Senses, m_SeenArrays == 0x300);
field_assert!(M_TIMELASTLOOKHIGHPRIORITY where CAI_Senses, m_TimeLastLookHighPriority == 0x318);
field_assert!(M_TIMELASTLOOKNPCS where CAI_Senses, m_TimeLastLookNPCs == 0x31c);
field_assert!(M_TIMELASTLOOKMISC where CAI_Senses, m_TimeLastLookMisc == 0x320);
field_assert!(M_TIMELASTLOOKCORPSE where CAI_Senses, m_TimeLastLookCorpse == 0x324);
field_assert!(M_ISENSINGFLAGS where CAI_Senses, m_iSensingFlags == 0x328);
field_assert!(M_MUTEX where CAI_Senses, m_mutex == 0x330);

impl DerefMut for CAI_Senses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_Senses {
    type Target = CAI_Component;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
