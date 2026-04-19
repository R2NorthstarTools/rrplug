#![allow(non_camel_case_types, non_snake_case)]
use std::{
    ops::{Deref, DerefMut},
    os::raw::c_void,
};

use crate::{
    bindings::server::{cai_component::CAI_Component, cai_path::CAI_Path, EHandle},
    field_assert,
    high::vector::Vector3,
    size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_Navigator {
    pub base: CAI_Component,
    pub CAI_DefMovementSink: *mut c_void, // +0x10 size: 0x8 (0x0 * 0x8) type 0
    pub m_navType: i32,                   // +0x18 size: 0x4 (0x1 * 0x4) type 5
    pub m_fNavComplete: bool,             // +0x1c size: 0x1 (0x1 * 0x1) type 6
    pub m_bLastNavFailed: bool,           // +0x1d size: 0x1 (0x1 * 0x1) type 6
    pub gap_1e: [u8; 2],
    pub m_pMotor: *mut c_void,     // +0x20 size: 0x8 (0x1 * 0x8) type 31
    pub m_pMoveProbe: *mut c_void, // +0x28 size: 0x8 (0x1 * 0x8) type 31
    pub m_pLocalNavigator: *mut c_void, // +0x30 size: 0x8 (0x1 * 0x8) type 31
    pub m_pAINetwork: *mut c_void, // +0x38 size: 0x8 (0x1 * 0x8) type 31
    pub m_pPath: *mut CAI_Path,    // +0x40 size: 0x8 (0x1 * 0x8) type 10
    pub m_utilityPath: [u8; 32],   // +0x48 size: 0x20 (0x1 * 0x20) type 0
    pub m_pClippedWaypoints: *mut c_void, // +0x68 size: 0x8 (0x1 * 0x8) type 31
    pub m_flTimeClipped: f32,      // +0x70 size: 0x4 (0x1 * 0x4) type 16
    pub m_PreviousMoveActivity: i32, // +0x74 size: 0x4 (0x1 * 0x4) type 5
    pub m_PreviousArrivalActivity: i32, // +0x78 size: 0x4 (0x1 * 0x4) type 5
    pub m_moveTransitionAnim: i32, // +0x7c size: 0x4 (0x1 * 0x4) type 5
    pub m_bValidateActivitySpeed: bool, // +0x80 size: 0x1 (0x1 * 0x1) type 6
    pub m_bCalledStartMove: bool,  // +0x81 size: 0x1 (0x1 * 0x1) type 6
    pub m_bAdjustMoveSpeedToSquad: bool, // +0x82 size: 0x1 (0x1 * 0x1) type 6
    pub m_bForcedSimplify: bool,   // +0x83 size: 0x1 (0x1 * 0x1) type 6
    pub m_flNextSimplifyTime: f32, // +0x84 size: 0x4 (0x1 * 0x4) type 16
    pub m_flLastSuccessfulSimplifyTime: f32, // +0x88 size: 0x4 (0x1 * 0x4) type 16
    pub m_flTimeLastAvoidanceTriangulate: f32, // +0x8c size: 0x4 (0x1 * 0x4) type 16
    pub m_timePathRebuildMax: f32, // +0x90 size: 0x4 (0x1 * 0x4) type 1
    pub m_timePathRebuildDelay: f32, // +0x94 size: 0x4 (0x1 * 0x4) type 1
    pub m_timePathRebuildFail: f32, // +0x98 size: 0x4 (0x1 * 0x4) type 16
    pub m_timePathRebuildNext: f32, // +0x9c size: 0x4 (0x1 * 0x4) type 16
    pub m_prevPathCorner: EHandle, // +0xa0 size: 0x4 (0x1 * 0x4) type 13
    pub m_nextPathCorner: EHandle, // +0xa4 size: 0x4 (0x1 * 0x4) type 13
    pub m_savePathCornerOnScheduleChange: bool, // +0xa8 size: 0x1 (0x1 * 0x1) type 6
    pub m_pathCornerDirectionForward: bool, // +0xa9 size: 0x1 (0x1 * 0x1) type 6
    pub m_fRememberStaleNodes: bool, // +0xaa size: 0x1 (0x1 * 0x1) type 6
    pub m_bNoPathcornerPathfinds: bool, // +0xab size: 0x1 (0x1 * 0x1) type 6
    pub m_checkClusterDangerAtTime: f32, // +0xac size: 0x4 (0x1 * 0x4) type 1
    pub m_bHadPath: bool,          // +0xb0 size: 0x1 (0x1 * 0x1) type 6
    pub m_bAnimWasMoving: bool,    // +0xb1 size: 0x1 (0x1 * 0x1) type 6
    pub m_setConditionPathInvolvesDangerousCluster: bool, // +0xb2 size: 0x1 (0x1 * 0x1) type 6
    pub m_bProbeHitPhysicsObject: bool, // +0xb3 size: 0x1 (0x1 * 0x1) type 6
    pub m_bPathBlockedByPhysicsObject: bool, // +0xb4 size: 0x1 (0x1 * 0x1) type 6
    pub m_bPathBlockedByNPC: bool, // +0xb5 size: 0x1 (0x1 * 0x1) type 6
    pub m_bProbeHitNPC: bool,      // +0xb6 size: 0x1 (0x1 * 0x1) type 6
    pub m_fPeerMoveWait: bool,     // +0xb7 size: 0x1 (0x1 * 0x1) type 6
    pub m_hPeerWaitingOn: EHandle, // +0xb8 size: 0x4 (0x1 * 0x4) type 13
    pub m_PeerWaitMoveTimer: [u8; 8], // +0xbc size: 0x8 (0x1 * 0x8) type 10
    pub m_PeerWaitClearTimer: [u8; 8], // +0xc4 size: 0x8 (0x1 * 0x8) type 10
    pub m_NextSidestepTimer: [u8; 8], // +0xcc size: 0x8 (0x1 * 0x8) type 10
    pub m_hBigStepGroundEnt: EHandle, // +0xd4 size: 0x4 (0x1 * 0x4) type 13
    pub m_hLastBlockingEnt: EHandle, // +0xd8 size: 0x4 (0x1 * 0x4) type 13
    pub m_hAvoidEnt: EHandle,      // +0xdc size: 0x4 (0x1 * 0x4) type 13
    pub m_avoidDistSqr: f32,       // +0xe0 size: 0x4 (0x1 * 0x4) type 1
    pub m_vPosBeginFailedSteer: Vector3, // +0xe4 size: 0xc (0x1 * 0xc) type 3
    pub m_timeBeginFailedSteer: f32, // +0xf0 size: 0x4 (0x1 * 0x4) type 16
    pub m_traverseRefYaw: f32,     // +0xf4 size: 0x4 (0x1 * 0x4) type 1
    pub m_traverseRefPos: Vector3, // +0xf8 size: 0xc (0x1 * 0xc) type 3
    pub m_traversePlantPos: Vector3, // +0x104 size: 0xc (0x1 * 0xc) type 3
    pub m_nNavFailCounter: i32,    // +0x110 size: 0x4 (0x1 * 0x4) type 5
    pub m_flLastNavFailTime: f32,  // +0x114 size: 0x4 (0x1 * 0x4) type 1
    pub m_flLastPathFindTime: f32, // +0x118 size: 0x4 (0x1 * 0x4) type 16
    pub m_moveFlags: i32,          // +0x11c size: 0x4 (0x1 * 0x4) type 5
}

size_assert!(CAI_NAVIGATOR where CAI_Navigator == 0x120);
field_assert!(CAI_DEFMOVEMENTSINK where CAI_Navigator, CAI_DefMovementSink == 0x8);
field_assert!(M_NAVTYPE where CAI_Navigator, m_navType == 0x10);
field_assert!(M_FNAVCOMPLETE where CAI_Navigator, m_fNavComplete == 0x14);
field_assert!(M_BLASTNAVFAILED where CAI_Navigator, m_bLastNavFailed == 0x15);
field_assert!(M_PMOTOR where CAI_Navigator, m_pMotor == 0x18);
field_assert!(M_PMOVEPROBE where CAI_Navigator, m_pMoveProbe == 0x20);
field_assert!(M_PLOCALNAVIGATOR where CAI_Navigator, m_pLocalNavigator == 0x28);
field_assert!(M_PAINETWORK where CAI_Navigator, m_pAINetwork == 0x30);
field_assert!(M_PPATH where CAI_Navigator, m_pPath == 0x38);
field_assert!(M_UTILITYPATH where CAI_Navigator, m_utilityPath == 0x40);
field_assert!(M_PCLIPPEDWAYPOINTS where CAI_Navigator, m_pClippedWaypoints == 0x60);
field_assert!(M_FLTIMECLIPPED where CAI_Navigator, m_flTimeClipped == 0x68);
field_assert!(M_PREVIOUSMOVEACTIVITY where CAI_Navigator, m_PreviousMoveActivity == 0x6c);
field_assert!(M_PREVIOUSARRIVALACTIVITY where CAI_Navigator, m_PreviousArrivalActivity == 0x70);
field_assert!(M_MOVETRANSITIONANIM where CAI_Navigator, m_moveTransitionAnim == 0x74);
field_assert!(M_BVALIDATEACTIVITYSPEED where CAI_Navigator, m_bValidateActivitySpeed == 0x78);
field_assert!(M_BCALLEDSTARTMOVE where CAI_Navigator, m_bCalledStartMove == 0x79);
field_assert!(M_BADJUSTMOVESPEEDTOSQUAD where CAI_Navigator, m_bAdjustMoveSpeedToSquad == 0x7a);
field_assert!(M_BFORCEDSIMPLIFY where CAI_Navigator, m_bForcedSimplify == 0x7b);
field_assert!(M_FLNEXTSIMPLIFYTIME where CAI_Navigator, m_flNextSimplifyTime == 0x7c);
field_assert!(M_FLLASTSUCCESSFULSIMPLIFYTIME where CAI_Navigator, m_flLastSuccessfulSimplifyTime == 0x80);
field_assert!(M_FLTIMELASTAVOIDANCETRIANGULATE where CAI_Navigator, m_flTimeLastAvoidanceTriangulate == 0x84);
field_assert!(M_TIMEPATHREBUILDMAX where CAI_Navigator, m_timePathRebuildMax == 0x88);
field_assert!(M_TIMEPATHREBUILDDELAY where CAI_Navigator, m_timePathRebuildDelay == 0x8c);
field_assert!(M_TIMEPATHREBUILDFAIL where CAI_Navigator, m_timePathRebuildFail == 0x90);
field_assert!(M_TIMEPATHREBUILDNEXT where CAI_Navigator, m_timePathRebuildNext == 0x94);
field_assert!(M_PREVPATHCORNER where CAI_Navigator, m_prevPathCorner == 0x98);
field_assert!(M_NEXTPATHCORNER where CAI_Navigator, m_nextPathCorner == 0x9c);
field_assert!(M_SAVEPATHCORNERONSCHEDULECHANGE where CAI_Navigator, m_savePathCornerOnScheduleChange == 0xa0);
field_assert!(M_PATHCORNERDIRECTIONFORWARD where CAI_Navigator, m_pathCornerDirectionForward == 0xa1);
field_assert!(M_FREMEMBERSTALENODES where CAI_Navigator, m_fRememberStaleNodes == 0xa2);
field_assert!(M_BNOPATHCORNERPATHFINDS where CAI_Navigator, m_bNoPathcornerPathfinds == 0xa3);
field_assert!(M_CHECKCLUSTERDANGERATTIME where CAI_Navigator, m_checkClusterDangerAtTime == 0xa4);
field_assert!(M_BHADPATH where CAI_Navigator, m_bHadPath == 0xa8);
field_assert!(M_BANIMWASMOVING where CAI_Navigator, m_bAnimWasMoving == 0xa9);
field_assert!(M_SETCONDITIONPATHINVOLVESDANGEROUSCLUSTER where CAI_Navigator, m_setConditionPathInvolvesDangerousCluster == 0xaa);
field_assert!(M_BPROBEHITPHYSICSOBJECT where CAI_Navigator, m_bProbeHitPhysicsObject == 0xab);
field_assert!(M_BPATHBLOCKEDBYPHYSICSOBJECT where CAI_Navigator, m_bPathBlockedByPhysicsObject == 0xac);
field_assert!(M_BPATHBLOCKEDBYNPC where CAI_Navigator, m_bPathBlockedByNPC == 0xad);
field_assert!(M_BPROBEHITNPC where CAI_Navigator, m_bProbeHitNPC == 0xae);
field_assert!(M_FPEERMOVEWAIT where CAI_Navigator, m_fPeerMoveWait == 0xaf);
field_assert!(M_HPEERWAITINGON where CAI_Navigator, m_hPeerWaitingOn == 0xb0);
field_assert!(M_PEERWAITMOVETIMER where CAI_Navigator, m_PeerWaitMoveTimer == 0xb4);
field_assert!(M_PEERWAITCLEARTIMER where CAI_Navigator, m_PeerWaitClearTimer == 0xbc);
field_assert!(M_NEXTSIDESTEPTIMER where CAI_Navigator, m_NextSidestepTimer == 0xc4);
field_assert!(M_HBIGSTEPGROUNDENT where CAI_Navigator, m_hBigStepGroundEnt == 0xcc);
field_assert!(M_HLASTBLOCKINGENT where CAI_Navigator, m_hLastBlockingEnt == 0xd0);
field_assert!(M_HAVOIDENT where CAI_Navigator, m_hAvoidEnt == 0xd4);
field_assert!(M_AVOIDDISTSQR where CAI_Navigator, m_avoidDistSqr == 0xd8);
field_assert!(M_VPOSBEGINFAILEDSTEER where CAI_Navigator, m_vPosBeginFailedSteer == 0xdc);
field_assert!(M_TIMEBEGINFAILEDSTEER where CAI_Navigator, m_timeBeginFailedSteer == 0xe8);
field_assert!(M_TRAVERSEREFYAW where CAI_Navigator, m_traverseRefYaw == 0xec);
field_assert!(M_TRAVERSEREFPOS where CAI_Navigator, m_traverseRefPos == 0xf0);
field_assert!(M_TRAVERSEPLANTPOS where CAI_Navigator, m_traversePlantPos == 0xfc);
field_assert!(M_NNAVFAILCOUNTER where CAI_Navigator, m_nNavFailCounter == 0x108);
field_assert!(M_FLLASTNAVFAILTIME where CAI_Navigator, m_flLastNavFailTime == 0x10c);
field_assert!(M_FLLASTPATHFINDTIME where CAI_Navigator, m_flLastPathFindTime == 0x110);
field_assert!(M_MOVEFLAGS where CAI_Navigator, m_moveFlags == 0x114);

impl DerefMut for CAI_Navigator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_Navigator {
    type Target = CAI_Component;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
