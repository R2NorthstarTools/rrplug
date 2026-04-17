#![allow(non_camel_case_types, non_snake_case)]
use std::{
    ffi::{c_char, c_void},
    ops::{Deref, DerefMut},
};

use crate::{
    bindings::{class_types::cbasecombatcharacter::CBaseCombatCharacter, cvar::convar::Color},
    high::vector::{matrix3x4_t, Quaternion, Vector3},
    size_assert,
};

pub type EHandle = i32;
type CBaseEntity = *mut c_void;

#[repr(C)]
pub struct fogparams_t {
    pub gap: [c_char; 8],
    pub botAlt: f32,        // 0x8 ( Size = 4 )
    pub topAlt: f32,        // 0xc ( Size = 4 )
    pub halfDistBot: f32,   // 0x10 ( Size = 4 )
    pub halfDistTop: f32,   // 0x14 ( Size = 4 )
    pub distColorStr: f32,  // 0x18 ( Size = 4 )
    pub dirColorStr: f32,   // 0x1c ( Size = 4 )
    pub distOffset: f32,    // 0x20 ( Size = 4 )
    pub densityScale: f32,  // 0x24 ( Size = 4 )
    pub halfAngleDeg: f32,  // 0x28 ( Size = 4 )
    pub HDRColorScale: f32, // 0x2c ( Size = 4 )
    pub distColor: Color,   // 0x30 ( Size = 4 )
    pub dirColor: Color,    // 0x34 ( Size = 4 )
    pub direction: Vector3, // 0x38 ( Size = 12 )
    pub minFadeTime: f32,   // 0x44 ( Size = 4 )
    pub forceOntoSky: bool, // 0x48 ( Size = 1 )
    pub enable: bool,       // 0x49 ( Size = 1 )
    pub gap_4a: [c_char; 2],
    pub id: i32, // 0x4c ( Size = 4 )
}
size_assert!(FOG_SIZE where fogparams_t == 80);

#[repr(C)]
pub struct sky3dparams_t {
    pub gap: [c_char; 8],
    pub scale: i32,        // 0x8 ( Size = 4 )
    pub cellNum: i32,      // 0xc ( Size = 4 )
    pub useWorldFog: bool, // 0x10 ( Size = 1 )
    pub gap_11: [c_char; 7],
    pub fog: fogparams_t, // 0x18 ( Size = 80 )
}
size_assert!(SKY_SIZE where sky3dparams_t == 104);

#[repr(C)]
pub struct fogplayerparamsstate_t {
    pub gap: [c_char; 8],
    pub enable: bool, // 0x8 ( Size = 1 )
    pub gap_9: [c_char; 3],
    pub botAlt: f32,        // 0xc ( Size = 4 )
    pub topAlt: f32,        // 0x10 ( Size = 4 )
    pub halfDistBot: f32,   // 0x14 ( Size = 4 )
    pub halfDistTop: f32,   // 0x18 ( Size = 4 )
    pub distOffset: f32,    // 0x1c ( Size = 4 )
    pub densityScale: f32,  // 0x20 ( Size = 4 )
    pub halfAngleDeg: f32,  // 0x24 ( Size = 4 )
    pub distColorStr: f32,  // 0x28 ( Size = 4 )
    pub dirColorStr: f32,   // 0x2c ( Size = 4 )
    pub HDRColorScale: f32, // 0x30 ( Size = 4 )
    pub minFadeTime: f32,   // 0x34 ( Size = 4 )
    pub forceOntoSky: bool, // 0x38 ( Size = 1 )
    pub distColor: Color,   // 0x39 ( Size = 4 )
    pub dirColor: Color,    // 0x3d ( Size = 4 )
    pub direction: Vector3, // 0x44 ( Size = 12 )
    pub id: i32,            // 0x50 ( Size = 4 )
    pub gap_54: [c_char; 4],
}
size_assert!(FOG_SIZE_PLAYER where fogplayerparamsstate_t == 88);

#[repr(C)]
pub struct fogplayerparams_t {
    pub gap: [c_char; 8],
    pub m_hCtrl: EHandle,                 // 0x8 ( Size = 4 )
    pub m_flTransitionStartTime: f32,     // 0xc ( Size = 4 )
    pub m_Old: fogplayerparamsstate_t,    // 0x10 ( Size = 88 )
    pub m_New: fogplayerparamsstate_t,    // 0x68 ( Size = 88 )
    pub m_OldSky: fogplayerparamsstate_t, // 0xc0 ( Size = 88 )
    pub m_NewSky: fogplayerparamsstate_t, // 0x118 ( Size = 88 )
}
size_assert!(FOG_PARAMS_SIZE where fogplayerparams_t == 368);

#[repr(C)]
pub struct audioparams_t {
    pub gap_0: [c_char; 8],
    pub localSound: [Vector3; 8], // 0x8 ( Size = 96 )
    pub soundscapeIndex: i32,     // 0x68 ( Size = 4 )
    pub localBits: i32,           // 0x6c ( Size = 4 )
    pub entIndex: i32,            // 0x70 ( Size = 4 )
    pub gap_74: [c_char; 4],
}
size_assert!(AUDIO_SIZE where audioparams_t == 120);

#[repr(C)]
pub struct CPlayerState {
    pub vftable: *const c_void,
    pub currentClass: i32,   // 0x8 ( Size = 4 )
    pub requestedClass: i32, // 0xc ( Size = 4 )
    pub onDeathClass: i32,   // 0x10 ( Size = 4 )
    pub oldClass: i32,       // 0x14 ( Size = 4 )
    pub a_angle: Vector3,    // 0x18 ( Size = 12 )
    pub gap_24: [c_char; 4],
    pub netname: u64,         // 0x28 ( Size = 8 )
    pub fixangle: i32,        // 0x30 ( Size = 4 )
    pub anglechange: Vector3, // 0x34 ( Size = 12 )
    pub index: i32,           // 0x40 ( Size = 4 )
    pub replay: bool,         // 0x44 ( Size = 1 )
    pub gap_45: [c_char; 3],
    pub lastPlayerView_tickcount: i32,  // 0x48 ( Size = 4 )
    pub lastPlayerView_origin: Vector3, // 0x4c ( Size = 12 )
    pub lastPlayerView_angle: Vector3,  // 0x58 ( Size = 12 )
    pub deadflag: bool,                 // 0x64 ( Size = 1 )
    pub gap_65: [c_char; 3],
    pub localViewAngles: Vector3, // 0x68 ( Size = 12 )
    pub worldViewAngles: Vector3, // 0x74 ( Size = 12 )
}
size_assert!(PLAYER_STATE_SIZE where CPlayerState == 128);

#[repr(C)]
pub struct Rodeo_PlayerData {
    pub vftable: *const c_void,
    pub stage: i32,     // 0x8 ( Size = 4 )
    pub canRodeo: bool, // 0xc ( Size = 1 )
    pub gap_d: [c_char; 3],
    pub rodeoCountParity: i32,              // 0x10 ( Size = 4 )
    pub startTime: f32,                     // 0x14 ( Size = 4 )
    pub endTime: f32,                       // 0x18 ( Size = 4 )
    pub targetEnt: EHandle,                 // 0x1c ( Size = 4 )
    pub prevEnt: EHandle,                   // 0x20 ( Size = 4 )
    pub prevEntCooldown: f32,               // 0x24 ( Size = 4 )
    pub pilot1pSequenceIndex: i32,          // 0x28 ( Size = 4 )
    pub pilot3pSequenceIndex: i32,          // 0x2c ( Size = 4 )
    pub targetAttachmentIndex: i32,         // 0x30 ( Size = 4 )
    pub rodeoStabilizeStrength: f32,        // 0x34 ( Size = 4 )
    pub rodeoStabilizeViewFirstFrame: bool, // 0x38 ( Size = 1 )
    pub gap_39: [c_char; 3],
    pub lastPlayerToWorld: matrix3x4_t,      // 0x3c ( Size = 48 )
    pub initialCameraCorrection: Quaternion, // 0x6c ( Size = 16 )
}
size_assert!(SIZE_RODEO where Rodeo_PlayerData == 128);

#[repr(C)]
pub struct ClassModValues {
    pub health: f32,                      // 0x0 ( Size = 4 )
    pub healthShield: f32,                // 0x4 ( Size = 4 )
    pub healthDoomed: f32,                // 0x8 ( Size = 4 )
    pub healthPerSegment: f32,            // 0xc ( Size = 4 )
    pub powerRegenRate: f32,              // 0x10 ( Size = 4 )
    pub dodgeDuration: f32,               // 0x14 ( Size = 4 )
    pub dodgeSpeed: f32,                  // 0x18 ( Size = 4 )
    pub dodgePowerDrain: f32,             // 0x1c ( Size = 4 )
    pub smartAmmoLockTimeModifier: f32,   // 0x20 ( Size = 4 )
    pub wallrunAccelerateVertical: f32,   // 0x24 ( Size = 4 )
    pub wallrunAccelerateHorizontal: f32, // 0x28 ( Size = 4 )
    pub wallrunMaxSpeedHorizontal: f32,   // 0x2c ( Size = 4 )
    pub wallrun_timeLimit: f32,           // 0x30 ( Size = 4 )
    pub wallrun_hangTimeLimit: f32,       // 0x34 ( Size = 4 )
    pub wallrunAllowed: bool,             // 0x38 ( Size = 1 )
    pub gap_39: [c_char; 3],
    pub wallrunAdsType: i32,     // 0x3c ( Size = 4 )
    pub doubleJumpAllowed: bool, // 0x40 ( Size = 1 )
    pub gap_41: [c_char; 3],
    pub pitchMaxUp: f32,           // 0x44 ( Size = 4 )
    pub pitchMaxDown: f32,         // 0x48 ( Size = 4 )
    pub mantlePitchLeveling: bool, // 0x4c ( Size = 1 )
    pub dodgeAllowed: bool,        // 0x4d ( Size = 1 )
    pub sprintAllowed: bool,       // 0x4e ( Size = 1 )
    pub stealthSounds: bool,       // 0x4f ( Size = 1 )
    pub hoverEnabled: bool,        // 0x50 ( Size = 1 )
    pub gap_51: [c_char; 3],
    pub grapple_power_regen_delay: f32, // 0x54 ( Size = 4 )
    pub grapple_power_regen_rate: f32,  // 0x58 ( Size = 4 )
    pub slideFOVScale: f32,             // 0x5c ( Size = 4 )
    pub slideFOVLerpInTime: f32,        // 0x60 ( Size = 4 )
    pub slideFOVLerpOutTime: f32,       // 0x64 ( Size = 4 )
    pub airSlowMoSpeed: f32,            // 0x68 ( Size = 4 )
    pub sharedEnergyTotal: i32,         // 0x6c ( Size = 4 )
    pub sharedEnergyRegenRate: f32,     // 0x70 ( Size = 4 )
}
size_assert!(SIZE_CLASS_MOD where ClassModValues == 116);

#[repr(C)]
pub struct PerPosClassModValues {
    pub speed_: f32,       // 0x0 ( Size = 4 )
    pub sprintSpeed_: f32, // 0x4 ( Size = 4 )
}
size_assert!(SIZE_PER_CLASS where PerPosClassModValues == 8);

#[repr(C)]
pub struct ThirdPersonViewData {
    pub vftable: *const c_void,
    pub m_thirdPersonEntViewOffset: Vector3, // 0x8 ( Size = 12 )
    pub m_thirdPersonEntPitchIsFreelook: bool, // 0x14 ( Size = 1 )
    pub m_thirdPersonEntYawIsFreelook: bool, // 0x15 ( Size = 1 )
    pub m_thirdPersonEntUseFixedDist: bool,  // 0x16 ( Size = 1 )
    pub m_thirdPersonEntPushedInByGeo: bool, // 0x17 ( Size = 1 )
    pub m_thirdPersonEntDrawViewmodel: bool, // 0x18 ( Size = 1 )
    pub gap_19: [c_char; 3],
    pub m_thirdPersonEntBlendTotalDuration: f32, // 0x1c ( Size = 4 )
    pub m_thirdPersonEntBlendEaseInDuration: f32, // 0x20 ( Size = 4 )
    pub m_thirdPersonEntBlendEaseOutDuration: f32, // 0x24 ( Size = 4 )
    pub m_thirdPersonEntFixedPitch: f32,         // 0x28 ( Size = 4 )
    pub m_thirdPersonEntFixedYaw: f32,           // 0x2c ( Size = 4 )
    pub m_thirdPersonEntFixedDist: f32,          // 0x30 ( Size = 4 )
    pub m_thirdPersonEntMinYaw: f32,             // 0x34 ( Size = 4 )
    pub m_thirdPersonEntMaxYaw: f32,             // 0x38 ( Size = 4 )
    pub m_thirdPersonEntMinPitch: f32,           // 0x3c ( Size = 4 )
    pub m_thirdPersonEntMaxPitch: f32,           // 0x40 ( Size = 4 )
    pub m_thirdPersonEntSpringToCenterRate: f32, // 0x44 ( Size = 4 )
    pub m_thirdPersonEntLookaheadLowerEntSpeed: f32, // 0x48 ( Size = 4 )
    pub m_thirdPersonEntLookaheadUpperEntSpeed: f32, // 0x4c ( Size = 4 )
    pub m_thirdPersonEntLookaheadMaxAngle: f32,  // 0x50 ( Size = 4 )
    pub m_thirdPersonEntLookaheadLerpAheadRate: f32, // 0x54 ( Size = 4 )
    pub m_thirdPersonEntLookaheadLerpToCenterRate: f32, // 0x58 ( Size = 4 )
}
size_assert!(SIZE_THRID where ThirdPersonViewData == 96);

#[repr(C)]
pub struct GrappleData {
    pub vftable: *const c_void,
    pub m_grappleVel: Vector3,           // 0x8 ( Size = 12 )
    pub m_grapplePoints: [Vector3; 4],   // 0x14 ( Size = 48 )
    pub m_grapplePointCount: i32,        // 0x44 ( Size = 4 )
    pub m_grappleAttached: bool,         // 0x48 ( Size = 1 )
    pub m_grapplePulling: bool,          // 0x49 ( Size = 1 )
    pub m_grappleRetracting: bool,       // 0x4a ( Size = 1 )
    pub m_grappleForcedRetracting: bool, // 0x4b ( Size = 1 )
    pub m_grappleUsedPower: f32,         // 0x4c ( Size = 4 )
    pub m_grapplePullTime: f32,          // 0x50 ( Size = 4 )
    pub m_grappleAttachTime: f32,        // 0x54 ( Size = 4 )
    pub m_grappleDetachTime: f32,        // 0x58 ( Size = 4 )
    pub m_grappleMeleeTarget: EHandle,   // 0x5c ( Size = 4 )
    pub m_grappleHasGoodVelocity: bool,  // 0x60 ( Size = 1 )
    pub gap_61: [c_char; 3],
    pub m_grappleLastGoodVelocityTime: f32, // 0x64 ( Size = 4 )
}
size_assert!(SIZE_GRAPPLE where GrappleData == 104);

#[repr(C)]
pub struct PlayerZiplineData {
    pub vftable: *const c_void,
    pub m_ziplineReenableWeapons: bool, // 0x8 ( Size = 1 )
    pub gap_9: [c_char; 3],
    pub m_mountingZiplineDuration: f32, // 0xc ( Size = 4 )
    pub m_mountingZiplineAlpha: f32,    // 0x10 ( Size = 4 )
    pub m_ziplineStartTime: f32,        // 0x14 ( Size = 4 )
    pub m_ziplineEndTime: f32,          // 0x18 ( Size = 4 )
    pub m_mountingZiplineSourcePosition: Vector3, // 0x1c ( Size = 12 )
    pub m_mountingZiplineSourceVelocity: Vector3, // 0x28 ( Size = 12 )
    pub m_mountingZiplineTargetPosition: Vector3, // 0x34 ( Size = 12 )
    pub m_ziplineUsePosition: Vector3,  // 0x40 ( Size = 12 )
}
size_assert!(SIZE_PLAYER_ZIP where PlayerZiplineData == 80);

#[repr(C)]
pub struct Player_OperatorData {
    pub vftable: *const c_void,
    pub diving: bool,        // 0x8 ( Size = 1 )
    pub cameraEnabled: bool, // 0x9 ( Size = 1 )
    pub gap_a: [c_char; 2],
    pub minPitch: f32,         // 0xc ( Size = 4 )
    pub maxPitch: f32,         // 0x10 ( Size = 4 )
    pub followDistance: f32,   // 0x14 ( Size = 4 )
    pub followHeight: f32,     // 0x18 ( Size = 4 )
    pub shootFromPlayer: bool, // 0x1c ( Size = 1 )
    pub gap_1d: [c_char; 3],
    pub smoothDuration: f32,                 // 0x20 ( Size = 4 )
    pub smoothFollowDistanceStartTime: f32,  // 0x24 ( Size = 4 )
    pub smoothFollowDistanceStartValue: f32, // 0x28 ( Size = 4 )
    pub smoothFollowDistanceEndValue: f32,   // 0x2c ( Size = 4 )
    pub smoothFollowHeightStartTime: f32,    // 0x30 ( Size = 4 )
    pub smoothFollowHeightStartValue: f32,   // 0x34 ( Size = 4 )
    pub smoothFollowHeightEndValue: f32,     // 0x38 ( Size = 4 )
    pub smoothMinPitchStartTime: f32,        // 0x3c ( Size = 4 )
    pub smoothMinPitchStartValue: f32,       // 0x40 ( Size = 4 )
    pub smoothMinPitchEndValue: f32,         // 0x44 ( Size = 4 )
    pub smoothMaxPitchStartTime: f32,        // 0x48 ( Size = 4 )
    pub smoothMaxPitchStartValue: f32,       // 0x4c ( Size = 4 )
    pub smoothMaxPitchEndValue: f32,         // 0x50 ( Size = 4 )
    pub forceDefaultFloorHeight: bool,       // 0x54 ( Size = 1 )
    pub gap_55: [c_char; 3],
    pub defaultFloorHeight: f32,        // 0x58 ( Size = 4 )
    pub ignoreWorldForMovement: bool,   // 0x5c ( Size = 1 )
    pub ignoreWorldForFloorTrace: bool, // 0x5d ( Size = 1 )
    pub gap_5e: [c_char; 2],
    pub moveGridSizeScale: f32,     // 0x60 ( Size = 4 )
    pub moveFloorHeightOffset: f32, // 0x64 ( Size = 4 )
    pub jumpIsDodge: bool,          // 0x68 ( Size = 1 )
    pub gap_69: [c_char; 3],
    pub jumpAcceleration: f32,  // 0x6c ( Size = 4 )
    pub jumpMaxSpeed: f32,      // 0x70 ( Size = 4 )
    pub hoverAcceleration: f32, // 0x74 ( Size = 4 )
    pub hoverMaxSpeed: f32,     // 0x78 ( Size = 4 )
}
size_assert!(SIZE_PLAYER_OPERATOR where Player_OperatorData == 128);

#[repr(C)]
pub struct Player_ViewOffsetEntityData {
    pub vftable: *const c_void,
    pub viewOffsetEntityHandle: EHandle, // 0x8 ( Size = 4 )
    pub lerpInDuration: f32,             // 0xc ( Size = 4 )
    pub lerpOutDuration: f32,            // 0x10 ( Size = 4 )
    pub stabilizePlayerEyeAngles: bool,  // 0x14 ( Size = 1 )
}
size_assert!(PLAYER_VIEW_SIZE where Player_ViewOffsetEntityData == 24);

#[repr(C)]
pub struct Player_AnimViewEntityData {
    pub vftable: *const c_void,
    pub animViewEntityHandle: EHandle,           // 0x8 ( Size = 4 )
    pub animViewEntityAngleLerpInDuration: f32,  // 0xc ( Size = 4 )
    pub animViewEntityOriginLerpInDuration: f32, // 0x10 ( Size = 4 )
    pub animViewEntityLerpOutDuration: f32,      // 0x14 ( Size = 4 )
    pub animViewEntityStabilizePlayerEyeAngles: bool, // 0x18 ( Size = 1 )
    pub gap_19: [c_char; 3],
    pub animViewEntityThirdPersonCameraParity: i32, // 0x1c ( Size = 4 )
    pub animViewEntityThirdPersonCameraAttachment: [i32; 6], // 0x20 ( Size = 24 )
    pub animViewEntityNumThirdPersonCameraAttachments: i32, // 0x38 ( Size = 4 )
    pub animViewEntityThirdPersonCameraVisibilityChecks: bool, // 0x3c ( Size = 1 )
    pub gap_3d: [c_char; 3],
    pub animViewEntityParity: i32,             // 0x40 ( Size = 4 )
    pub lastAnimViewEntityParity: i32,         // 0x44 ( Size = 4 )
    pub animViewEntityCameraPosition: Vector3, // 0x48 ( Size = 12 )
    pub animViewEntityCameraAngles: Vector3,   // 0x54 ( Size = 12 )
    pub animViewEntityBlendStartTime: f32,     // 0x60 ( Size = 4 )
    pub animViewEntityBlendStartEyePosition: Vector3, // 0x64 ( Size = 12 )
    pub animViewEntityBlendStartEyeAngles: Vector3, // 0x70 ( Size = 12 )
}
size_assert!(SIZE_PLAYER_ANIM where Player_AnimViewEntityData == 128);

#[repr(C)]
pub struct CurrentData_Player {
    pub vftable: *const c_void,
    pub m_flHullHeight: f32,          // 0x8 ( Size = 4 )
    pub m_traversalAnimProgress: f32, // 0xc ( Size = 4 )
    pub m_sprintTiltFrac: f32,        // 0x10 ( Size = 4 )
    pub m_angEyeAngles: Vector3,      // 0x14 ( Size = 12 )
}
size_assert!(SIZE_CURRENT where CurrentData_Player == 32);

#[repr(C)]
pub struct CurrentData_LocalPlayer {
    pub vftable: *const c_void,
    pub m_viewConeAngleMin: Vector3,        // 0x8 ( Size = 12 )
    pub m_viewConeAngleMax: Vector3,        // 0x14 ( Size = 12 )
    pub m_stepSmoothingOffset: Vector3,     // 0x20 ( Size = 12 )
    pub m_vecPunchBase_Angle: Vector3,      // 0x2c ( Size = 12 )
    pub m_vecPunchBase_AngleVel: Vector3,   // 0x38 ( Size = 12 )
    pub m_vecPunchWeapon_Angle: Vector3,    // 0x44 ( Size = 12 )
    pub m_vecPunchWeapon_AngleVel: Vector3, // 0x50 ( Size = 12 )
}
size_assert!(SIZE_CURRENT_DATA where CurrentData_LocalPlayer == 96);

#[repr(C)]
pub struct ScriptOriginatedDamageInfo {
    pub m_scriptDamageType: i32,       // 0x0 ( Size = 4 )
    pub m_damageSourceIdentifier: i32, // 0x4 ( Size = 4 )
    pub m_scriptAttackerClass: i32,    // 0x8 ( Size = 4 )
}
size_assert!(SIZE_SCRIPT_ORI where ScriptOriginatedDamageInfo == 12);

#[repr(C)]
pub struct CTakeDamageInfo {
    pub m_vecDamageForce: Vector3,                // 0x0 ( Size = 12 )
    pub m_vecDamagePosition: Vector3,             // 0xc ( Size = 12 )
    pub m_vecReportedPosition: Vector3,           // 0x18 ( Size = 12 )
    pub m_hInflictor: EHandle,                    // 0x24 ( Size = 4 )
    pub m_hAttacker: EHandle,                     // 0x28 ( Size = 4 )
    pub m_hWeapon: EHandle,                       // 0x2c ( Size = 4 )
    pub m_hWeaponFileInfo: std::os::raw::c_short, // 0x30 ( Size = 2 )
    pub m_forceKill: bool,                        // 0x32 ( Size = 1 )
    pub gap_33: [c_char; 1],
    pub m_flDamage: f32,                                // 0x34 ( Size = 4 )
    pub m_damageCriticalScale: f32,                     // 0x38 ( Size = 4 )
    pub m_flMaxDamage: f32,                             // 0x3c ( Size = 4 )
    pub m_flHeavyArmorDamageScale: f32,                 // 0x40 ( Size = 4 )
    pub m_bitsDamageType: i32,                          // 0x44 ( Size = 4 )
    pub m_flRadius: f32,                                // 0x48 ( Size = 4 )
    pub m_hitGroup: i32,                                // 0x4c ( Size = 4 )
    pub m_hitBox: i32,                                  // 0x50 ( Size = 4 )
    pub m_scriptDamageInfo: ScriptOriginatedDamageInfo, // 0x54 ( Size = 12 )
    pub m_deathPackage: i32,                            // 0x60 ( Size = 4 )
    pub m_distanceFromAttackOrigin: f32,                // 0x64 ( Size = 4 )
    pub m_distanceFromExplosionCenter: f32,             // 0x68 ( Size = 4 )
    pub m_doDeathForce: bool,                           // 0x6c ( Size = 1 )
    pub gap_6d: [c_char; 3],
    pub m_damageFlags: i32,     // 0x70 ( Size = 4 )
    pub m_flinchDirection: i32, // 0x74 ( Size = 4 )
}
size_assert!(SIZE_DAMAGE where CTakeDamageInfo == 120);

#[repr(C)]
pub struct PlayerMelee_PlayerData {
    pub vftable: *const c_void,
    pub attackActive: bool,                // 0x8 ( Size = 1 )
    pub attackRecoveryShouldBeQuick: bool, // 0x9 ( Size = 1 )
    pub gap_a: [c_char; 2],
    pub attackStartTime: f32,             // 0xc ( Size = 4 )
    pub attackHitEntity: EHandle,         // 0x10 ( Size = 4 )
    pub attackHitEntityTime: f32,         // 0x14 ( Size = 4 )
    pub attackLastHitNonWorldEntity: f32, // 0x18 ( Size = 4 )
    pub scriptedState: i32,               // 0x1c ( Size = 4 )
    pub pendingMeleePress: bool,          // 0x20 ( Size = 1 )
}
size_assert!(SIZE_MELEE where PlayerMelee_PlayerData == 40);

#[repr(C)]
pub struct CPlayerShared {
    pub vftable: *const c_void,
    pub m_nPlayerCond: i32,          // 0x8 ( Size = 4 )
    pub m_bLoadoutUnavailable: bool, // 0xc ( Size = 1 )
    pub gap_d: [c_char; 3],
    pub m_flCondExpireTimeLeft: [f32; 2], // 0x10 ( Size = 8 )
    pub m_pOuter: *mut c_void,            // 0x18 ( Size = 8 )
    pub m_flNextCritUpdate: f32,          // 0x20 ( Size = 4 )
    pub m_flTauntRemoveTime: f32,         // 0x24 ( Size = 4 )
    pub m_damageInfo: CTakeDamageInfo,    // 0x28 ( Size = 120 )
}
size_assert!(SIZE_SHARED where CPlayerShared == 160);

#[repr(C)]
pub struct StatusEffectTimedData {
    pub vftable: *const c_void,
    pub seComboVars: i32, // 0x8 ( Size = 4 )
    pub seTimeEnd: f32,   // 0xc ( Size = 4 )
    pub seEaseOut: f32,   // 0x10 ( Size = 4 )
}
size_assert!(SIZE_STATUS_EFFECT where StatusEffectTimedData == 24);

#[repr(C)]
pub struct StatusEffectEndlessData {
    pub vftable: *const c_void,
    pub seComboVars: i32, // 0x8 ( Size = 4 )
}
size_assert!(SIZE_ENDLESS where StatusEffectEndlessData == 16);

#[repr(C)]
pub struct PushHistoryEntry {
    pub time: f32,       // 0x0 ( Size = 4 )
    pub pushed: Vector3, // 0x4 ( Size = 12 )
}
size_assert!(SIZE_PUSH_HISTORY where PushHistoryEntry == 16);

#[repr(C)]
pub struct PredictableServerEvent {
    pub r#type: i32,               // 0x0 ( Size = 4 )
    pub deadlineTime: f32,         // 0x4 ( Size = 4 )
    pub fullSizeOfUnion: [i32; 4], // 0x8 ( Size = 16 )
}
size_assert!(SIZE_PREDICTABLE_EVENT where PredictableServerEvent == 24);

#[repr(C)]
pub struct CPlayerLocalData {
    pub vftable: *const c_void,
    pub m_iHideHUD: i32,             // 0x8 ( Size = 4 )
    pub m_vecOverViewpoint: Vector3, // 0xc ( Size = 12 )
    pub m_duckToggleOn: bool,        // 0x18 ( Size = 1 )
    pub gap_19: [c_char; 3],
    pub m_forceStance: i32,              // 0x1c ( Size = 4 )
    pub m_nDuckTransitionTimeMsecs: i32, // 0x20 ( Size = 4 )
    pub m_superJumpsUsed: i32,           // 0x24 ( Size = 4 )
    pub m_jumpedOffRodeo: bool,          // 0x28 ( Size = 1 )
    pub gap_29: [c_char; 3],
    pub m_flSuitPower: f32,                // 0x2c ( Size = 4 )
    pub m_flSuitJumpPower: f32,            // 0x30 ( Size = 4 )
    pub m_flSuitGrapplePower: f32,         // 0x34 ( Size = 4 )
    pub m_nStepside: i32,                  // 0x38 ( Size = 4 )
    pub m_flFallVelocity: f32,             // 0x3c ( Size = 4 )
    pub m_nOldButtons: i32,                // 0x40 ( Size = 4 )
    pub m_oldForwardMove: f32,             // 0x44 ( Size = 4 )
    pub m_pOldSkyCamera: *mut CBaseEntity, // 0x48 ( Size = 8 )
    pub m_accelScale: f32,                 // 0x50 ( Size = 4 )
    pub m_powerRegenRateScale: f32,        // 0x54 ( Size = 4 )
    pub m_dodgePowerDelayScale: f32,       // 0x58 ( Size = 4 )
    pub m_bDrawViewmodel: bool,            // 0x5c ( Size = 1 )
    pub gap_5d: [c_char; 3],
    pub m_flStepSize: f32,          // 0x60 ( Size = 4 )
    pub m_bAllowAutoMovement: bool, // 0x64 ( Size = 1 )
    pub gap_65: [c_char; 3],
    pub m_airSlowMoFrac: f32,               // 0x68 ( Size = 4 )
    pub predictableFlags: i32,              // 0x6c ( Size = 4 )
    pub m_bitsActiveDevices: i32,           // 0x70 ( Size = 4 )
    pub m_hSkyCamera: EHandle,              // 0x74 ( Size = 4 )
    pub m_skybox3d: sky3dparams_t,          // 0x78 ( Size = 104 )
    pub m_PlayerFog: fogplayerparams_t,     // 0xe0 ( Size = 368 )
    pub m_fog: fogparams_t,                 // 0x250 ( Size = 80 )
    pub m_audio: audioparams_t,             // 0x2a0 ( Size = 120 )
    pub m_animNearZ: f32,                   // 0x318 ( Size = 4 )
    pub m_airMoveBlockPlanes: [Vector3; 2], // 0x31c ( Size = 24 )
    pub m_airMoveBlockPlaneTime: f32,       // 0x334 ( Size = 4 )
    pub m_airMoveBlockPlaneCount: i32,      // 0x338 ( Size = 4 )
    pub m_queuedMeleePressTime: f32,        // 0x33c ( Size = 4 )
    pub m_queuedGrappleMeleeTime: f32,      // 0x340 ( Size = 4 )
    pub m_queuedMeleeAttackAnimEvent: bool, // 0x344 ( Size = 1 )
    pub m_disableMeleeUntilRelease: bool,   // 0x345 ( Size = 1 )
    pub gap_346: [c_char; 2],
    pub m_meleePressTime: f32,              // 0x348 ( Size = 4 )
    pub m_meleeDisabledCounter: i32,        // 0x34c ( Size = 4 )
    pub lastAttacker: EHandle,              // 0x350 ( Size = 4 )
    pub attackedCount: i32,                 // 0x354 ( Size = 4 )
    pub m_trackedChildProjectileCount: i32, // 0x358 ( Size = 4 )
    pub m_oneHandedWeaponUsage: bool,       // 0x35c ( Size = 1 )
    pub m_prevOneHandedWeaponUsage: bool,   // 0x35d ( Size = 1 )
    pub gap_35e: [c_char; 2],
    pub m_flCockpitEntryTime: f32,  // 0x360 ( Size = 4 )
    pub m_ejectStartTime: f32,      // 0x364 ( Size = 4 )
    pub m_disembarkStartTime: f32,  // 0x368 ( Size = 4 )
    pub m_hotDropImpactTime: f32,   // 0x36c ( Size = 4 )
    pub m_outOfBoundsDeadTime: f32, // 0x370 ( Size = 4 )
    pub m_objectiveIndex: i32,      // 0x374 ( Size = 4 )
    pub m_objectiveEntity: EHandle, // 0x378 ( Size = 4 )
    pub m_objectiveEndTime: f32,    // 0x37c ( Size = 4 )
    pub m_cinematicEventFlags: i32, // 0x380 ( Size = 4 )
    pub m_forcedDialogueOnly: bool, // 0x384 ( Size = 1 )
    pub gap_385: [c_char; 3],
    pub m_titanBuildTime: f32,         // 0x388 ( Size = 4 )
    pub m_titanBubbleShieldTime: f32,  // 0x38c ( Size = 4 )
    pub m_titanEmbarkEnabled: bool,    // 0x390 ( Size = 1 )
    pub m_titanDisembarkEnabled: bool, // 0x391 ( Size = 1 )
    pub gap_392: [c_char; 2],
    pub m_voicePackIndex: i32,            // 0x394 ( Size = 4 )
    pub m_playerAnimUpdateTime: f32,      // 0x398 ( Size = 4 )
    pub m_playerAnimLastAimTurnTime: f32, // 0x39c ( Size = 4 )
    pub m_playerAnimCurrentFeetYaw: f32,  // 0x3a0 ( Size = 4 )
    pub m_playerAnimEstimateYaw: f32,     // 0x3a4 ( Size = 4 )
    pub m_playerAnimGoalFeetYaw: f32,     // 0x3a8 ( Size = 4 )
    pub m_playerAnimJumping: bool,        // 0x3ac ( Size = 1 )
    pub gap_3ad: [c_char; 3],
    pub m_playerAnimJumpStartTime: f32,   // 0x3b0 ( Size = 4 )
    pub m_playerAnimFirstJumpFrame: bool, // 0x3b4 ( Size = 1 )
    pub m_playerAnimDodging: bool,        // 0x3b5 ( Size = 1 )
    pub gap_3b6: [c_char; 2],
    pub m_playerLandStartTime: f32,             // 0x3b8 ( Size = 4 )
    pub m_playerAnimJumpActivity: i32,          // 0x3bc ( Size = 4 )
    pub m_playerAnimLastWallRunNormal: Vector3, // 0x3c0 ( Size = 12 )
    pub m_playerAnimLanding: bool,              // 0x3cc ( Size = 1 )
    pub m_playerAnimShouldLand: bool,           // 0x3cd ( Size = 1 )
    pub gap_3ce: [c_char; 2],
    pub m_playerAnimLandStartTime: f32, // 0x3d0 ( Size = 4 )
    pub m_playerAnimInAirWalk: bool,    // 0x3d4 ( Size = 1 )
    pub gap_3d5: [c_char; 3],
    pub m_playerAnimPrevFrameSequenceMotionYaw: f32, // 0x3d8 ( Size = 4 )
    pub m_playerAnimMovementPlaybackRate: f32,       // 0x3dc ( Size = 4 )
    pub m_fake_playerAnimUpdateTime: f32,            // 0x3e0 ( Size = 4 )
    pub m_fake_playerAnimLastAimTurnTime: f32,       // 0x3e4 ( Size = 4 )
    pub m_fake_playerAnimCurrentFeetYaw: f32,        // 0x3e8 ( Size = 4 )
    pub m_fake_playerAnimEstimateYaw: f32,           // 0x3ec ( Size = 4 )
    pub m_fake_playerAnimGoalFeetYaw: f32,           // 0x3f0 ( Size = 4 )
    pub m_fake_playerAnimJumping: bool,              // 0x3f4 ( Size = 1 )
    pub gap_3f5: [c_char; 3],
    pub m_fake_playerAnimJumpStartTime: f32, // 0x3f8 ( Size = 4 )
    pub m_fake_playerAnimFirstJumpFrame: bool, // 0x3fc ( Size = 1 )
    pub m_fake_playerAnimDodging: bool,      // 0x3fd ( Size = 1 )
    pub gap_3fe: [c_char; 2],
    pub m_fake_playerLandStartTime: f32,    // 0x400 ( Size = 4 )
    pub m_fake_playerAnimJumpActivity: i32, // 0x404 ( Size = 4 )
    pub m_fake_playerAnimLastWallRunNormal: Vector3, // 0x408 ( Size = 12 )
    pub m_fake_playerAnimLanding: bool,     // 0x414 ( Size = 1 )
    pub m_fake_playerAnimShouldLand: bool,  // 0x415 ( Size = 1 )
    pub gap_416: [c_char; 2],
    pub m_fake_playerAnimLandStartTime: f32, // 0x418 ( Size = 4 )
    pub m_fake_playerAnimInAirWalk: bool,    // 0x41c ( Size = 1 )
    pub gap_41d: [c_char; 3],
    pub m_fake_playerAnimPrevFrameSequenceMotionYaw: f32, // 0x420 ( Size = 4 )
    pub m_fake_playerAnimMovementPlaybackRate: f32,       // 0x424 ( Size = 4 )
}
size_assert!(SIZE_LOCAL_DATA where CPlayerLocalData == 1064);

#[allow(non_snake_case)]
#[repr(C)]
pub struct CPlayer {
    pub base: CBaseCombatCharacter,
    pub m_szNetname: [c_char; 256], // 0x1498 ( Size = 256 )
    pub m_bZooming: bool,           // 0x1598 ( Size = 1 )
    pub m_zoomToggleOn: bool,       // 0x1599 ( Size = 1 )
    pub gap_159a: [c_char; 2],
    pub m_zoomBaseFrac: f32,                 // 0x159c ( Size = 4 )
    pub m_zoomBaseTime: f32,                 // 0x15a0 ( Size = 4 )
    pub m_zoomFullStartTime: f32,            // 0x15a4 ( Size = 4 )
    pub m_physicsSolidMask: i32,             // 0x15a8 ( Size = 4 )
    pub m_StuckLast: i32,                    // 0x15ac ( Size = 4 )
    pub m_Local: CPlayerLocalData,           // 0x15b0 ( Size = 1064 )
    pub m_PlayerFog: fogplayerparams_t,      // 0x19d8 ( Size = 368 )
    pub m_hTriggerTonemapList: [c_char; 32], // 0x1b48 ( Size = 32 ) // custom
    pub m_hColorCorrectionCtrl: EHandle,     // 0x1b68 ( Size = 4 )
    pub gap_1b6c: [c_char; 4],
    pub m_hTriggerSoundscapeList: [c_char; 32], // 0x1b70 ( Size = 32 ) // custom
    pub pl: CPlayerState,                       // 0x1b90 ( Size = 128 )
    pub m_rodeo: Rodeo_PlayerData,              // 0x1c10 ( Size = 128 )
    pub m_hasBadReputation: bool,               // 0x1c90 ( Size = 1 )
    pub m_communityName: [c_char; 64],          // 0x1c91 ( Size = 64 )
    pub m_communityClanTag: [c_char; 16],       // 0x1cd1 ( Size = 16 )
    pub m_factionName: [c_char; 16],            // 0x1ce1 ( Size = 16 )
    pub m_hardwareIcon: [c_char; 16],           // 0x1cf1 ( Size = 16 )
    pub m_happyHourActive: bool,                // 0x1d01 ( Size = 1 )
    pub gap_1d02: [c_char; 6],
    pub m_platformUserId: u64,                     // 0x1d08 ( Size = 8 )
    pub m_classModsActive: i32,                    // 0x1d10 ( Size = 4 )
    pub m_classModsActiveOld: i32,                 // 0x1d14 ( Size = 4 )
    pub m_classModValues: ClassModValues,          // 0x1d18 ( Size = 116 )
    pub m_posClassModsActive: [i32; 4],            // 0x1d8c ( Size = 16 )
    pub m_posClassModsActiveOld: [i32; 4],         // 0x1d9c ( Size = 16 )
    pub m_perPosValues: [PerPosClassModValues; 4], // 0x1dac ( Size = 8 )
    pub m_passives: [bool; 128],                   // 0x1dcc ( Size = 128 )
    pub m_communityId: i32,                        // 0x1e4c ( Size = 4 )
    pub m_nButtons: i32,                           // 0x1e50 ( Size = 4 )
    pub m_afButtonPressed: i32,                    // 0x1e54 ( Size = 4 )
    pub m_afButtonReleased: i32,                   // 0x1e58 ( Size = 4 )
    pub m_afButtonLast: i32,                       // 0x1e5c ( Size = 4 )
    pub m_afButtonDisabled: i32,                   // 0x1e60 ( Size = 4 )
    pub m_afButtonForced: i32,                     // 0x1e64 ( Size = 4 )
    pub m_forwardMove: f32,                        // 0x1e68 ( Size = 4 )
    pub m_sideMove: f32,                           // 0x1e6c ( Size = 4 )
    pub m_prevForwardMove: f32,                    // 0x1e70 ( Size = 4 )
    pub m_prevSideMove: f32,                       // 0x1e74 ( Size = 4 )
    pub m_bLagCompensation: bool,                  // 0x1e78 ( Size = 1 )
    pub m_bPredictWeapons: bool,                   // 0x1e79 ( Size = 1 )
    pub m_bPredictionEnabled: bool,                // 0x1e7a ( Size = 1 )
    pub m_wantedToMatchmake: bool,                 // 0x1e7b ( Size = 1 )
    pub m_skyCamera: EHandle,                      // 0x1e7c ( Size = 4 )
    pub m_titanSoulBeingRodeoed: EHandle,          // 0x1e80 ( Size = 4 )
    pub m_entitySyncingWithMe: EHandle,            // 0x1e84 ( Size = 4 )
    pub m_playerFlags: i32,                        // 0x1e88 ( Size = 4 )
    pub m_hasMic: bool,                            // 0x1e8c ( Size = 1 )
    pub m_inPartyChat: bool,                       // 0x1e8d ( Size = 1 )
    pub gap_1e8e: [c_char; 2],
    pub m_playerMoveSpeedScale: f32,         // 0x1e90 ( Size = 4 )
    pub m_gestureSequences: [i32; 4],        // 0x1e94 ( Size = 16 )
    pub m_gestureStartTimes: [f32; 4],       // 0x1ea4 ( Size = 16 )
    pub m_gestureBlendInDuration: [f32; 4],  // 0x1eb4 ( Size = 16 )
    pub m_gestureBlendOutDuration: [f32; 4], // 0x1ec4 ( Size = 16 )
    pub m_gestureFadeOutStartTime: [f32; 4], // 0x1ed4 ( Size = 16 )
    pub m_gestureFadeOutDuration: [f32; 4],  // 0x1ee4 ( Size = 16 )
    pub m_gestureAutoKillBitfield: i32,      // 0x1ef4 ( Size = 4 )
    pub m_bDropEnabled: bool,                // 0x1ef8 ( Size = 1 )
    pub m_bDuckEnabled: bool,                // 0x1ef9 ( Size = 1 )
    pub gap_1efa: [c_char; 2],
    pub m_iRespawnFrames: i32,                 // 0x1efc ( Size = 4 )
    pub m_afPhysicsFlags: i32,                 // 0x1f00 ( Size = 4 )
    pub m_remoteTurret: EHandle,               // 0x1f04 ( Size = 4 )
    pub m_flTimeLastTouchedGround: f32,        // 0x1f08 ( Size = 4 )
    pub m_flTimeLastJumped: f32,               // 0x1f0c ( Size = 4 )
    pub m_flTimeLastLanded: f32,               // 0x1f10 ( Size = 4 )
    pub m_upDirWhenLastTouchedGround: Vector3, // 0x1f14 ( Size = 12 )
    pub m_bHasJumpedSinceTouchedGround: bool,  // 0x1f20 ( Size = 1 )
    pub gap_1f21: [c_char; 3],
    pub m_holdToUseTimeLeft: f32,           // 0x1f24 ( Size = 4 )
    pub m_fTimeLastHurt: f32,               // 0x1f28 ( Size = 4 )
    pub m_fLastAimBotCheckTime: f32,        // 0x1f2c ( Size = 4 )
    pub m_accumDamageImpulseVel: Vector3,   // 0x1f30 ( Size = 12 )
    pub m_accumDamageImpulseTime: f32,      // 0x1f3c ( Size = 4 )
    pub m_damageImpulseNoDecelEndTime: f32, // 0x1f40 ( Size = 4 )
    pub m_hDmgEntity: EHandle,              // 0x1f44 ( Size = 4 )
    pub m_DmgTake: f32,                     // 0x1f48 ( Size = 4 )
    pub m_bitsDamageType: i32,              // 0x1f4c ( Size = 4 )
    pub m_bitsHUDDamage: i32,               // 0x1f50 ( Size = 4 )
    pub m_xpRate: f32,                      // 0x1f54 ( Size = 4 )
    pub m_flDeathTime: f32,                 // 0x1f58 ( Size = 4 )
    pub m_flDeathAnimTime: f32,             // 0x1f5c ( Size = 4 )
    pub m_frozen: bool,                     // 0x1f60 ( Size = 1 )
    pub m_stressAnimation: bool,            // 0x1f61 ( Size = 1 )
    pub gap_1f62: [c_char; 2],
    pub m_iObserverMode: i32,                  // 0x1f64 ( Size = 4 )
    pub m_iObserverLastMode: i32,              // 0x1f68 ( Size = 4 )
    pub m_hObserverTarget: EHandle,            // 0x1f6c ( Size = 4 )
    pub m_observerModeStaticPosition: Vector3, // 0x1f70 ( Size = 12 )
    pub m_observerModeStaticAngles: Vector3,   // 0x1f7c ( Size = 12 )
    pub m_isValidChaseObserverTarget: bool,    // 0x1f88 ( Size = 1 )
    pub gap_1f89: [c_char; 3],
    pub m_vphysicsCollisionState: i32, // 0x1f8c ( Size = 4 )
    pub m_bHasVPhysicsCollision: bool, // 0x1f90 ( Size = 1 )
    pub gap_1f91: [c_char; 3],
    pub m_fNextSuicideTime: f32,           // 0x1f94 ( Size = 4 )
    pub m_iSuicideCustomKillFlags: i32,    // 0x1f98 ( Size = 4 )
    pub m_preNoClipPhysicsFlags: i32,      // 0x1f9c ( Size = 4 )
    pub m_hTonemapController: EHandle,     // 0x1fa0 ( Size = 4 )
    pub m_activeBurnCardIndex: i32,        // 0x1fa4 ( Size = 4 )
    pub m_CommandContext: [c_char; 32],    // 0x1fa8 ( Size = 32 ) // void
    pub m_pPhysicsController: *mut c_void, // 0x1fc8 ( Size = 8 )
    pub m_pShadowStand: *mut c_void,       // 0x1fd0 ( Size = 8 )
    pub m_pShadowCrouch: *mut c_void,      // 0x1fd8 ( Size = 8 )
    pub m_oldOrigin: Vector3,              // 0x1fe0 ( Size = 12 )
    pub m_vecSmoothedVelocity: Vector3,    // 0x1fec ( Size = 12 )
    pub m_bTouchedPhysObject: bool,        // 0x1ff8 ( Size = 1 )
    pub m_bPhysicsWasFrozen: bool,         // 0x1ff9 ( Size = 1 )
    pub gap_1ffa: [c_char; 2],
    pub m_iTargetVolume: i32,                 // 0x1ffc ( Size = 4 )
    pub m_flDuckTime: f32,                    // 0x2000 ( Size = 4 )
    pub m_flDuckJumpTime: f32,                // 0x2004 ( Size = 4 )
    pub m_VDU: bool,                          // 0x2008 ( Size = 1 )
    pub m_fInitHUD: bool,                     // 0x2009 ( Size = 1 )
    pub m_fGameHUDInitialized: bool,          // 0x200a ( Size = 1 )
    pub m_fWeapon: bool,                      // 0x200b ( Size = 1 )
    pub m_iUpdateTime: i32,                   // 0x200c ( Size = 4 )
    pub m_iConnected: i32,                    // 0x2010 ( Size = 4 )
    pub m_iPlayerLocked: i32,                 // 0x2014 ( Size = 4 )
    pub m_gameStats: [i32; 12],               // 0x2018 ( Size = 48 )
    pub m_firstPersonProxy: EHandle,          // 0x2048 ( Size = 4 )
    pub m_predictedFirstPersonProxy: EHandle, // 0x204c ( Size = 4 )
    pub m_grappleHook: EHandle,               // 0x2050 ( Size = 4 )
    pub m_petTitan: EHandle,                  // 0x2054 ( Size = 4 )
    pub m_petTitanMode: i32,                  // 0x2058 ( Size = 4 )
    pub m_xp: i32,                            // 0x205c ( Size = 4 )
    pub m_generation: i32,                    // 0x2060 ( Size = 4 )
    pub m_rank: i32,                          // 0x2064 ( Size = 4 )
    pub m_serverForceIncreasePlayerListGenerationParity: i32, // 0x2068 ( Size = 4 )
    pub m_isPlayingRanked: bool,              // 0x206c ( Size = 1 )
    pub gap_206d: [c_char; 3],
    pub m_skill_mu: f32,                         // 0x2070 ( Size = 4 )
    pub m_hardpointEntity: EHandle,              // 0x2074 ( Size = 4 )
    pub m_nextTitanRespawnAvailable: f32,        // 0x2078 ( Size = 4 )
    pub m_activeViewmodelModifiers: [bool; 25],  // 0x207c ( Size = 25 )
    pub m_activeViewmodelModifiersChanged: bool, // 0x2095 ( Size = 1 )
    pub gap_2096: [c_char; 2],
    pub m_hViewModel: EHandle, // 0x2098 ( Size = 4 )
    pub gap_209c: [c_char; 4],
    pub m_LastCmd: [c_char; 312],       // 0x20a0 ( Size = 312 ) // void
    pub m_pCurrentCommand: *mut c_void, // 0x21d8 ( Size = 8 )
    pub m_flStepSoundTime: f32,         // 0x21e0 ( Size = 4 )
    pub m_flStepSoundReduceTime: f32,   // 0x21e4 ( Size = 4 )
    pub m_hThirdPersonEnt: EHandle,     // 0x21e8 ( Size = 4 )
    pub gap_21ec: [c_char; 4],
    pub m_thirdPerson: ThirdPersonViewData, // 0x21f0 ( Size = 96 )
    pub m_duckState: i32,                   // 0x2250 ( Size = 4 )
    pub m_StandHullMin: Vector3,            // 0x2254 ( Size = 12 )
    pub m_StandHullMax: Vector3,            // 0x2260 ( Size = 12 )
    pub m_DuckHullMin: Vector3,             // 0x226c ( Size = 12 )
    pub m_DuckHullMax: Vector3,             // 0x2278 ( Size = 12 )
    pub m_upDir: Vector3,                   // 0x2284 ( Size = 12 )
    pub m_upDirPredicted: Vector3,          // 0x2290 ( Size = 12 )
    pub m_lastWallRunStartPos: Vector3,     // 0x229c ( Size = 12 )
    pub m_wallRunStartTime: f32,            // 0x22a8 ( Size = 4 )
    pub m_wallRunClearTime: f32,            // 0x22ac ( Size = 4 )
    pub m_wallRunCount: i32,                // 0x22b0 ( Size = 4 )
    pub m_wallRunWeak: bool,                // 0x22b4 ( Size = 1 )
    pub gap_22b5: [c_char; 3],
    pub m_wallRunPushAwayTime: f32,    // 0x22b8 ( Size = 4 )
    pub m_wallrunFrictionScale: f32,   // 0x22bc ( Size = 4 )
    pub m_groundFrictionScale: f32,    // 0x22c0 ( Size = 4 )
    pub m_wallrunRetryTime: f32,       // 0x22c4 ( Size = 4 )
    pub m_wallrunRetryPos: Vector3,    // 0x22c8 ( Size = 12 )
    pub m_wallrunRetryNormal: Vector3, // 0x22d4 ( Size = 12 )
    pub m_wallHanging: bool,           // 0x22e0 ( Size = 1 )
    pub gap_22e1: [c_char; 3],
    pub m_wallHangStartTime: f32,                // 0x22e4 ( Size = 4 )
    pub m_wallHangTime: f32,                     // 0x22e8 ( Size = 4 )
    pub m_traversalType: i32,                    // 0x22ec ( Size = 4 )
    pub m_traversalState: i32,                   // 0x22f0 ( Size = 4 )
    pub m_traversalBegin: Vector3,               // 0x22f4 ( Size = 12 )
    pub m_traversalMid: Vector3,                 // 0x2300 ( Size = 12 )
    pub m_traversalEnd: Vector3,                 // 0x230c ( Size = 12 )
    pub m_traversalMidFrac: f32,                 // 0x2318 ( Size = 4 )
    pub m_traversalForwardDir: Vector3,          // 0x231c ( Size = 12 )
    pub m_traversalRefPos: Vector3,              // 0x2328 ( Size = 12 )
    pub m_traversalProgress: f32,                // 0x2334 ( Size = 4 )
    pub m_traversalStartTime: f32,               // 0x2338 ( Size = 4 )
    pub m_traversalHandAppearTime: f32,          // 0x233c ( Size = 4 )
    pub m_traversalReleaseTime: f32,             // 0x2340 ( Size = 4 )
    pub m_traversalBlendOutStartTime: f32,       // 0x2344 ( Size = 4 )
    pub m_traversalBlendOutStartOffset: Vector3, // 0x2348 ( Size = 12 )
    pub m_traversalYawDelta: f32,                // 0x2354 ( Size = 4 )
    pub m_traversalYawPoseParameter: i32,        // 0x2358 ( Size = 4 )
    pub m_wallDangleJumpOffTime: f32,            // 0x235c ( Size = 4 )
    pub m_wallDangleMayHangHere: bool,           // 0x2360 ( Size = 1 )
    pub m_wallDangleForceFallOff: bool,          // 0x2361 ( Size = 1 )
    pub m_wallDangleLastPushedForward: bool,     // 0x2362 ( Size = 1 )
    pub gap_2363: [c_char; 1],
    pub m_wallDangleDisableWeapon: i32,      // 0x2364 ( Size = 4 )
    pub m_wallDangleClimbProgressFloor: f32, // 0x2368 ( Size = 4 )
    pub m_prevMoveYaw: f32,                  // 0x236c ( Size = 4 )
    pub m_sprintTiltVel: f32,                // 0x2370 ( Size = 4 )
    pub m_sprintTiltPoseParameter: i32,      // 0x2374 ( Size = 4 )
    pub m_sprintFracPoseParameter: i32,      // 0x2378 ( Size = 4 )
    pub gap_237c: [c_char; 4],
    pub m_grapple: GrappleData,         // 0x2380 ( Size = 104 )
    pub m_grappleActive: bool,          // 0x23e8 ( Size = 1 )
    pub m_grappleNeedWindowCheck: bool, // 0x23e9 ( Size = 1 )
    pub gap_23ea: [c_char; 2],
    pub m_grappleNextWindowHint: EHandle, // 0x23ec ( Size = 4 )
    pub m_sliding: bool,                  // 0x23f0 ( Size = 1 )
    pub m_slideLongJumpAllowed: bool,     // 0x23f1 ( Size = 1 )
    pub gap_23f2: [c_char; 2],
    pub m_lastSlideTime: f32,     // 0x23f4 ( Size = 4 )
    pub m_lastSlideBoost: f32,    // 0x23f8 ( Size = 4 )
    pub m_activeZipline: EHandle, // 0x23fc ( Size = 4 )
    pub m_ziplineReverse: bool,   // 0x2400 ( Size = 1 )
    pub gap_2401: [c_char; 3],
    pub m_lastZipline: EHandle,                // 0x2404 ( Size = 4 )
    pub m_useLastZiplineCooldown: f32,         // 0x2408 ( Size = 4 )
    pub m_ziplineValid3pWeaponLayerAnim: bool, // 0x240c ( Size = 1 )
    pub gap_240d: [c_char; 3],
    pub m_ziplineState: i32, // 0x2410 ( Size = 4 )
    pub gap_2414: [c_char; 4],
    pub m_zipline: PlayerZiplineData,    // 0x2418 ( Size = 80 )
    pub m_operator: Player_OperatorData, // 0x2468 ( Size = 128 )
    pub m_viewOffsetEntity: Player_ViewOffsetEntityData, // 0x24e8 ( Size = 24 )
    pub m_animViewEntity: Player_AnimViewEntityData, // 0x2500 ( Size = 128 )
    pub m_highSpeedViewmodelAnims: bool, // 0x2580 ( Size = 1 )
    pub gap_2581: [c_char; 3],
    pub m_gravityGrenadeStatusEffect: i32, // 0x2584 ( Size = 4 )
    pub m_onSlopeTime: f32,                // 0x2588 ( Size = 4 )
    pub m_lastWallNormal: Vector3,         // 0x258c ( Size = 12 )
    pub m_dodgingInAir: bool,              // 0x2598 ( Size = 1 )
    pub m_dodging: bool,                   // 0x2599 ( Size = 1 )
    pub gap_259a: [c_char; 2],
    pub m_lastDodgeTime: f32,   // 0x259c ( Size = 4 )
    pub m_airSpeed: f32,        // 0x25a0 ( Size = 4 )
    pub m_airAcceleration: f32, // 0x25a4 ( Size = 4 )
    pub m_iSpawnParity: bool,   // 0x25a8 ( Size = 1 )
    pub m_boosting: bool,       // 0x25a9 ( Size = 1 )
    pub m_repeatedBoost: bool,  // 0x25aa ( Size = 1 )
    pub gap_25ab: [c_char; 1],
    pub m_boostMeter: f32, // 0x25ac ( Size = 4 )
    pub m_jetpack: bool,   // 0x25b0 ( Size = 1 )
    pub m_gliding: bool,   // 0x25b1 ( Size = 1 )
    pub gap_25b2: [c_char; 2],
    pub m_glideMeter: f32,                    // 0x25b4 ( Size = 4 )
    pub m_glideRechargeDelayAccumulator: f32, // 0x25b8 ( Size = 4 )
    pub m_hovering: bool,                     // 0x25bc ( Size = 1 )
    pub m_climbing: bool,                     // 0x25bd ( Size = 1 )
    pub m_isPerformingBoostAction: bool,      // 0x25be ( Size = 1 )
    pub gap_25bf: [c_char; 1],
    pub m_lastJumpHeight: f32,                    // 0x25c0 ( Size = 4 )
    pub m_numPingsUsed: i32,                      // 0x25c4 ( Size = 4 )
    pub m_numPingsAvailable: i32,                 // 0x25c8 ( Size = 4 )
    pub m_lastPingTime: f32,                      // 0x25cc ( Size = 4 )
    pub m_pingGroupStartTime: f32,                // 0x25d0 ( Size = 4 )
    pub m_pingGroupAccumulator: i32,              // 0x25d4 ( Size = 4 )
    pub m_lastBodySound1p: u64,                   // 0x25d8 ( Size = 8 )
    pub m_lastBodySound3p: u64,                   // 0x25e0 ( Size = 8 )
    pub m_lastFinishSound1p: u64,                 // 0x25e8 ( Size = 8 )
    pub m_lastFinishSound3p: u64,                 // 0x25f0 ( Size = 8 )
    pub m_primedSound1p: u64,                     // 0x25f8 ( Size = 8 )
    pub m_primedSound3p: u64,                     // 0x2600 ( Size = 8 )
    pub m_currentFramePlayer: CurrentData_Player, // 0x2608 ( Size = 32 )
    pub m_currentFrameLocalPlayer: CurrentData_LocalPlayer, // 0x2628 ( Size = 96 )
    pub m_nImpulse: i32,                          // 0x2688 ( Size = 4 )
    pub m_flFlashTime: f32,                       // 0x268c ( Size = 4 )
    pub m_flForwardMove: f32,                     // 0x2690 ( Size = 4 )
    pub m_flSideMove: f32,                        // 0x2694 ( Size = 4 )
    pub m_nNumCrateHudHints: i32,                 // 0x2698 ( Size = 4 )
    pub m_needStuckCheck: bool,                   // 0x269c ( Size = 1 )
    pub gap_269d: [c_char; 3],
    pub m_totalFrameTime: f32,                   // 0x26a0 ( Size = 4 )
    pub m_joinFrameTime: f32,                    // 0x26a4 ( Size = 4 )
    pub m_lastUCmdSimulationTicks: i32,          // 0x26a8 ( Size = 4 )
    pub m_lastUCmdSimulationRemainderTime: f32,  // 0x26ac ( Size = 4 )
    pub m_totalExtraClientCmdTimeAttempted: f32, // 0x26b0 ( Size = 4 )
    pub m_bGamePaused: bool,                     // 0x26b4 ( Size = 1 )
    pub m_bPlayerUnderwater: bool,               // 0x26b5 ( Size = 1 )
    pub gap_26b6: [c_char; 2],
    pub m_hPlayerViewEntity: EHandle, // 0x26b8 ( Size = 4 )
    pub m_bShouldDrawPlayerWhileUsingViewEntity: bool, // 0x26bc ( Size = 1 )
    pub gap_26bd: [c_char; 3],
    pub m_hConstraintEntity: EHandle,   // 0x26c0 ( Size = 4 )
    pub m_vecConstraintCenter: Vector3, // 0x26c4 ( Size = 12 )
    pub m_flConstraintRadius: f32,      // 0x26d0 ( Size = 4 )
    pub m_flConstraintWidth: f32,       // 0x26d4 ( Size = 4 )
    pub m_flConstraintSpeedFactor: f32, // 0x26d8 ( Size = 4 )
    pub m_bConstraintPastRadius: bool,  // 0x26dc ( Size = 1 )
    pub gap_26dd: [c_char; 3],
    pub m_lastActiveTime: f32,                   // 0x26e0 ( Size = 4 )
    pub m_flLaggedMovementValue: f32,            // 0x26e4 ( Size = 4 )
    pub m_lastMoveInputTime: f32,                // 0x26e8 ( Size = 4 )
    pub m_vNewVPhysicsPosition: Vector3,         // 0x26ec ( Size = 12 )
    pub m_vNewVPhysicsVelocity: Vector3,         // 0x26f8 ( Size = 12 )
    pub m_vNewVPhysicsWishVel: Vector3,          // 0x2704 ( Size = 12 )
    pub m_vecPreviouslyPredictedOrigin: Vector3, // 0x2710 ( Size = 12 )
    pub m_nBodyPitchPoseParam: i32,              // 0x271c ( Size = 4 )
    pub m_lastNavArea: u64,                      // 0x2720 ( Size = 8 ) // void
    pub m_szNetworkIDString: [c_char; 64],       // 0x2728 ( Size = 64 )
    pub m_squad: u64,                            // 0x2768 ( Size = 8 )
    pub m_SquadName: u64,                        // 0x2770 ( Size = 8 )
    pub m_gameMovementUtil: [c_char; 56],        // 0x2778 ( Size = 56 ) // void
    pub m_flTimeAllSuitDevicesOff: f32,          // 0x27b0 ( Size = 4 )
    pub m_bIsStickySprinting: bool,              // 0x27b4 ( Size = 1 )
    pub gap_27b5: [c_char; 3],
    pub m_fStickySprintMinTime: f32,       // 0x27b8 ( Size = 4 )
    pub m_bPlayedSprintStartEffects: bool, // 0x27bc ( Size = 1 )
    pub gap_27bd: [c_char; 3],
    pub m_autoSprintForced: i32, // 0x27c0 ( Size = 4 )
    pub m_fIsSprinting: bool,    // 0x27c4 ( Size = 1 )
    pub m_fIsWalking: bool,      // 0x27c5 ( Size = 1 )
    pub gap_27c6: [c_char; 2],
    pub m_useHeldTime: f32,              // 0x27c8 ( Size = 4 )
    pub m_sprintStartedTime: f32,        // 0x27cc ( Size = 4 )
    pub m_sprintStartedFrac: f32,        // 0x27d0 ( Size = 4 )
    pub m_sprintEndedTime: f32,          // 0x27d4 ( Size = 4 )
    pub m_sprintEndedFrac: f32,          // 0x27d8 ( Size = 4 )
    pub m_stickySprintStartTime: f32,    // 0x27dc ( Size = 4 )
    pub m_bSinglePlayerGameEnding: bool, // 0x27e0 ( Size = 1 )
    pub gap_27e1: [c_char; 3],
    pub m_ubEFNoInterpParity: i32, // 0x27e4 ( Size = 4 )
    pub m_viewConeActive: bool,    // 0x27e8 ( Size = 1 )
    pub m_viewConeParented: bool,  // 0x27e9 ( Size = 1 )
    pub gap_27ea: [c_char; 2],
    pub m_viewConeParity: i32,           // 0x27ec ( Size = 4 )
    pub m_lastViewConeParityTick: i32,   // 0x27f0 ( Size = 4 )
    pub m_viewConeLerpTime: f32,         // 0x27f4 ( Size = 4 )
    pub m_viewConeSpecificEnabled: bool, // 0x27f8 ( Size = 1 )
    pub gap_27f9: [c_char; 3],
    pub m_viewConeSpecific: Vector3,         // 0x27fc ( Size = 12 )
    pub m_viewConeRelativeAngleMin: Vector3, // 0x2808 ( Size = 12 )
    pub m_viewConeRelativeAngleMax: Vector3, // 0x2814 ( Size = 12 )
    pub m_hReservedSpawnPoint: EHandle,      // 0x2820 ( Size = 4 )
    pub m_hLastSpawnPoint: EHandle,          // 0x2824 ( Size = 4 )
    pub m_autoKickDisabled: bool,            // 0x2828 ( Size = 1 )
    pub gap_2829: [c_char; 3],
    pub m_movementCollisionNormal: Vector3, // 0x282c ( Size = 12 )
    pub m_groundNormal: Vector3,            // 0x2838 ( Size = 12 )
    pub m_stuckCharacter: EHandle,          // 0x2844 ( Size = 4 )
    pub m_title: [c_char; 32],              // 0x2848 ( Size = 32 )
    pub sentHUDScriptChecksum: bool,        // 0x2868 ( Size = 1 )
    pub m_bIsFullyConnected: bool,          // 0x2869 ( Size = 1 )
    pub gap_286a: [c_char; 2],
    pub m_lastDeathInfo: CTakeDamageInfo, // 0x286c ( Size = 120 )
    pub gap_28e4: [c_char; 4],
    pub m_melee: PlayerMelee_PlayerData, // 0x28e8 ( Size = 40 )
    pub m_lungeTargetEntity: EHandle,    // 0x2910 ( Size = 4 )
    pub m_isLungingToPosition: bool,     // 0x2914 ( Size = 1 )
    pub gap_2915: [c_char; 3],
    pub m_lungeTargetPosition: Vector3, // 0x2918 ( Size = 12 )
    pub m_lungeStartPositionOffset: Vector3, // 0x2924 ( Size = 12 )
    pub m_lungeStartPositionOffset_notLagCompensated: Vector3, // 0x2930 ( Size = 12 )
    pub m_lungeEndPositionOffset: Vector3, // 0x293c ( Size = 12 )
    pub m_lungeStartTime: f32,          // 0x2948 ( Size = 4 )
    pub m_lungeEndTime: f32,            // 0x294c ( Size = 4 )
    pub m_lungeCanFly: bool,            // 0x2950 ( Size = 1 )
    pub m_lungeLockPitch: bool,         // 0x2951 ( Size = 1 )
    pub gap_2952: [c_char; 2],
    pub m_lungeStartPitch: f32,  // 0x2954 ( Size = 4 )
    pub m_lungeSmoothTime: f32,  // 0x2958 ( Size = 4 )
    pub m_lungeMaxTime: f32,     // 0x295c ( Size = 4 )
    pub m_lungeMaxEndSpeed: f32, // 0x2960 ( Size = 4 )
    pub m_useCredit: bool,       // 0x2964 ( Size = 1 )
    pub gap_2965: [c_char; 3],
    pub m_smartAmmoNextTarget: u64, // 0x2968 ( Size = 8 )
    pub m_smartAmmoPrevTarget: u64, // 0x2970 ( Size = 8 )
    pub m_smartAmmoHighestLocksOnMeFractionValues: [f32; 4], // 0x2978 ( Size = 16 )
    pub m_smartAmmoHighestLocksOnMeEntities: [EHandle; 4], // 0x2988 ( Size = 16 )
    pub m_smartAmmoPreviousHighestLockOnMeFractionValue: f32, // 0x2998 ( Size = 4 )
    pub m_smartAmmoPendingHighestLocksOnMeFractionValues: [f32; 4], // 0x299c ( Size = 16 )
    pub gap_29ac: [c_char; 4],
    pub m_smartAmmoPendingHighestLocksOnMeEntities: [*mut CBaseEntity; 4], // 0x29b0 ( Size = 32 )
    pub m_smartAmmoRemoveFromTargetList: bool,                             // 0x29d0 ( Size = 1 )
    pub gap_29d1: [c_char; 3],
    pub m_delayedFlinchEvents: i32,           // 0x29d4 ( Size = 4 )
    pub m_delayedFlinchEventCount: u64,       // 0x29d8 ( Size = 8 )
    pub m_extraWeaponModNames: [c_char; 512], // 0x29e0 ( Size = 512 )
    pub m_extraWeaponModNamesArray: [c_char; 64], // 0x2be0 ( Size = 64 ) // void
    pub m_extraWeaponModNameCount: u64,       // 0x2c20 ( Size = 8 )
    pub m_pPlayerAISquad: u64,                // 0x2c28 ( Size = 8 ) // void
    pub m_flAreaCaptureScoreAccumulator: f32, // 0x2c30 ( Size = 4 )
    pub m_flCapPointScoreRate: f32,           // 0x2c34 ( Size = 4 )
    pub m_flConnectionTime: f32,              // 0x2c38 ( Size = 4 )
    pub m_fullyConnectedTime: f32,            // 0x2c3c ( Size = 4 )
    pub m_connectedForDurationCallback_duration: f32, // 0x2c40 ( Size = 4 )
    pub m_flLastForcedChangeTeamTime: f32,    // 0x2c44 ( Size = 4 )
    pub m_iBalanceScore: i32,                 // 0x2c48 ( Size = 4 )
    pub gap_2c4c: [c_char; 4],
    pub m_PlayerAnimState: u64,             // 0x2c50 ( Size = 8 )
    pub m_vWorldSpaceCenterHolder: Vector3, // 0x2c58 ( Size = 12 )
    pub m_vPrevGroundNormal: Vector3,       // 0x2c64 ( Size = 12 )
    pub m_threadedPostProcessJob: i32,      // 0x2c70 ( Size = 4 )
    pub gap_2c74: [c_char; 4],
    pub m_Shared: CPlayerShared, // 0x2c78 ( Size = 160 )
    pub m_statusEffectsTimedPlayerNV: [StatusEffectTimedData; 10], // 0x2d18 ( Size = 24 )
    pub m_statusEffectsEndlessPlayerNV: [StatusEffectEndlessData; 10], // 0x2e08 ( Size = 16 )
    pub m_pilotClassIndex: i32,  // 0x2ea8 ( Size = 4 )
    pub m_latestCommandRun: i32, // 0x2eac ( Size = 4 )
    pub m_nearbyPushers: [c_char; 480], // 0x2eb0 ( Size = 480 ) // void
    pub m_nearbyPusherCount: i32, // 0x3090 ( Size = 4 )
    pub m_pushHistory: [PushHistoryEntry; 16], // 0x3094 ( Size = 16 )
    pub m_pushHistoryEntryIndex: i32, // 0x3194 ( Size = 4 )
    pub m_baseVelocityLastServerTime: f32, // 0x3198 ( Size = 4 )
    pub m_pushedThisFrame: Vector3, // 0x319c ( Size = 12 )
    pub m_pushedThisSnapshotAccum: Vector3, // 0x31a8 ( Size = 12 )
    pub m_pushedFixedPointOffset: [i32; 3], // 0x31b4 ( Size = 12 )
    pub m_lastCommandContextWarnTime: f32, // 0x31c0 ( Size = 4 )
    pub m_pushAwayFromTopAcceleration: Vector3, // 0x31c4 ( Size = 12 )
    pub m_trackedState: [f32; 52], // 0x31d0 ( Size = 208 )
    pub m_prevTrackedState: i32, // 0x32a0 ( Size = 4 )
    pub m_prevTrackedStatePos: Vector3, // 0x32a4 ( Size = 12 )
    pub m_recordingAnim: u64,    // 0x32b0 ( Size = 8 ) // void
    pub m_animRecordFile: u64,   // 0x32b8 ( Size = 8 ) // void
    pub m_animRecordButtons: i32, // 0x32c0 ( Size = 4 ) //void
    pub m_prevAbsOrigin: Vector3, // 0x32c4 ( Size = 12 )
    pub m_sendMovementCallbacks: bool, // 0x32d0 ( Size = 1 )
    pub m_sendInputCallbacks: bool, // 0x32d1 ( Size = 1 )
    pub gap_32d2: [c_char; 2],
    pub m_predictableServerEvents: [PredictableServerEvent; 16], // 0x32d4 ( Size = 24 )
    pub m_predictableServerEventCount: i32,                      // 0x3454 ( Size = 4 )
    pub m_predictableServerEventAcked: i32,                      // 0x3458 ( Size = 4 )
    pub m_playerScriptNetDataGlobal: EHandle,                    // 0x345c ( Size = 4 )
    pub m_playerScriptNetDataExclusive: EHandle,                 // 0x3460 ( Size = 4 )
}
size_assert!(PLAYER_SIZE where CPlayer == 0x3468);

impl DerefMut for CPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CPlayer {
    type Target = CBaseCombatCharacter;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
