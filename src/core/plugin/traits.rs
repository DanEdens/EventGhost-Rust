use std::any::Any;
use async_trait::async_trait;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::core::{Error, Event};
use crate::core::config::Config;

/// Metadata about a plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    /// Unique plugin identifier
    pub id: Uuid,
    /// Plugin name
    pub name: String,
    /// Plugin description
    pub description: String,
    /// Plugin version
    pub version: String,
    /// Plugin author
    pub author: String,
    /// Plugin homepage
    pub homepage: Option<String>,
    /// Supported platforms
    pub platforms: Vec<String>,
    /// Plugin capabilities
    pub capabilities: Vec<String>,
}

/// Plugin capability flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginCapability {
    /// Can generate events
    EventGenerator,
    /// Can handle events
    EventHandler,
    /// Has GUI configuration
    Configurable,
    /// Can be hot-reloaded
    HotReload,
    /// Has persistent state
    Stateful,
}

/// Plugin state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginState {
    /// Plugin is created but not initialized
    Created,
    /// Plugin is initialized but not started
    Initialized,
    /// Plugin is running
    Running,
    /// Plugin is stopped
    Stopped,
    /// Plugin has failed
    Failed,
}

/// Base trait for plugin functionality
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Get plugin information
    fn get_info(&self) -> PluginInfo;
    
    /// Get plugin capabilities
    fn get_capabilities(&self) -> Vec<PluginCapability>;
    
    /// Get current plugin state
    fn get_state(&self) -> PluginState;
    
    /// Initialize the plugin
    async fn initialize(&mut self) -> Result<(), Error>;
    
    /// Start the plugin
    async fn start(&mut self) -> Result<(), Error>;
    
    /// Stop the plugin
    async fn stop(&mut self) -> Result<(), Error>;
    
    /// Handle an event
    async fn handle_event(&mut self, event: &dyn Event) -> Result<(), Error>;
    
    /// Get plugin configuration
    fn get_config(&self) -> Option<&Config>;
    
    /// Update plugin configuration
    async fn update_config(&mut self, config: Config) -> Result<(), Error>;
    
    /// Get plugin state as Any for downcasting
    fn as_any(&self) -> &dyn Any;
}

/// Trait for plugins that can generate events
#[async_trait]
pub trait EventGenerator {
    /// Generate an event
    async fn generate_event(&mut self) -> Result<Box<dyn Event>, Error>;
}

/// Trait for plugins that can be configured through GUI
#[async_trait]
pub trait Configurable {
    /// Show configuration dialog
    async fn show_config(&mut self) -> Result<bool, Error>;
    
    /// Validate configuration
    async fn validate_config(&self, config: &Config) -> Result<(), Error>;
}

/// Trait for plugins with persistent state
#[async_trait]
pub trait Stateful {
    /// Save plugin state
    async fn save_state(&self) -> Result<Vec<u8>, Error>;
    
    /// Restore plugin state
    async fn restore_state(&mut self, state: &[u8]) -> Result<(), Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin {
        info: PluginInfo,
        state: PluginState,
    }

    #[async_trait]
    impl Plugin for TestPlugin {
        // TODO: Implement test plugin
        unimplemented!()
    }

    #[tokio::test]
    async fn test_plugin_lifecycle() {
        // TODO: Test plugin lifecycle
        unimplemented!()
    }
} 