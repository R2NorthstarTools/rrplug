#![allow(non_camel_case_types, non_snake_case)]
use std::ffi::{c_char, c_void};

use super::{
    cplayer::{CPlayer, EHandle},
    cweaponx::CWeaponX,
};
use crate::{
    bindings::{cvar::convar::Color, DynamicCast},
    impl_vmethods,
    prelude::Vector3,
    size_assert,
};

#[repr(C)]
pub struct IServerNetworkable {
    vftable: *const c_void,
}

type edict_t = u16;

#[repr(C)]
pub struct CServerNetworkProperty {
    pub base: IServerNetworkable,
    pub m_pOuter: *mut CBaseEntity,
    pub m_pServerClass: *mut c_void,
    pub m_edict: edict_t,
    pub m_hParent: EHandle,
    pub unk: c_char,
}
size_assert!(SIZE_NET_PROP where CServerNetworkProperty == 40);

#[repr(C)]
pub struct CCollisionProperty {
    pub vftable: *const c_void,
    pub m_pOuter: *mut CBaseEntity, // 0x8 ( Size = 8 )
    pub m_vecMins: Vector3,         // 0x10 ( Size = 12 )
    pub m_vecMaxs: Vector3,         // 0x1c ( Size = 12 )
    pub m_usSolidFlags: i32,        // 0x28 ( Size = 4 )
    pub m_nSolidType: c_char,       // 0x2c ( Size = 1 )
    pub m_triggerBloat: c_char,     // 0x2d ( Size = 1 )
    pub gap_2e: [c_char; 2],
    pub m_flRadius: f32,                        // 0x30 ( Size = 4 )
    pub m_PartitionHandle: i16,                 // 0x34 ( Size = 2 )
    pub m_nSurroundType: c_char,                // 0x36 ( Size = 1 )
    pub m_bRemovedFromPartition: bool,          // 0x37 ( Size = 1 )
    pub m_spatialPartitionFlags: i32,           // 0x38 ( Size = 4 )
    pub m_vecSpecifiedSurroundingMins: Vector3, // 0x3c ( Size = 12 )
    pub m_vecSpecifiedSurroundingMaxs: Vector3, // 0x48 ( Size = 12 )
    pub m_vecSurroundingMins: Vector3,          // 0x54 ( Size = 12 )
    pub m_vecSurroundingMaxs: Vector3,          // 0x60 ( Size = 12 )
    pub m_hitboxTestRadius: f32,                // 0x6c ( Size = 4 )
}
size_assert!(SIZE_COLLISON_PROP where CCollisionProperty == 112);

#[repr(C)]
pub struct CBaseEntity {
    pub vftable: *const c_void,
    pub m_RefEHandle: EHandle, // 0x8 ( Size = 4 ) // handle
    pub gap_c: [c_char; 4],
    pub m_collideable: *mut c_void, // 0x10 ( Size = 8 )
    pub m_networkable: *mut c_void, // 0x18 ( Size = 8 )
    pub genericKeyValueCount: i32,  // 0x20 ( Size = 4 )
    pub gap_24: [c_char; 4],
    pub genericKeyValues: *mut c_void,     // 0x28 ( Size = 8 )
    pub m_pfnMoveDone: *mut c_void,        // 0x30 ( Size = 8 )
    pub m_pfnThink: *mut c_void,           // 0x38 ( Size = 8 )
    pub m_Network: CServerNetworkProperty, // 0x40 ( Size = 40 )
    pub m_entIndex: i32,                   // 0x68 ( Size = 4 )
    pub gap_6c: [c_char; 4],
    pub m_iClassname: i64,               // 0x70 ( Size = 8 )
    pub m_flAnimTime: f32,               // 0x78 ( Size = 4 )
    pub m_flSimulationTime: f32,         // 0x7c ( Size = 4 )
    pub m_creationTick: i32,             // 0x80 ( Size = 4 )
    pub m_nLastThinkTick: i32,           // 0x84 ( Size = 4 )
    pub m_PredictableID: i32,            // 0x88 ( Size = 4 )
    pub touchStamp: i32,                 // 0x8c ( Size = 4 )
    pub m_aThinkFunctions: [c_char; 32], // 0x90 ( Size = 32 ) CUtlVector /*CUtlVector*/
    pub m_entitySpawnTime: f32,          // 0xb0 ( Size = 4 )
    pub m_spawner: EHandle,              // 0xb4 ( Size = 4 )
    pub m_scriptClass: i32,              // 0xb8 ( Size = 4 )
    pub m_wantsDamageCallbacks: bool,    // 0xbc ( Size = 1 )
    pub m_wantsDeathCallbacks: bool,     // 0xbd ( Size = 1 )
    pub gap_be: [c_char; 2],
    pub m_nNextThinkTick: i32,                      // 0xc0 ( Size = 4 )
    pub m_fEffects: i32,                            // 0xc4 ( Size = 4 )
    pub m_ModelName: i64,                           // 0xc8 ( Size = 8 )
    pub m_target: i64,                              // 0xd0 ( Size = 8 )
    pub m_networkedFlags: i32,                      // 0xd8 ( Size = 4 )
    pub m_nRenderFX: c_char,                        // 0xdc ( Size = 1 )
    pub m_nRenderMode: c_char,                      // 0xdd ( Size = 1 )
    pub m_nModelIndex: i16,                         // 0xde ( Size = 2 )
    pub m_clrRender: Color,                         // 0xe0 ( Size = 4 )
    pub m_desiredHibernationType: i32,              // 0xe4 ( Size = 4 )
    pub m_scriptMinHibernationType: i32,            // 0xe8 ( Size = 4 )
    pub m_minSelfAndDescendantHibernationType: i32, // 0xec ( Size = 4 )
    pub m_actualHibernationType: i32,               // 0xf0 ( Size = 4 )
    pub m_hibernationQueueIndex: i32,               // 0xf4 ( Size = 4 )
    pub m_bRenderWithViewModels: bool,              // 0xf8 ( Size = 1 )
    pub gap_f9: [c_char; 3],
    pub m_nameVisibilityFlags: i32,   // 0xfc ( Size = 4 )
    pub m_cloakEndTime: f32,          // 0x100 ( Size = 4 )
    pub m_cloakFadeInEndTime: f32,    // 0x104 ( Size = 4 )
    pub m_cloakFadeOutStartTime: f32, // 0x108 ( Size = 4 )
    pub m_cloakFadeInDuration: f32,   // 0x10c ( Size = 4 )
    pub m_cloakFlickerAmount: f32,    // 0x110 ( Size = 4 )
    pub m_cloakFlickerEndTime: f32,   // 0x114 ( Size = 4 )
    pub m_cloakFadeOutDuration: f32,  // 0x118 ( Size = 4 )
    pub m_highlightIsNetworked: bool, // 0x11c ( Size = 1 )
    pub gap_11d: [c_char; 3],
    pub m_highlightParams: [Vector3; 16], // 0x120 ( Size = 192 )
    pub m_highlightFunctionBits: [i32; 8], // 0x1e0 ( Size = 32 )
    pub m_highlightPlayerVisibilityBits: [i32; 8], // 0x200 ( Size = 32 )
    pub m_highlightServerFadeBases: [f32; 2], // 0x220 ( Size = 8 )
    pub m_highlightServerFadeStartTimes: [f32; 2], // 0x228 ( Size = 8 )
    pub m_highlightServerFadeEndTimes: [f32; 2], // 0x230 ( Size = 8 )
    pub m_highlightServerContextID: i32,  // 0x238 ( Size = 4 )
    pub m_highlightTeamBits: i32,         // 0x23c ( Size = 4 )
    pub m_nextGrenadeTargetTime: f32,     // 0x240 ( Size = 4 )
    pub m_grenadeTargetDebounce: f32,     // 0x244 ( Size = 4 )
    pub m_nSimulationTick: i32,           // 0x248 ( Size = 4 )
    pub m_fDataObjectTypes: i32,          // 0x24c ( Size = 4 )
    pub m_nextVelocitySample: f32,        // 0x250 ( Size = 4 )
    pub m_velocitySamples: [Vector3; 5],  // 0x254 ( Size = 60 )
    pub m_iEFlags: i64,                   // 0x290 ( Size = 8 )
    pub m_fFlags: i32,                    // 0x298 ( Size = 4 )
    pub gap_29c: [c_char; 4],
    pub m_iName: i64,                         // 0x2a0 ( Size = 8 )
    pub m_scriptNameIndex: i32,               // 0x2a8 ( Size = 4 )
    pub m_instanceNameIndex: i32,             // 0x2ac ( Size = 4 )
    pub m_scriptName: [c_char; 64],           // 0x2b0 ( Size = 64 )
    pub m_instanceName: [c_char; 64],         // 0x2f0 ( Size = 64 )
    pub m_holdUsePrompt: i64,                 // 0x330 ( Size = 8 )
    pub m_pressUsePrompt: i64,                // 0x338 ( Size = 8 )
    pub m_attachmentLerpStartTime: f32,       // 0x340 ( Size = 4 )
    pub m_attachmentLerpEndTime: f32,         // 0x344 ( Size = 4 )
    pub m_attachmentLerpStartOrigin: Vector3, // 0x348 ( Size = 12 )
    pub m_attachmentLerpStartAngles: Vector3, // 0x354 ( Size = 12 )
    pub m_parentAttachmentType: i32,          // 0x360 ( Size = 4 )
    pub m_parentAttachmentIndex: i32,         // 0x364 ( Size = 4 )
    pub m_parentAttachmentHitbox: i32,        // 0x368 ( Size = 4 )
    pub m_parentAttachmentModel: i32,         // 0x36c ( Size = 4 )
    pub m_MoveType: c_char,                   // 0x370 ( Size = 1 )
    pub m_MoveCollide: c_char,                // 0x371 ( Size = 1 )
    pub gap_372: [c_char; 2],
    pub m_RestoreMoveTypeOnDetach: i32, // 0x374 ( Size = 4 )
    pub m_hMoveParent: EHandle,         // 0x378 ( Size = 4 )
    pub m_hMoveChild: EHandle,          // 0x37c ( Size = 4 )
    pub m_hMovePeer: EHandle,           // 0x380 ( Size = 4 )
    pub m_bIsActiveChild: bool,         // 0x384 ( Size = 1 )
    pub m_bPrevAbsOriginValid: bool,    // 0x385 ( Size = 1 )
    pub gap_386: [c_char; 2],
    pub m_Collision: CCollisionProperty,  // 0x388 ( Size = 112 )
    pub m_hOwnerEntity: EHandle,          // 0x3f8 ( Size = 4 )
    pub m_CollisionGroup: i32,            // 0x3fc ( Size = 4 )
    pub m_contents: i32,                  // 0x400 ( Size = 4 )
    pub m_baseSolidType: i32,             // 0x404 ( Size = 4 )
    pub m_pPhysicsObject: *mut c_void,    // 0x408 ( Size = 8 )
    pub m_flNavIgnoreUntilTime: f32,      // 0x410 ( Size = 4 )
    pub m_hGroundEntity: EHandle,         // 0x414 ( Size = 4 )
    pub m_flGroundChangeTime: f32,        // 0x418 ( Size = 4 )
    pub m_vecBaseVelocity: Vector3,       // 0x41c ( Size = 12 )
    pub m_vecAbsVelocity: Vector3,        // 0x428 ( Size = 12 )
    pub m_vecAngVelocity: Vector3,        // 0x434 ( Size = 12 )
    pub m_rgflCoordinateFrame: [f32; 12], // 0x440 ( Size = 48 )
    pub m_flFriction: f32,                // 0x470 ( Size = 4 )
    pub m_flLocalTime: f32,               // 0x474 ( Size = 4 )
    pub m_flVPhysicsUpdateLocalTime: f32, // 0x478 ( Size = 4 )
    pub m_flMoveDoneTime: f32,            // 0x47c ( Size = 4 )
    pub m_nPushEnumCount: i32,            // 0x480 ( Size = 4 )
    pub m_vecPrevAbsOrigin: Vector3,      // 0x484 ( Size = 12 )
    pub m_vecAbsOrigin: Vector3,          // 0x490 ( Size = 12 )
    pub m_angAbsRotation: Vector3,        // 0x49c ( Size = 12 )
    pub m_vecVelocity: Vector3,           // 0x4a8 ( Size = 12 )
    pub m_pBlocker: EHandle,              // 0x4b4 ( Size = 4 )
    pub m_iGlobalname: i64,               // 0x4b8 ( Size = 8 )
    pub m_iParent: i64,                   // 0x4c0 ( Size = 8 )
    pub m_iHammerID: i32,                 // 0x4c8 ( Size = 4 )
    pub m_flSpeed: f32,                   // 0x4cc ( Size = 4 )
    pub m_iMaxHealth: i32,                // 0x4d0 ( Size = 4 )
    pub m_iHealth: i32,                   // 0x4d4 ( Size = 4 )
    pub m_pfnTouch: *mut c_void,          // 0x4d8 ( Size = 8 )
    pub m_pfnUse: *mut c_void,            // 0x4e0 ( Size = 8 )
    pub m_pfnBlocked: *mut c_void,        // 0x4e8 ( Size = 8 )
    pub m_bClientSideRagdoll: bool,       // 0x4f0 ( Size = 1 )
    pub m_lifeState: c_char,              // 0x4f1 ( Size = 1 )
    pub m_forceVisibleInPhaseShift: bool, // 0x4f2 ( Size = 1 )
    pub m_baseTakeDamage: c_char,         // 0x4f3 ( Size = 1 )
    pub m_invulnerableToDamageCount: i32, // 0x4f4 ( Size = 4 )
    pub m_passDamageToParent: c_char,     // 0x4f8 ( Size = 1 )
    pub gap_4f9: [c_char; 3],
    pub m_deathVelocity: Vector3,           // 0x4fc ( Size = 12 )
    pub m_lastTitanFootstepDamageTime: f32, // 0x508 ( Size = 4 )
    pub m_flMaxspeed: f32,                  // 0x50c ( Size = 4 )
    pub m_visibilityFlags: i32,             // 0x510 ( Size = 4 )
    pub m_scriptVisible: bool,              // 0x514 ( Size = 1 )
    pub gap_515: [c_char; 3],
    pub m_OnUser1: [c_char; 40],   // 0x518 ( Size = 40 ) // custom
    pub m_OnDeath: [c_char; 40],   // 0x540 ( Size = 40 ) // custom
    pub m_OnDestroy: [c_char; 40], // 0x568 ( Size = 40 ) // custom
    pub m_cellwidth: i32,          // 0x590 ( Size = 4 )
    pub m_cellbits: i32,           // 0x594 ( Size = 4 )
    pub m_cellX: i32,              // 0x598 ( Size = 4 )
    pub m_cellY: i32,              // 0x59c ( Size = 4 )
    pub m_cellZ: i32,              // 0x5a0 ( Size = 4 )
    pub m_localOrigin: Vector3,    // 0x5a4 ( Size = 12 )
    pub m_localAngles: Vector3,    // 0x5b0 ( Size = 12 )
    pub m_vecViewOffset: Vector3,  // 0x5bc ( Size = 12 )
    pub m_ListByClass: i32,        // 0x5c8 ( Size = 4 )
    pub gap_5cc: [c_char; 4],
    pub m_pPrevByClass: *mut c_void,           // 0x5d0 ( Size = 8 )
    pub m_pNextByClass: *mut c_void,           // 0x5d8 ( Size = 8 )
    pub m_iInitialTeamNum: i32,                // 0x5e0 ( Size = 4 )
    pub m_iTeamNum: i32,                       // 0x5e4 ( Size = 4 )
    pub m_passThroughFlags: i32,               // 0x5e8 ( Size = 4 )
    pub m_passThroughThickness: i32,           // 0x5ec ( Size = 4 )
    pub m_passThroughDirection: f32,           // 0x5f0 ( Size = 4 )
    pub m_spawnflags: i32,                     // 0x5f4 ( Size = 4 )
    pub m_AIAddOn: i64,                        // 0x5f8 ( Size = 8 )
    pub m_flGravity: f32,                      // 0x600 ( Size = 4 )
    pub m_entityFadeDist: f32,                 // 0x604 ( Size = 4 )
    pub m_dissolveEffectEntityHandle: EHandle, // 0x608 ( Size = 4 )
    pub m_fadeDist: f32,                       // 0x60c ( Size = 4 )
    pub m_iSignifierName: i64,                 // 0x610 ( Size = 8 )
    pub m_collectedInvalidateFlags: i32,       // 0x618 ( Size = 4 )
    pub m_collectingInvalidateFlags: bool,     // 0x61c ( Size = 1 )
    pub gap_61d: [c_char; 3],
    pub m_lagCompensationCounter: i32, // 0x620 ( Size = 4 )
    pub m_bLagCompensate: bool,        // 0x624 ( Size = 1 )
    pub m_bNetworkQuantizeOriginAndAngles: bool, // 0x625 ( Size = 1 )
    pub m_bForcePurgeFixedupStrings: bool, // 0x626 ( Size = 1 )
    pub gap_627: [c_char; 1],
    pub m_pEvent: *mut c_void, // 0x628 ( Size = 8 )
    pub m_debugOverlays: i32,  // 0x630 ( Size = 4 )
    pub gap_634: [c_char; 4],
    pub m_pTimedOverlay: *mut c_void,    // 0x638 ( Size = 8 )
    pub m_ScriptScope: [c_char; 32],     // 0x640 ( Size = 32 ) // void
    pub m_hScriptInstance: i64,          // 0x660 ( Size = 8 ) // void
    pub m_iszScriptId: i64,              // 0x668 ( Size = 8 )
    pub m_bossPlayer: EHandle,           // 0x670 ( Size = 4 )
    pub m_usableType: i32,               // 0x674 ( Size = 4 )
    pub m_usablePriority: i32,           // 0x678 ( Size = 4 )
    pub m_usableRadius: f32,             // 0x67c ( Size = 4 )
    pub m_usableFOV: f32,                // 0x680 ( Size = 4 )
    pub m_usePromptSize: f32,            // 0x684 ( Size = 4 )
    pub m_hasDispatchedSpawn: bool,      // 0x688 ( Size = 1 )
    pub m_bDoDestroyCallback: bool,      // 0x689 ( Size = 1 )
    pub m_bDoPreSpawnCallback: bool,     // 0x68a ( Size = 1 )
    pub m_bDoOnSpawnedCallback: bool,    // 0x68b ( Size = 1 )
    pub m_spottedBeginTimes: [f32; 31],  // 0x68c ( Size = 124 )
    pub m_spottedLatestTimes: [f32; 31], // 0x708 ( Size = 124 )
    pub m_spottedByTeams: i32,           // 0x784 ( Size = 4 )
    pub m_minimapData: [c_char; 40],     // 0x788 ( Size = 40 ) // void
    pub m_shieldHealth: i32,             // 0x7b0 ( Size = 4 )
    pub m_shieldHealthMax: i32,          // 0x7b4 ( Size = 4 )
    pub m_areEntityLinksNetworked: bool, // 0x7b8 ( Size = 1 )
    pub gap_7b9: [c_char; 3],
    pub m_entitiesLinkedFromMeCount: i32, // 0x7bc ( Size = 4 )
    pub m_entitiesLinkedToMeCount: i32,   // 0x7c0 ( Size = 4 )
    pub m_entitiesLinkedFromMe: [EHandle; 64], // 0x7c4 ( Size = 256 )
    pub m_entitiesLinkedToMe: [EHandle; 64], // 0x8c4 ( Size = 256 )
    pub m_pusherWithChildrenRadius: f32,  // 0x9c4 ( Size = 4 )
    pub m_childPusherMoveHandlerCount: i32, // 0x9c8 ( Size = 4 )
    pub gap_9cc: [c_char; 4],
    pub m_statusEffectPlugin: *mut CBaseEntity, // 0x9d0 ( Size = 8 )
    pub gap_9d8: [c_char; 1],
    pub m_physDummyMotionEnabled: bool, // 0x9d9 ( Size = 1 )
}
size_assert!(SIZE_BASE where CBaseEntity == 0x9E0);

// recheck this
impl_vmethods! {
    impl CBaseEntity {
        pub fn some_get_origin_varient_02(vector: *mut Vector3) -> *mut Vector3 where offset(133);
        pub fn some_get_origin_varient_01(vector: *mut Vector3) -> *mut Vector3 where offset(134);
        pub fn eye_angles(vector: *mut Vector3) -> *mut Vector3 where offset(135);
        pub fn get_angles(vector: *mut Vector3) -> *mut Vector3 where offset(136);
        pub fn get_eye_position(vector: *mut Vector3) -> *mut Vector3 where offset(137);
        pub fn get_center_position(vector: *mut Vector3) -> *mut Vector3 where offset(138);
        pub fn get_origin(vector: *mut Vector3) -> *mut Vector3 where offset(139);
        pub fn get_forward_vector(vector: *mut Vector3, unk1: *const c_void, unk2: *const c_void) -> () where offset(140);
    }
}

impl DynamicCast<CPlayer> for CBaseEntity {
    fn dynamic_cast(&self) -> Option<&CPlayer> {
        crate::mid::server::ENTITY_CLASS_VTABLE
            .get()
            .filter(|vtable| std::ptr::addr_eq(vtable.cplayer, self.vftable))
            .and_then(|_| unsafe { std::ptr::from_ref(self).cast::<CPlayer>().as_ref() })
    }

    fn dynamic_cast_mut(&mut self) -> Option<&mut CPlayer> {
        crate::mid::server::ENTITY_CLASS_VTABLE
            .get()
            .filter(|vtable| std::ptr::addr_eq(vtable.cplayer, self.vftable))
            .and_then(|_| unsafe { std::ptr::from_mut(self).cast::<CPlayer>().as_mut() })
    }
}

impl DynamicCast<CWeaponX> for CBaseEntity {
    fn dynamic_cast(&self) -> Option<&CWeaponX> {
        crate::mid::server::ENTITY_CLASS_VTABLE
            .get()
            .filter(|vtable| std::ptr::addr_eq(vtable.weaponx, self.vftable))
            .and_then(|_| unsafe { std::ptr::from_ref(self).cast::<CWeaponX>().as_ref() })
    }

    fn dynamic_cast_mut(&mut self) -> Option<&mut CWeaponX> {
        crate::mid::server::ENTITY_CLASS_VTABLE
            .get()
            .filter(|vtable| std::ptr::addr_eq(vtable.weaponx, self.vftable))
            .and_then(|_| unsafe { std::ptr::from_mut(self).cast::<CWeaponX>().as_mut() })
    }
}
