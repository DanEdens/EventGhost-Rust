use crate::core::Error;
use crate::core::event::Event;
use uuid::Uuid;
use thiserror::Error;

/// Error type specific to action operations
#[derive(Debug, Error)]
pub enum ActionError {
    #[error("Action execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Action configuration failed: {0}")]
    ConfigurationFailed(String),
    #[error("Action validation failed: {0}")]
    ValidationFailed(String),
}

/// Base trait for all actions in EventGhost
/// 
/// Actions are the basic building blocks of EventGhost macros. They represent
/// operations that can be executed in response to events or user triggers.
#[async_trait::async_trait]
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
    fn configure(&mut self) -> Result<bool, Error> {
        Ok(false) // Temporarily return false for no configuration
    }
    
    /// Execute the action with an optional triggering event
    async fn execute(&mut self, event: &dyn Event) -> Result<(), Error>;
    
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
