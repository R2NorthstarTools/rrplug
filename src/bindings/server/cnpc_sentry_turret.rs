#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::{cai_base_npc::CAI_BaseNPC, EHandle},
    field_assert,
    prelude::Vector3,
    size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CNPC_SentryTurret {
    pub base: CAI_BaseNPC,
    pub m_turretState: i32,            // +0x20b0 size: 0x4 (0x1 * 0x4) type 5
    pub m_killCount: i32,              // +0x20b4 size: 0x4 (0x1 * 0x4) type 5
    pub m_titanKillCount: i32,         // +0x20b8 size: 0x4 (0x1 * 0x4) type 5
    pub m_eyeAttach: i32,              // +0x20bc size: 0x4 (0x1 * 0x4) type 5
    pub m_controlPanel: EHandle,       // +0x20c0 size: 0x4 (0x1 * 0x4) type 13
    pub m_bActive: bool,               // +0x20c4 size: 0x1 (0x1 * 0x1) type 6
    pub m_bEnabled: bool,              // +0x20c5 size: 0x1 (0x1 * 0x1) type 6
    pub m_bPitchSound: bool,           // +0x20c6 size: 0x1 (0x1 * 0x1) type 6
    pub m_bYawSound: bool,             // +0x20c7 size: 0x1 (0x1 * 0x1) type 6
    pub m_firstMuzzleAttachment: i32,  // +0x20c8 size: 0x4 (0x1 * 0x4) type 5
    pub m_numMuzzles: i32,             // +0x20cc size: 0x4 (0x1 * 0x4) type 5
    pub m_currentMuzzle: i32,          // +0x20d0 size: 0x4 (0x1 * 0x4) type 5
    pub m_vecGoalAngles: Vector3,      // +0x20d4 size: 0xc (0x1 * 0xc) type 3
    pub m_vecLastEnemyAngles: Vector3, // +0x20e0 size: 0xc (0x1 * 0xc) type 3
    pub m_vecLastDir: Vector3,         // +0x20ec size: 0xc (0x1 * 0xc) type 3
    pub m_OnDeploy: [u8; 40],          // +0x20f8 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnRetire: [u8; 40],          // +0x2120 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnTurretStateChange: [u8; 40], // +0x2148 size: 0x28 (0x1 * 0x28) type 11
    pub m_MaxYaw: f32,                 // +0x2170 size: 0x4 (0x1 * 0x4) type 1
}

size_assert!(CNPC_SENTRYTURRET where CNPC_SentryTurret == 0x2178);
field_assert!(M_TURRETSTATE where CNPC_SentryTurret, m_turretState == 0x20a8);
field_assert!(M_KILLCOUNT where CNPC_SentryTurret, m_killCount == 0x20ac);
field_assert!(M_TITANKILLCOUNT where CNPC_SentryTurret, m_titanKillCount == 0x20b0);
field_assert!(M_EYEATTACH where CNPC_SentryTurret, m_eyeAttach == 0x20b4);
field_assert!(M_CONTROLPANEL where CNPC_SentryTurret, m_controlPanel == 0x20b8);
field_assert!(M_BACTIVE where CNPC_SentryTurret, m_bActive == 0x20bc);
field_assert!(M_BENABLED where CNPC_SentryTurret, m_bEnabled == 0x20bd);
field_assert!(M_BPITCHSOUND where CNPC_SentryTurret, m_bPitchSound == 0x20be);
field_assert!(M_BYAWSOUND where CNPC_SentryTurret, m_bYawSound == 0x20bf);
field_assert!(M_FIRSTMUZZLEATTACHMENT where CNPC_SentryTurret, m_firstMuzzleAttachment == 0x20c0);
field_assert!(M_NUMMUZZLES where CNPC_SentryTurret, m_numMuzzles == 0x20c4);
field_assert!(M_CURRENTMUZZLE where CNPC_SentryTurret, m_currentMuzzle == 0x20c8);
field_assert!(M_VECGOALANGLES where CNPC_SentryTurret, m_vecGoalAngles == 0x20cc);
field_assert!(M_VECLASTENEMYANGLES where CNPC_SentryTurret, m_vecLastEnemyAngles == 0x20d8);
field_assert!(M_VECLASTDIR where CNPC_SentryTurret, m_vecLastDir == 0x20e4);
field_assert!(M_ONDEPLOY where CNPC_SentryTurret, m_OnDeploy == 0x20f0);
field_assert!(M_ONRETIRE where CNPC_SentryTurret, m_OnRetire == 0x2118);
field_assert!(M_ONTURRETSTATECHANGE where CNPC_SentryTurret, m_OnTurretStateChange == 0x2140);
field_assert!(M_MAXYAW where CNPC_SentryTurret, m_MaxYaw == 0x2168);

impl DerefMut for CNPC_SentryTurret {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CNPC_SentryTurret {
    type Target = CAI_BaseNPC;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
