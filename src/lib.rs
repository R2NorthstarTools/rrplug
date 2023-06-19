#![cfg(not(doctest))]
//! rrplug is a safe wrapper around the plugin system in [R2Northstar](https://northstar.tf/)
//!
//! this version is for plugins v2.
//!
//!
//! ## Setup with template
//!
//! install cargo-generate if you don't have it
//! ```bash
//! cargo install cargo-generate
//! ```
//!
//! ```bash
//! cargo generate -g  https://github.com/catornot/rrplug.git
//! ```
//!
//! ## Manual setup
//! for Northstar to be able to use the plugin it must be compiled into a dll.
//!
//! So natturaly you would want your plugin to be a cdylib lib crate.
//!
//! adding this to your `Cargo.toml` would make your lib crate compile into a dll.
//! ```
//! [lib]
//! crate-type = ["cdylib"]
//! ```
//!
//! ### Manifest
//! Northstar also requires plugins to have a `manifest.json` inserted into them.
//!
//! it is possible to do so with [windres](https://crates.io/crates/windres) and `build.rs`.
//!
//! #### `manifest.json`
//! create `manifest.json` with the following contents
//! ```json
//! {
//!     "name": "plugin_name",
//!     "displayname": "plugin_name",
//!     "description": "plugin_name",
//!     "api_version": "2",
//!     "version": "1.0",
//!     "run_on_server": false,
//!     "run_on_client": true
//! }
//! ```
//!
//! #### `build.rs`
//! get `Resource.rc` and `resource.h` from [ NorthstarDiscordRPC](https://github.com/R2Northstar/NorthstarDiscordRPC/tree/main/DiscordRPC).
//!
//! now we have everything to make the build script
//!
//! create `build.rs` in the root of your project with the following content
//! ```no_run
//! extern crate windres;
//! use windres::Build;
//!
//! fn main() {
//!     Build::new().compile("manifest\\Resource.rc").unwrap();
//! }
//! ```
//! and add windres as a build dependencie.
//!
//! ### Basic `lib.rs`
//! ```no_run
//! use rrplug::prelude::*;
//!
//! pub struct BasicPlugin;
//!
//! impl Plugin for BasicPlugin {
//!     fn new() -> Self {
//!         Self {}
//!     }
//!
//!     fn initialize(&mut self, plugin_data: &PluginData) {
//!         log::info!("yay logging :D");
//!     }
//!
//!     fn main(&self) {}
//! }
//!
//! entry!(BasicPlugin);
//! ```
//!

// #![warn(missing_docs)]

#[allow(missing_docs)]
pub mod bindings;
pub mod errors;
pub mod high;
pub mod low;
pub mod macros;
pub mod mid;
#[doc(hidden)]
pub mod nslog;
pub mod plugin;
pub mod prelude;

// could be changed to sqexternal
#[doc(hidden)]
pub use log;
#[doc(hidden)]
pub use once_cell::sync::OnceCell;
pub use rrplug_proc::{concommand, convar, sqfunction};

#[cfg(test)]
mod test {
    use crate as rrplug;
    use rrplug::prelude::*;
    use rrplug_proc::*;

    #[convar]
    fn test_convar(_old_string: String, _old_float: f32) -> () {}

    #[concommand]
    fn test_concommand(command: CCommandResult) {
        log::info!("test {:?}", command.get_args());
    }

    #[sqfunction(VM = "Server", ExportName = "test")]
    fn test_sqfunction(_test3: String, _test2: i32) -> String {
        Ok("ok".into())
    }
}
