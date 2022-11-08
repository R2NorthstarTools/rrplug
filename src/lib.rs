use r2rsplugins::prelude::*;
struct HelloWorld {
    // gamestate: Option<GameState<'a>>,
}

impl Plugin for HelloWorld {
    fn new() -> Self {
        Self {
            // gamestate: None,
        }
    }

    fn initialize(&mut self, external_plugin_data: ExternalPluginData) {
        println!("rust plugin initialized");


        // println!("{:?}", external_plugin_data.get_game_state_struct());

        // self.gamestate = Some(PluginData.wait().get_game_state_struct());
    }

    fn main(&self) {
        println!("hello world from rust");

        wait(1000);

        // println!("{:?}", PluginData.wait().get_game_state_struct());

        // let gamestate = self.gamestate.as_ref().unwrap();

        // loop {
        //     println!( "our score: {}", gamestate.our_score() );

        //     wait(3000)
        // }
    }
}

entry!(HelloWorld);

// goodies
// https://github.com/emma-miler/NorthstarPluginLibrary/blob/main/NorthstarPluginLibrary/lib/plugin_abi.h
