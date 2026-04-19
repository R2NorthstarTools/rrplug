#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::{
        cai_behaviorbase::CAI_BehaviorBase, cai_movemonitor::CAI_MoveMonitor,
        cbaseentity::CBaseEntity, crand_sim_timer::CRandSimTimer, crand_stopwatch::CRandStopwatch,
        csim_timer::CSimTimer, csimple_sim_timer::CSimpleSimTimer, EHandle,
    },
    field_assert,
    high::vector::Vector3,
    size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct AI_FollowParams_t {
    pub formation: i32,             // +0x0 size: 0x4 (0x1 * 0x4) type 5
    pub targetMoveTolerance: f32,   // +0x4 size: 0x4 (0x1 * 0x4) type 1
    pub goalTolerance: f32,         // +0x8 size: 0x4 (0x1 * 0x4) type 1
    pub goalCombatTolerance: f32,   // +0xc size: 0x4 (0x1 * 0x4) type 1
    pub bNormalMemoryDiscard: bool, // +0x10 size: 0x1 (0x1 * 0x1) type 6
}

size_assert!(AI_FOLLOWPARAMS_T  where AI_FollowParams_t == 0x14);
field_assert!(FORMATION  where AI_FollowParams_t, formation == 0x0);
field_assert!(TARGETMOVETOLERANCE  where AI_FollowParams_t, targetMoveTolerance == 0x4);
field_assert!(GOALTOLERANCE  where AI_FollowParams_t, goalTolerance == 0x8);
field_assert!(GOALCOMBATTOLERANCE  where AI_FollowParams_t, goalCombatTolerance == 0xc);
field_assert!(BNORMALMEMORYDISCARD  where AI_FollowParams_t, bNormalMemoryDiscard == 0x10);

#[repr(C)]
#[derive(Debug)]
pub struct AI_FollowNavInfo_t {
    pub flags: i32,                  // +0x0 size: 0x4 (0x1 * 0x4) type 5
    pub position: Vector3,           // +0x4 size: 0xc (0x1 * 0xc) type 15
    pub facingAngle: f32,            // +0x10 size: 0x4 (0x1 * 0x4) type 1
    pub Zrange: f32,                 // +0x14 size: 0x4 (0x1 * 0x4) type 1
    pub toleranceMargin: f32,        // +0x18 size: 0x4 (0x1 * 0x4) type 1
    pub followPointTolerance: f32,   // +0x1c size: 0x4 (0x1 * 0x4) type 1
    pub repathOnRouteTolerance: f32, // +0x20 size: 0x4 (0x1 * 0x4) type 1
    pub walkTolerance: f32,          // +0x24 size: 0x4 (0x1 * 0x4) type 1
}

size_assert!(AI_FOLLOWNAVINFO_T where AI_FollowNavInfo_t == 0x28);
field_assert!(FLAGS where AI_FollowNavInfo_t, flags == 0x0);
field_assert!(POSITION where AI_FollowNavInfo_t, position == 0x4);
field_assert!(FACINGANGLE where AI_FollowNavInfo_t, facingAngle == 0x10);
field_assert!(ZRANGE where AI_FollowNavInfo_t, Zrange == 0x14);
field_assert!(TOLERANCEMARGIN where AI_FollowNavInfo_t, toleranceMargin == 0x18);
field_assert!(FOLLOWPOINTTOLERANCE where AI_FollowNavInfo_t, followPointTolerance == 0x1c);
field_assert!(REPATHONROUTETOLERANCE where AI_FollowNavInfo_t, repathOnRouteTolerance == 0x20);
field_assert!(WALKTOLERANCE where AI_FollowNavInfo_t, walkTolerance == 0x24);

#[repr(C)]
#[derive(Debug)]
pub struct CAI_FollowBehavior {
    pub base: CAI_BehaviorBase,
    pub m_hFollowTarget: EHandle, // +0x50 size: 0x4 (0x1 * 0x4) type 13
    pub m_FollowNavGoal: AI_FollowNavInfo_t, // +0x54 size: 0x28 (0x1 * 0x28) type 10
    pub m_flTimeUpdatedFollowPosition: f32, // +0x7c size: 0x4 (0x1 * 0x4) type 16
    pub m_flTimeFollowTargetVisible: f32, // +0x80 size: 0x4 (0x1 * 0x4) type 16
    pub m_TargetMonitor: CAI_MoveMonitor, // +0x84 size: 0x18 (0x1 * 0x18) type 10
    pub m_bTargetUnreachable: bool, // +0x9c size: 0x1 (0x1 * 0x1) type 6
    pub m_bOffsetPosNotValid: bool, // +0x9d size: 0x1 (0x1 * 0x1) type 6
    pub m_bNavBlockedSkipFollowBehavior: bool, // +0x9e size: 0x1 (0x1 * 0x1) type 6
    pub gap_9f: [u8; 1],
    pub m_vFollowMoveAnchor: Vector3, // +0xa0 size: 0xc (0x1 * 0xc) type 15
    pub m_successfulFollowTargetMonitorSerialNumber: i32, // +0xac size: 0x4 (0x1 * 0x4) type 5
    pub m_successfulFollowMonitorMyPosition: CAI_MoveMonitor, // +0xb0 size: 0x18 (0x1 * 0x18) type 10
    pub m_flOriginalEnemyDiscardTime: f32,                    // +0xc8 size: 0x4 (0x1 * 0x4) type 1
    pub m_retryCoverTime: f32,                                // +0xcc size: 0x4 (0x1 * 0x4) type 1
    pub m_FollowDelay: CRandStopwatch, // +0xd0 size: 0x10 (0x1 * 0x10) type 10
    pub m_RepathOnFollowTimer: CSimpleSimTimer, // +0xe0 size: 0x4 (0x1 * 0x4) type 10
    pub m_CurrentFollowActivity: [u8; 4], // +0xe4 size: 0x4 (0x1 * 0x4) type 11
    pub m_TimeBlockUseWaitPoint: CRandSimTimer, // +0xe8 size: 0xc (0x1 * 0xc) type 10
    pub m_TimeCheckForWaitPoint: CSimTimer, // +0xf4 size: 0x8 (0x1 * 0x8) type 10
    pub gap_fc: [u8; 4],
    pub m_pInterruptWaitPoint: *mut CBaseEntity, // +0x100 size: 0x8 (0x1 * 0x8) type 12
    pub m_TimeBeforeSpreadFacing: CRandSimTimer, // +0x108 size: 0xc (0x1 * 0xc) type 10
    pub m_TimeNextSpreadFacing: CRandSimTimer,   // +0x114 size: 0xc (0x1 * 0xc) type 10
    pub m_hFollowManagerInfo: [u8; 16],          // +0x120 size: 0x10 (0x1 * 0x10) type 10
    pub m_followParams: AI_FollowParams_t,       // +0x130 size: 0x14 (0x1 * 0x14) type 10
}

size_assert!(CAI_FOLLOWBEHAVIOR  where CAI_FollowBehavior == 0x148);
field_assert!(SIZE_HFOLLOWTARGET  where CAI_FollowBehavior, m_hFollowTarget == 0x48);
field_assert!(SIZE_FOLLOWNAVGOAL  where CAI_FollowBehavior, m_FollowNavGoal == 0x4c);
field_assert!(SIZE_FLTIMEUPDATEDFOLLOWPOSITION  where CAI_FollowBehavior, m_flTimeUpdatedFollowPosition == 0x74);
field_assert!(SIZE_FLTIMEFOLLOWTARGETVISIBLE  where CAI_FollowBehavior, m_flTimeFollowTargetVisible == 0x78);
field_assert!(SIZE_TARGETMONITOR  where CAI_FollowBehavior, m_TargetMonitor == 0x7c);
field_assert!(SIZE_BTARGETUNREACHABLE  where CAI_FollowBehavior, m_bTargetUnreachable == 0x94);
field_assert!(SIZE_BOFFSETPOSNOTVALID  where CAI_FollowBehavior, m_bOffsetPosNotValid == 0x95);
field_assert!(SIZE_BNAVBLOCKEDSKIPFOLLOWBEHAVIOR  where CAI_FollowBehavior, m_bNavBlockedSkipFollowBehavior == 0x96);
field_assert!(SIZE_VFOLLOWMOVEANCHOR  where CAI_FollowBehavior, m_vFollowMoveAnchor == 0x98);
field_assert!(SIZE_SUCCESSFULFOLLOWTARGETMONITORSERIALNUMBER  where CAI_FollowBehavior, m_successfulFollowTargetMonitorSerialNumber == 0xa4);
field_assert!(SIZE_SUCCESSFULFOLLOWMONITORMYPOSITION  where CAI_FollowBehavior, m_successfulFollowMonitorMyPosition == 0xa8);
field_assert!(SIZE_FLORIGINALENEMYDISCARDTIME  where CAI_FollowBehavior, m_flOriginalEnemyDiscardTime == 0xc0);
field_assert!(SIZE_RETRYCOVERTIME  where CAI_FollowBehavior, m_retryCoverTime == 0xc4);
field_assert!(SIZE_FOLLOWDELAY  where CAI_FollowBehavior, m_FollowDelay == 0xc8);
field_assert!(SIZE_REPATHONFOLLOWTIMER  where CAI_FollowBehavior, m_RepathOnFollowTimer == 0xd8);
field_assert!(SIZE_CURRENTFOLLOWACTIVITY  where CAI_FollowBehavior, m_CurrentFollowActivity == 0xdc);
field_assert!(SIZE_TIMEBLOCKUSEWAITPOINT  where CAI_FollowBehavior, m_TimeBlockUseWaitPoint == 0xe0);
field_assert!(SIZE_TIMECHECKFORWAITPOINT  where CAI_FollowBehavior, m_TimeCheckForWaitPoint == 0xec);
field_assert!(SIZE_PINTERRUPTWAITPOINT  where CAI_FollowBehavior, m_pInterruptWaitPoint == 0xf8);
field_assert!(SIZE_TIMEBEFORESPREADFACING  where CAI_FollowBehavior, m_TimeBeforeSpreadFacing == 0x100);
field_assert!(SIZE_TIMENEXTSPREADFACING  where CAI_FollowBehavior, m_TimeNextSpreadFacing == 0x10c);
field_assert!(SIZE_HFOLLOWMANAGERINFO  where CAI_FollowBehavior, m_hFollowManagerInfo == 0x118);
field_assert!(SIZE_FOLLOWPARAMS  where CAI_FollowBehavior, m_followParams == 0x128);

impl DerefMut for CAI_FollowBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_FollowBehavior {
    type Target = CAI_BehaviorBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
