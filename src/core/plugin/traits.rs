use std::any::Any;
use std::fmt::Debug;
use async_trait::async_trait;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::core::{Error, Event};
use crate::core::config::Config;
use crate::core::event::{EventHandler, EventType};
use thiserror::Error as ThisError;

#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("Plugin operation failed: {0}")]
    Operation(String),
    #[error("Plugin state error: {0}")]
    State(String),
    #[error("Plugin configuration error: {0}")]
    Config(String),
    #[error("Plugin event error: {0}")]
    Event(String),
    #[error("Other error: {0}")]
    Other(String),
}

impl From<PluginError> for Error {
    fn from(err: PluginError) -> Self {
        Error::Plugin(err.to_string())
    }
}

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
    pub capabilities: Vec<PluginCapability>,
}

/// Plugin capability flags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    /// Action provider
    ActionProvider,
    /// Config provider
    ConfigProvider,
}

/// Plugin state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    Error(String),
}

/// Base trait for plugin functionality
#[async_trait]
pub trait Plugin: Send + Sync + Debug {
    /// Get plugin information
    fn get_info(&self) -> PluginInfo;
    
    /// Get plugin capabilities
    fn get_capabilities(&self) -> Vec<PluginCapability>;
    
    /// Get current plugin state
    fn get_state(&self) -> PluginState;
    
    /// Initialize the plugin
    async fn initialize(&mut self) -> Result<(), PluginError>;
    
    /// Start the plugin
    async fn start(&mut self) -> Result<(), PluginError>;
    
    /// Stop the plugin
    async fn stop(&mut self) -> Result<(), PluginError>;
    
    /// Handle an event
    async fn handle_event(&mut self, event: &dyn Event) -> Result<(), PluginError>;
    
    /// Get plugin configuration
    fn get_config(&self) -> Option<&Config>;
    
    /// Update plugin configuration
    async fn update_config(&mut self, config: Config) -> Result<(), PluginError>;
    
    /// Get plugin state as Any for downcasting
    fn as_any(&self) -> &dyn Any;

    /// Get plugin name
    fn get_name(&self) -> &str;
    
    /// Get plugin description
    fn get_description(&self) -> &str;
    
    /// Get plugin author
    fn get_author(&self) -> &str;
    
    /// Get plugin version
    fn get_version(&self) -> &str;

    /// Clone the plugin
    fn clone_box(&self) -> Box<dyn Plugin>;
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

#[async_trait]
impl<T: Plugin + Send + Sync> EventHandler for T {
    async fn handle_event(&mut self, event: &dyn Event) -> Result<(), Error> {
        if self.get_capabilities().contains(&PluginCapability::EventHandler) {
            self.handle_event(event).await.map_err(|e| Error::Plugin(e.to_string()))
        } else {
            Ok(())
        }
    }
    
    fn get_id(&self) -> &str {
        self.get_name()
    }
    
    fn get_supported_event_types(&self) -> Vec<EventType> {
        if self.get_capabilities().contains(&PluginCapability::EventHandler) {
            vec![EventType::Plugin, EventType::System]
        } else {
            vec![]
        }
    }
}

// Add a blanket implementation for cloning boxed plugins
impl Clone for Box<dyn Plugin> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug)]
struct TestPlugin {
    state: PluginState,
}

impl TestPlugin {
    fn new() -> Self {
        Self {
            state: PluginState::Created,
        }
    }
}

#[async_trait]
impl Plugin for TestPlugin {
    fn get_state(&self) -> PluginState {
        self.state.clone()
    }

    async fn initialize(&mut self) -> Result<(), PluginError> {
        self.state = PluginState::Initialized;
        Ok(())
    }

    async fn start(&mut self) -> Result<(), PluginError> {
        self.state = PluginState::Running;
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), PluginError> {
        self.state = PluginState::Stopped;
        Ok(())
    }

    async fn handle_event(&mut self, _event: &dyn Event) -> Result<(), PluginError> {
        Ok(())
    }

    async fn update_config(&mut self, _config: Config) -> Result<(), PluginError> {
        Ok(())
    }

    fn get_name(&self) -> &str {
        "Test Plugin"
    }

    fn get_description(&self) -> &str {
        "A plugin for testing"
    }

    fn get_author(&self) -> &str {
        "Test Author"
    }

    fn get_version(&self) -> &str {
        "1.0.0"
    }

    fn get_capabilities(&self) -> Vec<PluginCapability> {
        vec![PluginCapability::EventHandler]
    }

    fn get_info(&self) -> PluginInfo {
        PluginInfo {
            id: uuid::Uuid::new_v4(),
            name: self.get_name().to_string(),
            description: self.get_description().to_string(),
            version: self.get_version().to_string(),
            author: self.get_author().to_string(),
            homepage: None,
            platforms: vec!["all".to_string()],
            capabilities: self.get_capabilities(),
        }
    }

    fn get_config(&self) -> Option<&Config> {
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Plugin> {
        Box::new(Self { state: self.state.clone() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic tests that don't require cloning
    #[test]
    fn test_plugin_basics() {
        let plugin = TestPlugin::new();

        assert_eq!(plugin.get_name(), "Test Plugin");
        assert_eq!(plugin.get_state(), PluginState::Created);
    }

    // Tests requiring clone support are disabled
    #[cfg(feature = "phase2_plugin_clone")]
    mod clone_tests {
        use super::*;
        
        #[test]
        fn test_plugin_clone() {
            // Clone tests will be implemented in Phase 2
        }
    }
} 