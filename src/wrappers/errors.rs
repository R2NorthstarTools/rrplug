use thiserror::Error;

use super::northstar::ScriptVmType;

#[derive(Error, Debug)]
pub enum PluginCreationError {
    #[error("A function was actually a nullptr")]
    NoneFunction,

    #[error("failed to build Squirrel because of missing data {0}")]
    SquirrelMissingData(String),

    #[error("attempted to merge two difrent squirrel builders {0} and {1}")]
    MergeDifError(ScriptVmType,ScriptVmType),

    #[error("missing ScriptVmType when attempting a merge")]
    NoScriptVmType,
}

#[derive(Error, Debug)]
pub enum SquirrelGetError {
    #[error("some else has the lock on this ref to CSquirrelVM")]
    LockedCSquirrelVM
}