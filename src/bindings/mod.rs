#![allow(dead_code)]

#[allow(unused)]
mod bindings;

pub type UnsafeGameState = bindings::GameState;
pub type UnsafeServerInfo = bindings::ServerInfo;
pub type UnsafePlayerInfo = bindings::PlayerInfo;
pub type CPluginObject = bindings::PluginObject;

pub use bindings::PluginObject_DUMMY;
pub use bindings::PluginObject_GAMESTATE;
pub use bindings::PluginObject_PLAYERINFO;
pub use bindings::PluginObject_SERVERINFO;
pub use bindings::PluginObject_UNSUPPORTED;
pub use bindings::ABI_VERSION;

#[derive(Debug)]
pub enum PluginObject {
    Unsupported,
    GameState,
    PlayerInfo,
    ServerInfo,
    Dummy,
}

impl PluginObject {
    pub fn int(&self) -> i32 {
        match self {
            Self::Unsupported => PluginObject_UNSUPPORTED,
            Self::GameState => PluginObject_GAMESTATE,
            Self::PlayerInfo => PluginObject_PLAYERINFO,
            Self::ServerInfo => PluginObject_SERVERINFO,
            Self::Dummy => PluginObject_DUMMY,
        }
    }
    
    pub fn ptr_int(&self) -> *const i32 {
        self.int() as *const i32
    }
}

pub use bindings::GameStateInfoType;
pub use bindings::GameStateInfoType_connected;
pub use bindings::GameStateInfoType_highestScore;
pub use bindings::GameStateInfoType_loading;
pub use bindings::GameStateInfoType_map;
pub use bindings::GameStateInfoType_mapDisplayName;
pub use bindings::GameStateInfoType_ourScore;
pub use bindings::GameStateInfoType_players;
pub use bindings::GameStateInfoType_playlist;
pub use bindings::GameStateInfoType_playlistDisplayName;
pub use bindings::GameStateInfoType_secondHighestScore;

#[derive(Debug)]
pub enum GameStateEnum {
    Connected,
    HighestScore,
    Loading,
    Map,
    OurScore,
    Players,
    Playlist,
    PlaylistDisplayName,
    SecondHighestScore,
    MapDisplayName,
}

impl GameStateEnum {
    pub fn int(&self) -> i32 {
        match self {
            Self::Connected => GameStateInfoType_connected,
            Self::HighestScore => GameStateInfoType_highestScore,
            Self::Loading => GameStateInfoType_loading,
            Self::Map => GameStateInfoType_map,
            Self::OurScore => GameStateInfoType_ourScore,
            Self::Players => GameStateInfoType_players,
            Self::Playlist => GameStateInfoType_playlist,
            Self::PlaylistDisplayName => GameStateInfoType_playlistDisplayName,
            Self::SecondHighestScore => GameStateInfoType_secondHighestScore,
            Self::MapDisplayName => GameStateInfoType_mapDisplayName,
        }
    }
}

pub use bindings::ServerInfoType;
pub use bindings::ServerInfoType_description;
pub use bindings::ServerInfoType_endTime;
pub use bindings::ServerInfoType_id;
pub use bindings::ServerInfoType_maxPlayers;
pub use bindings::ServerInfoType_name;
pub use bindings::ServerInfoType_password;
pub use bindings::ServerInfoType_roundBased;
pub use bindings::ServerInfoType_scoreLimit;

#[derive(Debug)]
pub enum ServerEnum {
    Description,
    EndTime,
    Id,
    MaxPlayers,
    Name,
    Password,
    RoundBased,
    ScoreLimit,
}

impl ServerEnum {
    pub fn int(&self) -> i32 {
        match self {
            Self::Description => ServerInfoType_description,
            Self::EndTime => ServerInfoType_endTime,
            Self::Id => ServerInfoType_id,
            Self::MaxPlayers => ServerInfoType_maxPlayers,
            Self::Name => ServerInfoType_name,
            Self::Password => ServerInfoType_password,
            Self::RoundBased => ServerInfoType_roundBased,
            Self::ScoreLimit => ServerInfoType_scoreLimit,
        }
    }
}

pub use bindings::PlayerInfoType;
pub use bindings::PlayerInfoType_uid;

#[derive(Debug)]
pub enum PlayerEnum {
    Uid,
}

impl PlayerEnum {
    pub fn int(&self) -> i32 {
        match self {
            Self::Uid => PlayerInfoType_uid,
        }
    }
}
