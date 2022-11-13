//! contains wrappers to unsafe northstar function generate by bindgen from headers

use crate::bindings::*;
use std::ffi::c_void;

pub mod error;
mod utils;

pub use error::PluginError;

type GetPluginObject = unsafe extern "C" fn(*const CPluginObject) -> *const c_void;

/// Used to get [`GameState`], [`PlayerInfo`] and [`ServerInfo`]
/// 
/// ## example
/// ```
/// let game_state = external_plugin_data.get_game_state_struct();
/// ```
#[derive(Debug)]
pub struct ExternalPluginData {
    function: GetPluginObject,
}

impl ExternalPluginData {
    pub fn new(function: *const c_void) -> Self {
        unsafe {
            Self {
                function: std::mem::transmute(function),
            }
        }
    }

    /// returns the [`GameState`] struct in a option in case the derefence operation fails
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
    
    /// returns the [`ServerInfo`] struct in a option in case the derefence operation fails
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
    
    /// returns the [`PlayerInfo`] struct in a option in case the derefence operation fails
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

/// used to get game states from northstar
#[derive(Debug)]
pub struct GameState {
    gamestate_struct: UnsafeGameState,
}

impl GameState {
    pub fn new(gamestate_struct: UnsafeGameState) -> Self {
        Self { gamestate_struct }
    }
    
    /// retuns the frendly's team score
    /// 
    /// ## Panics
    /// panics if 
    /// the internal function is missing, 
    /// northstar returned a error code or
    /// couldn't get back the value
    pub fn our_score(&self) -> i32 {
        self.get_our_score().unwrap().unwrap()
    }
    
    /// safe way to get the our score
    ///
    /// returns a result type with [`Option<i32>`] and [`PluginError`]
    pub fn get_our_score(&self) -> Result<Option<i32>, PluginError> {
        self.get_int_value(GameStateEnum::OurScore.int())
    }
    
    /// retuns the second highest score
    /// 
    /// can either be the enemies's tema or frendly's team score
    /// 
    /// ## Panics
    /// panics if 
    /// the internal function is missing, 
    /// northstar returned a error code or
    /// couldn't get back the value
    pub fn second_highest_score(&self) -> i32 {
        self.get_second_highest_score().unwrap().unwrap()
    }
    
    /// safe way to get the second highest score
    ///
    /// returns a result type with [`Option<i32>`] and [`PluginError`]
    pub fn get_second_highest_score(&self) -> Result<Option<i32>, PluginError> {
        self.get_int_value(GameStateEnum::SecondHighestScore.int())
    }
    
    /// retuns the highest score
    /// 
    /// can either be the enemies's tema or frendly's team score
    /// 
    /// ## Panics
    /// panics if 
    /// the internal function is missing, 
    /// northstar returned a error code or
    /// couldn't get back the value
    pub fn highest_score(&self) -> i32 {
        self.get_highest_score().unwrap().unwrap()
    }
    
    /// safe way to get the highest score
    ///
    /// returns a result type with [`Option<i32>`] and [`PluginError`]
    pub fn get_highest_score(&self) -> Result<Option<i32>, PluginError> {
        self.get_int_value(GameStateEnum::HighestScore.int())
    }
    
    /// checks if we are in a loading sreen
    /// 
    /// NOTE: doesn't seam to work
    /// 
    /// ## Panics
    /// panics if 
    /// the internal function is missing, 
    /// northstar returned a error code or
    /// couldn't get back the value
    pub fn loading(&self) -> bool {
        self.get_loading().unwrap().unwrap()
    }
    
    /// safe way to check if the game is loading
    ///
    /// returns a result type with [`Option<bool>`] and [`PluginError`]
    pub fn get_loading(&self) -> Result<Option<bool>, PluginError> {
        self.get_bool_value(GameStateEnum::Loading.int())
    }
    
    /// returns the map 
    /// 
    /// eg: mp_box
    /// 
    /// ## Panics
    /// panics if 
    /// the internal function is missing, 
    /// northstar returned a error code or
    /// couldn't get back the value
    pub fn map(&self) -> String {
        self.get_map().unwrap().unwrap()
    }
    
    /// safe way to get the map
    ///
    /// returns a result type with [`Option<String>`] and [`PluginError`]
    pub fn get_map(&self) -> Result<Option<String>, PluginError> {
        self.get_char_value(GameStateEnum::Map.int())
    }
    
    /// returns the map's name
    /// 
    /// eg: mp_box -> Box
    /// 
    /// ## Panics
    /// panics if 
    /// the internal function is missing, 
    /// northstar returned a error code or
    /// couldn't get back the value
    pub fn map_display_name(&self) -> String {
        self.get_map_display_name().unwrap().unwrap()
    }
    
    /// safe way to get the map's name
    ///
    /// returns a result type with [`Option<String>`] and [`PluginError`]
    pub fn get_map_display_name(&self) -> Result<Option<String>, PluginError> {
        self.get_char_value(GameStateEnum::MapDisplayName.int())
    }
    
    /// returns the current playlist 
    /// 
    /// eg: tdm
    /// 
    /// ## Panics
    /// panics if 
    /// the internal function is missing, 
    /// northstar returned a error code or
    /// couldn't get back the value
    pub fn playlist(&self) -> String {
        self.get_playlist().unwrap().unwrap()
    }
    
    /// safe way to get the current playlist 
    ///
    /// returns a result type with [`Option<String>`] and [`PluginError`]
    pub fn get_playlist(&self) -> Result<Option<String>, PluginError> {
        self.get_char_value(GameStateEnum::Playlist.int())
    }
    
    /// returns the playlist's name 
    /// 
    /// eg: tdm -> Skirmish
    /// 
    /// ## Panics
    /// panics if 
    /// the internal function is missing, 
    /// northstar returned a error code or
    /// couldn't get back the value
    pub fn playlist_display_name(&self) -> String {
        self.get_playlist_display_name().unwrap().unwrap()
    }

    pub fn get_playlist_display_name(&self) -> Result<Option<String>, PluginError> {
        self.get_char_value(GameStateEnum::PlaylistDisplayName.int())
    }
    
    /// returns the amount of players present in the game
    /// 
    /// ## Panics
    /// panics if 
    /// the internal function is missing, 
    /// northstar returned a error code or
    /// couldn't get back the value
    pub fn players(&self) -> i32 {
        self.get_players().unwrap().unwrap()
    }
    
    /// safe way to get amount of players
    ///
    /// returns a result type with [`Option<i32>`] and [`PluginError`]
    pub fn get_players(&self) -> Result<Option<i32>, PluginError> {
        self.get_int_value(GameStateEnum::Players.int())
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

/// used to get server info from northstar
#[derive(Debug)]
pub struct ServerInfo {
    serverinfo_struct: UnsafeServerInfo,
}

impl ServerInfo {
    pub fn new(serverinfo_struct: UnsafeServerInfo) -> Self {
        Self { serverinfo_struct }
    }
    
    /// returns the game's end time
    /// 
    /// ## Panics
    /// panics if 
    /// the internal function is missing, 
    /// northstar returned a error code or
    /// couldn't get back the value
    pub fn end_time(&self) -> i32 {
        self.get_end_time().unwrap().unwrap()
    }
    
    /// safe way to get the game's end time
    ///
    /// returns a result type with [`Option<i32>`] and [`PluginError`]
    pub fn get_end_time(&self) -> Result<Option<i32>, PluginError> {
        self.get_int_value(ServerEnum::EndTime.int())
    }
    
    /// returns the server's description
    /// 
    /// NOTE: doesn't seam to work
    /// 
    /// ## Panics
    /// panics if 
    /// the internal function is missing, 
    /// northstar returned a error code or
    /// couldn't get back the value
    pub fn description(&self) -> String {
        self.get_description().unwrap().unwrap()
    }
    
    /// safe way to get the server's description
    ///
    /// returns a result type with [`Option<String>`] and [`PluginError`]
    pub fn get_description(&self) -> Result<Option<String>, PluginError> {
        self.get_char_value(ServerEnum::Description.int())
    }
    
    /// returns the server's name
    /// 
    /// NOTE: doesn't seam to work
    /// 
    /// ## Panics
    /// panics if 
    /// the internal function is missing, 
    /// northstar returned a error code or
    /// couldn't get back the value
    pub fn name(&self) -> String {
        self.get_name().unwrap().unwrap()
    }
    
    /// safe way to get the the server's name
    ///
    /// returns a result type with [`Option<String>`] and [`PluginError`]
    pub fn get_name(&self) -> Result<Option<String>, PluginError> {
        self.get_char_value(ServerEnum::Name.int())
    }
    
    /// returns the server's password
    /// 
    /// NOTE: doesn't seam to work
    /// 
    /// ## Panics
    /// panics if 
    /// the internal function is missing, 
    /// northstar returned a error code or
    /// couldn't get back the value
    pub fn password(&self) -> String {
        self.get_password().unwrap().unwrap()
    }
    
    /// safe way to get the server's password
    ///
    /// returns a result type with [`Option<String>`] and [`PluginError`]
    pub fn get_password(&self) -> Result<Option<String>, PluginError> {
        self.get_char_value(ServerEnum::Password.int())
    }
    
    /// calling this function would imidialty crash northstar
    pub fn id(&self) -> i32 {
        self.get_id().unwrap().unwrap()
    }
    
    /// calling this function would imidialty crash northstar
    pub fn get_id(&self) -> Result<Option<i32>, PluginError> {
        self.get_int_value(ServerEnum::Id.int())
    }
    
    /// returns max players
    /// 
    /// ## Panics
    /// panics if 
    /// the internal function is missing, 
    /// northstar returned a error code or
    /// couldn't get back the value
    pub fn max_players(&self) -> i32 {
        self.get_max_players().unwrap().unwrap()
    }
    
    /// safe way to get max players
    ///
    /// returns a result type with [`Option<i32>`] and [`PluginError`]
    pub fn get_max_players(&self) -> Result<Option<i32>, PluginError> {
        self.get_int_value(ServerEnum::MaxPlayers.int())
    }
    
    /// returns the score limit
    /// 
    /// ## Panics
    /// panics if 
    /// the internal function is missing, 
    /// northstar returned a error code or
    /// couldn't get back the value
    pub fn score_limit(&self) -> i32 {
        self.get_score_limit().unwrap().unwrap()
    }
    
    /// safe way of getting the score limit
    ///
    /// returns a result type with [`Option<i32>`] and [`PluginError`]
    pub fn get_score_limit(&self) -> Result<Option<i32>, PluginError> {
        self.get_int_value(ServerEnum::ScoreLimit.int())
    }
    
    /// check for round_based.
    /// 
    /// ## Panics
    /// panics if 
    /// the internal function is missing, 
    /// northstar returned a error code or
    /// couldn't get back the value
    pub fn round_based(&self) -> bool {
        self.get_round_based().unwrap().unwrap()
    }
    
    /// safe way to check if the server is round_based.
    ///
    /// returns a result type with [`Option<bool>`] and [`PluginError`]
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

/// Used to the uid from northstar
#[derive(Debug)]
pub struct PlayerInfo {
    playerinfo_struct: UnsafePlayerInfo,
}

impl PlayerInfo {
    pub fn new(playerinfo_struct: UnsafePlayerInfo) -> Self {
        Self { playerinfo_struct }
    }
    
    /// returns the uid of the player
    /// 
    /// ## Panics
    /// panics if 
    /// the internal function is missing, 
    /// northstar returned a error code or
    /// couldn't get back the value
    pub fn uid(&self) -> i32 {
        self.get_uid().unwrap().unwrap()
    }
    
    /// safe way to get the uid of the player
    ///
    /// returns a result type with [`Option<i32>`] and [`PluginError`]
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
