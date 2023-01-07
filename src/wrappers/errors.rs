use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegisterError {
    #[error("the vector storing SqFunction s is locked some where else")]
    LockedSqFunctionVec,

    #[error("A core function from c++ is null")]
    NoneFunction,

    #[error("A builder functin returned None")]
    NoneResult
}