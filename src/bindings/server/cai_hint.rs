#![allow(non_camel_case_types, non_snake_case)]
use std::{
    ops::{Deref, DerefMut},
    os::raw::c_char,
};

use crate::{
    bindings::server::{cbaseentity::CBaseEntity, EHandle},
    field_assert,
    high::vector::Vector3,
    size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct HintNodeData {
    pub strEntityName: *mut c_char, // +0x0 size: 0x8 (0x1 * 0x8) type 2
    pub vecPosition: Vector3,       // +0x8 size: 0xc (0x1 * 0xc) type 3
    pub nHintType: i16,             // +0x14 size: 0x2 (0x1 * 0x2) type 7
    pub gap_16: [u8; 2],
    pub nNodeID: i32, // +0x18 size: 0x4 (0x1 * 0x4) type 5
    pub gap_1c: [u8; 4],
    pub strGroup: *mut c_char,      // +0x20 size: 0x8 (0x1 * 0x8) type 2
    pub iszGenericType: *mut char,  // +0x28 size: 0x8 (0x1 * 0x8) type 2
    pub iszActivityName: *mut char, // +0x30 size: 0x8 (0x1 * 0x8) type 2
    pub nTargetWCNodeID: i32,       // +0x38 size: 0x4 (0x1 * 0x4) type 5
    pub fIgnoreFacing: i32,         // +0x3c size: 0x4 (0x1 * 0x4) type 5
    pub minState: i32,              // +0x40 size: 0x4 (0x1 * 0x4) type 5
    pub maxState: i32,              // +0x44 size: 0x4 (0x1 * 0x4) type 5
    pub nRadius: i32,               // +0x48 size: 0x4 (0x1 * 0x4) type 5
    pub nWCNodeID: i32,             // +0x4c size: 0x4 (0x1 * 0x4) type 5
}

size_assert!(HINTNODEDATA where HintNodeData == 0x50);
field_assert!(STRENTITYNAME where HintNodeData, strEntityName == 0x0);
field_assert!(VECPOSITION where HintNodeData, vecPosition == 0x8);
field_assert!(NHINTTYPE where HintNodeData, nHintType == 0x14);
field_assert!(NNODEID where HintNodeData, nNodeID == 0x18);
field_assert!(STRGROUP where HintNodeData, strGroup == 0x20);
field_assert!(ISZGENERICTYPE where HintNodeData, iszGenericType == 0x28);
field_assert!(ISZACTIVITYNAME where HintNodeData, iszActivityName == 0x30);
field_assert!(NTARGETWCNODEID where HintNodeData, nTargetWCNodeID == 0x38);
field_assert!(FIGNOREFACING where HintNodeData, fIgnoreFacing == 0x3c);
field_assert!(MINSTATE where HintNodeData, minState == 0x40);
field_assert!(MAXSTATE where HintNodeData, maxState == 0x44);
field_assert!(NRADIUS where HintNodeData, nRadius == 0x48);
field_assert!(NWCNODEID where HintNodeData, nWCNodeID == 0x4c);

#[repr(C)]
#[derive(Debug)]
pub struct CAI_Hint {
    pub base: CBaseEntity,
    pub m_NodeData: HintNodeData, // +0x9e0 size: 0x50 (0x1 * 0x50) type 10
    pub m_hintMovingGroundEnt: EHandle, // +0xa30 size: 0x4 (0x1 * 0x4) type 13
    pub m_hintGroundEnt: EHandle, // +0xa34 size: 0x4 (0x1 * 0x4) type 13
    pub m_hHintOwner: EHandle,    // +0xa38 size: 0x4 (0x1 * 0x4) type 13
    pub m_flNextUseTime: f32,     // +0xa3c size: 0x4 (0x1 * 0x4) type 16
    pub m_nodeFOV: f32,           // +0xa40 size: 0x4 (0x1 * 0x4) type 1
    pub m_nodeFOVcos: f32,        // +0xa44 size: 0x4 (0x1 * 0x4) type 1
    pub m_vecForward: Vector3,    // +0xa48 size: 0xc (0x1 * 0xc) type 3
    pub m_npcIdealYawAlign: f32,  // +0xa54 size: 0x4 (0x1 * 0x4) type 1
    pub m_advanceFromCoverScalar: f32, // +0xa58 size: 0x4 (0x1 * 0x4) type 1
    pub m_hintDisabled: bool,     // +0xa5c size: 0x1 (0x1 * 0x1) type 6
    pub gap_a5d: [u8; 1],
    pub ainData: i16,        // +0xa5e size: 0x2 (0x1 * 0x2) type 7
    pub polyAttachedTo: i32, // +0xa60 size: 0x4 (0x1 * 0x4) type 5
    pub gap_a64: [u8; 4],
    pub hintOnSamePoly_next: *mut CBaseEntity, // +0xa68 size: 0x8 (0x1 * 0x8) type 12
    pub hintOnSamePoly_prev: *mut CBaseEntity, // +0xa70 size: 0x8 (0x1 * 0x8) type 12
}

size_assert!(CAI_HINT where CAI_Hint == 0xa78);
field_assert!(NODEDATA where CAI_Hint, m_NodeData == 0x9d8);
field_assert!(HINTMOVINGGROUNDENT where CAI_Hint, m_hintMovingGroundEnt == 0xa28);
field_assert!(HINTGROUNDENT where CAI_Hint, m_hintGroundEnt == 0xa2c);
field_assert!(HHINTOWNER where CAI_Hint, m_hHintOwner == 0xa30);
field_assert!(FLNEXTUSETIME where CAI_Hint, m_flNextUseTime == 0xa34);
field_assert!(NODEFOV where CAI_Hint, m_nodeFOV == 0xa38);
field_assert!(NODEFOVCOS where CAI_Hint, m_nodeFOVcos == 0xa3c);
field_assert!(VECFORWARD where CAI_Hint, m_vecForward == 0xa40);
field_assert!(NPCIDEALYAWALIGN where CAI_Hint, m_npcIdealYawAlign == 0xa4c);
field_assert!(ADVANCEFROMCOVERSCALAR where CAI_Hint, m_advanceFromCoverScalar == 0xa50);
field_assert!(HINTDISABLED where CAI_Hint, m_hintDisabled == 0xa54);
field_assert!(AINDATA where CAI_Hint, ainData == 0xa56);
field_assert!(POLYATTACHEDTO where CAI_Hint, polyAttachedTo == 0xa58);
field_assert!(HINTONSAMEPOLY_NEXT where CAI_Hint, hintOnSamePoly_next == 0xa60);
field_assert!(HINTONSAMEPOLY_PREV where CAI_Hint, hintOnSamePoly_prev == 0xa68);

impl DerefMut for CAI_Hint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_Hint {
    type Target = CBaseEntity;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
