# rrplug
crate that provides function wappers and functions for [R2Northstar](https://github.com/R2Northstar/NorthstarLauncher) plugin creation.

## Plugin Support
currently only supports v1 plugins

v2 plugin support is coming

## Getting Started
add this to your Cargo.toml so your lib crate compiles into a dll.
```t
[lib]
crate-type = ["cdylib"]
```

create manifest.json with the following contents
```json
{
    "name": "plugin_name",
    "displayname": "plugin_name",
    "description": "plugin_name",
    "api_version": "1",
    "version": "1.0",
    "run_on_server": false,
    "run_on_client": true
}
```

get Resource.rc and resource.h from [ NorthstarDiscordRPC](https://github.com/R2Northstar/NorthstarDiscordRPC/tree/main/DiscordRPC).

create build.rs in the root of your project with the following content
```rust
extern crate windres;
use windres::Build;

fn main() {
    Build::new().compile("manifest\\Resource.rc").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=r2rsplugins\\headers\\Resource.rc");
    println!("cargo:rerun-if-changed=r2rsplugins/manifest.json");
}
```
and add windres as a build dependencie.

Finnaly shove this into lib.rs
```rust
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

entry!(HelloWorld);
```

Compile

Then Enojoy your hello world plugin

