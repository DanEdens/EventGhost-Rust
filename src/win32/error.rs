use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Named pipe error: {0}")]
    Pipe(#[from] pipe::PipeError),
    #[error("Window error: {0}")]
    Window(#[from] window::WindowError),
    #[error("Hook error: {0}")]
    Hook(#[from] hook::HookError),
    #[error("Registry error: {0}")]
    Registry(#[from] registry::RegistryError),
    #[error("Windows API error: {0}")]
    Windows(#[from] io::Error),
    #[error("Other error: {0}")]
    Other(String),
}

pub mod pipe {
    use super::*;
    
    #[derive(Debug, Error)]
    pub enum PipeError {
        #[error("Failed to create/open pipe: {0}")]
        Creation(io::Error),
        #[error("Failed to connect to pipe: {0}")]
        Connection(io::Error),
        #[error("Failed to read from pipe: {0}")]
        Read(io::Error),
        #[error("Failed to write to pipe: {0}")]
        Write(io::Error),
    }
}

pub mod window {
    use super::*;
    
    #[derive(Debug, Error)]
    pub enum WindowError {
        #[error("Window not found")]
        NotFound,
        #[error("Failed to get window info: {0}")]
        GetInfo(io::Error),
        #[error("Failed to set window state: {0}")]
        SetState(io::Error),
        #[error("Failed to send message: {0}")]
        SendMessage(io::Error),
        #[error("Invalid window handle")]
        InvalidHandle,
        #[error("Window operation failed: {0}")]
        Operation(io::Error),
    }
}

pub mod hook {
    use super::*;
    
    #[derive(Debug, Error)]
    pub enum HookError {
        #[error("Failed to create hook: {0}")]
        Creation(io::Error),
        #[error("Failed to remove hook: {0}")]
        Removal(io::Error),
        #[error("Hook already exists")]
        AlreadyExists,
        #[error("Invalid hook type")]
        InvalidType,
        #[error("Hook operation failed: {0}")]
        Operation(io::Error),
    }
}

pub mod registry {
    use super::*;
    
    #[derive(Debug, Error)]
    pub enum RegistryError {
        #[error("Failed to open key: {0}")]
        OpenKey(io::Error),
        #[error("Failed to read value: {0}")]
        ReadValue(io::Error),
        #[error("Failed to write value: {0}")]
        WriteValue(io::Error),
        #[error("Invalid registry path")]
        InvalidPath,
        #[error("Registry operation failed: {0}")]
        Operation(io::Error),
    }
} 