#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::{cai_behaviorbase::CAI_BehaviorBase, EHandle},
    field_assert,
    prelude::Vector3,
    size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_AssaultBehavior {
    pub base: CAI_BehaviorBase,
    pub m_assaultGroundRelativePos: Vector3, // +0x50 size: 0xc (0x1 * 0xc) type 3
    pub m_assaultMovingGroundEnt: EHandle,   // +0x5c size: 0x4 (0x1 * 0x4) type 13
    pub m_assaultAngles: Vector3,            // +0x60 size: 0xc (0x1 * 0xc) type 3
    pub m_cachedAssaultPosWorldspace: Vector3, // +0x6c size: 0xc (0x1 * 0xc) type 3
    pub m_assaultScriptedAnimRefPos: Vector3, // +0x78 size: 0xc (0x1 * 0xc) type 3
    pub m_assaultScriptedAnimRefAngles: Vector3, // +0x84 size: 0xc (0x1 * 0xc) type 3
    pub m_assaultScriptedAnimSequence: i32,  // +0x90 size: 0x4 (0x1 * 0x4) type 5
    pub gap_94: [u8; 4],
    pub m_assaultScriptedAnimCallback: [u8; 8], // +0x98 size: 0x8 (0x1 * 0x8) type 11
    pub m_flFightRadius: f32,                   // +0xa0 size: 0x4 (0x1 * 0x4) type 1
    pub m_flGoalRadius: f32,                    // +0xa4 size: 0x4 (0x1 * 0x4) type 1
    pub m_flGoalHeight: f32,                    // +0xa8 size: 0x4 (0x1 * 0x4) type 1
    pub m_flArrivalTolerance: f32,              // +0xac size: 0x4 (0x1 * 0x4) type 1
    pub m_bFaceAssaultPointAngles: bool,        // +0xb0 size: 0x1 (0x1 * 0x1) type 6
    pub m_bFinalDestination: bool,              // +0xb1 size: 0x1 (0x1 * 0x1) type 6
    pub m_bHitAssaultPoint: bool,               // +0xb2 size: 0x1 (0x1 * 0x1) type 6
    pub m_bAssaultActive: bool,                 // +0xb3 size: 0x1 (0x1 * 0x1) type 6
    pub m_bIsHaltingAssault: bool,              // +0xb4 size: 0x1 (0x1 * 0x1) type 6
    pub m_bDiverting: bool,                     // +0xb5 size: 0x1 (0x1 * 0x1) type 6
    pub gap_b6: [u8; 2],
    pub m_flTimeDeferScheduleSelection: f32, // +0xb8 size: 0x4 (0x1 * 0x4) type 16
    pub gap_bc: [u8; 4],
    pub m_OnAssaultClear: [u8; 40], // +0xc0 size: 0x28 (0x1 * 0x28) type 11
}

size_assert!(CAI_AssaultBehavior where CAI_AssaultBehavior == 0xe8);
field_assert!(+ SIZE_ASSAULTGROUNDRELATIVEPOS where CAI_AssaultBehavior, m_assaultGroundRelativePos == 0x48);
field_assert!(+ SIZE_ASSAULTMOVINGGROUNDENT where CAI_AssaultBehavior, m_assaultMovingGroundEnt == 0x54);
field_assert!(+ SIZE_ASSAULTANGLES where CAI_AssaultBehavior, m_assaultAngles == 0x58);
field_assert!(+ SIZE_CACHEDASSAULTPOSWORLDSPACE where CAI_AssaultBehavior, m_cachedAssaultPosWorldspace == 0x64);
field_assert!(+ SIZE_ASSAULTSCRIPTEDANIMREFPOS where CAI_AssaultBehavior, m_assaultScriptedAnimRefPos == 0x70);
field_assert!(+ SIZE_ASSAULTSCRIPTEDANIMREFANGLES where CAI_AssaultBehavior, m_assaultScriptedAnimRefAngles == 0x7c);
field_assert!(+ SIZE_ASSAULTSCRIPTEDANIMSEQUENCE where CAI_AssaultBehavior, m_assaultScriptedAnimSequence == 0x88);
field_assert!(+ SIZE_ASSAULTSCRIPTEDANIMCALLBACK where CAI_AssaultBehavior, m_assaultScriptedAnimCallback == 0x90);
field_assert!(+ SIZE_FLFIGHTRADIUS where CAI_AssaultBehavior, m_flFightRadius == 0x98);
field_assert!(+ SIZE_FLGOALRADIUS where CAI_AssaultBehavior, m_flGoalRadius == 0x9c);
field_assert!(+ SIZE_FLGOALHEIGHT where CAI_AssaultBehavior, m_flGoalHeight == 0xa0);
field_assert!(+ SIZE_FLARRIVALTOLERANCE where CAI_AssaultBehavior, m_flArrivalTolerance == 0xa4);
field_assert!(+ SIZE_BFACEASSAULTPOINTANGLES where CAI_AssaultBehavior, m_bFaceAssaultPointAngles == 0xa8);
field_assert!(+ SIZE_BFINALDESTINATION where CAI_AssaultBehavior, m_bFinalDestination == 0xa9);
field_assert!(+ SIZE_BHITASSAULTPOINT where CAI_AssaultBehavior, m_bHitAssaultPoint == 0xaa);
field_assert!(+ SIZE_BASSAULTACTIVE where CAI_AssaultBehavior, m_bAssaultActive == 0xab);
field_assert!(+ SIZE_BISHALTINGASSAULT where CAI_AssaultBehavior, m_bIsHaltingAssault == 0xac);
field_assert!(+ SIZE_BDIVERTING where CAI_AssaultBehavior, m_bDiverting == 0xad);
field_assert!(+ SIZE_FLTIMEDEFERSCHEDULESELECTION where CAI_AssaultBehavior, m_flTimeDeferScheduleSelection == 0xb0);
field_assert!(+ SIZE_ONASSAULTCLEAR where CAI_AssaultBehavior, m_OnAssaultClear == 0xb8);

impl DerefMut for CAI_AssaultBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_AssaultBehavior {
    type Target = CAI_BehaviorBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
