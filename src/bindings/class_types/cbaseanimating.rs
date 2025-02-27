#![allow(non_camel_case_types, non_snake_case)]

use std::{
    ffi::{c_char, c_void},
    ops::{Deref, DerefMut},
};

use crate::{
    bindings::class_types::cplayer::EHandle,
    high::vector::{matrix3x4_t, Quaternion, Vector3},
    size_assert,
};

use super::cbaseentity::CBaseEntity;

#[repr(C)]
pub struct AnimRelativeData {
    vftable: *const c_void,
    m_animInitialPos: Vector3,             // 0x8 ( Size = 12 )
    m_animInitialVel: Vector3,             // 0x14 ( Size = 12 )
    m_animInitialRot: Quaternion,          // 0x20 ( Size = 16 )
    m_animInitialCorrectPos: Vector3,      // 0x30 ( Size = 12 )
    m_animInitialCorrectRot: Quaternion,   // 0x3c ( Size = 16 )
    m_animEntityToRefOffset: Vector3,      // 0x4c ( Size = 12 )
    m_animEntityToRefRotation: Quaternion, // 0x58 ( Size = 16 )
    m_animBeginTime: f32,                  // 0x68 ( Size = 4 )
    m_animEndTime: f32,                    // 0x6c ( Size = 4 )
    m_animScriptSequence: i32,             // 0x70 ( Size = 4 )
    m_animScriptModel: i32,                // 0x74 ( Size = 4 )
    m_animIgnoreParentRot: bool,           // 0x78 ( Size = 1 )
    gap_79: [c_char; 3],
    m_animMotionMode: i32, // 0x7c ( Size = 4 )
}
size_assert!(SIZE_ANIM where AnimRelativeData == 128);

#[repr(C)]
pub struct PredictedAnimEventData {
    vftable: *const c_void,
    m_predictedAnimEventTimes: [f32; 8],   // 0x8 ( Size = 32 )
    m_predictedAnimEventIndices: [i32; 8], // 0x28 ( Size = 32 )
    m_predictedAnimEventCount: i32,        // 0x48 ( Size = 4 )
    m_predictedAnimEventTarget: EHandle,   // 0x4c ( Size = 4 )
    m_predictedAnimEventSequence: i32,     // 0x50 ( Size = 4 )
    m_predictedAnimEventModel: i32,        // 0x54 ( Size = 4 )
    m_predictedAnimEventsReadyToFireTime: f32, // 0x58 ( Size = 4 )
}
size_assert!(SIZE_PREDICATED_ANIM where PredictedAnimEventData == 96);

#[repr(C)]
pub struct CBaseAnimating {
    base: CBaseEntity,
    m_bCanUseFastPathFromServer: bool, // 0x9e0 ( Size = 1 )
    gap_9e1: [c_char; 3],
    m_flGroundSpeed: f32,                  // 0x9e4 ( Size = 4 )
    m_flLastEventCheck: f32,               // 0x9e8 ( Size = 4 )
    m_nForceBone: i32,                     // 0x9ec ( Size = 4 )
    m_vecForce: Vector3,                   // 0x9f0 ( Size = 12 )
    m_nSkin: i32,                          // 0x9fc ( Size = 4 )
    m_nBody: i32,                          // 0xa00 ( Size = 4 )
    m_camoIndex: i32,                      // 0xa04 ( Size = 4 )
    m_decalIndex: i32,                     // 0xa08 ( Size = 4 )
    m_nHitboxSet: i32,                     // 0xa0c ( Size = 4 )
    m_flModelScale: f32,                   // 0xa10 ( Size = 4 )
    m_nRagdollImpactFXTableId: i32,        // 0xa14 ( Size = 4 )
    m_flSkyScaleStartTime: f32,            // 0xa18 ( Size = 4 )
    m_flSkyScaleEndTime: f32,              // 0xa1c ( Size = 4 )
    m_flSkyScaleStartValue: f32,           // 0xa20 ( Size = 4 )
    m_flSkyScaleEndValue: f32,             // 0xa24 ( Size = 4 )
    m_SequenceTransitioner: [c_char; 376], // 0xa28 ( Size = 376 ) // void
    m_flIKGroundContactTime: f32,          // 0xba0 ( Size = 4 )
    m_flIKGroundMinHeight: f32,            // 0xba4 ( Size = 4 )
    m_flIKGroundMaxHeight: f32,            // 0xba8 ( Size = 4 )
    m_flEstIkFloor: f32,                   // 0xbac ( Size = 4 )
    m_flEstIkOffset: f32,                  // 0xbb0 ( Size = 4 )
    gap_bb4: [c_char; 4],
    m_pIk: *mut c_void,        // 0xbb8 ( Size = 8 )
    m_ikPrevSequence: i32,     // 0xbc0 ( Size = 4 )
    m_bSequenceFinished: bool, // 0xbc4 ( Size = 1 )
    m_bSequenceLooped: bool,   // 0xbc5 ( Size = 1 )
    m_bSequenceLoops: bool,    // 0xbc6 ( Size = 1 )
    gap_bc7: [c_char; 1],
    m_numSequenceLoops: i32,               // 0xbc8 ( Size = 4 )
    m_continueAnimatingAfterRagdoll: bool, // 0xbcc ( Size = 1 )
    m_useLockedAnimDeltaYaw: bool,         // 0xbcd ( Size = 1 )
    gap_bce: [c_char; 2],
    m_lockedAnimDeltaYaw: f32, // 0xbd0 ( Size = 4 )
    m_threadedBoneSetup: bool, // 0xbd4 ( Size = 1 )
    m_settingUpBones: bool,    // 0xbd5 ( Size = 1 )
    gap_bd6: [c_char; 2],
    m_flDissolveStartTime: f32,          // 0xbd8 ( Size = 4 )
    m_baseAnimatingActivity: i32,        // 0xbdc ( Size = 4 )
    m_flPoseParameter: [f32; 11],        // 0xbe0 ( Size = 44 )
    m_poseParameterOverTimeActive: bool, // 0xc0c ( Size = 1 )
    gap_c0d: [c_char; 3],
    m_poseParameterGoalValue: [f32; 11],    // 0xc10 ( Size = 44 )
    m_poseParameterEndTime: [f32; 11],      // 0xc3c ( Size = 44 )
    m_lastTimeSetPoseParametersSameAs: f32, // 0xc68 ( Size = 4 )
    m_bClientSideAnimation: bool,           // 0xc6c ( Size = 1 )
    m_bReallyClientSideAnimation: bool,     // 0xc6d ( Size = 1 )
    gap_c6e: [c_char; 2],
    m_nNewSequenceParity: i32, // 0xc70 ( Size = 4 )
    m_nResetEventsParity: i32, // 0xc74 ( Size = 4 )
    m_boneCacheHandle: i64,    // 0xc78 ( Size = 8 )
    m_fBoneCacheFlags: i16,    // 0xc80 ( Size = 2 )
    gap_c82: [c_char; 2],
    m_animNetworkFlags: i32,              // 0xc84 ( Size = 4 )
    m_animActive: bool,                   // 0xc88 ( Size = 1 )
    m_animCollisionEnabled: bool,         // 0xc89 ( Size = 1 )
    m_animInitialCorrection: bool,        // 0xc8a ( Size = 1 )
    m_animWaitingForCleanup: bool,        // 0xc8b ( Size = 1 )
    m_animWaitingForCleanupTime: i32,     // 0xc8c ( Size = 4 )
    m_recordedAnim: i64,                  // 0xc90 ( Size = 8 )
    m_recordedAnimIndex: i32,             // 0xc98 ( Size = 4 )
    m_recordedAnimCachedFrameIndex: i32,  // 0xc9c ( Size = 4 )
    m_recordedAnimPlaybackRate: f32,      // 0xca0 ( Size = 4 )
    m_recordedAnimPlaybackTime: f32,      // 0xca4 ( Size = 4 )
    m_recordedAnimTransform: matrix3x4_t, // 0xca8 ( Size = 48 )
    m_recordedAnimPlaybackEnt: EHandle,   // 0xcd8 ( Size = 4 )
    m_recordedAnimBlendTime: f32,         // 0xcdc ( Size = 4 )
    m_recordedAnimBlendOffset: Vector3,   // 0xce0 ( Size = 12 )
    m_recordedAnimBlendAngles: Vector3,   // 0xcec ( Size = 12 )
    m_animRelativeData: AnimRelativeData, // 0xcf8 ( Size = 128 )
    m_syncingWithEntity: EHandle,         // 0xd78 ( Size = 4 )
    gap_d7c: [c_char; 4],
    m_predictedAnimEventData: PredictedAnimEventData, // 0xd80 ( Size = 96 )
    m_animRefEntityAttachmentIndex: i32,              // 0xde0 ( Size = 4 )
    m_fireAttachmentSmartAmmoIndex: i32,              // 0xde4 ( Size = 4 )
    m_fireAttachmentChestFocusIndex: i32,             // 0xde8 ( Size = 4 )
    m_fireAttachmentModelIndex: i32,                  // 0xdec ( Size = 4 )
    m_keyHitboxes: [c_char; 160],                     // 0xdf0 ( Size = 160 ) // void
    m_pStudioHdr: *mut c_void,                        // 0xe90 ( Size = 8 )
    m_animSequence: i32,                              // 0xe98 ( Size = 4 )
    m_animCycle: f32,                                 // 0xe9c ( Size = 4 )
    m_animModelIndex: i32,                            // 0xea0 ( Size = 4 )
    m_animStartTime: f32,                             // 0xea4 ( Size = 4 )
    m_animStartCycle: f32,                            // 0xea8 ( Size = 4 )
    m_animPlaybackRate: f32,                          // 0xeac ( Size = 4 )
    m_animFrozen: bool,                               // 0xeb0 ( Size = 1 )
}
size_assert!(SIZE_BASE_ANIMATING where CBaseAnimating == 0xeb8);

impl DerefMut for CBaseAnimating {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CBaseAnimating {
    type Target = CBaseEntity;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
