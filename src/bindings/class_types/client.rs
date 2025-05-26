use std::ffi::{c_char, c_void};

use crate::{field_assert, size_assert};

const PERSISTENCE_MAX_SIZE: usize = 0xDDCD;

#[allow(non_camel_case_types, non_snake_case)]
#[repr(C)]
pub struct CClient {
    pub vftable: *const c_void,
    pub vftable2: *const c_void,
    pub m_nUserID: u32,
    pub m_nHandle: u16,
    pub m_szServerName: [c_char; 64],
    pub m_nReputation: i64,
    pub pad_0014: [c_char; 182],
    pub m_szClientName: [c_char; 64],
    pub pad_0015: [c_char; 252],
    pub m_ConVars: *mut c_void, // TODO: add keyvalues
    pub pad_0368: [c_char; 8],
    pub m_pServer: *mut c_void, // TODO: add server
    pub pad_0378: [c_char; 32],
    pub m_NetChannel: *mut c_void, // TODO: add net channel
    pub pad_03A8: [c_char; 8],
    pub m_nSignonState: SignonState,
    pub m_nDeltaTick: i32,
    pub m_nOriginID: u64,
    pub m_nStringTableAckTick: i32,
    pub m_nSignonTick: i32,
    pub pad_03C0: [c_char; 160],
    pub m_szClanTag: [c_char; 16],
    pub pad2: [c_char; 284],
    pub m_bFakePlayer: bool,
    pub m_bReceivedPacket: bool,
    pub m_bLowViolence: bool,
    pub m_bFullyAuthenticated: bool,
    pub pad_05A4: [c_char; 24],
    pub m_nPersistenceState: PersistenceReady,
    pub pad_05C0: [c_char; 89],
    pub m_PersistenceBuffer: [c_char; PERSISTENCE_MAX_SIZE],
    pub pad: [c_char; 4665],
    pub m_UID: [c_char; 32],
    pub pad0: [c_char; 0x1E208],
}

size_assert!(SIZE_CLIENT where CClient == 0x2D728);
field_assert!(M_SZSERVERNAME where CClient, m_szServerName == 0x16);
field_assert!(M_CONVARS where CClient, m_ConVars == 0x258);
field_assert!(M_NSIGNONSTATE where CClient, m_nSignonState == 0x2A0);
field_assert!(M_SZCLANTAG where CClient, m_szClanTag == 0x358);
field_assert!(M_BFAKEPLAYER where CClient, m_bFakePlayer == 0x484);
field_assert!(M_NPERSISTENCESTATE where CClient, m_nPersistenceState == 0x4A0);
field_assert!(M_PERSISTENCEBUFFER where CClient, m_PersistenceBuffer == 0x4FA);
field_assert!(M_UID where CClient, m_UID == 0xF500);

#[repr(i32)]
#[derive(Clone, Debug, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
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
