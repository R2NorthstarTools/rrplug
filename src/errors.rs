//! Errors produced by rrplug that can be retured to the user

use thiserror::Error;

/// Errors that may happen during the registration proccess of anything
///
/// can be usually ignored since these erorrs would happen rarely and only when something goes wrong with northstar
#[derive(Error, Debug)]
pub enum RegisterError {
    #[error("A core function from c++ is null")]
    NoneFunction,

    #[error("A builder functin returned None")]
    NoneResult,
}

impl RegisterError {
    pub fn log(&self) {
        log::error!("{}", self)
    }
}

#[derive(Error, Debug)]
pub enum CallError {
    #[error("{0} function wasn't found on the sqvm; is it global?")]
    FunctionNotFound(String),

    #[error("function failed to execute")]
    FunctionFailedToExecute,
}

impl CallError {
    pub fn log(&self) {
        log::error!("{}", self)
    }
}

#[derive(Error, Debug)]
pub enum SQCompileError {
    #[error("provided code failed to compile")]
    CompileError,

    #[error("compiled buffer failed to execute")]
    BufferFailedToExecute,
}

impl SQCompileError {
    pub fn log(&self) {
        log::error!("{}", self)
    }
}