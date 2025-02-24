use uuid::Uuid;
use async_trait::async_trait;
use std::any::Any;
use crate::core::plugin::traits::{Plugin, PluginInfo, PluginState, PluginCapability, PluginError};
use crate::core::Error;
use crate::core::config::Config;
use crate::core::event::{Event, EventType, EventPayload};
use log::{info, warn, error};

#[derive(Debug, Clone)]
pub struct LoggerPlugin {
    info: PluginInfo,
    state: PluginState,
    config: Option<Config>,
}

impl LoggerPlugin {
    pub fn new() -> Self {
        Self {
            info: PluginInfo {
                id: Uuid::new_v4(),
                name: "Logger".to_string(),
                description: "A simple logging plugin".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                author: "EventGhost-Rust Team".to_string(),
                homepage: None,
                platforms: vec!["windows".to_string()],
                capabilities: vec![
                    PluginCapability::EventHandler,
                    PluginCapability::HotReload,
                    PluginCapability::Configurable,
                ],
            },
            state: PluginState::Created,
            config: None,
        }
    }
}

#[async_trait]
impl Plugin for LoggerPlugin {
    fn get_info(&self) -> PluginInfo {
        self.info.clone()
    }

    fn get_capabilities(&self) -> Vec<PluginCapability> {
        vec![
            PluginCapability::EventHandler,
            PluginCapability::HotReload,
            PluginCapability::Configurable,
        ]
    }

    fn get_state(&self) -> PluginState {
        self.state.clone()
    }

    async fn initialize(&mut self) -> Result<(), PluginError> {
        info!("[Logger] Initializing...");
        self.state = PluginState::Initialized;
        Ok(())
    }

    async fn start(&mut self) -> Result<(), PluginError> {
        info!("[Logger] Starting...");
        self.state = PluginState::Running;
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), PluginError> {
        info!("[Logger] Stopping...");
        self.state = PluginState::Stopped;
        Ok(())
    }

    async fn handle_event(&mut self, event: &dyn Event) -> Result<(), PluginError> {
        match event.get_type() {
            EventType::System => {
                info!("[Logger] System event: {:?} from {:?}", event.get_payload(), event.get_source());
            }
            EventType::Plugin => {
                info!("[Logger] Plugin event: {:?} from {:?}", event.get_payload(), event.get_source());
            }
            EventType::User => {
                info!("[Logger] User event: {:?} from {:?}", event.get_payload(), event.get_source());
            }
            EventType::Internal => {
                info!("[Logger] Internal event: {:?} from {:?}", event.get_payload(), event.get_source());
            }
            EventType::KeyPress => {
                info!("[Logger] KeyPress event: {:?} from {:?}", event.get_payload(), event.get_source());
            }
        }
        Ok(())
    }

    fn get_config(&self) -> Option<&Config> {
        self.config.as_ref()
    }

    async fn update_config(&mut self, config: Config) -> Result<(), PluginError> {
        info!("[Logger] Updating configuration...");
        self.config = Some(config);
        Ok(())
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

    fn clone_box(&self) -> Box<dyn Plugin> {
        Box::new(self.clone())
    }
}

#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    let plugin = Box::new(LoggerPlugin::new());
    Box::into_raw(plugin)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::mocks::MockEvent;

    #[tokio::test]
    async fn test_logger_plugin() {
        let mut plugin = LoggerPlugin::new();
        
        // Test initialization
        plugin.initialize().await.unwrap();
        assert_eq!(plugin.get_state(), PluginState::Initialized);
        
        // Test starting
        plugin.start().await.unwrap();
        assert_eq!(plugin.get_state(), PluginState::Running);
        
        // Test event handling
        let event = MockEvent::new(
            "test_event",
            EventType::System,
            EventPayload::Text("test message".to_string()),
        );
        plugin.handle_event(&event).await.unwrap();
        
        // Test stopping
        plugin.stop().await.unwrap();
        assert_eq!(plugin.get_state(), PluginState::Stopped);
    }
} 