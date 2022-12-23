//! rrplug is a safe wrapper around the plugin system in [R2Norhtstar](https://northstar.tf/)
//! 
//! # Basic Setup
//! for Northstar to be able to use the plugin it must be compiled into a dll.
//! 
//! So natturaly you would want your plugin to be a cdylib lib crate.
//! 
//! adding this to your Cargo.toml would make your lib crate compile into a dll. 
//! ```
//! [lib]
//! crate-type = ["cdylib"]
//! ```
//! 
//! ### manifest
//! Northstar also requires plugins to have a manifest.json inserted into them.
//! 
//! it is possible to do so with [windres](https://crates.io/crates/windres) and build.rs.
//! 
//! #### manifest.json
//! create manifest.json with the following contents
//! ```json
//! {
//!     "name": "plugin_name",
//!     "displayname": "plugin_name",
//!     "description": "plugin_name",
//!     "api_version": "1",
//!     "version": "1.0",
//!     "run_on_server": false,
//!     "run_on_client": true
//! }
//! ```
//! 
//! #### build.rs
//! get Resource.rc and resource.h from [ NorthstarDiscordRPC](https://github.com/R2Northstar/NorthstarDiscordRPC/tree/main/DiscordRPC).
//! 
//! now we have everything to make the build script
//! 
//! create build.rs in the root of your project with the following content
//! ```
//! extern crate windres;
//! use windres::Build;
//! 
//! fn main() {
//!     Build::new().compile("manifest\\Resource.rc").unwrap();
//! 
//!     println!("cargo:rerun-if-changed=build.rs");
//!     println!("cargo:rerun-if-changed=Cargo.lock");
//!     println!("cargo:rerun-if-changed=r2rsplugins\\headers\\Resource.rc");
//!     println!("cargo:rerun-if-changed=r2rsplugins/manifest.json");
//! }
//! ```
//! and add windres as a build dependencie.
//! 
//! ### basic lib.rs
//! ```
//! use rrplug::prelude::*;
//!
//! struct HelloWorld {
//!     gamestate: Option<GameState>,
//! }
//!
//! impl Plugin for HelloWorld {
//!     fn new() -> Self {
//!         Self {
//!             gamestate: None,
//!         }
//!     }
//! 
//!     fn initialize(&mut self, external_plugin_data: ExternalPluginData) {
//!         self.gamestate = external_plugin_data.get_game_state_struct();
//!         println!("rust plugin initialized");
//!     }
//! 
//!     fn main(&self) {
//!         let gamestate = self.gamestate.as_ref().unwrap();
//!         println!("hello northstar our score is {}", gamestate.our_score());
//!     }
//! }
//! 
//! entry!(HelloWorld);
//! ```

pub mod bindings;
pub mod macros;
// pub mod ffi;
pub mod plugin;
pub mod prelude;
pub mod nslog;
pub mod wrappers;

pub use rrplug_proc::sqfunction;
#[doc(hidden)]
pub use once_cell::sync::OnceCell;
#[doc(hidden)]
pub use log;