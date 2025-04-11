//! Plugin system for EventGhost
//! 
//! This module provides the core plugin functionality including:
//! - Plugin traits and types
//! - Plugin registry for management
//! - Plugin loading and unloading
//! - Plugin configuration
//! - Plugin state management

pub mod loader;
pub mod registry;
pub mod traits;

pub use self::traits::*;
pub use registry::PluginRegistry;
pub use loader::PluginLoader;

// Re-export common types
pub use registry::RegistryError;
pub use loader::LoaderError;

#[cfg(any(test, feature = "testing"))]
mod test_utils {
    use super::*;
    use crate::core::Error;
    use crate::core::event::{Event, EventType};
    use crate::core::config::{Config, ConfigStore, ConfigError};
    use crate::core::plugin::traits::{Plugin, PluginInfo, PluginState, PluginCapability, PluginError};
    use async_trait::async_trait;
    use std::any::Any;
    use uuid::Uuid;

    /// Mock plugin for testing
    #[derive(Debug, Clone)]
    pub struct MockPlugin {
        state: PluginState,
        name: String,
        description: String,
        author: String,
        version: String,
        id: Uuid,
        config: Option<Config>,
    }

    impl MockPlugin {
        pub fn new() -> Self {
            Self {
                state: PluginState::Created,
                name: "Mock Plugin".to_string(),
                description: "A mock plugin for testing".to_string(),
                author: "Test Author".to_string(),
                version: "1.0.0".to_string(),
                id: Uuid::new_v4(),
                config: Some(Config::new()),
            }
        }
    }

    #[async_trait]
    impl Plugin for MockPlugin {
        fn get_info(&self) -> PluginInfo {
            PluginInfo {
                id: self.id,
                name: self.name.clone(),
                description: self.description.clone(),
                author: self.author.clone(),
                version: self.version.clone(),
                homepage: None,
                platforms: vec!["all".to_string()],
                capabilities: vec![PluginCapability::EventHandler, PluginCapability::HotReload],
            }
        }
        
        fn get_capabilities(&self) -> Vec<PluginCapability> {
            vec![PluginCapability::EventHandler, PluginCapability::HotReload]
        }
        
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
            self.state = PluginState::Initialized;
            Ok(())
        }

        async fn handle_event(&mut self, _event: &dyn Event) -> Result<(), PluginError> {
            Ok(())
        }

        fn get_config(&self) -> Option<&Config> {
            self.config.as_ref()
        }

        async fn update_config(&mut self, config: Config) -> Result<(), PluginError> {
            self.config = Some(config);
            Ok(())
        }
        
        fn as_any(&self) -> &dyn Any {
            self
        }
        
        fn get_name(&self) -> &str {
            &self.name
        }
        
        fn get_description(&self) -> &str {
            &self.description
        }
        
        fn get_author(&self) -> &str {
            &self.author
        }
        
        fn get_version(&self) -> &str {
            &self.version
        }
        
        fn clone_box(&self) -> Box<dyn Plugin> {
            Box::new(self.clone())
        }
    }
}

// Re-export MockPlugin for tests
#[cfg(any(test, feature = "testing"))]
pub use test_utils::MockPlugin; 
