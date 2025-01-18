use std::error::Error as StdError;
use std::fmt;
use crate::core::config::ConfigError;
use crate::eg::action::ActionError;
use crate::core::plugin::{RegistryError, LoaderError};
use crate::win32;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    
    #[error("Config error: {0}")]
    Config(#[from] ConfigError),
    
    #[error("Action error: {0}")]
    Action(#[from] ActionError),
    
    #[error("Registry error: {0}")]
    Registry(String),
    
    #[error("Loader error: {0}")]
    Loader(String),
    
    #[error("Property error: {0}")]
    Property(String),
    
    #[error("Tree error: {0}")]
    Tree(String),

    #[error("Windows error: {0}")]
    Win32(#[from] win32::Error),
}

impl From<RegistryError> for Error {
    fn from(err: RegistryError) -> Self {
        Error::Registry(err.to_string())
    }
}

impl From<LoaderError> for Error {
    fn from(err: LoaderError) -> Self {
        Error::Loader(err.to_string())
    }
} 