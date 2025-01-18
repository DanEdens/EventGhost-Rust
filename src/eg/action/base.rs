use crate::core::Error;
use crate::eg::classes::ConfigDialog;
use crate::core::event::Event;
use uuid::Uuid;
use std::fmt;

/// Error type specific to action operations
#[derive(Debug)]
pub enum ActionError {
    /// Action execution failed
    ExecutionFailed(String),
    /// Action configuration failed
    ConfigurationFailed(String),
    /// Action validation failed
    ValidationFailed(String),
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionError::ExecutionFailed(msg) => write!(f, "Action execution failed: {}", msg),
            ActionError::ConfigurationFailed(msg) => write!(f, "Action configuration failed: {}", msg),
            ActionError::ValidationFailed(msg) => write!(f, "Action validation failed: {}", msg),
        }
    }
}

impl std::error::Error for ActionError {}

impl From<ActionError> for Error {
    fn from(err: ActionError) -> Self {
        Error::Action(err.to_string())
    }
}

/// Base trait for all actions in EventGhost
/// 
/// Actions are the basic building blocks of EventGhost macros. They represent
/// operations that can be executed in response to events or user triggers.
pub trait ActionBase: Send + Sync {
    /// Get the display name of the action
    fn get_name(&self) -> &str;
    
    /// Get a human-readable description of what the action does
    fn get_description(&self) -> &str;
    
    /// Get the unique identifier for this action instance
    fn get_id(&self) -> Uuid;
    
    /// Get the ID of the plugin that owns this action
    fn get_plugin_id(&self) -> Uuid;
    
    /// Open a configuration dialog for this action if available
    fn configure(&mut self) -> Option<ConfigDialog>;
    
    /// Execute the action with an optional triggering event
    fn execute(&mut self, event: Option<&dyn Event>) -> Result<(), Error>;
    
    /// Check if the action can be executed with the given event
    fn can_execute(&self, event: Option<&dyn Event>) -> bool;
    
    /// Create a deep clone of this action
    fn clone_action(&self) -> Box<dyn ActionBase>;
}

/// Metadata about an action
#[derive(Debug, Clone)]
pub struct ActionInfo {
    pub name: String,
    pub description: String,
    pub id: Uuid,
    pub plugin_id: Uuid,
} 