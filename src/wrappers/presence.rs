use crate::bindings::plugin_abi::PluginGameStatePresence;
#[cfg(feature = "presence")]
use crate::bindings::plugin_abi::{
    GameState_INGAME, GameState_LOADING, GameState_LOBBY, GameState_MAINMENU,
};

use super::errors::GamePresenceError;

#[cfg(feature = "presence")]
macro_rules! c_char_to_string {
    ($char_ptr: expr) => {
        unsafe {
            std::ffi::CStr::from_ptr($char_ptr)
                .to_string_lossy()
                .to_string()
        }
    };
}

/*
pub struct PluginGameStatePresence {
    pub id: *const ::std::os::raw::c_char,
    pub name: *const ::std::os::raw::c_char,
    pub description: *const ::std::os::raw::c_char,
    pub password: *const ::std::os::raw::c_char,
    pub is_server: bool,
    pub is_local: bool,
    pub state: GameState,
    pub map: *const ::std::os::raw::c_char,
    pub map_displayname: *const ::std::os::raw::c_char,
    pub playlist: *const ::std::os::raw::c_char,
    pub playlist_displayname: *const ::std::os::raw::c_char,
    pub current_players: ::std::os::raw::c_int,
    pub max_players: ::std::os::raw::c_int,
    pub own_score: ::std::os::raw::c_int,
    pub other_highest_score: ::std::os::raw::c_int,
    pub max_score: ::std::os::raw::c_int,
    pub timestamp_end: ::std::os::raw::c_int,
}
*/

#[repr(u32)]
pub enum GameStateEnum {
    Loading = 0,
    MainMenu = 1,
    Lobby = 2,
    InGame = 3,
}

pub struct GamePresence(&'static PluginGameStatePresence);

impl GamePresence {
    #[doc(hidden)]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn new(presence: *const PluginGameStatePresence) -> Result<Self, GamePresenceError> {
        match unsafe { presence.as_ref() } {
            Some(p) => Ok(Self(p)),
            None => Err(GamePresenceError::NullGamePresenceError),
        }
    }

    #[cfg(not(feature = "presence"))]
    pub fn get_raw_game_presence(&self) -> &PluginGameStatePresence {
        self.0
    }
}

#[cfg(feature = "presence")]
impl GamePresence {
    pub fn get_id(&self) -> String {
        c_char_to_string!(self.0.id)
    }

    pub fn get_name(&self) -> String {
        c_char_to_string!(self.0.name)
    }

    pub fn get_description(&self) -> String {
        c_char_to_string!(self.0.description)
    }

    pub fn get_password(&self) -> String {
        c_char_to_string!(self.0.password)
    }

    pub fn is_server(&self) -> bool {
        self.0.is_server
    }

    pub fn is_local(&self) -> bool {
        self.0.is_local
    }

    #[allow(non_upper_case_globals)]
    pub fn get_state(&self) -> Option<GameStateEnum> {
        Some(match self.0.state {
            GameState_LOADING => GameStateEnum::Loading,
            GameState_MAINMENU => GameStateEnum::MainMenu,
            GameState_LOBBY => GameStateEnum::Lobby,
            GameState_INGAME => GameStateEnum::InGame,
            _ => return None,
        })
    }

    pub fn get_map(&self) -> String {
        c_char_to_string!(self.0.map)
    }

    pub fn get_map_displayname(&self) -> String {
        c_char_to_string!(self.0.map_displayname)
    }

    pub fn get_playlist(&self) -> String {
        c_char_to_string!(self.0.playlist)
    }

    pub fn get_playlist_displayname(&self) -> String {
        c_char_to_string!(self.0.playlist_displayname)
    }

    pub fn get_current_players(&self) -> i32 {
        self.0.current_players
    }

    pub fn get_max_players(&self) -> i32 {
        self.0.max_players
    }

    pub fn get_own_score(&self) -> i32 {
        self.0.own_score
    }

    pub fn get_max_score(&self) -> i32 {
        self.0.max_score
    }

    pub fn get_other_highest_score(&self) -> i32 {
        self.0.other_highest_score
    }

    pub fn get_timestamp_end(&self) -> i32 {
        self.0.timestamp_end
    }
}
