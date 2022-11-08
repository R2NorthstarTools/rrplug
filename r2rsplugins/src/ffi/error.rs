use thiserror::Error;


#[derive(Error, Debug)]
pub enum PluginError {
    #[error("operation failed, error int is {0}",)]
    Failed(i32),
}