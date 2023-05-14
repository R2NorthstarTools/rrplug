# rrplug
crate that provides function wappers and functions for [R2Northstar](https://github.com/R2Northstar/NorthstarLauncher) plugin creation.

## Plugin Support
the `v1` branch is for v1 plugins

the `v2` branch is for v2 plugins

the `master` branch is the newest version

## Getting Started
add this to your `Cargo.toml` so your lib crate compiles into a dll.
```t
[lib]
crate-type = ["cdylib"]
```

create `manifest.json` with the following contents
```json
{
    "name": "plugin_name",
    "displayname": "plugin_name",
    "description": "plugin_name",
    "api_version": "2",
    "version": "1.0",
    "run_on_server": false,
    "run_on_client": true
}
```

get `Resource.rc` and `resource.h` from [ NorthstarDiscordRPC](https://github.com/R2Northstar/NorthstarDiscordRPC/tree/main/DiscordRPC).

create `build.rs` in the root of your project with the following content
```rust
extern crate windres;
use windres::Build;

fn main() {
    Build::new().compile("manifest\\Resource.rc").unwrap();
}
```
and add windres as a build dependencie.

Finnaly shove this into `lib.rs`
```rust
use rrplug::prelude::*;

pub struct HelloWorld;

impl Plugin for HelloWorld {
    fn new() -> Self {
        Self {}
    }

    fn initialize(&mut self, plugin_data: &PluginData) {
        log::info!("Hello World");
    }

    fn main(&self) {}
}

entry!(HelloWorld);
```

Compile

Then enjoy your hello world plugin

## rrplug template

install cargo-generate if you don't have it
```bash
cargo install cargo-generate
```

```bash
cargo generate -g  https://github.com/catornot/rrplug.git
```
