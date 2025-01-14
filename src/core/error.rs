use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Plugin error: {0}")]
    Plugin(String),
    
    #[error("Event error: {0}")]
    Event(String),
    
    #[error("GUI error: {0}")]
    Gui(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("System error: {0}")]
    System(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Named pipe error: {0}")]
    NamedPipe(String),
} 