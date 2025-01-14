#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Plugin error: {0}")]
    Plugin(String),
    
    #[error("Event error: {0}")]
    Event(String),
    
    #[error("GUI error: {0}")]
    Gui(String),
    
    #[error("Property error: {0}")]
    Property(String),
    
    #[error("Config error: {0}")]
    Config(String),
    
    #[error("Tree error: {0}")]
    Tree(String),
    
    #[error("Document error: {0}")]
    Document(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
} 