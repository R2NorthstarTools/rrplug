use std::ffi::{c_char, c_void};

use crate::{high::vector::Vector3, offset_struct};

// todo: structure this in a better way

// thx clippy; had to rename evrything manually >:(
offset_struct! {
    pub struct CBasePlayer {
        vtable: *const CBasePlayerVtable where offset(0x0),
        player_index: u32 where offset(0x58),
        grapple_active: bool where offset(0x23E8),
        platform_user_id: u32 where offset(0x1D08),
        class_mods_active: i32 where offset(0x1D10),
        pos_class_mods_active: i32 where offset(0x1D8C),
        passives: bool where offset(0x1DCC),
        selected_offhand: i32 where offset(0x4948),
        selected_offhand_pending_hybrid_action: i32 where offset(0x1358),
        player_flags: i32 where offset(0x1E88),
        last_ucmd_simulation_ticks: i32 where offset(0x26A8),
        last_ucmd_simulation_remainder_time: f32 where offset(0x26AC),
        remote_turret: i32 where offset(0x1F04),
        ground_entity: i32 where offset(0x414),
        titan_soul: i32 where offset(0x13B8),
        pet_titan: i32 where offset(0x2054),
        health: i32 where offset(0x4D4),
        max_health: i32 where offset(0x4D0),
        life_state: i32 where offset(0x4F1),
        fl_maxspeed: f32 where offset(0x50C),
        flags: i32 where offset(0x298),
        observer_mode: i32 where offset(0x1F64),
        observer_target: i32 where offset(0x1F6C),
        view_model: i32 where offset(0x2098),
        uefnointerp_parity: i32 where offset(0x27E4),
        active_burn_card_index: i32 where offset(0x1FA4),
        color_correction_ctrl: i32 where offset(0x1B68),
        player_fog_ctrl: i32 where offset(0x19E0),
        should_draw_player_while_using_view_entity: bool where offset(0x26BC),
        title: [c_char;32] where offset(0x2848),
        use_credit: bool where offset(0x2964),
        damage_impulse_no_decel_end_time: f32 where offset(0x1F40),
        has_mic: bool where offset(0x1E8C),
        in_party_chat: bool where offset(0x1E8D),
        player_move_speed_scale: f32 where offset(0x1E90),
        fl_deattime: f32 where offset(0x1F58),
        spawn_parity: bool where offset(0x25A8),
        up_dir: Vector3 where offset(0x102284),
        last_dodge_time: f32 where offset(0x259C),
        wall_hanging: bool where offset(0x22E0),
        traversal_type: i32 where offset(0x22EC),
        traversal_state: i32 where offset(0x22F0),
        traversal_repos: Vector3 where offset(0x2328),
        traversal_forward_dir: Vector3 where offset(0x231C),
        traversal_yaw_delta: f32 where offset(0x2354),
        traversal_yaw_pose_parameter: i32 where offset(0x2358),
        grapple_hook: i32 where offset(0x2050),
        auto_sprint_forced: i32 where offset(0x27C0),
        is_sprinting: bool where offset(0x27C4),
        sprint_started_time: f32 where offset(0x27CC),
        sprint_started_frac: f32 where offset(0x27D0),
        sprint_ended_time: f32 where offset(0x27D4),
        sprint_ended_frac: f32 where offset(0x27D8),
        sticky_sprint_start_time: f32 where offset(0x27DC),
        smart_ammo_previous_highest_lock_on_me_fraction_value: f32 where offset(0x2998),
        active_zipline: i32 where offset(0x23FC),
        zipline_reverse: bool where offset(0x2400),
        zipline_state: i32 where offset(0x2410),
        duck_state: i32 where offset(0x2250),
        stand_hull_min: Vector3 where offset(0x2254),
        stand_hull_max: Vector3 where offset(0x2260),
        duck_hull_min: Vector3 where offset(0x226C),
        duck_hull_max: Vector3 where offset(0x2278),
        xp: i32 where offset(0x205C),
        generation: i32 where offset(0x2060),
        rank: i32 where offset(0x2064),
        server_force_increase_player_list_generation_parity: i32 where offset(0x2068),
        is_playing_ranked: bool where offset(0x206C),
        skill_mu: f32 where offset(0x2070),
        titan_soul_being_rodeoed: i32 where offset(0x1E80),
        entity_syncing_witme: i32 where offset(0x1E84),
        next_titan_respawn_available: f32 where offset(0x2078),
        has_bad_reputation: bool where offset(0x1C90),
        community_name: [c_char;64] where offset(0x1C91),
        community_clan_tag: [c_char;16] where offset(0x1CD1),
        faction_name: [c_char;16] where offset(0x1CE1),
        hardware_icon: [c_char;16] where offset(0x1CF1),
        happy_hour_active: bool where offset(0x1D01),
        gesture_auto_kill_bitfield: i32 where offset(0x1EF4),
        pilot_class_index: i32 where offset(0x2EA8),
        vec_abs_origin: Vector3 where offset(0x100490),
        is_performing_boost_action: bool where offset(0x25BE),
        zipline_valid3p_weapon_layer_anim: bool where offset(0x240C),
        player_script_net_data_global: i32 where offset(0x345C),
        zooming: i32 where offset(0x1598), // this is like a bool but a i32. what?
        zoom_toggle_on: bool where offset(0x1599),
        zoom_base_frac: f32 where offset(0x159C),
        zoom_base_time: f32 where offset(0x15A0),
        zoom_full_start_time: f32 where offset(0x15A4),
        camo_index: i32 where offset(0xA04),
        decal_index: i32 where offset(0xA08),
        team: i32 where offset(0x5E4),
    }
}

pub type GetVector3Function =
    unsafe extern "C" fn(*const CBasePlayer, *mut Vector3) -> *mut Vector3;

// not tested
#[repr(C)]
pub struct CBasePlayerVtable {
    unk1: [*const c_void; 133],
    some_get_origin_varient_02: GetVector3Function,
    some_get_origin_varient_01: GetVector3Function,
    get_angles_02: GetVector3Function,
    get_angles: GetVector3Function,
    get_eye_position: GetVector3Function,
    get_center_position: GetVector3Function,
    some_get_origin_varient_03: GetVector3Function,
    unk2: [*const c_void; 104],
}

const PERSISTENCE_MAX_SIZE: usize = 0xDDCD;

offset_struct! {
    pub struct CBaseClient {
        __size: () where offset(0x2D728),
        edict: u16 where offset(0x14),
        name: [c_char;64] where offset(0x16),
        con_vars: *const c_void where offset(0x258), // TODO: add KeyValues later
        // net_channel: *const c_void where offset(0x290), this seams to be invalid :/
        signon: SignonState where offset(0x2A0),
        clan_tag: [c_char;16] where offset(0x358),
        fake_player: bool where offset(0x484),
        persistence_ready: PersistenceReady where offset(0x4A0),
        persistence_buffer: [c_char;PERSISTENCE_MAX_SIZE] where offset(0x4FA),
        uid: [c_char;32] where offset(0xF500),
    }
}

#[repr(i32)]
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum SignonState {
    #[default]
    NONE = 0, // no state yet; about to connect
    CHALLENGE = 1,   // client challenging server; all OOB packets
    CONNECTED = 2,   // client is connected to server; netchans ready
    NEW = 3,         // just got serverinfo and string tables
    PRESPAWN = 4,    // received signon buffers
    GETTINGDATA = 5, // respawn-defined signonstate, assumedly this is for persistence
    SPAWN = 6,       // ready to receive entity packets
    FIRSTSNAP = 7,   // another respawn-defined one
    FULL = 8,        // we are fully connected; first non-delta packet received
    CHANGELEVEL = 9, // server is changing level; please wait
}

#[repr(i8)]
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum PersistenceReady {
    #[default]
    NotReady, // todo: check if this correct
    Ready = 3,
    ReadyRemote,
}
