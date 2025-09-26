#![allow(non_camel_case_types, non_snake_case)]
use std::{
    ffi::c_void,
    ops::{Deref, DerefMut},
};

use super::{cbasecombatweapon::CBaseCombatWeapon, cbaseentity::CBaseEntity, cplayer::EHandle};

use crate::size_assert;

#[repr(C)]
pub struct SmartAmmo_WeaponData {
    pub vftable: *const c_void,
    pub numTargetEntities: i32,                 // 0x8 ( Size: 4 )
    pub targetEntities: [EHandle; 8],           // 0xc ( Size: 32 )
    pub fractions: [f32; 8],                    // 0x2c ( Size: 32 )
    pub previousFractions: [f32; 8],            // 0x4c ( Size: 32 )
    pub currentFractions: [f32; 8],             // 0x6c ( Size: 32 )
    pub visiblePoints: [i32; 8],                // 0x8c ( Size: 32 )
    pub lastVisibleTimes: [f32; 8],             // 0xac ( Size: 32 )
    pub lastFullLockTimes: [f32; 8],            // 0xcc ( Size: 32 )
    pub storedTargets: [EHandle; 8],            // 0xec ( Size: 32 )
    pub lastNewTargetTime: f32,                 // 0x10c ( Size: 4 )
    pub trackerCount: i32,                      // 0x110 ( Size: 4 )
    pub trackerEntities: [EHandle; 8],          // 0x114 ( Size: 32 )
    pub trackerLocks: [i32; 8],                 // 0x134 ( Size: 32 )
    pub trackerTimes: [f32; 8],                 // 0x154 ( Size: 32 )
    pub gap_174: [u8; 4],                       // TODO: the offsets here are wrong
    pub previousTargetEntities: [EHandle; 10],  // 0x178 ( Size: 40 )
    pub previousTrackerEntities: [EHandle; 10], // 0x1a0 ( Size: 40 )
    pub previousTrackerLocks: [[u8; 5]; 8],     // 0x1c8 ( Size: 40 )
}

size_assert!(SIZE_AMMO where SmartAmmo_WeaponData == 0x1f0);

#[repr(C)]
pub struct WeaponPlayerData {
    pub vftable: *const c_void,
    pub m_spread: f32,                                     // 0x8 ( Size: 4 )
    pub m_spreadStartTime: f32,                            // 0xc ( Size: 4 )
    pub m_spreadStartFracHip: f32,                         // 0x10 ( Size: 4 )
    pub m_spreadStartFracADS: f32,                         // 0x14 ( Size: 4 )
    pub m_kickSpreadHipfire: f32,                          // 0x18 ( Size: 4 )
    pub m_kickSpreadADS: f32,                              // 0x1c ( Size: 4 )
    pub m_kickTime: f32,                                   // 0x20 ( Size: 4 )
    pub m_kickScaleBase: f32,                              // 0x24 ( Size: 4 )
    pub m_semiAutoTriggerHoldTime: f32,                    // 0x28 ( Size: 4 )
    pub m_semiAutoTriggerDown: bool,                       // 0x2c ( Size: 1 )
    pub m_pendingTriggerPull: bool,                        // 0x2d ( Size: 1 )
    pub m_semiAutoNeedsRechamber: bool,                    // 0x2e ( Size: 1 )
    pub m_pendingReloadAttempt: bool,                      // 0x2f ( Size: 1 )
    pub m_offhandHybridNormalMode: bool,                   // 0x30 ( Size: 1 )
    pub m_pendingoffhandHybridToss: bool,                  // 0x31 ( Size: 1 )
    pub m_fastHolster: bool,                               // 0x32 ( Size: 1 )
    pub m_didFirstDeploy: bool,                            // 0x33 ( Size: 1 )
    pub m_shouldCatch: bool,                               // 0x34 ( Size: 1 )
    pub m_clipModelIsHidden: bool,                         // 0x35 ( Size: 1 )
    pub m_customActivityPlayRaiseOnComplete: bool,         // 0x36 ( Size: 1 )
    pub m_segmentedReloadEndSeqRequired: bool,             // 0x37 ( Size: 1 )
    pub m_segmentedReloadStartedEmpty: bool,               // 0x38 ( Size: 1 )
    pub m_segmentedReloadStartedOneHanded: bool,           // 0x39 ( Size: 1 )
    pub m_segmentedReloadCanRestartLoop: bool,             // 0x3a ( Size: 1 )
    pub m_segmentedReloadLoopFireLocked: bool,             // 0x3b ( Size: 1 )
    pub m_customActivityAttachedModelIndex: i32,           // 0x3c ( Size: 4 )
    pub m_customActivityAttachedModelAttachmentIndex: i32, // 0x40 ( Size: 4 )
    pub m_fireRateLerp_startTime: f32,                     // 0x44 ( Size: 4 )
    pub m_fireRateLerp_startFraction: f32,                 // 0x48 ( Size: 4 )
    pub m_fireRateLerp_stopTime: f32,                      // 0x4c ( Size: 4 )
    pub m_fireRateLerp_stopFraction: f32,                  // 0x50 ( Size: 4 )
    pub m_chargeAnimIndex: i32,                            // 0x54 ( Size: 4 )
    pub m_chargeAnimIndexOld: i32,                         // 0x58 ( Size: 4 )
    pub m_proScreen_owner: EHandle,                        // 0x5c ( Size: 4 )
    pub m_proScreen_int0: i32,                             // 0x60 ( Size: 4 )
    pub m_proScreen_int1: i32,                             // 0x64 ( Size: 4 )
    pub m_proScreen_int2: i32,                             // 0x68 ( Size: 4 )
    pub m_proScreen_float0: f32,                           // 0x6c ( Size: 4 )
    pub m_proScreen_float1: f32,                           // 0x70 ( Size: 4 )
    pub m_proScreen_float2: f32,                           // 0x74 ( Size: 4 )
    pub m_reloadMilestone: i32,                            // 0x78 ( Size: 4 )
    pub m_fullReloadStartTime: f32,                        // 0x7c ( Size: 4 )
    pub m_scriptTime0: f32,                                // 0x80 ( Size: 4 )
    pub m_scriptFlags0: i32,                               // 0x84 ( Size: 4 )
    pub m_curZoomFOV: f32,                                 // 0x88 ( Size: 4 )
    pub m_targetZoomFOV: f32,                              // 0x8c ( Size: 4 )
    pub m_zoomFOVLerpTime: f32,                            // 0x90 ( Size: 4 )
    pub m_zoomFOVLerpEndTime: f32,                         // 0x94 ( Size: 4 )
    pub m_latestDryfireTime: f32,                          // 0x98 ( Size: 4 )
    pub m_lastRequestedAttackTime: f32,                    // 0x9c ( Size: 4 )
    pub m_currentAltFireAnimIndex: i32,                    // 0xa0 ( Size: 4 )
}

size_assert!(SIZE_WEAPON_DATA where WeaponPlayerData == 0xa8);

#[repr(C)]
pub struct CWeaponX {
    pub base: CBaseCombatWeapon,
    pub m_weapState: i32,     // 0xfc0 ( Size: 4 )
    pub m_allowedToUse: bool, // 0xfc4 ( Size: 1 )
    pub m_discarded: bool,    // 0xfc5 ( Size: 1 )
    pub gap_fc6: [u8; 2],
    pub m_forcedADS: i32,               // 0xfc8 ( Size: 4 )
    pub m_forceRelease: i32,            // 0xfcc ( Size: 4 )
    pub m_forceReleaseFromServer: bool, // 0xfd0 ( Size: 1 )
    pub gap_fd1: [u8; 3],
    pub m_customActivity: i32,          // 0xfd4 ( Size: 4 )
    pub m_customActivitySequence: i32,  // 0xfd8 ( Size: 4 )
    pub m_customActivityOwner: EHandle, // 0xfdc ( Size: 4 )
    pub m_customActivityEndTime: f32,   // 0xfe0 ( Size: 4 )
    pub gap_fe4: [u8; 4],
    pub m_playerData: WeaponPlayerData, // 0xfe8 ( Size: 168 )
    pub m_smartAmmoEnable: bool,        // 0x1090 ( Size: 1 )
    pub gap_1091: [u8; 7],
    pub m_smartAmmo: SmartAmmo_WeaponData, // 0x1098 ( Size: 496 )
    pub m_needsReloadCheck: bool,          // 0x1288 ( Size: 1 )
    pub m_needsCooldown: bool,             // 0x1289 ( Size: 1 )
    pub m_needsEmptyCycleCheck: bool,      // 0x128a ( Size: 1 )
    pub gap_128b: [u8; 1],
    pub m_skinOverride: i32,         // 0x128c ( Size: 4 )
    pub m_skinOverrideIsValid: bool, // 0x1290 ( Size: 1 )
    pub gap_1291: [u8; 3],
    pub m_chargeStartTime: f32,       // 0x1294 ( Size: 4 )
    pub m_chargeEndTime: f32,         // 0x1298 ( Size: 4 )
    pub m_lastChargeFrac: f32,        // 0x129c ( Size: 4 )
    pub m_lastRegenTime: f32,         // 0x12a0 ( Size: 4 )
    pub m_stockPileWasDraining: bool, // 0x12a4 ( Size: 1 )
    pub gap_12a5: [u8; 3],
    pub m_lastChargeLevel: i32,                // 0x12a8 ( Size: 4 )
    pub m_chargeEnergyDepleteStepCounter: i32, // 0x12ac ( Size: 4 )
    pub m_burstFireCount: i32,                 // 0x12b0 ( Size: 4 )
    pub m_burstFireIndex: i32,                 // 0x12b4 ( Size: 4 )
    pub m_shotCount: i32,                      // 0x12b8 ( Size: 4 )
    pub m_sustainedDischargeEndTime: f32,      // 0x12bc ( Size: 4 )
    pub m_modBitfieldFromPlayer: i32,          // 0x12c0 ( Size: 4 )
    pub m_modBitfieldInternal: i32,            // 0x12c4 ( Size: 4 )
    pub m_modBitfieldCurrent: i32,             // 0x12c8 ( Size: 4 )
    pub m_curSharedEnergyCost: i32,            // 0x12cc ( Size: 4 )
    pub m_scriptActivated: bool,               // 0x12d0 ( Size: 1 )
    pub m_isLoadoutPickup: bool,               // 0x12d1 ( Size: 1 )
    pub gap_12d2: [u8; 2],
    pub m_utilityEnt: EHandle,                     // 0x12d4 ( Size: 4 )
    pub m_weaponNameIndex: i32,                    // 0x12d8 ( Size: 4 )
    pub m_animModelIndexPredictingClientOnly: i32, // 0x12dc ( Size: 4 )
    pub m_animSequencePredictingClientOnly: i32,   // 0x12e0 ( Size: 4 )
    pub gap_12e4: [u8; 4],
    pub m_weaponScriptCB_: [u8; 296],  // 0x12e8 ( Size: 296 )
    pub m_modVars: [u8; 3232],         // 0x1410 ( Size: 3232 )
    pub m_tracerAttachment: [i32; 2],  // 0x20b0 ( Size: 8 )
    pub m_damageSourceIdentifier: i32, // 0x20b8 ( Size: 4 )
    pub m_activityModifierSymbolForNameIsSet: bool, // 0x20bc ( Size: 1 )
    pub gap_20bd: [u8; 1],
    pub m_activityModifierSymbolForName: [u8; 2], // 0x20be ( Size: 2 )
    pub m_hasAltAnim_adsIn: [bool; 3],            // 0x20c0 ( Size: 3 )
    pub m_hasAltAnim_adsOut: [bool; 3],           // 0x20c3 ( Size: 3 )
    pub m_hasAltAnim_idle: [bool; 3],             // 0x20c6 ( Size: 3 )
    pub m_hasAltAnim_attack: [bool; 3],           // 0x20c9 ( Size: 3 )
    pub m_hasAltAnim_oneHandedAdsIn: [bool; 3],   // 0x20cc ( Size: 3 )
    pub m_hasAltAnim_oneHandedAdsOut: [bool; 3],  // 0x20cf ( Size: 3 )
    pub m_hasAltAnim_oneHandedIdle: [bool; 3],    // 0x20d2 ( Size: 3 )
    pub m_hasAltAnim_oneHandedAttack: [bool; 3],  // 0x20d5 ( Size: 3 )
    pub m_loopSoundActive_1p: bool,               // 0x20d8 ( Size: 1 )
    pub m_loopSoundActive_3p: bool,               // 0x20d9 ( Size: 1 )
    pub m_loopSoundActive_3pAsNPC: bool,          // 0x20da ( Size: 1 )
    pub gap_20db: [u8; 1],
    pub m_loopSoundLastAttackClockTime: f32, // 0x20dc ( Size: 4 )
    pub m_loopSoundLastAttackClockTimeWithFireDelay: f32, // 0x20e0 ( Size: 4 )
    pub m_loopSoundCurrentParity: i32,       // 0x20e4 ( Size: 4 )
    pub m_loopSoundActiveParity_1p: i32,     // 0x20e8 ( Size: 4 )
    pub m_attackKickScale: f32,              // 0x20ec ( Size: 4 )
    pub m_attackKickRollScale: f32,          // 0x20f0 ( Size: 4 )
    pub m_prevViewModelWpnStr: i16,          // 0x20f4 ( Size: 2 )
    pub m_prevWorldModelWpnStr: i16,         // 0x20f6 ( Size: 2 )
    pub m_prevHolsterModelWpnStr: i16,       // 0x20f8 ( Size: 2 )
    pub gap_20fa: [u8; 2],
    pub newProjectiles: [EHandle; 8], // 0x20fc ( Size: 32 )
    pub newProjectileCount: i32,      // 0x211c ( Size: 4 )
    pub m_smartAmmoNextWeapon: *mut CBaseEntity, // 0x2120 ( Size: 8 )
    pub m_smartAmmoPrevWeapon: *mut CBaseEntity, // 0x2128 ( Size: 8 )
    pub m_npcUseCheckTime: f32,       // 0x2130 ( Size: 4 )
    pub m_npcUseCheckDist: f32,       // 0x2134 ( Size: 4 )
    pub m_sustainedDischargeNextPulseTime: f32, // 0x2138 ( Size: 4 )
    pub gap_213c: [u8; 4],            // TODO: this is different from the data map
}

size_assert!(SIZE_WEAPONX where CWeaponX == 0x2140);

impl DerefMut for CWeaponX {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CWeaponX {
    type Target = CBaseCombatWeapon;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
