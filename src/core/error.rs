use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Registry error: {0}")]
    Registry(#[from] crate::core::plugin::registry::RegistryError),
    
    #[error("Tree error: {0}")]
    Tree(String),
    
    #[error("Property error: {0}")]
    Property(String),
    
    #[error("Loader error: {0}")]
    Loader(#[from] crate::core::plugin::loader::LoaderError),
    
    #[error("Action error: {0}")]
    Action(#[from] crate::eg::action::base::ActionError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Plugin error: {0}")]
    Plugin(String),
    
    #[error("Event error: {0}")]
    Event(String),
    
    #[error("Config error: {0}")]
    Config(String),
    
    #[error("Event bus error: {0}")]
    EventBus(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Duplicate ID: {0}")]
    DuplicateId(String),
    
    #[error("MQTT error: {0}")]
    Mqtt(String),
    
    #[error("Redis error: {0}")]
    Redis(String),
    
    #[error("Concurrency error: {0}")]
    Concurrency(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Deserialization error: {0}")]
    Deserialization(String),
    
    #[error("Type error: {0}")]
    Type(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Other error: {0}")]
    Other(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Other(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Other(s.to_string())
    }
}

// Re-export the error type
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("Plugin not found: {0}")]
    PluginNotFound(String),

    #[error("Plugin load failed: {0}")]
    LoadFailed(String),

    #[error("Plugin already loaded: {0}")]
    AlreadyLoaded(String),
}

#[derive(Debug, Error)]
pub enum LoaderError {
    #[error("Failed to load plugin: {0}")]
    LoadFailed(String),

    #[error("Invalid plugin: {0}")]
    InvalidPlugin(String),
}

#[derive(Debug, Error)]
pub enum ActionError {
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Invalid action: {0}")]
    InvalidAction(String),
} 