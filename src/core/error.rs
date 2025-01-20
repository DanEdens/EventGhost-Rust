use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Config error: {0}")]
    Config(String),
    
    #[error("Plugin error: {0}")]
    Plugin(String),
    
    #[error("Event error: {0}")]
    Event(String),
    
    #[error("UI error: {0}")]
    UI(String),
    
    #[error("Other error: {0}")]
    Other(String),
    
    #[error("Property error: {0}")]
    Property(String),
    
    #[error("Tree error: {0}")]
    Tree(String),
    
    #[error("Registry error: {0}")]
    Registry(String),
    
    #[error("Action error: {0}")]
    Action(String),
    
    #[error("Loader error: {0}")]
    Loader(String),

    #[error("Dialog error: {0}")]
    Dialog(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<RegistryError> for Error {
    fn from(err: RegistryError) -> Self {
        Error::Registry(err.to_string())
    }
}

impl From<ActionError> for Error {
    fn from(err: ActionError) -> Self {
        Error::Action(err.to_string())
    }
}

impl From<LoaderError> for Error {
    fn from(err: LoaderError) -> Self {
        Error::Loader(err.to_string())
    }
}

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