use std::ffi::{c_char, c_void};

use crate::offset_struct;

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
