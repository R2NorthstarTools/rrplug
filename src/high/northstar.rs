//! wrappers for structs that are passed to the plugin

use std::fmt::Display;

use crate::bindings::squirrelclasstypes::{SQFunction, ScriptContext};

// TODO: move this into squirrel.rs
/// function type which is used in `register_sq_functions` to get [`SQFuncInfo`]
pub type FuncSQFuncInfo = fn() -> SQFuncInfo;

/// holds infomation about a sq function for it to be registered corretly
///
/// it creates a native closure btw but sqfunction is also a valid name for it.
/// sqfunction is used in a lot of places with diffrent meanings `¯\_(ツ)_/¯`
#[derive(Debug, PartialEq, Eq)]
pub struct SQFuncInfo {
    /// the name used in source code
    pub cpp_func_name: &'static str,
    /// name of the defined
    pub sq_func_name: &'static str,
    /// the arguments of the function in squirrel form
    ///
    /// # Example
    /// ```
    /// let types = "string name, int id";
    /// ```
    pub types: String,
    /// the return value of the function in squirrel form
    pub return_type: String,
    /// the which vm should be used to register the function on
    pub vm: ScriptVmType,
    /// the actual function pointer
    pub function: SQFunction,
}

/// All the possible vm types titanfall 2 has
///
/// `UiClient` is used for function registration on both vms
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScriptVmType {
    /// server vm
    Server,
    /// client vm
    Client,
    /// ui vm
    Ui,
    /// ui and client used in sqfunction registration
    UiClient,
}

// todo find a better way to do this my brain is just melting rn
impl ScriptVmType {
    /// checks if the other [`ScriptVmType`] matches the current one
    ///
    /// `UiClient` is equal to `Ui` or `Client`
    pub fn is_right_vm(&self, other: &Self) -> bool {
        self == other
            || (self == &Self::UiClient && (other == &Self::Client || other == &Self::Ui))
            || (other == &Self::UiClient && (self == &Self::Client || self == &Self::Ui))
    }
}

impl Display for ScriptVmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{self:?}").to_uppercase();
        write!(f, "{s}")
    }
}

impl From<ScriptContext> for ScriptVmType {
    fn from(i: ScriptContext) -> Self {
        match i {
            ScriptContext::SERVER => Self::Server,
            ScriptContext::CLIENT => Self::Client,
            ScriptContext::UI => Self::Ui,
        }
    }
}

impl From<i32> for ScriptVmType {
    fn from(i: i32) -> Self {
        match i {
            0 => ScriptVmType::Server,
            1 => ScriptVmType::Client,
            2 => ScriptVmType::Ui,
            _ => ScriptVmType::Ui,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<ScriptContext> for ScriptVmType {
    fn into(self) -> ScriptContext {
        match self {
            ScriptVmType::Server => ScriptContext::SERVER,
            ScriptVmType::Client => ScriptContext::CLIENT,
            ScriptVmType::Ui => ScriptContext::UI,
            ScriptVmType::UiClient => {
                #[cfg(debug_assertions)]
                log::warn!("ScriptVmType::UiClient is interpreted as ScriptVmType::Client");
                ScriptContext::CLIENT
            }
        }
    }
}
