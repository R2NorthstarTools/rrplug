use thiserror::Error;
use std::string::FromUtf8Error;


/// enum with all possible recoverable errors generated by the northstar functions
#[derive(Error, Debug)]
pub enum PluginError {
    #[error("operation failed, error code is {0}")]
    Failed(i32),
    #[error("failed to create because of {0}")]
    StringError(FromUtf8Error),
}