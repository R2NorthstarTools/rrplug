#![allow(non_camel_case_types, non_snake_case)]
use std::{
    ops::{Deref, DerefMut},
    os::raw::c_char,
};

use crate::{
    bindings::server::{cai_base_npc::CAI_BaseNPC, EHandle},
    field_assert,
    prelude::Vector3,
    size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_TrackPather {
    pub base: CAI_BaseNPC,
    pub m_vecDesiredPosition: Vector3, // +0x20b0 size: 0xc (0x1 * 0xc) type 15
    pub m_vecGoalOrientation: Vector3, // +0x20bc size: 0xc (0x1 * 0xc) type 3
    pub m_pCurrentPathTarget: EHandle, // +0x20c8 size: 0x4 (0x1 * 0x4) type 13
    pub m_pDestPathTarget: EHandle,    // +0x20cc size: 0x4 (0x1 * 0x4) type 13
    pub m_pLastPathTarget: EHandle,    // +0x20d0 size: 0x4 (0x1 * 0x4) type 13
    pub m_pTargetNearestPath: EHandle, // +0x20d4 size: 0x4 (0x1 * 0x4) type 13
    pub m_strCurrentPathName: *mut c_char, // +0x20d8 size: 0x8 (0x1 * 0x8) type 2
    pub m_strDestPathName: *mut c_char, // +0x20e0 size: 0x8 (0x1 * 0x8) type 2
    pub m_strLastPathName: *mut c_char, // +0x20e8 size: 0x8 (0x1 * 0x8) type 2
    pub m_strTargetNearestPathName: *mut c_char, // +0x20f0 size: 0x8 (0x1 * 0x8) type 2
    pub m_vecLastGoalCheckPosition: Vector3, // +0x20f8 size: 0xc (0x1 * 0xc) type 15
    pub m_flEnemyPathUpdateTime: f32,  // +0x2104 size: 0x4 (0x1 * 0x4) type 16
    pub m_bForcedMove: bool,           // +0x2108 size: 0x1 (0x1 * 0x1) type 6
    pub m_bPatrolling: bool,           // +0x2109 size: 0x1 (0x1 * 0x1) type 6
    pub m_bPatrolBreakable: bool,      // +0x210a size: 0x1 (0x1 * 0x1) type 6
    pub m_bLeading: bool,              // +0x210b size: 0x1 (0x1 * 0x1) type 6
    pub m_flTargetDistanceThreshold: f32, // +0x210c size: 0x4 (0x1 * 0x4) type 1
    pub m_flAvoidDistance: f32,        // +0x2110 size: 0x4 (0x1 * 0x4) type 1
    pub m_flTargetTolerance: f32,      // +0x2114 size: 0x4 (0x1 * 0x4) type 1
    pub m_vecSegmentStartPoint: Vector3, // +0x2118 size: 0xc (0x1 * 0xc) type 15
    pub m_vecSegmentStartSplinePoint: Vector3, // +0x2124 size: 0xc (0x1 * 0xc) type 15
    pub m_bMovingForward: bool,        // +0x2130 size: 0x1 (0x1 * 0x1) type 6
    pub m_bChooseFarthestPoint: bool,  // +0x2131 size: 0x1 (0x1 * 0x1) type 6
    pub gap_2132: [u8; 2],
    pub m_flFarthestPathDist: f32, // +0x2134 size: 0x4 (0x1 * 0x4) type 1
    pub m_flPathMaxSpeed: f32,     // +0x2138 size: 0x4 (0x1 * 0x4) type 1
    pub m_flTargetDistFromPath: f32, // +0x213c size: 0x4 (0x1 * 0x4) type 1
    pub m_flLeadDistance: f32,     // +0x2140 size: 0x4 (0x1 * 0x4) type 1
    pub m_vecTargetPathDir: Vector3, // +0x2144 size: 0xc (0x1 * 0xc) type 3
    pub m_vecTargetPathPoint: Vector3, // +0x2150 size: 0xc (0x1 * 0xc) type 15
    pub m_nPauseState: i32,        // +0x215c size: 0x4 (0x1 * 0x4) type 5
    pub m_OnArrived: [u8; 40],     // +0x2160 size: 0x28 (0x1 * 0x28) type 11
    pub m_pathDestSpeed: f32,      // +0x2188 size: 0x4 (0x1 * 0x4) type 1
    pub m_defaultDesiredSpeed: f32, // +0x218c size: 0x4 (0x1 * 0x4) type 1
}

size_assert!(CAI_TRACKPATHER where CAI_TrackPather == 0x2190);
field_assert!(+ SIZE_VECDESIREDPOSITION where CAI_TrackPather, m_vecDesiredPosition == 0x20a8);
field_assert!(+ SIZE_VECGOALORIENTATION where CAI_TrackPather, m_vecGoalOrientation == 0x20b4);
field_assert!(+ SIZE_PCURRENTPATHTARGET where CAI_TrackPather, m_pCurrentPathTarget == 0x20c0);
field_assert!(+ SIZE_PDESTPATHTARGET where CAI_TrackPather, m_pDestPathTarget == 0x20c4);
field_assert!(+ SIZE_PLASTPATHTARGET where CAI_TrackPather, m_pLastPathTarget == 0x20c8);
field_assert!(+ SIZE_PTARGETNEARESTPATH where CAI_TrackPather, m_pTargetNearestPath == 0x20cc);
field_assert!(+ SIZE_STRCURRENTPATHNAME where CAI_TrackPather, m_strCurrentPathName == 0x20d0);
field_assert!(+ SIZE_STRDESTPATHNAME where CAI_TrackPather, m_strDestPathName == 0x20d8);
field_assert!(+ SIZE_STRLASTPATHNAME where CAI_TrackPather, m_strLastPathName == 0x20e0);
field_assert!(+ SIZE_STRTARGETNEARESTPATHNAME where CAI_TrackPather, m_strTargetNearestPathName == 0x20e8);
field_assert!(+ SIZE_VECLASTGOALCHECKPOSITION where CAI_TrackPather, m_vecLastGoalCheckPosition == 0x20f0);
field_assert!(+ SIZE_FLENEMYPATHUPDATETIME where CAI_TrackPather, m_flEnemyPathUpdateTime == 0x20fc);
field_assert!(+ SIZE_BFORCEDMOVE where CAI_TrackPather, m_bForcedMove == 0x2100);
field_assert!(+ SIZE_BPATROLLING where CAI_TrackPather, m_bPatrolling == 0x2101);
field_assert!(+ SIZE_BPATROLBREAKABLE where CAI_TrackPather, m_bPatrolBreakable == 0x2102);
field_assert!(+ SIZE_BLEADING where CAI_TrackPather, m_bLeading == 0x2103);
field_assert!(+ SIZE_FLTARGETDISTANCETHRESHOLD where CAI_TrackPather, m_flTargetDistanceThreshold == 0x2104);
field_assert!(+ SIZE_FLAVOIDDISTANCE where CAI_TrackPather, m_flAvoidDistance == 0x2108);
field_assert!(+ SIZE_FLTARGETTOLERANCE where CAI_TrackPather, m_flTargetTolerance == 0x210c);
field_assert!(+ SIZE_VECSEGMENTSTARTPOINT where CAI_TrackPather, m_vecSegmentStartPoint == 0x2110);
field_assert!(+ SIZE_VECSEGMENTSTARTSPLINEPOINT where CAI_TrackPather, m_vecSegmentStartSplinePoint == 0x211c);
field_assert!(+ SIZE_BMOVINGFORWARD where CAI_TrackPather, m_bMovingForward == 0x2128);
field_assert!(+ SIZE_BCHOOSEFARTHESTPOINT where CAI_TrackPather, m_bChooseFarthestPoint == 0x2129);
field_assert!(+ SIZE_FLFARTHESTPATHDIST where CAI_TrackPather, m_flFarthestPathDist == 0x212c);
field_assert!(+ SIZE_FLPATHMAXSPEED where CAI_TrackPather, m_flPathMaxSpeed == 0x2130);
field_assert!(+ SIZE_FLTARGETDISTFROMPATH where CAI_TrackPather, m_flTargetDistFromPath == 0x2134);
field_assert!(+ SIZE_FLLEADDISTANCE where CAI_TrackPather, m_flLeadDistance == 0x2138);
field_assert!(+ SIZE_VECTARGETPATHDIR where CAI_TrackPather, m_vecTargetPathDir == 0x213c);
field_assert!(+ SIZE_VECTARGETPATHPOINT where CAI_TrackPather, m_vecTargetPathPoint == 0x2148);
field_assert!(+ SIZE_NPAUSESTATE where CAI_TrackPather, m_nPauseState == 0x2154);
field_assert!(+ SIZE_ONARRIVED where CAI_TrackPather, m_OnArrived == 0x2158);
field_assert!(+ SIZE_PATHDESTSPEED where CAI_TrackPather, m_pathDestSpeed == 0x2180);
field_assert!(+ SIZE_DEFAULTDESIREDSPEED where CAI_TrackPather, m_defaultDesiredSpeed == 0x2184);

impl DerefMut for CAI_TrackPather {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_TrackPather {
    type Target = CAI_BaseNPC;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
