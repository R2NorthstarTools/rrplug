use r2rsplugins::prelude::*;

struct HelloWorld;

impl Plugin for HelloWorld {
    fn new() -> Self {
        Self {}
    }

    fn initialize(&self, getPluginData_external: c_void) {
        println!( "rust initialized" );
        // let getPluginData: PluginObject = getPluginData_external;
        // let gameStatePtr: GameStateInfoType = PluginObject_GAMESTATE;
        // let serverInfoPtr: ServerInfoType = PluginObject_SERVERINFO;
        // let playerInfoPtr: PlayerInfoType = PluginObject_PLAYERINFO;
        
        // GameState{ getGameStateChar: todo!(), getGameStateInt: todo!(), getGameStateBool: todo!() }

        // .getGameStateChar(  )
    }

    fn main(&self) {
        println!("hello world from rust");

        wait( 3000 );

        panic!("thx for living");
    }
}

entry!(HelloWorld);

// goodies
// https://github.com/emma-miler/NorthstarPluginLibrary/blob/main/NorthstarPluginLibrary/lib/plugin_abi.h
