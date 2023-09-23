//! Errors produced by rrplug that can be retured to the user

use thiserror::Error;

/// Errors that may happen during the registration proccess of anything
///
/// can be usually ignored since these erorrs would happen rarely and only when something goes wrong with northstar
#[derive(Error, Debug)]
pub enum RegisterError {
    /// A function crutial to some systems was null (this is fatal I think)
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

/// Errors created by calls to sqvm functions
#[derive(Error, Debug)]
pub enum CallError {
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
    
    /// buffer didn't execute corretly
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
