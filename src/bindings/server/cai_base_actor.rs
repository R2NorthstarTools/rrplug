#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::{cai_baseactorbase::CAI_BaseActorBase, EHandle},
    field_assert,
    high::vector::Vector3,
    size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_BaseActor {
    pub base: CAI_BaseActorBase,
    pub m_flAccumYawDelta: f32,   // +0x20b8 size: 0x4 (0x1 * 0x4) type 1
    pub m_flAccumYawScale: f32,   // +0x20bc size: 0x4 (0x1 * 0x4) type 1
    pub m_fLatchedPositions: i32, // +0x20c0 size: 0x4 (0x1 * 0x4) type 5
    pub m_latchedEyeOrigin: Vector3, // +0x20c4 size: 0xc (0x1 * 0xc) type 3
    pub m_latchedEyeDirection: Vector3, // +0x20d0 size: 0xc (0x1 * 0xc) type 3
    pub m_latchedHeadDirection: Vector3, // +0x20dc size: 0xc (0x1 * 0xc) type 3
    pub m_latchedEyeDirMutex: [u8; 40], // +0x20e8 size: 0x28 (0x1 * 0x28) type 0
    pub m_goalHeadDirection: Vector3, // +0x2110 size: 0xc (0x1 * 0xc) type 3
    pub m_goalHeadInfluence: f32, // +0x211c size: 0x4 (0x1 * 0x4) type 1
    pub m_goalSpineYaw: f32,      // +0x2120 size: 0x4 (0x1 * 0x4) type 1
    pub m_goalBodyYaw: f32,       // +0x2124 size: 0x4 (0x1 * 0x4) type 1
    pub m_goalHeadCorrection: Vector3, // +0x2128 size: 0xc (0x1 * 0xc) type 3
    pub m_hLookTarget: EHandle,   // +0x2134 size: 0x4 (0x1 * 0x4) type 13
    pub m_lookQueue: [u8; 32],    // +0x2138 size: 0x20 (0x1 * 0x20) type 11
    pub m_randomLookQueue: [u8; 32], // +0x2158 size: 0x20 (0x1 * 0x20) type 11
    pub m_flNextRandomLookTime: f32, // +0x2178 size: 0x4 (0x1 * 0x4) type 16
    pub gap_217c: [u8; 4],
    pub m_onNewLookTarget: [u8; 40], // +0x2180 size: 0x28 (0x1 * 0x28) type 11
    pub m_hAimTarget: EHandle,       // +0x21a8 size: 0x4 (0x1 * 0x4) type 13
    pub m_hAimTargetHint: EHandle,   // +0x21ac size: 0x4 (0x1 * 0x4) type 13
    pub m_viewTarget: Vector3,       // +0x21b0 size: 0xc (0x1 * 0xc) type 15
    pub m_viewTargetActive: bool,    // +0x21bc size: 0x1 (0x1 * 0x1) type 6
    pub gap_21bd: [u8; 3],
    pub m_attachHeadFocus: i32,    // +0x21c0 size: 0x4 (0x1 * 0x4) type 5
    pub m_attachChest: i32,        // +0x21c4 size: 0x4 (0x1 * 0x4) type 5
    pub m_attachForward: i32,      // +0x21c8 size: 0x4 (0x1 * 0x4) type 5
    pub m_ParameterBodyYaw: i32,   // +0x21cc size: 0x4 (0x1 * 0x4) type 5
    pub m_ParameterSpineYaw: i32,  // +0x21d0 size: 0x4 (0x1 * 0x4) type 5
    pub m_ParameterNeckTrans: i32, // +0x21d4 size: 0x4 (0x1 * 0x4) type 5
    pub m_ParameterHeadYaw: i32,   // +0x21d8 size: 0x4 (0x1 * 0x4) type 5
    pub m_ParameterHeadPitch: i32, // +0x21dc size: 0x4 (0x1 * 0x4) type 5
    pub m_ParameterHeadRoll: i32,  // +0x21e0 size: 0x4 (0x1 * 0x4) type 5
    pub m_ParameterGestureHeight: i32, // +0x21e4 size: 0x4 (0x1 * 0x4) type 5
    pub m_ParameterGestureWidth: i32, // +0x21e8 size: 0x4 (0x1 * 0x4) type 5
}

size_assert!(CAI_BASEACTOR where CAI_BaseActor == 0x21f0);
field_assert!(SIZE_FLACCUMYAWDELTA where CAI_BaseActor, m_flAccumYawDelta == 0x20b0);
field_assert!(SIZE_FLACCUMYAWSCALE where CAI_BaseActor, m_flAccumYawScale == 0x20b4);
field_assert!(SIZE_FLATCHEDPOSITIONS where CAI_BaseActor, m_fLatchedPositions == 0x20b8);
field_assert!(SIZE_LATCHEDEYEORIGIN where CAI_BaseActor, m_latchedEyeOrigin == 0x20bc);
field_assert!(SIZE_LATCHEDEYEDIRECTION where CAI_BaseActor, m_latchedEyeDirection == 0x20c8);
field_assert!(SIZE_LATCHEDHEADDIRECTION where CAI_BaseActor, m_latchedHeadDirection == 0x20d4);
field_assert!(SIZE_LATCHEDEYEDIRMUTEX where CAI_BaseActor, m_latchedEyeDirMutex == 0x20e0);
field_assert!(SIZE_GOALHEADDIRECTION where CAI_BaseActor, m_goalHeadDirection == 0x2108);
field_assert!(SIZE_GOALHEADINFLUENCE where CAI_BaseActor, m_goalHeadInfluence == 0x2114);
field_assert!(SIZE_GOALSPINEYAW where CAI_BaseActor, m_goalSpineYaw == 0x2118);
field_assert!(SIZE_GOALBODYYAW where CAI_BaseActor, m_goalBodyYaw == 0x211c);
field_assert!(SIZE_GOALHEADCORRECTION where CAI_BaseActor, m_goalHeadCorrection == 0x2120);
field_assert!(SIZE_HLOOKTARGET where CAI_BaseActor, m_hLookTarget == 0x212c);
field_assert!(SIZE_LOOKQUEUE where CAI_BaseActor, m_lookQueue == 0x2130);
field_assert!(SIZE_RANDOMLOOKQUEUE where CAI_BaseActor, m_randomLookQueue == 0x2150);
field_assert!(SIZE_FLNEXTRANDOMLOOKTIME where CAI_BaseActor, m_flNextRandomLookTime == 0x2170);
field_assert!(SIZE_ONNEWLOOKTARGET where CAI_BaseActor, m_onNewLookTarget == 0x2178);
field_assert!(SIZE_HAIMTARGET where CAI_BaseActor, m_hAimTarget == 0x21a0);
field_assert!(SIZE_HAIMTARGETHINT where CAI_BaseActor, m_hAimTargetHint == 0x21a4);
field_assert!(SIZE_VIEWTARGET where CAI_BaseActor, m_viewTarget == 0x21a8);
field_assert!(SIZE_VIEWTARGETACTIVE where CAI_BaseActor, m_viewTargetActive == 0x21b4);
field_assert!(SIZE_ATTACHHEADFOCUS where CAI_BaseActor, m_attachHeadFocus == 0x21b8);
field_assert!(SIZE_ATTACHCHEST where CAI_BaseActor, m_attachChest == 0x21bc);
field_assert!(SIZE_ATTACHFORWARD where CAI_BaseActor, m_attachForward == 0x21c0);
field_assert!(SIZE_PARAMETERBODYYAW where CAI_BaseActor, m_ParameterBodyYaw == 0x21c4);
field_assert!(SIZE_PARAMETERSPINEYAW where CAI_BaseActor, m_ParameterSpineYaw == 0x21c8);
field_assert!(SIZE_PARAMETERNECKTRANS where CAI_BaseActor, m_ParameterNeckTrans == 0x21cc);
field_assert!(SIZE_PARAMETERHEADYAW where CAI_BaseActor, m_ParameterHeadYaw == 0x21d0);
field_assert!(SIZE_PARAMETERHEADPITCH where CAI_BaseActor, m_ParameterHeadPitch == 0x21d4);
field_assert!(SIZE_PARAMETERHEADROLL where CAI_BaseActor, m_ParameterHeadRoll == 0x21d8);
field_assert!(SIZE_PARAMETERGESTUREHEIGHT where CAI_BaseActor, m_ParameterGestureHeight == 0x21dc);
field_assert!(SIZE_PARAMETERGESTUREWIDTH where CAI_BaseActor, m_ParameterGestureWidth == 0x21e0);

impl DerefMut for CAI_BaseActor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_BaseActor {
    type Target = CAI_BaseActorBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
