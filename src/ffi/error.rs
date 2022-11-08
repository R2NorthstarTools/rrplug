use thiserror::Error;


#[derive(Error, Debug)]
pub enum PluginError {
    #[error("operation failed, error code is {0}",)]
    Failed(i32),
}