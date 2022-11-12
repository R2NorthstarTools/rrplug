// this wouldn't work on its own look at the docs first

use rrplug::prelude::*;

struct HelloWorld {
    gamestate: Option<GameState>,
}

impl Plugin for HelloWorld {
    fn new() -> Self {
        Self {
            gamestate: None,
        }
    }

    fn initialize(&mut self, external_plugin_data: ExternalPluginData) {
        self.gamestate = external_plugin_data.get_game_state_struct();
        println!("rust plugin initialized");
    }

    fn main(&self) {
        let gamestate = self.gamestate.as_ref().unwrap();
        println!("hello northstar our score is {}", gamestate.our_score());
    }
}