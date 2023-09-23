//! has low abstractions to northstar stuff

#[cfg(doc)]
use crate::bindings::cvar::{command::ConCommand, convar::ConVar};
use crate::bindings::plugin_abi::CreateObjectFunc;
use once_cell::sync::OnceCell;

/// create object function
///
/// used to create [`ConCommand`] and [`ConVar`] by leaking the object which is fine since they need to last for the whole lifetime of the game
pub static CREATE_OBJECT_FUNC: OnceCell<CreateObjectFunc> = OnceCell::new();
