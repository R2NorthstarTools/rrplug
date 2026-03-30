//! Errors produced by rrplug that can be returned to the user

use std::{ffi::NulError, path::Path};
use thiserror::Error;

use crate::prelude::ScriptContext;

/// Errors that may happen during the registration process of anything
///
/// can be usually ignored since these errors would happen rarely and only when something goes wrong with northstar
#[derive(Error, Debug)]
pub enum RegisterError {
    /// invalid cstring
    #[error("some attribute contained a null char")]
    InvalidCString(#[from] NulError),

    /// A function crucial to some systems was null (this is fatal I think)
    #[error("A core function from c++ is null")]
    NoneFunction,

    /// A function that creates objects returned a null pointer
    #[error("A builder function returned None")]
    NoneResult,
}

impl RegisterError {
    /// logs the error with the builtin logger
    pub fn log(&self) {
        log::error!("{}", self)
    }
}

/// Errors that may happen when querying a cvar
#[derive(Error, Debug)]
pub enum CVarQueryError {
    /// invalid cstring
    #[error("some attribute contained a null char")]
    InvalidCString(#[from] NulError),

    /// the requested cvar doesn't exist
    #[error("the requested cvar doesn't exist")]
    NotFound,

    /// happens if the cvar interface isn't init
    #[error("the cvar interface doesn't exists yet?")]
    NoCVarInterface,
}

impl CVarQueryError {
    /// logs the error with the builtin logger
    pub fn log(&self) {
        log::error!("{}", self)
    }
}

/// Errors created by calls to sqvm functions
#[derive(Error, Debug)]
pub enum CallError {
    /// invalid function string
    #[error("function string contained a null char")]
    InvalidFunctionCString(#[from] NulError),

    /// the function that was called isn't on the sqvm
    #[error("{0} function wasn't found on the sqvm; is it global?")]
    FunctionNotFound(String),

    /// the execution of the function failed for some reason
    ///
    /// the reason is not exposed by the sqvm :(
    #[error("function failed to execute")]
    FunctionFailedToExecute,
}

impl CallError {
    /// logs the error with the builtin logger
    pub fn log(&self) {
        log::error!("{}", self)
    }
}

/// Errors from compiling a buffer on the sqvm
#[derive(Error, Debug)]
pub enum SQCompileError {
    /// the buffer failed the compile
    ///
    /// the reason is only provided if the buffer is compiled to display it
    #[error("provided code failed to compile")]
    CompileError,

    /// buffer didn't execute correctly
    #[error("compiled buffer failed to execute")]
    BufferFailedToExecute,
}

impl SQCompileError {
    /// logs the error with the builtin logger
    pub fn log(&self) {
        log::error!("{}", self)
    }
}

/// Handles errors when trying to convert a c_char pointer to [`&str`]
#[derive(Error, Debug, Default, PartialEq)]
pub enum CStringPtrError {
    /// when the pointer is null
    #[default]
    #[error("literally nothing like the pointer is null")]
    None,

    /// when the char pointer failed to be parsed as [`&str`]
    #[error("invalid string (0)")]
    Utf8Error(#[from] std::str::Utf8Error),
}

impl CStringPtrError {
    /// logs the error with the builtin logger
    pub fn log(&self) {
        log::error!("{}", self)
    }
}

/// errors that can happen when using completion feature of concommands
#[derive(Error, Debug)]
pub enum CompletionError {
    /// happens when completion slots are exhausted
    ///
    /// this happens because completion is just a 64 * 128 char buffer split into 64 chunks of 128 bytes
    #[error("no more completion slots remain")]
    NoCompletionSlotsLeft,
}

impl CompletionError {
    /// logs the error with the builtin logger
    pub fn log(&self) {
        log::error!("{}", self)
    }
}

/// errors that happen when acquiring external interfaces from a pointer or dll name
#[derive(Error, Debug)]
pub enum InterfaceGetterError<'a> {
    /// the name of the dll or interface is not a valid cstring
    #[error(transparent)]
    InvalidFunctionCString(#[from] NulError),

    /// when the dll doesn't have a create interface
    #[error("dll {0} doesn't have a create interface function")]
    NullCreateInterface(usize),

    /// an error from the win api
    #[error(transparent)]
    WinApiError(#[from] windows::core::Error),

    /// happens when `CreateInterface` returns a null pointer aka the interface doesn't exists
    #[error("{0} wasn't found in the module; check the name or dll name")]
    InterfaceNotFound(&'a str),
}

impl InterfaceGetterError<'_> {
    /// logs the error with the builtin logger
    pub fn log(&self) {
        log::error!("{}", self)
    }
}

/// errors that happen when acquiring external interfaces from a pointer or dll name
#[derive(Error, Debug)]
pub enum SQFunctionRegistrationError {
    /// happens when the sqfunction could not be registered
    #[error("{0} not be registered")]
    FailedRegistration(&'static str),

    /// happens when the sqfunction gets attempted to be registered in the wrong wm context
    #[error("{0} was attempted to be registered in the wrong context : expected {1} got {2}")]
    WrongContext(&'static str, ScriptContext, &'static str),
}

impl SQFunctionRegistrationError {
    /// logs the error with the builtin logger
    pub fn log(&self) {
        log::error!("{}", self)
    }
}

/// errors that happen when acquiring external interfaces from a pointer or dll name
#[derive(Error, Debug)]
pub enum FsOpenError<'a> {
    /// error for failed path conversions
    #[error(transparent)]
    PathToCstringFailed(#[from] NulError),

    /// error for failed path conversions
    #[error("{0} could not convert path to String")]
    PathToStringFailed(&'a Path),

    /// happens when the file wasn't found
    #[error("{0} could not be found")]
    NotFound(&'a Path),
}

impl FsOpenError<'_> {
    /// logs the error with the builtin logger
    pub fn log(&self) {
        log::error!("{}", self)
    }
}
