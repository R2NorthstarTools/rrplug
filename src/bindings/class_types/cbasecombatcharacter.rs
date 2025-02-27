#![allow(non_camel_case_types, non_snake_case)]
use std::{
    ffi::{c_char, c_void},
    ops::{Deref, DerefMut},
};

use super::{cbaseanimatingoverlay::CBaseAnimatingOverlay, cplayer::EHandle};
use crate::{high::vector::Vector3, size_assert};

#[repr(C)]
pub struct WeaponDropInfo {
    weaponPosition: Vector3,      // 0x0 ( Size = 12 )
    prevDropFrameCounter: c_char, // 0xc ( Size = 1 )
    dropFrameCounter: c_char,     // 0xd ( Size = 1 )
    gap_e: [c_char; 2],
    weaponAngles: Vector3,   // 0x10 ( Size = 12 )
    weaponPositionTime: f32, // 0x1c ( Size = 4 )
}
size_assert!(SIZE_DROP where WeaponDropInfo == 32);

#[repr(C)]
pub struct WeaponInventory {
    vftable: *const c_void,
    weapons: [EHandle; 4],        // 0x8 ( Size = 16 )
    activeWeapon: EHandle,        // 0x18 ( Size = 4 )
    offhandWeapons: [EHandle; 6], // 0x1c ( Size = 24 )
}
size_assert!(SIZE_WEAPON where WeaponInventory == 56);

#[repr(C)]
pub struct CTether {
    vftable: *const c_void,
    pos: Vector3,       // 0x8 ( Size = 12 )
    health: f32,        // 0x14 ( Size = 4 )
    nextSoundTime: f32, // 0x18 ( Size = 4 )
    creationTime: f32,  // 0x1c ( Size = 4 )
    scriptID: i32,      // 0x20 ( Size = 4 )
}
size_assert!(SIZE_TETHER where CTether == 40);

#[repr(C)]
pub struct CBaseCombatCharacter {
    base: CBaseAnimatingOverlay,
    m_bPreventWeaponPickup: bool, // 0x1210 ( Size = 1 )
    gap_1211: [c_char; 3],
    m_phaseShiftTimeStart: f32,         // 0x1214 ( Size = 4 )
    m_phaseShiftTimeEnd: f32,           // 0x1218 ( Size = 4 )
    m_flNextAttack: f32,                // 0x121c ( Size = 4 )
    m_lastFiredTime: f32,               // 0x1220 ( Size = 4 )
    m_raiseFromMeleeEndTime: f32,       // 0x1224 ( Size = 4 )
    m_sharedEnergyCount: i32,           // 0x1228 ( Size = 4 )
    m_sharedEnergyTotal: i32,           // 0x122c ( Size = 4 )
    m_lastSharedEnergyRegenTime: f32,   // 0x1230 ( Size = 4 )
    m_sharedEnergyRegenRate: f32,       // 0x1234 ( Size = 4 )
    m_sharedEnergyRegenDelay: f32,      // 0x1238 ( Size = 4 )
    m_lastSharedEnergyTakeTime: f32,    // 0x123c ( Size = 4 )
    m_eHull: i32,                       // 0x1240 ( Size = 4 )
    m_fieldOfViewCos: f32,              // 0x1244 ( Size = 4 )
    m_HackedGunPos: Vector3,            // 0x1248 ( Size = 12 )
    m_impactEnergyScale: f32,           // 0x1254 ( Size = 4 )
    m_weaponDropInfo: WeaponDropInfo,   // 0x1258 ( Size = 32 )
    m_physicsMass: f32,                 // 0x1278 ( Size = 4 )
    m_flDamageAccumulator: f32,         // 0x127c ( Size = 4 )
    m_prevHealth: i32,                  // 0x1280 ( Size = 4 )
    m_healthChangeRate: f32,            // 0x1284 ( Size = 4 )
    m_healthChangeAmount: f32,          // 0x1288 ( Size = 4 )
    m_healthChangeStartTime: f32,       // 0x128c ( Size = 4 )
    m_lastHealthChangeTime: f32,        // 0x1290 ( Size = 4 )
    m_lastHitGroup: i32,                // 0x1294 ( Size = 4 )
    m_lastDamageDir: Vector3,           // 0x1298 ( Size = 12 )
    m_lastDamageForce: Vector3,         // 0x12a4 ( Size = 12 )
    m_deathPackage: i32,                // 0x12b0 ( Size = 4 )
    m_deathDirection2DInverse: Vector3, // 0x12b4 ( Size = 12 )
    m_CurrentWeaponProficiency: i32,    // 0x12c0 ( Size = 4 )
    m_flEnemyAccurcyMultiplier: f32,    // 0x12c4 ( Size = 4 )
    m_npcPriorityOverride: i32,         // 0x12c8 ( Size = 4 )
    m_healthPerSegment: i32,            // 0x12cc ( Size = 4 )
    m_hTriggerFogList: [c_char; 32],    // 0x12d0 ( Size = 32 ) // custom
    m_hLastFogTrigger: EHandle,         // 0x12f0 ( Size = 4 )
    gap_12f4: [c_char; 4],
    m_inventory: WeaponInventory,          // 0x12f8 ( Size = 56 )
    m_selectedWeapon: EHandle,             // 0x1330 ( Size = 4 )
    m_latestPrimaryWeapon: EHandle,        // 0x1334 ( Size = 4 )
    m_latestNonOffhandWeapon: EHandle,     // 0x1338 ( Size = 4 )
    m_lastCycleSlot: i32,                  // 0x133c ( Size = 4 )
    m_removeWeaponOnSelectSwitch: EHandle, // 0x1340 ( Size = 4 )
    m_weaponGettingSwitchedOut: EHandle,   // 0x1344 ( Size = 4 )
    m_showNewWeapon3p: bool,               // 0x1348 ( Size = 1 )
    gap_1349: [c_char; 3],
    m_weaponPermission: i32,                    // 0x134c ( Size = 4 )
    m_weaponDisabled: bool,                     // 0x1350 ( Size = 1 )
    m_hudInfo_visibilityTestAlwaysPasses: bool, // 0x1351 ( Size = 1 )
    gap_1352: [c_char; 2],
    m_selectedOffhand: EHandle,                // 0x1354 ( Size = 4 )
    m_selectedOffhandPendingHybridAction: i32, // 0x1358 ( Size = 4 )
    m_doOffhandAnim: bool,                     // 0x135c ( Size = 1 )
    m_wantInventoryChangedScriptCall: bool,    // 0x135d ( Size = 1 )
    m_doInventoryChangedScriptCall: bool,      // 0x135e ( Size = 1 )
    gap_135f: [c_char; 1],
    m_cloakReactEndTime: f32, // 0x1360 ( Size = 4 )
    gap_1364: [c_char; 4],
    m_tethers: [CTether; 2],                  // 0x1368 ( Size = 40 )
    m_titanSoul: EHandle,                     // 0x13b8 ( Size = 4 )
    m_lastFootstepDamagePos: Vector3,         // 0x13bc ( Size = 12 )
    m_muzzleAttachment: [i32; 2],             // 0x13c8 ( Size = 8 )
    m_prevNearestNode: i32,                   // 0x13d0 ( Size = 4 )
    m_nearestNode: i32,                       // 0x13d4 ( Size = 4 )
    m_nearestNodeCheckTime: f32,              // 0x13d8 ( Size = 4 )
    m_nearestNodeCheckPos: Vector3,           // 0x13dc ( Size = 12 )
    m_nearestPolyRef: [i32; 4],               // 0x13e8 ( Size = 16 )
    m_nearestPolyCheckPos: [Vector3; 4],      // 0x13f8 ( Size = 48 )
    m_meleeExecutionUnstuckPosition: Vector3, // 0x1428 ( Size = 12 )
    m_meleeExecutionEntityBlocker: EHandle,   // 0x1434 ( Size = 4 )
    m_contextAction: i32,                     // 0x1438 ( Size = 4 )
    m_targetInfoIconName: [c_char; 64],       // 0x143c ( Size = 64 )
    m_rodeoRiders: [EHandle; 5],              // 0x147c ( Size = 20 )
    m_numRodeoSlots: i32,                     // 0x1490 ( Size = 4 )
}
size_assert!(SIZE_COMBAT_CHAR where CBaseCombatCharacter == 0x1498);

impl DerefMut for CBaseCombatCharacter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CBaseCombatCharacter {
    type Target = CBaseAnimatingOverlay;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
