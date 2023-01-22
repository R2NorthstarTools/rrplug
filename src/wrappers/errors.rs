//! Errors produced by rrplug that can be retured to the user

use thiserror::Error;

/// Errors that may happen during the registration proccess of anything
///
/// can be usually ignored since these erorrs would happen rarely and only when something goes wrong with northstar
#[derive(Error, Debug)]
pub enum RegisterError {
    #[error("the vector storing SqFunction s is locked some where else")]
    LockedSqFunctionVec,

    #[error("A core function from c++ is null")]
    NoneFunction,

    #[error("A builder functin returned None")]
    NoneResult,
}
