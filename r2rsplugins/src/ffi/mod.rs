use crate::bindings::*;
use std::ffi::c_void;

pub mod error;
mod utils;

use error::PluginError;

pub type UnsafeGameState = crate::bindings::GameState;

type GetPluginGameState = unsafe extern "C" fn(*const PluginObject) -> &'static UnsafeGameState;
type GetPluginServerInfo = unsafe extern "C" fn(*const PluginObject) -> &'static ServerInfo;
type GetPluginPlayerInfo = unsafe extern "C" fn(*const PluginObject) -> &'static PlayerInfo;

pub struct ExternalPluginData {
    function: &'static c_void,
}

// TODO: reading from the a rust book about ffi s I learned that (*)(int) is c++ for Some(int)
// so I suppose the &c_void sould be a Option<&c_void>
// or I might be wrong :(

/// Used to wrap unsafe c_void into a **safe** struct
impl ExternalPluginData {
    pub fn new<'a>(function: &'static c_void) -> Self {
        Self { function }
    }

    /// use this if you know magic I guess
    pub unsafe fn get_external_function(&self) -> &'_ c_void {
        self.function
    }

    pub fn get_game_state_struct(&self) -> GameState {
        unsafe {
            let get_plugin_object: &GetPluginGameState = std::mem::transmute(self.function);
            let game_state = get_plugin_object(PluginObject_GAMESTATE as *const i32);
            return GameState::new(game_state);
        }
    }

    pub fn get_server_info_struct<'a>(&self) -> &'a ServerInfo {
        unsafe {
            let get_plugin_object: &GetPluginServerInfo = std::mem::transmute(self.function);
            let server_info = get_plugin_object(PluginObject_SERVERINFO as *const i32);
            return server_info;
        }
    }

    pub fn get_player_info_struct<'a>(&self) -> &'a PlayerInfo {
        unsafe {
            let get_plugin_object: &GetPluginPlayerInfo = std::mem::transmute(self.function);
            let player_info = get_plugin_object(PluginObject_PLAYERINFO as *const i32);
            return player_info;
        }
    }
}

impl Clone for ExternalPluginData {
    fn clone(&self) -> Self {
        Self { function: self.function }
    }
}

pub struct GameState<'a> {
    gamestate_struct: &'a UnsafeGameState,
}

impl GameState<'_> {
    pub(crate) fn new<'a>(gamestate_struct: &'a UnsafeGameState) -> GameState {
        GameState { gamestate_struct }
    }

    pub unsafe fn get_internal_struct(&self) -> &'_ UnsafeGameState {
        self.gamestate_struct
    }

    pub fn our_score(&self) -> i32 {
        // match self.get_int_value( GameStateInfoType_ourScore ) {
        //     Ok(option) => Ok(option.unwrap()),
        //     Err(err) => Err(err),
        // }

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

    pub fn map(&self) -> Box<[i8]> {
        self.get_map().unwrap().unwrap()
    }

    pub fn get_map(&self) -> Result<Option<Box<[i8]>>, PluginError> {
        self.get_char_value(GameStateInfoType_map)
    }

    pub fn map_display_name(&self) -> Box<[i8]> {
        self.get_map_display_name().unwrap().unwrap()
    }

    pub fn get_map_display_name(&self) -> Result<Option<Box<[i8]>>, PluginError> {
        self.get_char_value(GameStateInfoType_mapDisplayName)
    }

    pub fn playlist(&self) -> Box<[i8]> {
        self.get_playlist().unwrap().unwrap()
    }

    pub fn get_playlist(&self) -> Result<Option<Box<[i8]>>, PluginError> {
        self.get_char_value(GameStateInfoType_playlist)
    }

    pub fn playlist_display_name(&self) -> Box<[i8]> {
        self.get_playlist_display_name().unwrap().unwrap()
    }

    pub fn get_playlist_display_name(&self) -> Result<Option<Box<[i8]>>, PluginError> {
        self.get_char_value(GameStateInfoType_playlistDisplayName)
    }

    pub fn players(&self) -> i32 {
        self.get_players().unwrap().unwrap()
    }

    pub fn get_players(&self) -> Result<Option<i32>, PluginError> {
        self.get_int_value(GameStateInfoType_players)
    }    

    fn get_int_value<'a>(&self, gamestate_type: i32) -> Result<Option<i32>, PluginError> {
        let func = match self.gamestate_struct.getGameStateInt {
            None => return Ok(None),
            Some(func) => func,
        };

        let mut int = Box::new(0);
        let ptr = int.as_mut();

        unsafe {
            let result = func(ptr, gamestate_type);

            if result != 0 {
                return Err(PluginError::Failed(result));
            }
        }

        Ok(Some(*ptr))
    }

    fn get_bool_value<'a>(&self, gamestate_type: i32) -> Result<Option<bool>, PluginError> {
        let func = match self.gamestate_struct.getGameStateBool {
            None => return Ok(None),
            Some(func) => func,
        };

        let mut boolean = Box::new(false);
        let ptr = boolean.as_mut();

        unsafe {
            let result = func(ptr, gamestate_type);

            if result != 0 {
                return Err(PluginError::Failed(result));
            }
        }

        Ok(Some(*ptr))
    }

    fn get_char_value<'a>(&self, gamestate_type: i32) -> Result<Option<Box<[i8]>>, PluginError> {
        match self.gamestate_struct.getGameStateChar {
            None => Ok(None),
            Some(func) => utils::use_get_char_value_func(&func, gamestate_type),
        }
    }
}
