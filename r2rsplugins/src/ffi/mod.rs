use crate::bindings::*;
use std::ffi::c_void;

pub mod error;
pub mod utils;

use error::PluginError;

pub type UnsafeGameState = crate::bindings::GameState;

pub type GetPluginGameState = unsafe extern "C" fn(*const PluginObject) -> UnsafeGameState;
pub type GetPluginServerInfo = unsafe extern "C" fn(*const PluginObject) -> ServerInfo;
pub type GetPluginPlayerInfo = unsafe extern "C" fn(*const PluginObject) -> PlayerInfo;

/// Used to wrap unsafe c_void into a **safe** struct
#[derive(Debug)]
pub struct ExternalPluginData {
    function: &'static c_void,
}

// TODO: reading from the a rust book about ffi s I learned that (*)(int) is c++ for Some(int)
// so I suppose the &c_void sould be a Option<&c_void>
// or I might be wrong :(

impl ExternalPluginData {
    pub fn new<'a>(function: &'static c_void) -> Self {
        Self { function }
    }

    /// use this if you know magic I guess
    pub unsafe fn get_external_function(&self) -> &'_ c_void {
        self.function
    }

    // pub fn get_game_state_struct(&self) -> Option<GameState> {
    //     unsafe {
    //         println!("transmuting");
    //         // let get_plugin_object: &GetPluginGameState = std::mem::transmute(self.function);
    //         let get_plugin_object: &Option<GetPluginGameState> = std::mem::transmute(self.function);

    //         println!("got {:?}", get_plugin_object);

    //         match get_plugin_object {
    //             None => return None,
    //             Some(func) => {
    //                 let game_state: &'static UnsafeGameState =
    //                     func(PluginObject_GAMESTATE as *const i32);
    //                 return Some(GameState::new(game_state));
    //             }
    //         }
    //     }
    // }

    // pub fn get_server_info_struct<'a>(&self) -> &'a ServerInfo {
    //     unsafe {
    //         let get_plugin_object: &GetPluginServerInfo = std::mem::transmute(self.function);
    //         let server_info: &'static ServerInfo =
    //             get_plugin_object(PluginObject_SERVERINFO as *const i32);
    //         return server_info;
    //     }
    // }

    // pub fn get_player_info_struct<'a>(&self) -> &'a PlayerInfo {
    //     unsafe {
    //         let get_plugin_object: &GetPluginPlayerInfo = std::mem::transmute(self.function);
    //         let player_info: &'static PlayerInfo =
    //             get_plugin_object(PluginObject_PLAYERINFO as *const i32);
    //         return player_info;
    //     }
    // }
}

impl Clone for ExternalPluginData {
    fn clone(&self) -> Self {
        Self {
            function: self.function,
        }
    }
}

#[derive(Debug)]
pub struct GameState<'a> {
    gamestate_struct: &'a UnsafeGameState,
}

impl GameState<'_> {
    pub fn new<'a>(gamestate_struct: &'a UnsafeGameState) -> GameState {
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
        match self.gamestate_struct.getGameStateInt {
            None => Ok(None),
            Some(func) => utils::use_get_int_value_func(func, gamestate_type),
        }
    }

    fn get_bool_value<'a>(&self, gamestate_type: i32) -> Result<Option<bool>, PluginError> {
        match self.gamestate_struct.getGameStateBool {
            None => Ok(None),
            Some(func) => utils::use_get_bool_value_func(func, gamestate_type),
        }
    }

    fn get_char_value<'a>(&self, gamestate_type: i32) -> Result<Option<Box<[i8]>>, PluginError> {
        match self.gamestate_struct.getGameStateChar {
            None => Ok(None),
            Some(func) => utils::use_get_char_value_func(func, gamestate_type),
        }
    }
}

#[derive(Debug)]
pub enum Return {
    GameState(UnsafeGameState),
    PlayerInfo(PlayerInfo),
    ServerInfo(ServerInfo),
}

pub fn test(get_plugin_data_external: Option<unsafe extern "C" fn(*const PluginObject) -> Return>) {
    println!("got {:?}", get_plugin_data_external);

    // wait(3000);

    unsafe {
        let game_state = get_plugin_data_external.unwrap()(
            crate::bindings::PluginObject_GAMESTATE as *const i32,
        );

        println!("gamestate struct: {:?}", game_state);

        match game_state {
            Return::ServerInfo(game_state) => {
                let thing = utils::use_get_int_value_func(
                    game_state.getServerInfoInt.unwrap(),
                    ServerInfoType_maxPlayers,
                );

                println!("{:?}", thing);
            }
            Return::GameState(game_state) => {
                println!("oh no");
            }
            Return::PlayerInfo(game_state) => {
                println!("oh no");
            }
        };

        let player_info = get_plugin_data_external.unwrap()(
            crate::bindings::PluginObject_PLAYERINFO as *const i32,
        );

        println!("player_info struct: {:?}", player_info);

        let server_info = get_plugin_data_external.unwrap()(
            crate::bindings::PluginObject_SERVERINFO as *const i32,
        );

        println!("server_info struct: {:?}", server_info);

        // let thing = game_state as *const UnsafeGameState;

        // test_2(thing);

        // wait(3000);

        // let thing = utils::use_get_int_value_func( game_state.getGameStateInt.unwrap(), GameStateInfoType_ourScore );

        // println!( "{:?}", thing );
    }
}

// fn test_2(game_state: Option<UnsafeGameState>) {
//     println!("gamestate struct: {:?}", game_state);
// }
