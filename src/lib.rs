use r2rsplugins::prelude::*;
struct HelloWorld<'a> {
    gamestate: Option<GameState<'a>>,
}

impl Plugin for HelloWorld<'_> {
    fn new() -> Self {
        Self {
            gamestate: None,
        }
    }

    fn initialize(&mut self) {
        println!("rust plugin initialized");

        self.gamestate = Some(EPD.wait().get_game_state_struct());
    }

    fn main(&self) {
        println!("hello world from rust");

        let gamestate = self.gamestate.as_ref().unwrap();

        loop {
            println!( "our score: {}", gamestate.our_score() );

            wait(3000)
        }
    }
}

entry!(HelloWorld);

// goodies
// https://github.com/emma-miler/NorthstarPluginLibrary/blob/main/NorthstarPluginLibrary/lib/plugin_abi.h
