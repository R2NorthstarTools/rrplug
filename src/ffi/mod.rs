use crate::bindings::*;
use std::ffi::c_void;

pub mod error;
mod utils;

pub use error::PluginError;

pub type GetPluginObject = unsafe extern "C" fn(*const CPluginObject) -> *const c_void;

/// Used to wrap a unsafe c_void into a **safe** struct
#[derive(Debug)]
pub struct ExternalPluginData {
    function: GetPluginObject,
}

// TODO: reading from the a rust book about ffi s I learned that (*)(int) is c++ for Some(int)
// so I suppose the &c_void sould be a Option<&c_void>
// or I might be wrong :(

impl ExternalPluginData {
    pub fn new<'a>(function: *const c_void) -> Self {
        unsafe {
            Self {
                function: std::mem::transmute(function),
            }
        }
    }

    /// use this if you know magic I guess
    pub unsafe fn get_external_function(&self) -> GetPluginObject {
        self.function
    }

    /// ## get_game_state_struct
    /// returns the GameState struct
    pub fn get_game_state_struct(&self) -> Option<GameState> {
        unsafe {
            let func = self.function;

            let returned_struct = func(PluginObject::GameState.ptr_int());

            let game_state: *const UnsafeGameState = std::mem::transmute(returned_struct);

            match game_state.as_ref() {
                None => None,
                Some(game_state) => Some(GameState::new(*game_state)),
            }
        }
    }

    pub fn get_server_info_struct(&self) -> Option<ServerInfo> {
        unsafe {
            let func = self.function;

            let returned_struct = func(PluginObject::ServerInfo.ptr_int());

            let server_info: *const UnsafeServerInfo = std::mem::transmute(returned_struct);

            match server_info.as_ref() {
                None => None,
                Some(server_info) => Some(ServerInfo::new(*server_info)),
            }
        }
    }

    pub fn get_player_info_struct(&self) -> Option<PlayerInfo> {
        unsafe {
            let func = self.function;

            let returned_struct = func(PluginObject::PlayerInfo.ptr_int());

            let player_info: *const UnsafePlayerInfo = std::mem::transmute(returned_struct);

            match player_info.as_ref() {
                None => None,
                Some(player_info) => Some(PlayerInfo::new(*player_info)),
            }
        }
    }
}

impl Clone for ExternalPluginData {
    fn clone(&self) -> Self {
        Self {
            function: self.function,
        }
    }
}

#[derive(Debug)]
pub struct GameState {
    gamestate_struct: UnsafeGameState,
}

impl GameState {
    pub fn new(gamestate_struct: UnsafeGameState) -> Self {
        Self { gamestate_struct }
    }

    pub unsafe fn get_internal_struct(&self) -> UnsafeGameState {
        self.gamestate_struct
    }

    pub fn our_score(&self) -> i32 {
        self.get_our_score().unwrap().unwrap()
    }

    pub fn get_our_score(&self) -> Result<Option<i32>, PluginError> {
        self.get_int_value(GameStateInfoType_ourScore)
    }

    pub fn second_highest_score(&self) -> i32 {
        self.get_second_highest_score().unwrap().unwrap()
    }

    pub fn get_second_highest_score(&self) -> Result<Option<i32>, PluginError> {
        self.get_int_value(GameStateInfoType_ourScore)
    }

    pub fn highest_score(&self) -> i32 {
        self.get_highest_score().unwrap().unwrap()
    }

    pub fn get_highest_score(&self) -> Result<Option<i32>, PluginError> {
        self.get_int_value(GameStateInfoType_highestScore)
    }

    pub fn loading(&self) -> bool {
        self.get_loading().unwrap().unwrap()
    }

    pub fn get_loading(&self) -> Result<Option<bool>, PluginError> {
        self.get_bool_value(GameStateInfoType_loading)
    }

    pub fn map(&self) -> String {
        self.get_map().unwrap().unwrap()
    }

    pub fn get_map(&self) -> Result<Option<String>, PluginError> {
        self.get_char_value(GameStateInfoType_map)
    }

    pub fn map_display_name(&self) -> String {
        self.get_map_display_name().unwrap().unwrap()
    }

    pub fn get_map_display_name(&self) -> Result<Option<String>, PluginError> {
        self.get_char_value(GameStateInfoType_mapDisplayName)
    }

    pub fn playlist(&self) -> String {
        self.get_playlist().unwrap().unwrap()
    }

    pub fn get_playlist(&self) -> Result<Option<String>, PluginError> {
        self.get_char_value(GameStateInfoType_playlist)
    }

    pub fn playlist_display_name(&self) -> String {
        self.get_playlist_display_name().unwrap().unwrap()
    }

    pub fn get_playlist_display_name(&self) -> Result<Option<String>, PluginError> {
        self.get_char_value(GameStateInfoType_playlistDisplayName)
    }

    pub fn players(&self) -> i32 {
        self.get_players().unwrap().unwrap()
    }

    pub fn get_players(&self) -> Result<Option<i32>, PluginError> {
        self.get_int_value(GameStateInfoType_players)
    }

    fn get_int_value(&self, gamestate_type: i32) -> Result<Option<i32>, PluginError> {
        match self.gamestate_struct.getGameStateInt {
            None => Ok(None),
            Some(func) => utils::use_get_int_value_func(func, gamestate_type),
        }
    }

    fn get_bool_value(&self, gamestate_type: i32) -> Result<Option<bool>, PluginError> {
        match self.gamestate_struct.getGameStateBool {
            None => Ok(None),
            Some(func) => utils::use_get_bool_value_func(func, gamestate_type),
        }
    }

    fn get_char_value(&self, gamestate_type: i32) -> Result<Option<String>, PluginError> {
        match self.gamestate_struct.getGameStateChar {
            None => Ok(None),
            Some(func) => utils::use_get_char_value_func(func, gamestate_type),
        }
    }
}

#[derive(Debug)]
pub struct ServerInfo {
    serverinfo_struct: UnsafeServerInfo,
}

impl ServerInfo {
    pub fn new(serverinfo_struct: UnsafeServerInfo) -> Self {
        Self { serverinfo_struct }
    }

    pub unsafe fn get_internal_struct(&self) -> UnsafeServerInfo {
        self.serverinfo_struct
    }

    pub fn end_time(&self) -> i32 {
        self.get_end_time().unwrap().unwrap()
    }

    pub fn get_end_time(&self) -> Result<Option<i32>, PluginError> {
        self.get_int_value(ServerEnum::EndTime.int())
    }

    pub fn description(&self) -> String {
        self.get_description().unwrap().unwrap()
    }

    pub fn get_description(&self) -> Result<Option<String>, PluginError> {
        self.get_char_value(ServerEnum::Description.int())
    }

    pub fn name(&self) -> String {
        self.get_name().unwrap().unwrap()
    }

    pub fn get_name(&self) -> Result<Option<String>, PluginError> {
        self.get_char_value(ServerEnum::Name.int())
    }

    pub fn password(&self) -> String {
        self.get_password().unwrap().unwrap()
    }

    pub fn get_password(&self) -> Result<Option<String>, PluginError> {
        self.get_char_value(ServerEnum::Password.int())
    }

    pub fn id(&self) -> i32 {
        self.get_id().unwrap().unwrap()
    }

    pub fn get_id(&self) -> Result<Option<i32>, PluginError> {
        self.get_int_value(ServerEnum::Id.int())
    }

    pub fn max_players(&self) -> i32 {
        self.get_max_players().unwrap().unwrap()
    }

    pub fn get_max_players(&self) -> Result<Option<i32>, PluginError> {
        self.get_int_value(ServerEnum::MaxPlayers.int())
    }

    pub fn score_limit(&self) -> i32 {
        self.get_score_limit().unwrap().unwrap()
    }

    pub fn get_score_limit(&self) -> Result<Option<i32>, PluginError> {
        self.get_int_value(ServerEnum::ScoreLimit.int())
    }

    pub fn round_based(&self) -> bool {
        self.get_round_based().unwrap().unwrap()
    }

    pub fn get_round_based(&self) -> Result<Option<bool>, PluginError> {
        self.get_bool_value(ServerEnum::RoundBased.int())
    }

    fn get_int_value(&self, gamestate_type: i32) -> Result<Option<i32>, PluginError> {
        match self.serverinfo_struct.getServerInfoInt {
            None => Ok(None),
            Some(func) => utils::use_get_int_value_func(func, gamestate_type),
        }
    }

    fn get_bool_value(&self, gamestate_type: i32) -> Result<Option<bool>, PluginError> {
        match self.serverinfo_struct.getServerInfoBool {
            None => Ok(None),
            Some(func) => utils::use_get_bool_value_func(func, gamestate_type),
        }
    }

    fn get_char_value(&self, gamestate_type: i32) -> Result<Option<String>, PluginError> {
        match self.serverinfo_struct.getServerInfoChar {
            None => Ok(None),
            Some(func) => utils::use_get_char_value_func(func, gamestate_type),
        }
    }
}

#[derive(Debug)]
pub struct PlayerInfo {
    playerinfo_struct: UnsafePlayerInfo,
}

impl PlayerInfo {
    pub fn new(playerinfo_struct: UnsafePlayerInfo) -> Self {
        Self { playerinfo_struct }
    }

    pub fn uid(&self) -> i32 {
        self.get_uid().unwrap().unwrap()
    }

    pub fn get_uid(&self) -> Result<Option<i32>, PluginError> {
        self.get_int_value(PlayerEnum::Uid.int())
    }

    fn get_int_value(&self, gamestate_type: i32) -> Result<Option<i32>, PluginError> {
        match self.playerinfo_struct.getPlayerInfoInt {
            None => Ok(None),
            Some(func) => utils::use_get_int_value_func(func, gamestate_type),
        }
    }
}
