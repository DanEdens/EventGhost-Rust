use std::any::Any;
use async_trait::async_trait;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::core::{Error, Event};
use crate::core::config::Config;
use crate::core::event::{EventHandler, EventType};

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
#[derive(Debug, Clone, PartialEq, Eq)]
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

    /// Get plugin name
    fn get_name(&self) -> &str;
    
    /// Get plugin description
    fn get_description(&self) -> &str;
    
    /// Get plugin author
    fn get_author(&self) -> &str;
    
    /// Get plugin version
    fn get_version(&self) -> &str;
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
            self.handle_event(event).await
        } else {
            Ok(())
        }
    }
    
    fn get_id(&self) -> &str {
        &self.get_info().name
    }
    
    fn get_supported_event_types(&self) -> Vec<EventType> {
        if self.get_capabilities().contains(&PluginCapability::EventHandler) {
            vec![EventType::Plugin, EventType::System]
        } else {
            vec![]
        }
    }
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
        fn get_info(&self) -> PluginInfo {
            self.info.clone()
        }

        fn get_capabilities(&self) -> Vec<PluginCapability> {
            vec![]
        }

        fn get_state(&self) -> PluginState {
            self.state
        }

        async fn initialize(&mut self) -> Result<(), Error> {
            unimplemented!();
        }

        async fn start(&mut self) -> Result<(), Error> {
            unimplemented!();
        }

        async fn stop(&mut self) -> Result<(), Error> {
            unimplemented!();
        }

        async fn handle_event(&mut self, _event: &dyn Event) -> Result<(), Error> {
            unimplemented!();
        }

        fn get_config(&self) -> Option<&Config> {
            unimplemented!();
        }

        async fn update_config(&mut self, _config: Config) -> Result<(), Error> {
            unimplemented!();
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn get_name(&self) -> &str {
            &self.info.name
        }

        fn get_description(&self) -> &str {
            &self.info.description
        }

        fn get_author(&self) -> &str {
            &self.info.author
        }

        fn get_version(&self) -> &str {
            &self.info.version
        }
    }

    // Basic tests that don't require cloning
    #[test]
    fn test_plugin_basics() {
        let plugin = TestPlugin {
            info: PluginInfo {
                id: Uuid::new_v4(),
                name: "Test Plugin".to_string(),
                description: "A test plugin".to_string(),
                version: "0.1.0".to_string(),
                author: "Test Author".to_string(),
                homepage: None,
                platforms: vec![],
                capabilities: vec![],
            },
            state: PluginState::Created,
        };

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