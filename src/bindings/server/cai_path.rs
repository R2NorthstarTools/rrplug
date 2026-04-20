#![allow(non_camel_case_types, non_snake_case)]
use std::{
    ops::{Deref, DerefMut},
    os::raw::c_void,
};

use crate::{
    bindings::server::{cai_component::CAI_Component, EHandle},
    field_assert,
    high::vector::Vector3,
    size_assert,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MovementTurn {
    pub turnAct: i32,               // +0x0 size: 0x4 (0x1 * 0x4) type 5
    pub sequence: i32,              // +0x4 size: 0x4 (0x1 * 0x4) type 5
    pub forwardDistance: f32,       // +0x8 size: 0x4 (0x1 * 0x4) type 1
    pub perpendicularDistance: f32, // +0xc size: 0x4 (0x1 * 0x4) type 1
    pub forwardMoveAdjust: f32,     // +0x10 size: 0x4 (0x1 * 0x4) type 1
}

size_assert!(MOVEMENTTURN where MovementTurn == 0x14);
field_assert!(TURNACT where MovementTurn, turnAct == 0x0);
field_assert!(SIZE_SEQUENCE where MovementTurn, sequence == 0x4);
field_assert!(FORWARDDISTANCE where MovementTurn, forwardDistance == 0x8);
field_assert!(PERPENDICULARDISTANCE where MovementTurn, perpendicularDistance == 0xc);
field_assert!(FORWARDMOVEADJUST where MovementTurn, forwardMoveAdjust == 0x10);

#[repr(C)]
#[derive(Debug)]
pub struct CAI_Path {
    pub base: CAI_Component,
    pub m_Waypoints: *mut c_void, // +0x10 size: 0x8 (0x1 * 0x8) type 0
    pub m_clusterPath: [u8; 32],  // +0x18 size: 0x20 (0x1 * 0x20) type 0
    pub m_clusterPathNoExclusions: [u8; 32], // +0x38 size: 0x20 (0x1 * 0x20) type 0
    pub m_goalTolerance: f32,     // +0x58 size: 0x4 (0x1 * 0x4) type 1
    pub m_activity: [u8; 4],      // +0x5c size: 0x4 (0x1 * 0x4) type 11
    pub m_sequence: i32,          // +0x60 size: 0x4 (0x1 * 0x4) type 5
    pub m_scriptMoveSequence: i32, // +0x64 size: 0x4 (0x1 * 0x4) type 5
    pub m_target: EHandle,        // +0x68 size: 0x4 (0x1 * 0x4) type 13
    pub m_waypointTolerance: f32, // +0x6c size: 0x4 (0x1 * 0x4) type 1
    pub m_movementTurnsAreValid: bool, // +0x70 size: 0x1 (0x1 * 0x1) type 6
    pub gap_71: [u8; 3],
    pub m_leftTurn: MovementTurn, // +0x74 size: 0x14 (0x1 * 0x14) type 10
    pub m_rightTurn: MovementTurn, // +0x88 size: 0x14 (0x1 * 0x14) type 10
    pub m_arrivalActivity: [u8; 4], // +0x9c size: 0x4 (0x1 * 0x4) type 11
    pub m_arrivalSequence: i32,   // +0xa0 size: 0x4 (0x1 * 0x4) type 5
    pub m_animArrivalSequence: i32, // +0xa4 size: 0x4 (0x1 * 0x4) type 5
    pub m_fAnimArrivalDist: f32,  // +0xa8 size: 0x4 (0x1 * 0x4) type 1
    pub m_fAnimArrivalYaw: f32,   // +0xac size: 0x4 (0x1 * 0x4) type 1
    pub m_fAnimArrivalYawOffset: f32, // +0xb0 size: 0x4 (0x1 * 0x4) type 1
    pub m_animArrivalIdealStartPosition: Vector3, // +0xb4 size: 0xc (0x1 * 0xc) type 3
    pub m_animArrivalFlags: i32,  // +0xc0 size: 0x4 (0x1 * 0x4) type 5
    pub m_bAnimArrivalFail: bool, // +0xc4 size: 0x1 (0x1 * 0x1) type 6
    pub gap_c5: [u8; 3],
    pub m_iLastNodeReached: i32,   // +0xc8 size: 0x4 (0x1 * 0x4) type 5
    pub m_goalNode: i32,           // +0xcc size: 0x4 (0x1 * 0x4) type 5
    pub m_startGroundEnt: EHandle, // +0xd0 size: 0x4 (0x1 * 0x4) type 13
    pub m_goalGroundEnt: EHandle,  // +0xd4 size: 0x4 (0x1 * 0x4) type 13
    pub m_bUsedSquadCachedPath: bool, // +0xd8 size: 0x1 (0x1 * 0x1) type 6
    pub gap_d9: [u8; 3],
    pub m_goalPos: Vector3, // +0xdc size: 0xc (0x1 * 0xc) type 15
    pub m_goalPos_worldSpaceCached: Vector3, // +0xe8 size: 0xc (0x1 * 0xc) type 15
    pub m_goalType: i32,    // +0xf4 size: 0x4 (0x1 * 0x4) type 5
    pub m_goalFlags: i32,   // +0xf8 size: 0x4 (0x1 * 0x4) type 5
    pub m_routeStartTime: f32, // +0xfc size: 0x4 (0x1 * 0x4) type 16
    pub m_goalStoppingDistance: f32, // +0x100 size: 0x4 (0x1 * 0x4) type 1
}

size_assert!(CAI_PATH where CAI_Path == 0x108);
field_assert!(+ WAYPOINTS where CAI_Path, m_Waypoints == 0x8);
field_assert!(+ CLUSTERPATH where CAI_Path, m_clusterPath == 0x10);
field_assert!(+ CLUSTERPATHNOEXCLUSIONS where CAI_Path, m_clusterPathNoExclusions == 0x30);
field_assert!(+ GOALTOLERANCE where CAI_Path, m_goalTolerance == 0x50);
field_assert!(+ ACTIVITY where CAI_Path, m_activity == 0x54);
field_assert!(+ SEQUENCE where CAI_Path, m_sequence == 0x58);
field_assert!(+ SCRIPTMOVESEQUENCE where CAI_Path, m_scriptMoveSequence == 0x5c);
field_assert!(+ TARGET where CAI_Path, m_target == 0x60);
field_assert!(+ WAYPOINTTOLERANCE where CAI_Path, m_waypointTolerance == 0x64);
field_assert!(+ MOVEMENTTURNSAREVALID where CAI_Path, m_movementTurnsAreValid == 0x68);
field_assert!(+ LEFTTURN where CAI_Path, m_leftTurn == 0x6c);
field_assert!(+ RIGHTTURN where CAI_Path, m_rightTurn == 0x80);
field_assert!(+ ARRIVALACTIVITY where CAI_Path, m_arrivalActivity == 0x94);
field_assert!(+ ARRIVALSEQUENCE where CAI_Path, m_arrivalSequence == 0x98);
field_assert!(+ ANIMARRIVALSEQUENCE where CAI_Path, m_animArrivalSequence == 0x9c);
field_assert!(+ FANIMARRIVALDIST where CAI_Path, m_fAnimArrivalDist == 0xa0);
field_assert!(+ FANIMARRIVALYAW where CAI_Path, m_fAnimArrivalYaw == 0xa4);
field_assert!(+ FANIMARRIVALYAWOFFSET where CAI_Path, m_fAnimArrivalYawOffset == 0xa8);
field_assert!(+ ANIMARRIVALIDEALSTARTPOSITION where CAI_Path, m_animArrivalIdealStartPosition == 0xac);
field_assert!(+ ANIMARRIVALFLAGS where CAI_Path, m_animArrivalFlags == 0xb8);
field_assert!(+ BANIMARRIVALFAIL where CAI_Path, m_bAnimArrivalFail == 0xbc);
field_assert!(+ ILASTNODEREACHED where CAI_Path, m_iLastNodeReached == 0xc0);
field_assert!(+ GOALNODE where CAI_Path, m_goalNode == 0xc4);
field_assert!(+ STARTGROUNDENT where CAI_Path, m_startGroundEnt == 0xc8);
field_assert!(+ GOALGROUNDENT where CAI_Path, m_goalGroundEnt == 0xcc);
field_assert!(+ BUSEDSQUADCACHEDPATH where CAI_Path, m_bUsedSquadCachedPath == 0xd0);
field_assert!(+ GOALPOS where CAI_Path, m_goalPos == 0xd4);
field_assert!(+ GOALPOS_WORLDSPACECACHED where CAI_Path, m_goalPos_worldSpaceCached == 0xe0);
field_assert!(+ GOALTYPE where CAI_Path, m_goalType == 0xec);
field_assert!(+ GOALFLAGS where CAI_Path, m_goalFlags == 0xf0);
field_assert!(+ ROUTESTARTTIME where CAI_Path, m_routeStartTime == 0xf4);
field_assert!(+ GOALSTOPPINGDISTANCE where CAI_Path, m_goalStoppingDistance == 0xf8);

impl DerefMut for CAI_Path {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_Path {
    type Target = CAI_Component;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
