#![allow(non_camel_case_types, non_snake_case)]
use std::{
    ops::{Deref, DerefMut},
    os::raw::{c_char, c_void},
};

use crate::{
    bindings::server::{
        cai_enemies::CAI_Enemies,
        cai_local_navigator::CAI_LocalNavigator,
        cai_motor::CAI_Motor,
        cai_move_probe::CAI_MoveProbe,
        cai_moveandshootoverlay::CAI_MoveAndShootOverlay,
        cai_navigator::CAI_Navigator,
        cai_pathfinder::CAI_Pathfinder,
        cai_senses::CAI_Senses,
        cai_shot_regulator::CAI_ShotRegulator,
        cai_tacticalservicess::CAI_TacticalServices,
        cbasecombatcharacter::CBaseCombatCharacter,
        cbaseentity::CBaseEntity,
        cplayer::{StatusEffectEndlessData, StatusEffectTimedData},
        crand_stopwatch::CRandStopwatch,
        csimple_sim_timer::CSimpleSimTimer,
        csound::CSound,
        cstopwatch::CStopwatch,
        EHandle,
    },
    field_assert,
    prelude::Vector3,
    size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct AIScheduleState_t {
    pub iCurTask: i32,           // +0x0 size: 0x4 (0x1 * 0x4) type 5
    pub fTaskStatus: i32,        // +0x4 size: 0x4 (0x1 * 0x4) type 5
    pub timeStarted: f32,        // +0x8 size: 0x4 (0x1 * 0x4) type 16
    pub timeCurTaskStarted: f32, // +0xc size: 0x4 (0x1 * 0x4) type 16
    pub failedSchedID: i32,      // +0x10 size: 0x4 (0x1 * 0x4) type 5
    pub gap_14: [u8; 4],
    pub taskFailureCode: i64,          // +0x18 size: 0x8 (0x1 * 0x8) type 29
    pub iTaskInterrupt: i32,           // +0x20 size: 0x4 (0x1 * 0x4) type 5
    pub bTaskRanAutomovement: bool,    // +0x24 size: 0x1 (0x1 * 0x1) type 6
    pub bTaskRequestRunTillWait: bool, // +0x25 size: 0x1 (0x1 * 0x1) type 6
    pub doYawUpdatesForThisDecisionInterval: bool, // +0x26 size: 0x1 (0x1 * 0x1) type 6
    pub doYawUpdatesDuringThisTask: bool, // +0x27 size: 0x1 (0x1 * 0x1) type 6
    pub updateIdealYawToEnemyDuringThisTask: bool, // +0x28 size: 0x1 (0x1 * 0x1) type 6
    pub bScheduleWasInterrupted: bool, // +0x29 size: 0x1 (0x1 * 0x1) type 6
    pub bStopRunTasks: bool,           // +0x2a size: 0x1 (0x1 * 0x1) type 6
}

size_assert!(AISCHEDULESTATE where AIScheduleState_t == 0x30);
field_assert!(SIZE_ICURTASK where AIScheduleState_t, iCurTask == 0x0);
field_assert!(SIZE_FTASKSTATUS where AIScheduleState_t, fTaskStatus == 0x4);
field_assert!(SIZE_TIMESTARTED where AIScheduleState_t, timeStarted == 0x8);
field_assert!(SIZE_TIMECURTASKSTARTED where AIScheduleState_t, timeCurTaskStarted == 0xc);
field_assert!(SIZE_FAILEDSCHEDID where AIScheduleState_t, failedSchedID == 0x10);
field_assert!(SIZE_TASKFAILURECODE where AIScheduleState_t, taskFailureCode == 0x18);
field_assert!(SIZE_ITASKINTERRUPT where AIScheduleState_t, iTaskInterrupt == 0x20);
field_assert!(SIZE_BTASKRANAUTOMOVEMENT where AIScheduleState_t, bTaskRanAutomovement == 0x24);
field_assert!(SIZE_BTASKREQUESTRUNTILLWAIT where AIScheduleState_t, bTaskRequestRunTillWait == 0x25);
field_assert!(SIZE_DOYAWUPDATESFORTHISDECISIONINTERVAL where AIScheduleState_t, doYawUpdatesForThisDecisionInterval == 0x26);
field_assert!(SIZE_DOYAWUPDATESDURINGTHISTASK where AIScheduleState_t, doYawUpdatesDuringThisTask == 0x27);
field_assert!(SIZE_UPDATEIDEALYAWTOENEMYDURINGTHISTASK where AIScheduleState_t, updateIdealYawToEnemyDuringThisTask == 0x28);
field_assert!(SIZE_BSCHEDULEWASINTERRUPTED where AIScheduleState_t, bScheduleWasInterrupted == 0x29);
field_assert!(SIZE_BSTOPRUNTASKS where AIScheduleState_t, bStopRunTasks == 0x2a);

#[repr(C)]
#[derive(Debug)]
pub struct ShootingCoverState {
    pub lockedHint: *mut CBaseEntity, // +0x0 size: 0x8 (0x1 * 0x8) type 12
    pub advancesLeftFromLockedHint: i32, // +0x8 size: 0x4 (0x1 * 0x4) type 5
    pub burstCount: i32,              // +0xc size: 0x4 (0x1 * 0x4) type 5
    pub burstStartTime: f32,          // +0x10 size: 0x4 (0x1 * 0x4) type 16
    pub burstCompletedSuccessfully: bool, // +0x14 size: 0x1 (0x1 * 0x1) type 6
    pub burstUsingLeanAnims: bool,    // +0x15 size: 0x1 (0x1 * 0x1) type 6
    pub usingPotentialThreatPos: bool, // +0x16 size: 0x1 (0x1 * 0x1) type 6
    pub gap_17: [u8; 1],
    pub potentialThreatPos: Vector3, // +0x18 size: 0xc (0x1 * 0xc) type 3
    pub nextMoveToCoverRetryTime: f32, // +0x24 size: 0x4 (0x1 * 0x4) type 16
    pub hasApproxLOSToEnemy_expireTime: f32, // +0x28 size: 0x4 (0x1 * 0x4) type 16
    pub hasApproxLOSToEnemy_checkTime: f32, // +0x2c size: 0x4 (0x1 * 0x4) type 16
    pub recentlyUsedCover: [RecentlyUsedCover; 5], // +0x30 size: 0x10 (0x5 * 0x3) type 10
    pub recentlyUsedCoverNewestIdx: i32, // +0x80 size: 0x4 (0x1 * 0x4) type 5
    pub consecutiveIdleSchedCount: i32, // +0x84 size: 0x4 (0x1 * 0x4) type 5
    pub numberOfIdlesBeforeSchedCheck: i32, // +0x88 size: 0x4 (0x1 * 0x4) type 5
    pub onPathToShootingCoverFail_retryTime: f32, // +0x8c size: 0x4 (0x1 * 0x4) type 16
    pub onTooCloseForCover_expireTime: f32, // +0x90 size: 0x4 (0x1 * 0x4) type 16
}

size_assert!(SIZE_SHOOTINGCOVERSTATE where ShootingCoverState == 0x98);
field_assert!(SIZE_LOCKEDHINT where ShootingCoverState, lockedHint == 0x0);
field_assert!(SIZE_ADVANCESLEFTFROMLOCKEDHINT where ShootingCoverState, advancesLeftFromLockedHint == 0x8);
field_assert!(SIZE_BURSTCOUNT where ShootingCoverState, burstCount == 0xc);
field_assert!(SIZE_BURSTSTARTTIME where ShootingCoverState, burstStartTime == 0x10);
field_assert!(SIZE_BURSTCOMPLETEDSUCCESSFULLY where ShootingCoverState, burstCompletedSuccessfully == 0x14);
field_assert!(SIZE_BURSTUSINGLEANANIMS where ShootingCoverState, burstUsingLeanAnims == 0x15);
field_assert!(SIZE_USINGPOTENTIALTHREATPOS where ShootingCoverState, usingPotentialThreatPos == 0x16);
field_assert!(SIZE_POTENTIALTHREATPOS where ShootingCoverState, potentialThreatPos == 0x18);
field_assert!(SIZE_NEXTMOVETOCOVERRETRYTIME where ShootingCoverState, nextMoveToCoverRetryTime == 0x24);
field_assert!(SIZE_HASAPPROXLOSTOENEMY_EXPIRETIME where ShootingCoverState, hasApproxLOSToEnemy_expireTime == 0x28);
field_assert!(SIZE_HASAPPROXLOSTOENEMY_CHECKTIME where ShootingCoverState, hasApproxLOSToEnemy_checkTime == 0x2c);
field_assert!(SIZE_RECENTLYUSEDCOVER where ShootingCoverState, recentlyUsedCover == 0x30);
field_assert!(SIZE_RECENTLYUSEDCOVERNEWESTIDX where ShootingCoverState, recentlyUsedCoverNewestIdx == 0x80);
field_assert!(SIZE_CONSECUTIVEIDLESCHEDCOUNT where ShootingCoverState, consecutiveIdleSchedCount == 0x84);
field_assert!(SIZE_NUMBEROFIDLESBEFORESCHEDCHECK where ShootingCoverState, numberOfIdlesBeforeSchedCheck == 0x88);
field_assert!(SIZE_ONPATHTOSHOOTINGCOVERFAIL_RETRYTIME where ShootingCoverState, onPathToShootingCoverFail_retryTime == 0x8c);
field_assert!(SIZE_ONTOOCLOSEFORCOVER_EXPIRETIME where ShootingCoverState, onTooCloseForCover_expireTime == 0x90);

#[repr(C)]
#[derive(Debug)]
pub struct RecentlyUsedCover {
    pub hint: *mut CBaseEntity, // +0x0 size: 0x8 (0x1 * 0x8) type 12
    pub nextUseTime: f32,       // +0x8 size: 0x4 (0x1 * 0x4) type 16
}

size_assert!(RECENTLYUSEDCOVER where RecentlyUsedCover== 0x10);
field_assert!(SIZE_HINT where RecentlyUsedCover, hint== 0x0);
field_assert!(SIZE_NEXTUSETIME where RecentlyUsedCover, nextUseTime== 0x8);

#[repr(C)]
#[derive(Debug)]
pub struct AISyncedMeleeState {
    pub desiredMelee: *mut c_void,  // +0x0 size: 0x8 (0x1 * 0x8) type 0
    pub additionalYaw: f32,         // +0x8 size: 0x4 (0x1 * 0x4) type 1
    pub meleePartner: EHandle,      // +0xc size: 0x4 (0x1 * 0x4) type 13
    pub ragdollOnInterrupt: bool,   // +0x10 size: 0x1 (0x1 * 0x1) type 6
    pub continueAnimToFinish: bool, // +0x11 size: 0x1 (0x1 * 0x1) type 6
    pub gap_12: [u8; 6],
    pub debouncedMelees: [u8; 48], // +0x18 size: 0x30 (0x1 * 0x30) type 11
    pub pressToInitiate_debounceExpireTime: f32, // +0x48 size: 0x4 (0x1 * 0x4) type 16
    pub pressToInitiate_pos: Vector3, // +0x4c size: 0xc (0x1 * 0xc) type 15
    pub pressToInitiate_dist: f32, // +0x58 size: 0x4 (0x1 * 0x4) type 1
}

size_assert!(SIZE_AISYNCEDMELEESTATE  where AISyncedMeleeState == 0x60);
field_assert!(SIZE_DESIREDMELEE  where AISyncedMeleeState, desiredMelee == 0x0);
field_assert!(SIZE_ADDITIONALYAW  where AISyncedMeleeState, additionalYaw == 0x8);
field_assert!(SIZE_MELEEPARTNER  where AISyncedMeleeState, meleePartner == 0xc);
field_assert!(SIZE_RAGDOLLONINTERRUPT  where AISyncedMeleeState, ragdollOnInterrupt == 0x10);
field_assert!(SIZE_CONTINUEANIMTOFINISH  where AISyncedMeleeState, continueAnimToFinish == 0x11);
field_assert!(SIZE_DEBOUNCEDMELEES  where AISyncedMeleeState, debouncedMelees == 0x18);
field_assert!(SIZE_PRESSTOINITIATE_DEBOUNCEEXPIRETIME  where AISyncedMeleeState, pressToInitiate_debounceExpireTime == 0x48);
field_assert!(SIZE_PRESSTOINITIATE_POS  where AISyncedMeleeState, pressToInitiate_pos == 0x4c);
field_assert!(SIZE_PRESSTOINITIATE_DIST  where AISyncedMeleeState, pressToInitiate_dist == 0x58);

#[repr(C)]
#[derive(Debug)]
pub struct CAI_BaseNPC {
    pub base: CBaseCombatCharacter,
    pub CAI_DefMovementSink: *mut c_void, // +0x1498 size: 0x8 (0x0 * 0x8) type 0
    pub IAI_BehaviorBridge: *mut c_void,  // +0x14a0 size: 0x8 (0x0 * 0x8) type 0
    pub m_threadedPostProcessJob: i32,    // +0x14a8 size: 0x4 (0x1 * 0x4) type 5
    pub m_bDoPostProcess: bool,           // +0x14ac size: 0x1 (0x1 * 0x1) type 6
    pub m_bCustomEnemySearch: bool,       // +0x14ad size: 0x1 (0x1 * 0x1) type 6
    pub m_bPlayerSpottable: bool,         // +0x14ae size: 0x1 (0x1 * 0x1) type 6
    pub m_bAddedToSpottableList: bool,    // +0x14af size: 0x1 (0x1 * 0x1) type 6
    pub m_pPrevSchedule: *mut c_void,     // +0x14b0 size: 0x8 (0x1 * 0x8) type 31
    pub m_pSchedule: *mut c_void,         // +0x14b8 size: 0x8 (0x1 * 0x8) type 31
    pub m_defaultScriptScheduleID: i32,   // +0x14c0 size: 0x4 (0x1 * 0x4) type 5
    pub gap_14c4: [u8; 4],
    pub m_ScheduleState: AIScheduleState_t, // +0x14c8 size: 0x30 (0x1 * 0x30) type 10
    pub m_failSchedule: i32,                // +0x14f8 size: 0x4 (0x1 * 0x4) type 5
    pub m_bDoPostRestoreRefindPath: bool,   // +0x14fc size: 0x1 (0x1 * 0x1) type 6
    pub m_bDoScheduleChangeSignal: bool,    // +0x14fd size: 0x1 (0x1 * 0x1) type 6
    pub m_spawned: bool,                    // +0x14fe size: 0x1 (0x1 * 0x1) type 6
    pub m_bUsingStandardThinkTime: bool,    // +0x14ff size: 0x1 (0x1 * 0x1) type 6
    pub m_flLastRealThinkTime: f32,         // +0x1500 size: 0x4 (0x1 * 0x4) type 16
    pub m_flLastThinkDuration: f32,         // +0x1504 size: 0x4 (0x1 * 0x4) type 16
    pub m_iFrameBlocked: i32,               // +0x1508 size: 0x4 (0x1 * 0x4) type 5
    pub gap_150c: [u8; 4],
    pub m_pScheduleEvent: *mut c_void, // +0x1510 size: 0x8 (0x1 * 0x8) type 31
    pub m_dangerousClusterConditionAllowedAtTime: f32, // +0x1518 size: 0x4 (0x1 * 0x4) type 1
    pub m_poseAim_Pitch: i32,          // +0x151c size: 0x4 (0x1 * 0x4) type 5
    pub m_poseAim_Yaw: i32,            // +0x1520 size: 0x4 (0x1 * 0x4) type 5
    pub m_poseMove_Yaw: i32,           // +0x1524 size: 0x4 (0x1 * 0x4) type 5
    pub m_poseMove_Lean: i32,          // +0x1528 size: 0x4 (0x1 * 0x4) type 5
    pub m_offsetOfCurrentAimFromDesiredAim_Cos: f32, // +0x152c size: 0x4 (0x1 * 0x4) type 1
    pub m_haveDesiredAimDir: bool,     // +0x1530 size: 0x1 (0x1 * 0x1) type 6
    pub m_sequenceHasAimEvent: bool,   // +0x1531 size: 0x1 (0x1 * 0x1) type 6
    pub m_sequenceHasAimPoseParams: bool, // +0x1532 size: 0x1 (0x1 * 0x1) type 6
    pub gap_1533: [u8; 1],
    pub m_lastAimSequence: i32, // +0x1534 size: 0x4 (0x1 * 0x4) type 5
    pub m_missPlayerLastWindow: i32, // +0x1538 size: 0x4 (0x1 * 0x4) type 5
    pub m_missPlayerLastOffset: Vector3, // +0x153c size: 0xc (0x1 * 0xc) type 3
    pub m_pPrimaryBehavior: *mut c_void, // +0x1548 size: 0x8 (0x1 * 0x8) type 31
    pub m_Behaviors: [u8; 32],  // +0x1550 size: 0x20 (0x1 * 0x20) type 0
    pub m_Conditions: [u8; 12], // +0x1570 size: 0xc (0x1 * 0xc) type 0
    pub m_CustomInterruptConditions: [u8; 12], // +0x157c size: 0xc (0x1 * 0xc) type 0
    pub m_bForceConditionsGather: bool, // +0x1588 size: 0x1 (0x1 * 0x1) type 6
    pub m_bConditionsGathered: bool, // +0x1589 size: 0x1 (0x1 * 0x1) type 6
    pub m_bInterruptableByScript: bool, // +0x158a size: 0x1 (0x1 * 0x1) type 6
    pub gap_158b: [u8; 1],
    pub m_movingGroundEnt: EHandle, // +0x158c size: 0x4 (0x1 * 0x4) type 13
    pub m_groundRelativePos: Vector3, // +0x1590 size: 0xc (0x1 * 0xc) type 3
    pub m_groundRelativeAngles: Vector3, // +0x159c size: 0xc (0x1 * 0xc) type 3
    pub m_NPCState: i32,            // +0x15a8 size: 0x4 (0x1 * 0x4) type 5
    pub m_NPCPrevState: i32,        // +0x15ac size: 0x4 (0x1 * 0x4) type 5
    pub m_NPCAlertnessState: i32,   // +0x15b0 size: 0x4 (0x1 * 0x4) type 5
    pub m_lastSchedSelectorState: i32, // +0x15b4 size: 0x4 (0x1 * 0x4) type 5
    pub m_flLastSchedSelectChangeTime: f32, // +0x15b8 size: 0x4 (0x1 * 0x4) type 1
    pub m_flLastStateChangeTime: f32, // +0x15bc size: 0x4 (0x1 * 0x4) type 1
    pub m_thinkInterval: f32,       // +0x15c0 size: 0x4 (0x1 * 0x4) type 1
    pub m_IdealNPCState: i32,       // +0x15c4 size: 0x4 (0x1 * 0x4) type 5
    pub m_Efficiency: i32,          // +0x15c8 size: 0x4 (0x1 * 0x4) type 5
    pub m_MoveEfficiency: i32,      // +0x15cc size: 0x4 (0x1 * 0x4) type 5
    pub m_flNextDecisionTime: f32,  // +0x15d0 size: 0x4 (0x1 * 0x4) type 16
    pub m_searchPathID: i16,        // +0x15d4 size: 0x2 (0x1 * 0x2) type 7
    pub m_bDefenseActive: bool,     // +0x15d6 size: 0x1 (0x1 * 0x1) type 6
    pub m_bAlwaysAlert: bool,       // +0x15d7 size: 0x1 (0x1 * 0x1) type 6
    pub m_scriptedAnimForceInterrupt: bool, // +0x15d8 size: 0x1 (0x1 * 0x1) type 6
    pub m_bWakeSquad: bool,         // +0x15d9 size: 0x1 (0x1 * 0x1) type 6
    pub gap_15da: [u8; 2],
    pub m_flWakeRadius: f32,           // +0x15dc size: 0x4 (0x1 * 0x4) type 1
    pub m_nWakeTick: u32,              // +0x15e0 size: 0x4 (0x1 * 0x4) type 17
    pub m_SleepState: i32,             // +0x15e4 size: 0x4 (0x1 * 0x4) type 5
    pub m_SleepFlags: i32,             // +0x15e8 size: 0x4 (0x1 * 0x4) type 5
    pub m_translatedActivity: [u8; 4], // +0x15ec size: 0x4 (0x1 * 0x4) type 11
    pub m_IdealActivity: [u8; 4],      // +0x15f0 size: 0x4 (0x1 * 0x4) type 11
    pub m_nIdealSequence: i32,         // +0x15f4 size: 0x4 (0x1 * 0x4) type 5
    pub m_IdealTranslatedActivity: [u8; 4], // +0x15f8 size: 0x4 (0x1 * 0x4) type 11
    pub m_IdealWeaponActivity: [u8; 4], // +0x15fc size: 0x4 (0x1 * 0x4) type 11
    pub m_idealSequenceAlreadyResolved: bool, // +0x1600 size: 0x1 (0x1 * 0x1) type 6
    pub gap_1601: [u8; 3],
    pub m_seqFinishedInSolid: i32, // +0x1604 size: 0x4 (0x1 * 0x4) type 5
    pub m_activeActModifiers: [u8; 64], // +0x1608 size: 0x40 (0x1 * 0x40) type 11
    pub m_scriptIdleSequence: i32, // +0x1648 size: 0x4 (0x1 * 0x4) type 5
    pub m_scriptAttackSequence: i32, // +0x164c size: 0x4 (0x1 * 0x4) type 5
    pub m_scriptDeathActivity: i32, // +0x1650 size: 0x4 (0x1 * 0x4) type 5
    pub m_didSetIndoorActivityOverride: bool, // +0x1654 size: 0x1 (0x1 * 0x1) type 6
    pub m_isInsideIndoorArea: bool, // +0x1655 size: 0x1 (0x1 * 0x1) type 6
    pub m_requestSpecialRangeAttack: bool, // +0x1656 size: 0x1 (0x1 * 0x1) type 6
    pub gap_1657: [u8; 1],
    pub m_specialRangeAttackCount: i32, // +0x1658 size: 0x4 (0x1 * 0x4) type 5
    pub gap_165c: [u8; 4],
    pub m_aiSettings: *mut c_char, // +0x1660 size: 0x8 (0x1 * 0x8) type 2
    pub m_aiSettingsIndex: i32,    // +0x1668 size: 0x4 (0x1 * 0x4) type 5
    pub m_subclass: i32,           // +0x166c size: 0x4 (0x1 * 0x4) type 5
    pub m_pSenses: *mut CAI_Senses, // +0x1670 size: 0x8 (0x1 * 0x8) type 10
    pub m_pLockedBestSound: CSound, // +0x1678 size: 0x28 (0x1 * 0x28) type 10
    pub m_aimDir: Vector3,         // +0x16a0 size: 0xc (0x1 * 0xc) type 3
    pub m_aimDirValid: bool,       // +0x16ac size: 0x1 (0x1 * 0x1) type 6
    pub gap_16ad: [u8; 3],
    pub m_weaponBlockedTimer: f32, // +0x16b0 size: 0x4 (0x1 * 0x4) type 1
    pub m_weaponBlockedTimeOut: f32, // +0x16b4 size: 0x4 (0x1 * 0x4) type 1
    pub m_moveDirection: i32,      // +0x16b8 size: 0x4 (0x1 * 0x4) type 5
    pub m_hEnemy: EHandle,         // +0x16bc size: 0x4 (0x1 * 0x4) type 13
    pub m_hEnemySecondary: EHandle, // +0x16c0 size: 0x4 (0x1 * 0x4) type 13
    pub m_distToEnemyLKP_AdjustForHeightDiff: f32, // +0x16c4 size: 0x4 (0x1 * 0x4) type 1
    pub m_distToEnemyLKP: f32,     // +0x16c8 size: 0x4 (0x1 * 0x4) type 1
    pub m_distToEnemyLKPCenterToCenter: f32, // +0x16cc size: 0x4 (0x1 * 0x4) type 1
    pub m_hTargetEnt: EHandle,     // +0x16d0 size: 0x4 (0x1 * 0x4) type 13
    pub gap_16d4: [u8; 4],
    pub m_updateSquadEnemyQueue: [u8; 32], // +0x16d8 size: 0x20 (0x1 * 0x20) type 0
    pub m_notifyEnemyToSquadTimer: CStopwatch, // +0x16f8 size: 0xc (0x1 * 0xc) type 10
    pub m_notifyEnemyToTeamTime: f32,      // +0x1704 size: 0x4 (0x1 * 0x4) type 16
    pub m_GiveUpOnDeadEnemyTimer: CRandStopwatch, // +0x1708 size: 0x10 (0x1 * 0x10) type 10
    pub m_chooseEnemySanityTimer: CSimpleSimTimer, // +0x1718 size: 0x4 (0x1 * 0x4) type 10
    pub m_EnemiesSerialNumber: i32,        // +0x171c size: 0x4 (0x1 * 0x4) type 5
    pub m_flAcceptableTimeSeenEnemy: f32,  // +0x1720 size: 0x4 (0x1 * 0x4) type 16
    pub m_UpdateEnemyPosTimer: CSimpleSimTimer, // +0x1724 size: 0x4 (0x1 * 0x4) type 10
    pub m_ForceUpdateEnemyPos: bool,       // +0x1728 size: 0x1 (0x1 * 0x1) type 6
    pub gap_1729: [u8; 3],
    pub m_afCapability: i32,         // +0x172c size: 0x4 (0x1 * 0x4) type 5
    pub m_flags: i32,                // +0x1730 size: 0x4 (0x1 * 0x4) type 5
    pub m_flMoveWaitFinished: f32,   // +0x1734 size: 0x4 (0x1 * 0x4) type 16
    pub m_hTempBlockingEnt: EHandle, // +0x1738 size: 0x4 (0x1 * 0x4) type 13
    pub gap_173c: [u8; 4],
    pub m_UnreachableEnts: [u8; 32], // +0x1740 size: 0x20 (0x1 * 0x20) type 11
    pub m_pNavigator: *mut CAI_Navigator, // +0x1760 size: 0x8 (0x1 * 0x8) type 10
    pub m_pLocalNavigator: *mut CAI_LocalNavigator, // +0x1768 size: 0x8 (0x1 * 0x8) type 10
    pub m_pPathfinder: *mut CAI_Pathfinder, // +0x1770 size: 0x8 (0x1 * 0x8) type 10
    pub m_pMoveProbe: *mut CAI_MoveProbe, // +0x1778 size: 0x8 (0x1 * 0x8) type 10
    pub m_pMotor: *mut CAI_Motor,    // +0x1780 size: 0x8 (0x1 * 0x8) type 10
    pub m_hGoalEnt: EHandle,         // +0x1788 size: 0x4 (0x1 * 0x4) type 13
    pub m_flTimeLastMovement: f32,   // +0x178c size: 0x4 (0x1 * 0x4) type 16
    pub m_longJumpCheckTime: f32,    // +0x1790 size: 0x4 (0x1 * 0x4) type 16
    pub m_prevGroundNormal: Vector3, // +0x1794 size: 0xc (0x1 * 0xc) type 3
    pub m_CheckOnGroundTimer: CSimpleSimTimer, // +0x17a0 size: 0x4 (0x1 * 0x4) type 10
    pub m_vDefaultEyeOffset: Vector3, // +0x17a4 size: 0xc (0x1 * 0xc) type 3
    pub m_flNextEyeLookTime: f32,    // +0x17b0 size: 0x4 (0x1 * 0x4) type 16
    pub m_flEyeIntegRate: f32,       // +0x17b4 size: 0x4 (0x1 * 0x4) type 1
    pub m_vEyeLookTarget: Vector3,   // +0x17b8 size: 0xc (0x1 * 0xc) type 15
    pub m_vCurEyeTarget: Vector3,    // +0x17c4 size: 0xc (0x1 * 0xc) type 15
    pub m_flHeadYaw: f32,            // +0x17d0 size: 0x4 (0x1 * 0x4) type 1
    pub m_flHeadPitch: f32,          // +0x17d4 size: 0x4 (0x1 * 0x4) type 1
    pub m_animRefAdjustThinkCount: i32, // +0x17d8 size: 0x4 (0x1 * 0x4) type 5
    pub m_animRefAdjustPerThink: Vector3, // +0x17dc size: 0xc (0x1 * 0xc) type 3
    pub m_animRefDidAdjust: bool,    // +0x17e8 size: 0x1 (0x1 * 0x1) type 6
    pub m_animParentedOnPlay: bool,  // +0x17e9 size: 0x1 (0x1 * 0x1) type 6
    pub gap_17ea: [u8; 2],
    pub m_scriptAnimSavedCollisionGroup: i32, // +0x17ec size: 0x4 (0x1 * 0x4) type 5
    pub m_scriptAnimSavedFlags: i32,          // +0x17f0 size: 0x4 (0x1 * 0x4) type 5
    pub m_scriptAnimStartPolyRef: i32,        // +0x17f4 size: 0x4 (0x1 * 0x4) type 5
    pub m_enemyChangeScriptCallback: [u8; 16], // +0x17f8 size: 0x10 (0x1 * 0x10) type 11
    pub m_scriptThread: [u8; 8],              // +0x1808 size: 0x8 (0x1 * 0x8) type 11
    pub m_scriptFuncName: *mut c_char,        // +0x1810 size: 0x8 (0x1 * 0x8) type 2
    pub m_deathScriptFuncName: *mut c_char,   // +0x1818 size: 0x8 (0x1 * 0x8) type 2
    pub m_pEnemies: *mut CAI_Enemies,         // +0x1820 size: 0x8 (0x1 * 0x8) type 10
    pub m_afMemory: i32,                      // +0x1828 size: 0x4 (0x1 * 0x4) type 5
    pub m_hEnemyOccluder: EHandle,            // +0x182c size: 0x4 (0x1 * 0x4) type 13
    pub m_hScriptEnemy: EHandle,              // +0x1830 size: 0x4 (0x1 * 0x4) type 13
    pub m_hNearestVisibleFriendlyPlayer: EHandle, // +0x1834 size: 0x4 (0x1 * 0x4) type 13
    pub m_lastDamageFlags: i32,               // +0x1838 size: 0x4 (0x1 * 0x4) type 5
    pub m_lastDamageType: i32,                // +0x183c size: 0x4 (0x1 * 0x4) type 5
    pub m_strafeDodgeDamage: f32,             // +0x1840 size: 0x4 (0x1 * 0x4) type 1
    pub m_lastLightPainTime: f32,             // +0x1844 size: 0x4 (0x1 * 0x4) type 16
    pub m_lastHeavyPainTime: f32,             // +0x1848 size: 0x4 (0x1 * 0x4) type 16
    pub m_flSumDamage: f32,                   // +0x184c size: 0x4 (0x1 * 0x4) type 1
    pub m_flLastDamageTime: f32,              // +0x1850 size: 0x4 (0x1 * 0x4) type 16
    pub m_flLastSawPlayerTime: f32,           // +0x1854 size: 0x4 (0x1 * 0x4) type 16
    pub m_flLastAttackTime: f32,              // +0x1858 size: 0x4 (0x1 * 0x4) type 16
    pub m_flAlertEventTime: f32,              // +0x185c size: 0x4 (0x1 * 0x4) type 16
    pub m_flNextRangeAttackSecondary: f32,    // +0x1860 size: 0x4 (0x1 * 0x4) type 16
    pub m_flNextMeleeAllowTime: f32,          // +0x1864 size: 0x4 (0x1 * 0x4) type 16
    pub m_flNextMeleeAltAllowTime: f32,       // +0x1868 size: 0x4 (0x1 * 0x4) type 16
    pub m_meleeComboCount: i32,               // +0x186c size: 0x4 (0x1 * 0x4) type 5
    pub m_bIgnoreUnseenEnemies: bool,         // +0x1870 size: 0x1 (0x1 * 0x1) type 6
    pub gap_1871: [u8; 7],
    pub m_ShotRegulator: CAI_ShotRegulator, // +0x1878 size: 0x30 (0x1 * 0x30) type 10
    pub m_syncedMelee: AISyncedMeleeState,  // +0x18a8 size: 0x60 (0x1 * 0x60) type 10
    pub m_pSquad: *mut c_void,              // +0x1908 size: 0x8 (0x1 * 0x8) type 31
    pub m_SquadName: *mut c_char,           // +0x1910 size: 0x8 (0x1 * 0x8) type 2
    pub m_iMySquadSlot: i32,                // +0x1918 size: 0x4 (0x1 * 0x4) type 5
    pub m_squadAssignment: i32,             // +0x191c size: 0x4 (0x1 * 0x4) type 5
    pub m_squadAssignedRange: f32,          // +0x1920 size: 0x4 (0x1 * 0x4) type 1
    pub m_squadAssignedNodeStartUseTime: f32, // +0x1924 size: 0x4 (0x1 * 0x4) type 1
    pub m_squadAssignedNode: i32,           // +0x1928 size: 0x4 (0x1 * 0x4) type 5
    pub m_lockedNode: i32,                  // +0x192c size: 0x4 (0x1 * 0x4) type 5
    pub m_currentWeaponBarrel: i32,         // +0x1930 size: 0x4 (0x1 * 0x4) type 5
    pub m_bAutoSquad: bool,                 // +0x1934 size: 0x1 (0x1 * 0x1) type 6
    pub m_bDidDeathCleanUp: bool,           // +0x1935 size: 0x1 (0x1 * 0x1) type 6
    pub m_bOptionalSprint: bool,            // +0x1936 size: 0x1 (0x1 * 0x1) type 6
    pub m_bClearNodeOnScheduleFail: bool,   // +0x1937 size: 0x1 (0x1 * 0x1) type 6
    pub m_bRunningFromEnemy: bool,          // +0x1938 size: 0x1 (0x1 * 0x1) type 6
    pub m_runFromEnemyRetry: i8,            // +0x1939 size: 0x1 (0x1 * 0x1) type 8
    pub m_disableArrivalOnce: i8,           // +0x193a size: 0x1 (0x1 * 0x1) type 8
    pub gap_193b: [u8; 5],
    pub m_pTacticalServices: *mut CAI_TacticalServices, // +0x1940 size: 0x8 (0x1 * 0x8) type 10
    pub m_flWaitFinished: f32,                          // +0x1948 size: 0x4 (0x1 * 0x4) type 16
    pub m_flNextFlinchTime: f32,                        // +0x194c size: 0x4 (0x1 * 0x4) type 16
    pub m_flNextCheckFaceEnemyTime: f32,                // +0x1950 size: 0x4 (0x1 * 0x4) type 16
    pub gap_1954: [u8; 4],
    pub m_moveAndShootOverlay: CAI_MoveAndShootOverlay, // +0x1958 size: 0x20 (0x1 * 0x20) type 10
    pub m_forceShootOverlay: bool,                      // +0x1978 size: 0x1 (0x1 * 0x1) type 6
    pub m_weaponTemporarilySwitchedByAnim: bool,        // +0x1979 size: 0x1 (0x1 * 0x1) type 6
    pub m_strafeDir: bool,                              // +0x197a size: 0x1 (0x1 * 0x1) type 6
    pub gap_197b: [u8; 1],
    pub m_strafeCount: i32,        // +0x197c size: 0x4 (0x1 * 0x4) type 5
    pub m_flSavePositionTime: f32, // +0x1980 size: 0x4 (0x1 * 0x4) type 1
    pub m_vSavePosition: Vector3,  // +0x1984 size: 0xc (0x1 * 0xc) type 15
    pub m_vInterruptSavePosition: Vector3, // +0x1990 size: 0xc (0x1 * 0xc) type 15
    pub m_vLastAutoMoveDelta: Vector3, // +0x199c size: 0xc (0x1 * 0xc) type 3
    pub m_autoMoveAdjust_originalSpace: Vector3, // +0x19a8 size: 0xc (0x1 * 0xc) type 3
    pub m_vLastPatrolDir: Vector3, // +0x19b4 size: 0xc (0x1 * 0xc) type 3
    pub m_pHintNode: *mut CBaseEntity, // +0x19c0 size: 0x8 (0x1 * 0x8) type 12
    pub m_pSafeHintNode: EHandle,  // +0x19c8 size: 0x4 (0x1 * 0x4) type 13
    pub m_safeHintType: i32,       // +0x19cc size: 0x4 (0x1 * 0x4) type 5
    pub m_flDistTooFar: f32,       // +0x19d0 size: 0x4 (0x1 * 0x4) type 1
    pub m_flDistTooClose: f32,     // +0x19d4 size: 0x4 (0x1 * 0x4) type 1
    pub m_minEngagementDistVsWeak: f32, // +0x19d8 size: 0x4 (0x1 * 0x4) type 1
    pub m_maxEngagementDistVsWeak: f32, // +0x19dc size: 0x4 (0x1 * 0x4) type 1
    pub m_minEngagementDistVsStrong: f32, // +0x19e0 size: 0x4 (0x1 * 0x4) type 1
    pub m_maxEngagementDistVsStrong: f32, // +0x19e4 size: 0x4 (0x1 * 0x4) type 1
    pub m_dangerousAreaRadius: f32, // +0x19e8 size: 0x4 (0x1 * 0x4) type 1
    pub m_minEnemyDist: f32,       // +0x19ec size: 0x4 (0x1 * 0x4) type 1
    pub m_disengageEnemyDist: f32, // +0x19f0 size: 0x4 (0x1 * 0x4) type 1
    pub gap_19f4: [u8; 4],
    pub m_spawnEquipment_0_: *mut c_char, // +0x19f8 size: 0x8 (0x1 * 0x8) type 2
    pub m_spawnEquipment_1_: *mut c_char, // +0x1a00 size: 0x8 (0x1 * 0x8) type 2
    pub m_grenadeWeapon: *mut CBaseEntity, // +0x1a08 size: 0x8 (0x1 * 0x8) type 12
    pub m_grenadeWeaponName: *mut c_char, // +0x1a10 size: 0x8 (0x1 * 0x8) type 2
    pub m_lastValidGrenadeThrowOrigin: Vector3, // +0x1a18 size: 0xc (0x1 * 0xc) type 3
    pub m_throwGrenadeAllowedTime: f32,   // +0x1a24 size: 0x4 (0x1 * 0x4) type 16
    pub m_rangeAttackTwitchAllowedTime: f32, // +0x1a28 size: 0x4 (0x1 * 0x4) type 16
    pub m_smartAmmoLockTime: f32,         // +0x1a2c size: 0x4 (0x1 * 0x4) type 16
    pub m_smartAmmoLocks: i32,            // +0x1a30 size: 0x4 (0x1 * 0x4) type 5
    pub m_smartAmmoWeapon: EHandle,       // +0x1a34 size: 0x4 (0x1 * 0x4) type 13
    pub m_meleeWeapon: EHandle,           // +0x1a38 size: 0x4 (0x1 * 0x4) type 13
    pub m_reactFriendlyChance: i32,       // +0x1a3c size: 0x4 (0x1 * 0x4) type 5
    pub m_reactBulletChance: i32,         // +0x1a40 size: 0x4 (0x1 * 0x4) type 5
    pub m_reactChance: i32,               // +0x1a44 size: 0x4 (0x1 * 0x4) type 5
    pub m_lastReactTime: f32,             // +0x1a48 size: 0x4 (0x1 * 0x4) type 1
    pub m_dangerousAreaReactionTime: f32, // +0x1a4c size: 0x4 (0x1 * 0x4) type 1
    pub m_MovementCollisionGroup: i32,    // +0x1a50 size: 0x4 (0x1 * 0x4) type 5
    pub m_moveDeflectionSearchRadius: f32, // +0x1a54 size: 0x4 (0x1 * 0x4) type 1
    pub m_moveDeflectionDebounceExpireTime: f32, // +0x1a58 size: 0x4 (0x1 * 0x4) type 16
    pub gap_1a5c: [u8; 4],
    pub shootingCover: ShootingCoverState, // +0x1a60 size: 0x98 (0x1 * 0x98) type 10
    pub m_OnDamaged: [u8; 40],             // +0x1af8 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnFoundEnemy: [u8; 40],          // +0x1b20 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnSeeEnemy: [u8; 40],            // +0x1b48 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnCantSeeEnemy: [u8; 40],        // +0x1b70 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnNoticePotentialEnemy: [u8; 40], // +0x1b98 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnGainEnemyLOS: [u8; 40],        // +0x1bc0 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnLostEnemyLOS: [u8; 40],        // +0x1be8 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnLostEnemy: [u8; 40],           // +0x1c10 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnFoundPlayer: [u8; 40],         // +0x1c38 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnLostPlayerLOS: [u8; 40],       // +0x1c60 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnLostPlayer: [u8; 40],          // +0x1c88 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnHearPlayer: [u8; 40],          // +0x1cb0 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnHearCombat: [u8; 40],          // +0x1cd8 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnSleep: [u8; 40],               // +0x1d00 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnWake: [u8; 40],                // +0x1d28 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnStateChange: [u8; 40],         // +0x1d50 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnFailedToPath: [u8; 40],        // +0x1d78 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnEnterGoalRadius: [u8; 40],     // +0x1da0 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnFinishedAssault: [u8; 40],     // +0x1dc8 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnSawCorpse: [u8; 40],           // +0x1df0 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnGiveWeapon: [u8; 40],          // +0x1e18 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnTakeWeapon: [u8; 40],          // +0x1e40 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnSpecialAttack: [u8; 40],       // +0x1e68 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnScheduleChange: [u8; 40],      // +0x1e90 size: 0x28 (0x1 * 0x28) type 11
    pub m_OnSyncedMeleeBegin: [u8; 40],    // +0x1eb8 size: 0x28 (0x1 * 0x28) type 11
    pub m_queuedOutput: [u8; 32],          // +0x1ee0 size: 0x20 (0x1 * 0x20) type 0
    pub m_pInitializedPhysicsObject: [u8; 8], // +0x1f00 size: 0x8 (0x1 * 0x8) type 11
    pub m_fIsUsingSmallHull: bool,         // +0x1f08 size: 0x1 (0x1 * 0x1) type 6
    pub m_bHadMovingGround: bool,          // +0x1f09 size: 0x1 (0x1 * 0x1) type 6
    pub m_bCheckContacts: bool,            // +0x1f0a size: 0x1 (0x1 * 0x1) type 6
    pub m_bAllowShoot: bool,               // +0x1f0b size: 0x1 (0x1 * 0x1) type 6
    pub m_bActiveForSmartAmmo: bool,       // +0x1f0c size: 0x1 (0x1 * 0x1) type 6
    pub m_bEnemyValidSmartAmmoTarget: bool, // +0x1f0d size: 0x1 (0x1 * 0x1) type 6
    pub m_bAllowBlockBullets: bool,        // +0x1f0e size: 0x1 (0x1 * 0x1) type 6
    pub m_blockingBullets: bool,           // +0x1f0f size: 0x1 (0x1 * 0x1) type 6
    pub m_reactingSurprisedReason: i32,    // +0x1f10 size: 0x4 (0x1 * 0x4) type 5
    pub m_desireCrouch: bool,              // +0x1f14 size: 0x1 (0x1 * 0x1) type 6
    pub m_isCrouching: bool,               // +0x1f15 size: 0x1 (0x1 * 0x1) type 6
    pub m_bAutoMovementBlocked: bool,      // +0x1f16 size: 0x1 (0x1 * 0x1) type 6
    pub m_bAllowPushDuringAnim: bool,      // +0x1f17 size: 0x1 (0x1 * 0x1) type 6
    pub m_desireStandOverrideExpireTime: f32, // +0x1f18 size: 0x4 (0x1 * 0x4) type 16
    pub gap_1f1c: [u8; 4],
    pub m_schedSelectorHistory: [u8; 40], // +0x1f20 size: 0x28 (0x1 * 0x28) type 0
    pub m_behaviorSelectorID: i32,        // +0x1f48 size: 0x4 (0x1 * 0x4) type 5
    pub gap_1f4c: [u8; 4],
    pub m_failText: *mut c_void, // +0x1f50 size: 0x8 (0x1 * 0x8) type 31
    pub m_interruptText: *mut c_void, // +0x1f58 size: 0x8 (0x1 * 0x8) type 31
    pub m_failedSchedule: *mut c_void, // +0x1f60 size: 0x8 (0x1 * 0x8) type 31
    pub m_interuptSchedule: *mut c_void, // +0x1f68 size: 0x8 (0x1 * 0x8) type 31
    pub m_nDebugCurIndex: i32,   // +0x1f70 size: 0x4 (0x1 * 0x4) type 5
    pub m_flAccuracyMultiplier: f32, // +0x1f74 size: 0x4 (0x1 * 0x4) type 1
    pub m_flAccuracyMultiplierForEnemy: f32, // +0x1f78 size: 0x4 (0x1 * 0x4) type 1
    pub m_LastMissFastPlayerTime: f32, // +0x1f7c size: 0x4 (0x1 * 0x4) type 1
    pub m_LastSuppressionTime: f32, // +0x1f80 size: 0x4 (0x1 * 0x4) type 1
    pub m_LastShootAccuracy: f32, // +0x1f84 size: 0x4 (0x1 * 0x4) type 1
    pub m_TotalShots: i32,       // +0x1f88 size: 0x4 (0x1 * 0x4) type 5
    pub m_TotalHits: i32,        // +0x1f8c size: 0x4 (0x1 * 0x4) type 5
    pub m_flSoundWaitTime: f32,  // +0x1f90 size: 0x4 (0x1 * 0x4) type 16
    pub m_nSoundPriority: i32,   // +0x1f94 size: 0x4 (0x1 * 0x4) type 5
    pub m_lastTauntTime: f32,    // +0x1f98 size: 0x4 (0x1 * 0x4) type 16
    pub m_freezeTime: f32,       // +0x1f9c size: 0x4 (0x1 * 0x4) type 16
    pub m_freezeCycle: f32,      // +0x1fa0 size: 0x4 (0x1 * 0x4) type 1
    pub m_prevShieldHealth: i32, // +0x1fa4 size: 0x4 (0x1 * 0x4) type 5
    pub m_healthEvalMultiplier: i32, // +0x1fa8 size: 0x4 (0x1 * 0x4) type 5
    pub m_aiMovementSpeed: f32,  // +0x1fac size: 0x4 (0x1 * 0x4) type 1
    pub m_aiSprinting: bool,     // +0x1fb0 size: 0x1 (0x1 * 0x1) type 6
    pub gap_1fb1: [u8; 3],
    pub m_aiNetworkFlags: i32, // +0x1fb4 size: 0x4 (0x1 * 0x4) type 5
    pub m_isHologram: bool,    // +0x1fb8 size: 0x1 (0x1 * 0x1) type 6
    pub gap_1fb9: [u8; 3],
    pub m_fireteamSlotIndex: i32, // +0x1fbc size: 0x4 (0x1 * 0x4) type 5
    pub m_statusEffectsTimedNPCNV: [StatusEffectTimedData; 3], // +0x1fc0 size: 0x18 (0x3 * 0x8) type 10
    pub m_statusEffectsEndlessNPCNV: [StatusEffectEndlessData; 4], // +0x2008 size: 0x10 (0x4 * 0x4) type 10
    pub m_title: [i8; 32], // +0x2048 size: 0x20 (0x20 * 0x1) type 8
    pub m_tethered: bool,  // +0x2068 size: 0x1 (0x1 * 0x1) type 6
    pub gap_2069: [u8; 3],
    pub m_nAITraceMask: i32,          // +0x206c size: 0x4 (0x1 * 0x4) type 5
    pub m_flBoostSpeed: f32,          // +0x2070 size: 0x4 (0x1 * 0x4) type 1
    pub m_blockPeriodStartTime: f32,  // +0x2074 size: 0x4 (0x1 * 0x4) type 16
    pub m_blockBulletCount: i32,      // +0x2078 size: 0x4 (0x1 * 0x4) type 5
    pub m_dodgePeriodStartTime: f32,  // +0x207c size: 0x4 (0x1 * 0x4) type 16
    pub m_lastDodgeTime: f32,         // +0x2080 size: 0x4 (0x1 * 0x4) type 16
    pub m_dodgeCount: i32,            // +0x2084 size: 0x4 (0x1 * 0x4) type 5
    pub m_dodgeMissile: EHandle,      // +0x2088 size: 0x4 (0x1 * 0x4) type 13
    pub m_dodgeFromPos: Vector3,      // +0x208c size: 0xc (0x1 * 0xc) type 3
    pub m_dangerousArea: *mut c_void, // +0x2098 size: 0x8 (0x1 * 0x8) type 0
    pub m_dangerousAreaDebounceExpireTime: f32, // +0x20a0 size: 0x4 (0x1 * 0x4) type 16
    pub m_grappled: bool,             // +0x20a4 size: 0x1 (0x1 * 0x1) type 6
    pub m_grappleEndtiming: bool,     // +0x20a5 size: 0x1 (0x1 * 0x1) type 6
    pub gap_20a6: [u8; 2],
    pub m_grappleRestoreMoveType: i32, // +0x20a8 size: 0x4 (0x1 * 0x4) type 5
}

size_assert!( CAI_BASENPC where CAI_BaseNPC == 0x20b0);
field_assert!(+ SIZEAI_DEFMOVEMENTSINK where CAI_BaseNPC, CAI_DefMovementSink == 0x1490);
field_assert!(+ SIZEAI_BEHAVIORBRIDGE where CAI_BaseNPC, IAI_BehaviorBridge == 0x1498);
field_assert!(+ SIZE_THREADEDPOSTPROCESSJOB where CAI_BaseNPC, m_threadedPostProcessJob == 0x14a0);
field_assert!(+ SIZE_BDOPOSTPROCESS where CAI_BaseNPC, m_bDoPostProcess == 0x14a4);
field_assert!(+ SIZE_BCUSTOMENEMYSEARCH where CAI_BaseNPC, m_bCustomEnemySearch == 0x14a5);
field_assert!(+ SIZE_BPLAYERSPOTTABLE where CAI_BaseNPC, m_bPlayerSpottable == 0x14a6);
field_assert!(+ SIZE_BADDEDTOSPOTTABLELIST where CAI_BaseNPC, m_bAddedToSpottableList == 0x14a7);
field_assert!(+ SIZE_PPREVSCHEDULE where CAI_BaseNPC, m_pPrevSchedule == 0x14a8);
field_assert!(+ SIZE_PSCHEDULE where CAI_BaseNPC, m_pSchedule == 0x14b0);
field_assert!(+ SIZE_DEFAULTSCRIPTSCHEDULEID where CAI_BaseNPC, m_defaultScriptScheduleID == 0x14b8);
field_assert!(+ SIZE_SCHEDULESTATE where CAI_BaseNPC, m_ScheduleState == 0x14c0);
field_assert!(+ SIZE_FAILSCHEDULE where CAI_BaseNPC, m_failSchedule == 0x14f0);
field_assert!(+ SIZE_BDOPOSTRESTOREREFINDPATH where CAI_BaseNPC, m_bDoPostRestoreRefindPath == 0x14f4);
field_assert!(+ SIZE_BDOSCHEDULECHANGESIGNAL where CAI_BaseNPC, m_bDoScheduleChangeSignal == 0x14f5);
field_assert!(+ SIZE_SPAWNED where CAI_BaseNPC, m_spawned == 0x14f6);
field_assert!(+ SIZE_BUSINGSTANDARDTHINKTIME where CAI_BaseNPC, m_bUsingStandardThinkTime == 0x14f7);
field_assert!(+ SIZE_FLLASTREALTHINKTIME where CAI_BaseNPC, m_flLastRealThinkTime == 0x14f8);
field_assert!(+ SIZE_FLLASTTHINKDURATION where CAI_BaseNPC, m_flLastThinkDuration == 0x14fc);
field_assert!(+ SIZE_IFRAMEBLOCKED where CAI_BaseNPC, m_iFrameBlocked == 0x1500);
field_assert!(+ SIZE_PSCHEDULEEVENT where CAI_BaseNPC, m_pScheduleEvent == 0x1508);
field_assert!(+ SIZE_DANGEROUSCLUSTERCONDITIONALLOWEDATTIME where CAI_BaseNPC, m_dangerousClusterConditionAllowedAtTime == 0x1510);
field_assert!(+ SIZE_POSEAIM_PITCH where CAI_BaseNPC, m_poseAim_Pitch == 0x1514);
field_assert!(+ SIZE_POSEAIM_YAW where CAI_BaseNPC, m_poseAim_Yaw == 0x1518);
field_assert!(+ SIZE_POSEMOVE_YAW where CAI_BaseNPC, m_poseMove_Yaw == 0x151c);
field_assert!(+ SIZE_POSEMOVE_LEAN where CAI_BaseNPC, m_poseMove_Lean == 0x1520);
field_assert!(+ SIZE_OFFSETOFCURRENTAIMFROMDESIREDAIM_COS where CAI_BaseNPC, m_offsetOfCurrentAimFromDesiredAim_Cos == 0x1524);
field_assert!(+ SIZE_HAVEDESIREDAIMDIR where CAI_BaseNPC, m_haveDesiredAimDir == 0x1528);
field_assert!(+ SIZE_SEQUENCEHASAIMEVENT where CAI_BaseNPC, m_sequenceHasAimEvent == 0x1529);
field_assert!(+ SIZE_SEQUENCEHASAIMPOSEPARAMS where CAI_BaseNPC, m_sequenceHasAimPoseParams == 0x152a);
field_assert!(+ SIZE_LASTAIMSEQUENCE where CAI_BaseNPC, m_lastAimSequence == 0x152c);
field_assert!(+ SIZE_MISSPLAYERLASTWINDOW where CAI_BaseNPC, m_missPlayerLastWindow == 0x1530);
field_assert!(+ SIZE_MISSPLAYERLASTOFFSET where CAI_BaseNPC, m_missPlayerLastOffset == 0x1534);
field_assert!(+ SIZE_PPRIMARYBEHAVIOR where CAI_BaseNPC, m_pPrimaryBehavior == 0x1540);
field_assert!(+ SIZE_BEHAVIORS where CAI_BaseNPC, m_Behaviors == 0x1548);
field_assert!(+ SIZE_CONDITIONS where CAI_BaseNPC, m_Conditions == 0x1568);
field_assert!(+ SIZE_CUSTOMINTERRUPTCONDITIONS where CAI_BaseNPC, m_CustomInterruptConditions == 0x1574);
field_assert!(+ SIZE_BFORCECONDITIONSGATHER where CAI_BaseNPC, m_bForceConditionsGather == 0x1580);
field_assert!(+ SIZE_BCONDITIONSGATHERED where CAI_BaseNPC, m_bConditionsGathered == 0x1581);
field_assert!(+ SIZE_BINTERRUPTABLEBYSCRIPT where CAI_BaseNPC, m_bInterruptableByScript == 0x1582);
field_assert!(+ SIZE_MOVINGGROUNDENT where CAI_BaseNPC, m_movingGroundEnt == 0x1584);
field_assert!(+ SIZE_GROUNDRELATIVEPOS where CAI_BaseNPC, m_groundRelativePos == 0x1588);
field_assert!(+ SIZE_GROUNDRELATIVEANGLES where CAI_BaseNPC, m_groundRelativeAngles == 0x1594);
field_assert!(+ SIZE_NPCSTATE where CAI_BaseNPC, m_NPCState == 0x15a0);
field_assert!(+ SIZE_NPCPREVSTATE where CAI_BaseNPC, m_NPCPrevState == 0x15a4);
field_assert!(+ SIZE_NPCALERTNESSSTATE where CAI_BaseNPC, m_NPCAlertnessState == 0x15a8);
field_assert!(+ SIZE_LASTSCHEDSELECTORSTATE where CAI_BaseNPC, m_lastSchedSelectorState == 0x15ac);
field_assert!(+ SIZE_FLLASTSCHEDSELECTCHANGETIME where CAI_BaseNPC, m_flLastSchedSelectChangeTime == 0x15b0);
field_assert!(+ SIZE_FLLASTSTATECHANGETIME where CAI_BaseNPC, m_flLastStateChangeTime == 0x15b4);
field_assert!(+ SIZE_THINKINTERVAL where CAI_BaseNPC, m_thinkInterval == 0x15b8);
field_assert!(+ SIZE_IDEALNPCSTATE where CAI_BaseNPC, m_IdealNPCState == 0x15bc);
field_assert!(+ SIZE_EFFICIENCY where CAI_BaseNPC, m_Efficiency == 0x15c0);
field_assert!(+ SIZE_MOVEEFFICIENCY where CAI_BaseNPC, m_MoveEfficiency == 0x15c4);
field_assert!(+ SIZE_FLNEXTDECISIONTIME where CAI_BaseNPC, m_flNextDecisionTime == 0x15c8);
field_assert!(+ SIZE_SEARCHPATHID where CAI_BaseNPC, m_searchPathID == 0x15cc);
field_assert!(+ SIZE_BDEFENSEACTIVE where CAI_BaseNPC, m_bDefenseActive == 0x15ce);
field_assert!(+ SIZE_BALWAYSALERT where CAI_BaseNPC, m_bAlwaysAlert == 0x15cf);
field_assert!(+ SIZE_SCRIPTEDANIMFORCEINTERRUPT where CAI_BaseNPC, m_scriptedAnimForceInterrupt == 0x15d0);
field_assert!(+ SIZE_BWAKESQUAD where CAI_BaseNPC, m_bWakeSquad == 0x15d1);
field_assert!(+ SIZE_FLWAKERADIUS where CAI_BaseNPC, m_flWakeRadius == 0x15d4);
field_assert!(+ SIZE_NWAKETICK where CAI_BaseNPC, m_nWakeTick == 0x15d8);
field_assert!(+ SIZE_SLEEPSTATE where CAI_BaseNPC, m_SleepState == 0x15dc);
field_assert!(+ SIZE_SLEEPFLAGS where CAI_BaseNPC, m_SleepFlags == 0x15e0);
field_assert!(+ SIZE_TRANSLATEDACTIVITY where CAI_BaseNPC, m_translatedActivity == 0x15e4);
field_assert!(+ SIZE_IDEALACTIVITY where CAI_BaseNPC, m_IdealActivity == 0x15e8);
field_assert!(+ SIZE_NIDEALSEQUENCE where CAI_BaseNPC, m_nIdealSequence == 0x15ec);
field_assert!(+ SIZE_IDEALTRANSLATEDACTIVITY where CAI_BaseNPC, m_IdealTranslatedActivity == 0x15f0);
field_assert!(+ SIZE_IDEALWEAPONACTIVITY where CAI_BaseNPC, m_IdealWeaponActivity == 0x15f4);
field_assert!(+ SIZE_IDEALSEQUENCEALREADYRESOLVED where CAI_BaseNPC, m_idealSequenceAlreadyResolved == 0x15f8);
field_assert!(+ SIZE_SEQFINISHEDINSOLID where CAI_BaseNPC, m_seqFinishedInSolid == 0x15fc);
field_assert!(+ SIZE_ACTIVEACTMODIFIERS where CAI_BaseNPC, m_activeActModifiers == 0x1600);
field_assert!(+ SIZE_SCRIPTIDLESEQUENCE where CAI_BaseNPC, m_scriptIdleSequence == 0x1640);
field_assert!(+ SIZE_SCRIPTATTACKSEQUENCE where CAI_BaseNPC, m_scriptAttackSequence == 0x1644);
field_assert!(+ SIZE_SCRIPTDEATHACTIVITY where CAI_BaseNPC, m_scriptDeathActivity == 0x1648);
field_assert!(+ SIZE_DIDSETINDOORACTIVITYOVERRIDE where CAI_BaseNPC, m_didSetIndoorActivityOverride == 0x164c);
field_assert!(+ SIZE_ISINSIDEINDOORAREA where CAI_BaseNPC, m_isInsideIndoorArea == 0x164d);
field_assert!(+ SIZE_REQUESTSPECIALRANGEATTACK where CAI_BaseNPC, m_requestSpecialRangeAttack == 0x164e);
field_assert!(+ SIZE_SPECIALRANGEATTACKCOUNT where CAI_BaseNPC, m_specialRangeAttackCount == 0x1650);
field_assert!(+ SIZE_AISETTINGS where CAI_BaseNPC, m_aiSettings == 0x1658);
field_assert!(+ SIZE_AISETTINGSINDEX where CAI_BaseNPC, m_aiSettingsIndex == 0x1660);
field_assert!(+ SIZE_SUBCLASS where CAI_BaseNPC, m_subclass == 0x1664);
field_assert!(+ SIZE_PSENSES where CAI_BaseNPC, m_pSenses == 0x1668);
field_assert!(+ SIZE_PLOCKEDBESTSOUND where CAI_BaseNPC, m_pLockedBestSound == 0x1670);
field_assert!(+ SIZE_AIMDIR where CAI_BaseNPC, m_aimDir == 0x1698);
field_assert!(+ SIZE_AIMDIRVALID where CAI_BaseNPC, m_aimDirValid == 0x16a4);
field_assert!(+ SIZE_WEAPONBLOCKEDTIMER where CAI_BaseNPC, m_weaponBlockedTimer == 0x16a8);
field_assert!(+ SIZE_WEAPONBLOCKEDTIMEOUT where CAI_BaseNPC, m_weaponBlockedTimeOut == 0x16ac);
field_assert!(+ SIZE_MOVEDIRECTION where CAI_BaseNPC, m_moveDirection == 0x16b0);
field_assert!(+ SIZE_HENEMY where CAI_BaseNPC, m_hEnemy == 0x16b4);
field_assert!(+ SIZE_HENEMYSECONDARY where CAI_BaseNPC, m_hEnemySecondary == 0x16b8);
field_assert!(+ SIZE_DISTTOENEMYLKP_ADJUSTFORHEIGHTDIFF where CAI_BaseNPC, m_distToEnemyLKP_AdjustForHeightDiff == 0x16bc);
field_assert!(+ SIZE_DISTTOENEMYLKP where CAI_BaseNPC, m_distToEnemyLKP == 0x16c0);
field_assert!(+ SIZE_DISTTOENEMYLKPCENTERTOCENTER where CAI_BaseNPC, m_distToEnemyLKPCenterToCenter == 0x16c4);
field_assert!(+ SIZE_HTARGETENT where CAI_BaseNPC, m_hTargetEnt == 0x16c8);
field_assert!(+ SIZE_UPDATESQUADENEMYQUEUE where CAI_BaseNPC, m_updateSquadEnemyQueue == 0x16d0);
field_assert!(+ SIZE_NOTIFYENEMYTOSQUADTIMER where CAI_BaseNPC, m_notifyEnemyToSquadTimer == 0x16f0);
field_assert!(+ SIZE_NOTIFYENEMYTOTEAMTIME where CAI_BaseNPC, m_notifyEnemyToTeamTime == 0x16fc);
field_assert!(+ SIZE_GIVEUPONDEADENEMYTIMER where CAI_BaseNPC, m_GiveUpOnDeadEnemyTimer == 0x1700);
field_assert!(+ SIZE_CHOOSEENEMYSANITYTIMER where CAI_BaseNPC, m_chooseEnemySanityTimer == 0x1710);
field_assert!(+ SIZE_ENEMIESSERIALNUMBER where CAI_BaseNPC, m_EnemiesSerialNumber == 0x1714);
field_assert!(+ SIZE_FLACCEPTABLETIMESEENENEMY where CAI_BaseNPC, m_flAcceptableTimeSeenEnemy == 0x1718);
field_assert!(+ SIZE_UPDATEENEMYPOSTIMER where CAI_BaseNPC, m_UpdateEnemyPosTimer == 0x171c);
field_assert!(+ SIZE_FORCEUPDATEENEMYPOS where CAI_BaseNPC, m_ForceUpdateEnemyPos == 0x1720);
field_assert!(+ SIZE_AFCAPABILITY where CAI_BaseNPC, m_afCapability == 0x1724);
field_assert!(+ SIZE_FLAGS where CAI_BaseNPC, m_flags == 0x1728);
field_assert!(+ SIZE_FLMOVEWAITFINISHED where CAI_BaseNPC, m_flMoveWaitFinished == 0x172c);
field_assert!(+ SIZE_HTEMPBLOCKINGENT where CAI_BaseNPC, m_hTempBlockingEnt == 0x1730);
field_assert!(+ SIZE_UNREACHABLEENTS where CAI_BaseNPC, m_UnreachableEnts == 0x1738);
field_assert!(+ SIZE_PNAVIGATOR where CAI_BaseNPC, m_pNavigator == 0x1758);
field_assert!(+ SIZE_PLOCALNAVIGATOR where CAI_BaseNPC, m_pLocalNavigator == 0x1760);
field_assert!(+ SIZE_PPATHFINDER where CAI_BaseNPC, m_pPathfinder == 0x1768);
field_assert!(+ SIZE_PMOVEPROBE where CAI_BaseNPC, m_pMoveProbe == 0x1770);
field_assert!(+ SIZE_PMOTOR where CAI_BaseNPC, m_pMotor == 0x1778);
field_assert!(+ SIZE_HGOALENT where CAI_BaseNPC, m_hGoalEnt == 0x1780);
field_assert!(+ SIZE_FLTIMELASTMOVEMENT where CAI_BaseNPC, m_flTimeLastMovement == 0x1784);
field_assert!(+ SIZE_LONGJUMPCHECKTIME where CAI_BaseNPC, m_longJumpCheckTime == 0x1788);
field_assert!(+ SIZE_PREVGROUNDNORMAL where CAI_BaseNPC, m_prevGroundNormal == 0x178c);
field_assert!(+ SIZE_CHECKONGROUNDTIMER where CAI_BaseNPC, m_CheckOnGroundTimer == 0x1798);
field_assert!(+ SIZE_VDEFAULTEYEOFFSET where CAI_BaseNPC, m_vDefaultEyeOffset == 0x179c);
field_assert!(+ SIZE_FLNEXTEYELOOKTIME where CAI_BaseNPC, m_flNextEyeLookTime == 0x17a8);
field_assert!(+ SIZE_FLEYEINTEGRATE where CAI_BaseNPC, m_flEyeIntegRate == 0x17ac);
field_assert!(+ SIZE_VEYELOOKTARGET where CAI_BaseNPC, m_vEyeLookTarget == 0x17b0);
field_assert!(+ SIZE_VCUREYETARGET where CAI_BaseNPC, m_vCurEyeTarget == 0x17bc);
field_assert!(+ SIZE_FLHEADYAW where CAI_BaseNPC, m_flHeadYaw == 0x17c8);
field_assert!(+ SIZE_FLHEADPITCH where CAI_BaseNPC, m_flHeadPitch == 0x17cc);
field_assert!(+ SIZE_ANIMREFADJUSTTHINKCOUNT where CAI_BaseNPC, m_animRefAdjustThinkCount == 0x17d0);
field_assert!(+ SIZE_ANIMREFADJUSTPERTHINK where CAI_BaseNPC, m_animRefAdjustPerThink == 0x17d4);
field_assert!(+ SIZE_ANIMREFDIDADJUST where CAI_BaseNPC, m_animRefDidAdjust == 0x17e0);
field_assert!(+ SIZE_ANIMPARENTEDONPLAY where CAI_BaseNPC, m_animParentedOnPlay == 0x17e1);
field_assert!(+ SIZE_SCRIPTANIMSAVEDCOLLISIONGROUP where CAI_BaseNPC, m_scriptAnimSavedCollisionGroup == 0x17e4);
field_assert!(+ SIZE_SCRIPTANIMSAVEDFLAGS where CAI_BaseNPC, m_scriptAnimSavedFlags == 0x17e8);
field_assert!(+ SIZE_SCRIPTANIMSTARTPOLYREF where CAI_BaseNPC, m_scriptAnimStartPolyRef == 0x17ec);
field_assert!(+ SIZE_ENEMYCHANGESCRIPTCALLBACK where CAI_BaseNPC, m_enemyChangeScriptCallback == 0x17f0);
field_assert!(+ SIZE_SCRIPTTHREAD where CAI_BaseNPC, m_scriptThread == 0x1800);
field_assert!(+ SIZE_SCRIPTFUNCNAME where CAI_BaseNPC, m_scriptFuncName == 0x1808);
field_assert!(+ SIZE_DEATHSCRIPTFUNCNAME where CAI_BaseNPC, m_deathScriptFuncName == 0x1810);
field_assert!(+ SIZE_PENEMIES where CAI_BaseNPC, m_pEnemies == 0x1818);
field_assert!(+ SIZE_AFMEMORY where CAI_BaseNPC, m_afMemory == 0x1820);
field_assert!(+ SIZE_HENEMYOCCLUDER where CAI_BaseNPC, m_hEnemyOccluder == 0x1824);
field_assert!(+ SIZE_HSCRIPTENEMY where CAI_BaseNPC, m_hScriptEnemy == 0x1828);
field_assert!(+ SIZE_HNEARESTVISIBLEFRIENDLYPLAYER where CAI_BaseNPC, m_hNearestVisibleFriendlyPlayer == 0x182c);
field_assert!(+ SIZE_LASTDAMAGEFLAGS where CAI_BaseNPC, m_lastDamageFlags == 0x1830);
field_assert!(+ SIZE_LASTDAMAGETYPE where CAI_BaseNPC, m_lastDamageType == 0x1834);
field_assert!(+ SIZE_STRAFEDODGEDAMAGE where CAI_BaseNPC, m_strafeDodgeDamage == 0x1838);
field_assert!(+ SIZE_LASTLIGHTPAINTIME where CAI_BaseNPC, m_lastLightPainTime == 0x183c);
field_assert!(+ SIZE_LASTHEAVYPAINTIME where CAI_BaseNPC, m_lastHeavyPainTime == 0x1840);
field_assert!(+ SIZE_FLSUMDAMAGE where CAI_BaseNPC, m_flSumDamage == 0x1844);
field_assert!(+ SIZE_FLLASTDAMAGETIME where CAI_BaseNPC, m_flLastDamageTime == 0x1848);
field_assert!(+ SIZE_FLLASTSAWPLAYERTIME where CAI_BaseNPC, m_flLastSawPlayerTime == 0x184c);
field_assert!(+ SIZE_FLLASTATTACKTIME where CAI_BaseNPC, m_flLastAttackTime == 0x1850);
field_assert!(+ SIZE_FLALERTEVENTTIME where CAI_BaseNPC, m_flAlertEventTime == 0x1854);
field_assert!(+ SIZE_FLNEXTRANGEATTACKSECONDARY where CAI_BaseNPC, m_flNextRangeAttackSecondary == 0x1858);
field_assert!(+ SIZE_FLNEXTMELEEALLOWTIME where CAI_BaseNPC, m_flNextMeleeAllowTime == 0x185c);
field_assert!(+ SIZE_FLNEXTMELEEALTALLOWTIME where CAI_BaseNPC, m_flNextMeleeAltAllowTime == 0x1860);
field_assert!(+ SIZE_MELEECOMBOCOUNT where CAI_BaseNPC, m_meleeComboCount == 0x1864);
field_assert!(+ SIZE_BIGNOREUNSEENENEMIES where CAI_BaseNPC, m_bIgnoreUnseenEnemies == 0x1868);
field_assert!(+ SIZE_SHOTREGULATOR where CAI_BaseNPC, m_ShotRegulator == 0x1870);
field_assert!(+ SIZE_SYNCEDMELEE where CAI_BaseNPC, m_syncedMelee == 0x18a0);
field_assert!(+ SIZE_PSQUAD where CAI_BaseNPC, m_pSquad == 0x1900);
field_assert!(+ SIZE_SQUADNAME where CAI_BaseNPC, m_SquadName == 0x1908);
field_assert!(+ SIZE_IMYSQUADSLOT where CAI_BaseNPC, m_iMySquadSlot == 0x1910);
field_assert!(+ SIZE_SQUADASSIGNMENT where CAI_BaseNPC, m_squadAssignment == 0x1914);
field_assert!(+ SIZE_SQUADASSIGNEDRANGE where CAI_BaseNPC, m_squadAssignedRange == 0x1918);
field_assert!(+ SIZE_SQUADASSIGNEDNODESTARTUSETIME where CAI_BaseNPC, m_squadAssignedNodeStartUseTime == 0x191c);
field_assert!(+ SIZE_SQUADASSIGNEDNODE where CAI_BaseNPC, m_squadAssignedNode == 0x1920);
field_assert!(+ SIZE_LOCKEDNODE where CAI_BaseNPC, m_lockedNode == 0x1924);
field_assert!(+ SIZE_CURRENTWEAPONBARREL where CAI_BaseNPC, m_currentWeaponBarrel == 0x1928);
field_assert!(+ SIZE_BAUTOSQUAD where CAI_BaseNPC, m_bAutoSquad == 0x192c);
field_assert!(+ SIZE_BDIDDEATHCLEANUP where CAI_BaseNPC, m_bDidDeathCleanUp == 0x192d);
field_assert!(+ SIZE_BOPTIONALSPRINT where CAI_BaseNPC, m_bOptionalSprint == 0x192e);
field_assert!(+ SIZE_BCLEARNODEONSCHEDULEFAIL where CAI_BaseNPC, m_bClearNodeOnScheduleFail == 0x192f);
field_assert!(+ SIZE_BRUNNINGFROMENEMY where CAI_BaseNPC, m_bRunningFromEnemy == 0x1930);
field_assert!(+ SIZE_RUNFROMENEMYRETRY where CAI_BaseNPC, m_runFromEnemyRetry == 0x1931);
field_assert!(+ SIZE_DISABLEARRIVALONCE where CAI_BaseNPC, m_disableArrivalOnce == 0x1932);
field_assert!(+ SIZE_PTACTICALSERVICES where CAI_BaseNPC, m_pTacticalServices == 0x1938);
field_assert!(+ SIZE_FLWAITFINISHED where CAI_BaseNPC, m_flWaitFinished == 0x1940);
field_assert!(+ SIZE_FLNEXTFLINCHTIME where CAI_BaseNPC, m_flNextFlinchTime == 0x1944);
field_assert!(+ SIZE_FLNEXTCHECKFACEENEMYTIME where CAI_BaseNPC, m_flNextCheckFaceEnemyTime == 0x1948);
field_assert!(+ SIZE_MOVEANDSHOOTOVERLAY where CAI_BaseNPC, m_moveAndShootOverlay == 0x1950);
field_assert!(+ SIZE_FORCESHOOTOVERLAY where CAI_BaseNPC, m_forceShootOverlay == 0x1970);
field_assert!(+ SIZE_WEAPONTEMPORARILYSWITCHEDBYANIM where CAI_BaseNPC, m_weaponTemporarilySwitchedByAnim == 0x1971);
field_assert!(+ SIZE_STRAFEDIR where CAI_BaseNPC, m_strafeDir == 0x1972);
field_assert!(+ SIZE_STRAFECOUNT where CAI_BaseNPC, m_strafeCount == 0x1974);
field_assert!(+ SIZE_FLSAVEPOSITIONTIME where CAI_BaseNPC, m_flSavePositionTime == 0x1978);
field_assert!(+ SIZE_VSAVEPOSITION where CAI_BaseNPC, m_vSavePosition == 0x197c);
field_assert!(+ SIZE_VINTERRUPTSAVEPOSITION where CAI_BaseNPC, m_vInterruptSavePosition == 0x1988);
field_assert!(+ SIZE_VLASTAUTOMOVEDELTA where CAI_BaseNPC, m_vLastAutoMoveDelta == 0x1994);
field_assert!(+ SIZE_AUTOMOVEADJUST_ORIGINALSPACE where CAI_BaseNPC, m_autoMoveAdjust_originalSpace == 0x19a0);
field_assert!(+ SIZE_VLASTPATROLDIR where CAI_BaseNPC, m_vLastPatrolDir == 0x19ac);
field_assert!(+ SIZE_PHINTNODE where CAI_BaseNPC, m_pHintNode == 0x19b8);
field_assert!(+ SIZE_PSAFEHINTNODE where CAI_BaseNPC, m_pSafeHintNode == 0x19c0);
field_assert!(+ SIZE_SAFEHINTTYPE where CAI_BaseNPC, m_safeHintType == 0x19c4);
field_assert!(+ SIZE_FLDISTTOOFAR where CAI_BaseNPC, m_flDistTooFar == 0x19c8);
field_assert!(+ SIZE_FLDISTTOOCLOSE where CAI_BaseNPC, m_flDistTooClose == 0x19cc);
field_assert!(+ SIZE_MINENGAGEMENTDISTVSWEAK where CAI_BaseNPC, m_minEngagementDistVsWeak == 0x19d0);
field_assert!(+ SIZE_MAXENGAGEMENTDISTVSWEAK where CAI_BaseNPC, m_maxEngagementDistVsWeak == 0x19d4);
field_assert!(+ SIZE_MINENGAGEMENTDISTVSSTRONG where CAI_BaseNPC, m_minEngagementDistVsStrong == 0x19d8);
field_assert!(+ SIZE_MAXENGAGEMENTDISTVSSTRONG where CAI_BaseNPC, m_maxEngagementDistVsStrong == 0x19dc);
field_assert!(+ SIZE_DANGEROUSAREARADIUS where CAI_BaseNPC, m_dangerousAreaRadius == 0x19e0);
field_assert!(+ SIZE_MINENEMYDIST where CAI_BaseNPC, m_minEnemyDist == 0x19e4);
field_assert!(+ SIZE_DISENGAGEENEMYDIST where CAI_BaseNPC, m_disengageEnemyDist == 0x19e8);
field_assert!(+ SIZE_SPAWNEQUIPMENT_0_ where CAI_BaseNPC, m_spawnEquipment_0_ == 0x19f0);
field_assert!(+ SIZE_SPAWNEQUIPMENT_1_ where CAI_BaseNPC, m_spawnEquipment_1_ == 0x19f8);
field_assert!(+ SIZE_GRENADEWEAPON where CAI_BaseNPC, m_grenadeWeapon == 0x1a00);
field_assert!(+ SIZE_GRENADEWEAPONNAME where CAI_BaseNPC, m_grenadeWeaponName == 0x1a08);
field_assert!(+ SIZE_LASTVALIDGRENADETHROWORIGIN where CAI_BaseNPC, m_lastValidGrenadeThrowOrigin == 0x1a10);
field_assert!(+ SIZE_THROWGRENADEALLOWEDTIME where CAI_BaseNPC, m_throwGrenadeAllowedTime == 0x1a1c);
field_assert!(+ SIZE_RANGEATTACKTWITCHALLOWEDTIME where CAI_BaseNPC, m_rangeAttackTwitchAllowedTime == 0x1a20);
field_assert!(+ SIZE_SMARTAMMOLOCKTIME where CAI_BaseNPC, m_smartAmmoLockTime == 0x1a24);
field_assert!(+ SIZE_SMARTAMMOLOCKS where CAI_BaseNPC, m_smartAmmoLocks == 0x1a28);
field_assert!(+ SIZE_SMARTAMMOWEAPON where CAI_BaseNPC, m_smartAmmoWeapon == 0x1a2c);
field_assert!(+ SIZE_MELEEWEAPON where CAI_BaseNPC, m_meleeWeapon == 0x1a30);
field_assert!(+ SIZE_REACTFRIENDLYCHANCE where CAI_BaseNPC, m_reactFriendlyChance == 0x1a34);
field_assert!(+ SIZE_REACTBULLETCHANCE where CAI_BaseNPC, m_reactBulletChance == 0x1a38);
field_assert!(+ SIZE_REACTCHANCE where CAI_BaseNPC, m_reactChance == 0x1a3c);
field_assert!(+ SIZE_LASTREACTTIME where CAI_BaseNPC, m_lastReactTime == 0x1a40);
field_assert!(+ SIZE_DANGEROUSAREAREACTIONTIME where CAI_BaseNPC, m_dangerousAreaReactionTime == 0x1a44);
field_assert!(+ SIZE_MOVEMENTCOLLISIONGROUP where CAI_BaseNPC, m_MovementCollisionGroup == 0x1a48);
field_assert!(+ SIZE_MOVEDEFLECTIONSEARCHRADIUS where CAI_BaseNPC, m_moveDeflectionSearchRadius == 0x1a4c);
field_assert!(+ SIZE_MOVEDEFLECTIONDEBOUNCEEXPIRETIME where CAI_BaseNPC, m_moveDeflectionDebounceExpireTime == 0x1a50);
field_assert!(+ SIZEHOOTINGCOVER where CAI_BaseNPC, shootingCover == 0x1a58);
field_assert!(+ SIZE_ONDAMAGED where CAI_BaseNPC, m_OnDamaged == 0x1af0);
field_assert!(+ SIZE_ONFOUNDENEMY where CAI_BaseNPC, m_OnFoundEnemy == 0x1b18);
field_assert!(+ SIZE_ONSEEENEMY where CAI_BaseNPC, m_OnSeeEnemy == 0x1b40);
field_assert!(+ SIZE_ONCANTSEEENEMY where CAI_BaseNPC, m_OnCantSeeEnemy == 0x1b68);
field_assert!(+ SIZE_ONNOTICEPOTENTIALENEMY where CAI_BaseNPC, m_OnNoticePotentialEnemy == 0x1b90);
field_assert!(+ SIZE_ONGAINENEMYLOS where CAI_BaseNPC, m_OnGainEnemyLOS == 0x1bb8);
field_assert!(+ SIZE_ONLOSTENEMYLOS where CAI_BaseNPC, m_OnLostEnemyLOS == 0x1be0);
field_assert!(+ SIZE_ONLOSTENEMY where CAI_BaseNPC, m_OnLostEnemy == 0x1c08);
field_assert!(+ SIZE_ONFOUNDPLAYER where CAI_BaseNPC, m_OnFoundPlayer == 0x1c30);
field_assert!(+ SIZE_ONLOSTPLAYERLOS where CAI_BaseNPC, m_OnLostPlayerLOS == 0x1c58);
field_assert!(+ SIZE_ONLOSTPLAYER where CAI_BaseNPC, m_OnLostPlayer == 0x1c80);
field_assert!(+ SIZE_ONHEARPLAYER where CAI_BaseNPC, m_OnHearPlayer == 0x1ca8);
field_assert!(+ SIZE_ONHEARCOMBAT where CAI_BaseNPC, m_OnHearCombat == 0x1cd0);
field_assert!(+ SIZE_ONSLEEP where CAI_BaseNPC, m_OnSleep == 0x1cf8);
field_assert!(+ SIZE_ONWAKE where CAI_BaseNPC, m_OnWake == 0x1d20);
field_assert!(+ SIZE_ONSTATECHANGE where CAI_BaseNPC, m_OnStateChange == 0x1d48);
field_assert!(+ SIZE_ONFAILEDTOPATH where CAI_BaseNPC, m_OnFailedToPath == 0x1d70);
field_assert!(+ SIZE_ONENTERGOALRADIUS where CAI_BaseNPC, m_OnEnterGoalRadius == 0x1d98);
field_assert!(+ SIZE_ONFINISHEDASSAULT where CAI_BaseNPC, m_OnFinishedAssault == 0x1dc0);
field_assert!(+ SIZE_ONSAWCORPSE where CAI_BaseNPC, m_OnSawCorpse == 0x1de8);
field_assert!(+ SIZE_ONGIVEWEAPON where CAI_BaseNPC, m_OnGiveWeapon == 0x1e10);
field_assert!(+ SIZE_ONTAKEWEAPON where CAI_BaseNPC, m_OnTakeWeapon == 0x1e38);
field_assert!(+ SIZE_ONSPECIALATTACK where CAI_BaseNPC, m_OnSpecialAttack == 0x1e60);
field_assert!(+ SIZE_ONSCHEDULECHANGE where CAI_BaseNPC, m_OnScheduleChange == 0x1e88);
field_assert!(+ SIZE_ONSYNCEDMELEEBEGIN where CAI_BaseNPC, m_OnSyncedMeleeBegin == 0x1eb0);
field_assert!(+ SIZE_QUEUEDOUTPUT where CAI_BaseNPC, m_queuedOutput == 0x1ed8);
field_assert!(+ SIZE_PINITIALIZEDPHYSICSOBJECT where CAI_BaseNPC, m_pInitializedPhysicsObject == 0x1ef8);
field_assert!(+ SIZE_FISUSINGSMALLHULL where CAI_BaseNPC, m_fIsUsingSmallHull == 0x1f00);
field_assert!(+ SIZE_BHADMOVINGGROUND where CAI_BaseNPC, m_bHadMovingGround == 0x1f01);
field_assert!(+ SIZE_BCHECKCONTACTS where CAI_BaseNPC, m_bCheckContacts == 0x1f02);
field_assert!(+ SIZE_BALLOWSHOOT where CAI_BaseNPC, m_bAllowShoot == 0x1f03);
field_assert!(+ SIZE_BACTIVEFORSMARTAMMO where CAI_BaseNPC, m_bActiveForSmartAmmo == 0x1f04);
field_assert!(+ SIZE_BENEMYVALIDSMARTAMMOTARGET where CAI_BaseNPC, m_bEnemyValidSmartAmmoTarget == 0x1f05);
field_assert!(+ SIZE_BALLOWBLOCKBULLETS where CAI_BaseNPC, m_bAllowBlockBullets == 0x1f06);
field_assert!(+ SIZE_BLOCKINGBULLETS where CAI_BaseNPC, m_blockingBullets == 0x1f07);
field_assert!(+ SIZE_REACTINGSURPRISEDREASON where CAI_BaseNPC, m_reactingSurprisedReason == 0x1f08);
field_assert!(+ SIZE_DESIRECROUCH where CAI_BaseNPC, m_desireCrouch == 0x1f0c);
field_assert!(+ SIZE_ISCROUCHING where CAI_BaseNPC, m_isCrouching == 0x1f0d);
field_assert!(+ SIZE_BAUTOMOVEMENTBLOCKED where CAI_BaseNPC, m_bAutoMovementBlocked == 0x1f0e);
field_assert!(+ SIZE_BALLOWPUSHDURINGANIM where CAI_BaseNPC, m_bAllowPushDuringAnim == 0x1f0f);
field_assert!(+ SIZE_DESIRESTANDOVERRIDEEXPIRETIME where CAI_BaseNPC, m_desireStandOverrideExpireTime == 0x1f10);
field_assert!(+ SIZE_SCHEDSELECTORHISTORY where CAI_BaseNPC, m_schedSelectorHistory == 0x1f18);
field_assert!(+ SIZE_BEHAVIORSELECTORID where CAI_BaseNPC, m_behaviorSelectorID == 0x1f40);
field_assert!(+ SIZE_FAILTEXT where CAI_BaseNPC, m_failText == 0x1f48);
field_assert!(+ SIZE_INTERRUPTTEXT where CAI_BaseNPC, m_interruptText == 0x1f50);
field_assert!(+ SIZE_FAILEDSCHEDULE where CAI_BaseNPC, m_failedSchedule == 0x1f58);
field_assert!(+ SIZE_INTERUPTSCHEDULE where CAI_BaseNPC, m_interuptSchedule == 0x1f60);
field_assert!(+ SIZE_NDEBUGCURINDEX where CAI_BaseNPC, m_nDebugCurIndex == 0x1f68);
field_assert!(+ SIZE_FLACCURACYMULTIPLIER where CAI_BaseNPC, m_flAccuracyMultiplier == 0x1f6c);
field_assert!(+ SIZE_FLACCURACYMULTIPLIERFORENEMY where CAI_BaseNPC, m_flAccuracyMultiplierForEnemy == 0x1f70);
field_assert!(+ SIZE_LASTMISSFASTPLAYERTIME where CAI_BaseNPC, m_LastMissFastPlayerTime == 0x1f74);
field_assert!(+ SIZE_LASTSUPPRESSIONTIME where CAI_BaseNPC, m_LastSuppressionTime == 0x1f78);
field_assert!(+ SIZE_LASTSHOOTACCURACY where CAI_BaseNPC, m_LastShootAccuracy == 0x1f7c);
field_assert!(+ SIZE_TOTALSHOTS where CAI_BaseNPC, m_TotalShots == 0x1f80);
field_assert!(+ SIZE_TOTALHITS where CAI_BaseNPC, m_TotalHits == 0x1f84);
field_assert!(+ SIZE_FLSOUNDWAITTIME where CAI_BaseNPC, m_flSoundWaitTime == 0x1f88);
field_assert!(+ SIZE_NSOUNDPRIORITY where CAI_BaseNPC, m_nSoundPriority == 0x1f8c);
field_assert!(+ SIZE_LASTTAUNTTIME where CAI_BaseNPC, m_lastTauntTime == 0x1f90);
field_assert!(+ SIZE_FREEZETIME where CAI_BaseNPC, m_freezeTime == 0x1f94);
field_assert!(+ SIZE_FREEZECYCLE where CAI_BaseNPC, m_freezeCycle == 0x1f98);
field_assert!(+ SIZE_PREVSHIELDHEALTH where CAI_BaseNPC, m_prevShieldHealth == 0x1f9c);
field_assert!(+ SIZE_HEALTHEVALMULTIPLIER where CAI_BaseNPC, m_healthEvalMultiplier == 0x1fa0);
field_assert!(+ SIZE_AIMOVEMENTSPEED where CAI_BaseNPC, m_aiMovementSpeed == 0x1fa4);
field_assert!(+ SIZE_AISPRINTING where CAI_BaseNPC, m_aiSprinting == 0x1fa8);
field_assert!(+ SIZE_AINETWORKFLAGS where CAI_BaseNPC, m_aiNetworkFlags == 0x1fac);
field_assert!(+ SIZE_ISHOLOGRAM where CAI_BaseNPC, m_isHologram == 0x1fb0);
field_assert!(+ SIZE_FIRETEAMSLOTINDEX where CAI_BaseNPC, m_fireteamSlotIndex == 0x1fb4);
field_assert!(+ SIZE_STATUSEFFECTSTIMEDNPCNV where CAI_BaseNPC, m_statusEffectsTimedNPCNV == 0x1fb8);
field_assert!(+ SIZE_STATUSEFFECTSENDLESSNPCNV where CAI_BaseNPC, m_statusEffectsEndlessNPCNV == 0x2000);
field_assert!(+ SIZE_TITLE where CAI_BaseNPC, m_title == 0x2040);
field_assert!(+ SIZE_TETHERED where CAI_BaseNPC, m_tethered == 0x2060);
field_assert!(+ SIZE_NAITRACEMASK where CAI_BaseNPC, m_nAITraceMask == 0x2064);
field_assert!(+ SIZE_FLBOOSTSPEED where CAI_BaseNPC, m_flBoostSpeed == 0x2068);
field_assert!(+ SIZE_BLOCKPERIODSTARTTIME where CAI_BaseNPC, m_blockPeriodStartTime == 0x206c);
field_assert!(+ SIZE_BLOCKBULLETCOUNT where CAI_BaseNPC, m_blockBulletCount == 0x2070);
field_assert!(+ SIZE_DODGEPERIODSTARTTIME where CAI_BaseNPC, m_dodgePeriodStartTime == 0x2074);
field_assert!(+ SIZE_LASTDODGETIME where CAI_BaseNPC, m_lastDodgeTime == 0x2078);
field_assert!(+ SIZE_DODGECOUNT where CAI_BaseNPC, m_dodgeCount == 0x207c);
field_assert!(+ SIZE_DODGEMISSILE where CAI_BaseNPC, m_dodgeMissile == 0x2080);
field_assert!(+ SIZE_DODGEFROMPOS where CAI_BaseNPC, m_dodgeFromPos == 0x2084);
field_assert!(+ SIZE_DANGEROUSAREA where CAI_BaseNPC, m_dangerousArea == 0x2090);
field_assert!(+ SIZE_DANGEROUSAREADEBOUNCEEXPIRETIME where CAI_BaseNPC, m_dangerousAreaDebounceExpireTime == 0x2098);
field_assert!(+ SIZE_GRAPPLED where CAI_BaseNPC, m_grappled == 0x209c);
field_assert!(+ SIZE_GRAPPLEENDTIMING where CAI_BaseNPC, m_grappleEndtiming == 0x209d);
field_assert!(+ SIZE_GRAPPLERESTOREMOVETYPE where CAI_BaseNPC, m_grappleRestoreMoveType == 0x20a0);

impl DerefMut for CAI_BaseNPC {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_BaseNPC {
    type Target = CBaseCombatCharacter;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
