//! titanfall 2's server related stuff

use std::ffi::c_void;

use crate::{
    bindings::{server, server::cbaseentity::CBaseEntity, DynamicCast},
    offset_functions,
};

macro_rules! impl_dynamic_cast {
    ( $($vtable_entry:ident = $class_ty:ty;)* ) => {
        $(
            impl DynamicCast<$class_ty> for CBaseEntity {
                fn dynamic_cast(&self) -> Option<&$class_ty> {
                    crate::mid::server::ENTITY_CLASS_VTABLE
                        .get()
                        .filter(|vtable| std::ptr::addr_eq(vtable.$vtable_entry, self.vftable))
                        .and_then(|_| unsafe { std::ptr::from_ref(self).cast::<$class_ty>().as_ref() })
                }

                fn dynamic_cast_mut(&mut self) -> Option<&mut $class_ty> {
                    crate::mid::server::ENTITY_CLASS_VTABLE
                        .get()
                        .filter(|vtable| std::ptr::addr_eq(vtable.$vtable_entry, self.vftable))
                        .and_then(|_| unsafe { std::ptr::from_mut(self).cast::<$class_ty>().as_mut() })
                }
            }
        )*
    };
}

offset_functions! {
   ENTITY_CLASS_VTABLE + EntityClassVtable for WhichDll::Server => {
        cplayer = *const c_void where offset(0x9524F8);
        weaponx = *const c_void where offset(0x98E2B8);
        cplayerdecoy = *const c_void where offset(0x8B4F58);
        cbaseanimating = *const c_void where offset(0x873EA8);
        baseanimatingoverlay = *const c_void where offset(0x878768);
        cbasecombatcharacter = *const c_void where offset(0x879678);
        cbasecombatweapon = *const c_void where offset(0x87EC58);
        cbaseentity = *const c_void where offset(0x924FC8);
        ctitan_soul = *const c_void where offset(0x97B008);

        cbase_helicopter = *const c_void where offset(0x96B748);
        cnpc_bullseye = *const c_void where offset(0x937BC8);
        cnpc_drone = *const c_void where offset(0x970F28);
        cnpc_dropship = *const c_void where offset(0x971FA8);
        cnpc_flyer = *const c_void where offset(0x9730F8);
        cnpc_gunship = *const c_void where offset(0x9740A8);
        cnpc_marvin = *const c_void where offset(0x975128);
        cnpc_melee_only = *const c_void where offset(0x9760B8);
        cnpc_sentry_turret = *const c_void where offset(0x97CE08);
        cnpc_soldier = *const c_void where offset(0x9770F8);
        cnpc_titan = *const c_void where offset(0x97A0A8);

        cai_assault_behavior = *const c_void where offset(0x90D9A8);
        cai_base_actor = *const c_void where offset(0x90F7D8);
        cai_base_npc = *const c_void where offset(0x8FF138);
        cai_baseflyingbot = *const c_void where offset(0x90A408);
        cai_behaviorbase = *const c_void where offset(0x90CB68);
        cai_changetarget = *const c_void where offset(0x8E4B98);
        cai_combatant = *const c_void where offset(0x910728);
        cai_component = *const c_void where offset(0x8FEFC8);
        cai_dynamiclink = *const c_void where offset(0x913C78);
        cai_dynamiclinkcontroller = *const c_void where offset(0x914428);
        cai_enemies = *const c_void where offset(0x918BF0);
        cai_followbehavior = *const c_void where offset(0x90E028);
        cai_hint = *const c_void where offset(0x915A78);
        cai_local_navigator = *const c_void where offset(0x918A58);
        cai_motor = *const c_void where offset(0x90F638);
        cai_move_probe = *const c_void where offset(0x919190);
        cai_moveandshootoverlay = *const c_void where offset(0x919388);
        cai_navigator = *const c_void where offset(0x919688);
        cai_networkmanager = *const c_void where offset(0x91A768);
        cai_path = *const c_void where offset(0x91CC18);
        cai_pathfinder = *const c_void where offset(0x900088);
        cai_patrol_behavior = *const c_void where offset(0x90EE18);
        cai_radial_link_controller = *const c_void where offset(0x914BD8);
        cai_search_behavior = *const c_void where offset(0x90EBD8);
        cai_senses = *const c_void where offset(0x91F808);
        cai_skit_node = *const c_void where offset(0x9222A8);
        cai_tacticalservicess = *const c_void where offset(0x9000A8);
        cai_testhull = *const c_void where offset(0x917698);
        cai_track_pather = *const c_void where offset(0x9688C8);
   }
}

impl_dynamic_cast! {
        cplayer = server::cplayer::CPlayer;
        weaponx = server::cweaponx::CWeaponX;
        cplayerdecoy = server::cplayerdecoy::CPlayerDecoy;
        cbaseanimating = server::cbaseanimating::CBaseAnimating;
        baseanimatingoverlay = server::cbaseanimatingoverlay::CBaseAnimatingOverlay;
        cbasecombatcharacter = server::cbasecombatcharacter::CBaseCombatCharacter;
        cbasecombatweapon = server::cbasecombatweapon::CBaseCombatWeapon;
        cbaseentity = server::cbaseentity::CBaseEntity;
        ctitan_soul = server::ctitan_soul::CTitanSoul;

        cbase_helicopter = server::cbase_helicopter::CBaseHelicopter;
        cnpc_bullseye = server::cnpc_bullseye::CNPC_Bullseye;
        cnpc_drone = server::cnpc_drone::CNPC_Drone;
        cnpc_dropship = server::cnpc_dropship::CNPC_Dropship;
        cnpc_flyer = server::cnpc_flyer::CNPC_Flyer;
        cnpc_gunship = server::cnpc_gunship::CNPC_Gunship;
        cnpc_marvin = server::cnpc_marvin::CNPC_Marvin;
        cnpc_melee_only = server::cnpc_melee_only::CNPC_MeleeOnly;
        cnpc_sentry_turret = server::cnpc_sentry_turret::CNPC_SentryTurret;
        cnpc_soldier = server::cnpc_soldier::CNPC_Soldier;
        cnpc_titan = server::cnpc_titan::CNPC_Titan;

        cai_assault_behavior = server::cai_assault_behavior::CAI_AssaultBehavior;
        cai_base_actor = server::cai_base_actor::CAI_BaseActor;
        cai_base_npc = server::cai_base_npc::CAI_BaseNPC;
        cai_baseflyingbot = server::cai_baseflyingbot::CAI_BaseFlyingBot;
        cai_behaviorbase = server::cai_behaviorbase::CAI_BehaviorBase;
        cai_changetarget = server::cai_changetarget::CAI_ChangeTarget;
        cai_combatant = server::cai_combatant::CAI_Combatant;
        cai_component = server::cai_component::CAI_Component;
        cai_dynamiclink = server::cai_dynamiclink::CAI_DynamicLink;
        cai_dynamiclinkcontroller = server::cai_dynamiclinkcontroller::CAI_DynamicLinkController;
        cai_enemies = server::cai_enemies::CAI_Enemies;
        cai_followbehavior = server::cai_followbehavior::CAI_FollowBehavior;
        cai_hint = server::cai_hint::CAI_Hint;
        cai_local_navigator = server::cai_local_navigator::CAI_LocalNavigator;
        cai_motor = server::cai_motor::CAI_Motor;
        cai_move_probe = server::cai_move_probe::CAI_MoveProbe ;
        cai_moveandshootoverlay = server::cai_moveandshootoverlay::CAI_MoveAndShootOverlay;
        cai_navigator = server::cai_navigator::CAI_Navigator;
        cai_networkmanager = server::cai_networkmanager::CAI_NetworkManager;
        cai_path = server::cai_path::CAI_Path;
        cai_pathfinder = server::cai_pathfinder::CAI_Pathfinder;
        cai_patrol_behavior = server::cai_patrol_behavior::CAI_PatrolBehavior;
        cai_radial_link_controller = server::cai_radial_link_controller::CAI_RadialLinkController;
        cai_search_behavior = server::cai_search_behavior::CAI_SearchBehavior;
        cai_senses = server::cai_senses::CAI_Senses;
        cai_skit_node = server::cai_skit_node::CAI_SkitNode;
        cai_tacticalservicess = server::cai_tacticalservicess::CAI_TacticalServices;
        cai_testhull = server::cai_testhull::CAI_TestHull;
        cai_track_pather = server::cai_track_pather::CAI_TrackPather;
}
