use eventghost::core::plugin::traits::{Plugin, PluginInfo, PluginState, PluginCapability, PluginError};
use eventghost::core::event::{Event, EventType, EventPayload};
use eventghost::core::config::Config;
use async_trait::async_trait;
use uuid::Uuid;
use log::{info, warn, error};
use std::any::Any;

#[derive(Clone)]
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
        self.info.capabilities.clone()
    }

    fn get_state(&self) -> PluginState {
        self.state.clone()
    }

    async fn initialize(&mut self) -> Result<(), PluginError> {
        info!("[Logger] Initializing...");
        if self.state != PluginState::Created {
            return Err(PluginError::State("Plugin already initialized".into()));
        }
        self.state = PluginState::Initialized;
        Ok(())
    }

    async fn start(&mut self) -> Result<(), PluginError> {
        info!("[Logger] Starting...");
        if self.state != PluginState::Initialized {
            return Err(PluginError::State("Plugin not initialized".into()));
        }
        self.state = PluginState::Running;
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), PluginError> {
        info!("[Logger] Stopping...");
        if self.state != PluginState::Running {
            return Err(PluginError::State("Plugin not running".into()));
        }
        self.state = PluginState::Stopped;
        Ok(())
    }

    async fn handle_event(&mut self, event: &dyn Event) -> Result<(), PluginError> {
        if self.state != PluginState::Running {
            return Err(PluginError::State("Plugin not running".into()));
        }

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
    use eventghost::testing::mocks::MockEvent;

    #[tokio::test]
    async fn test_logger_plugin() {
        let mut plugin = LoggerPlugin::new();
        
        // Test initialization
        assert!(plugin.initialize().await.is_ok());
        assert_eq!(plugin.get_state(), PluginState::Initialized);
        
        // Test starting
        assert!(plugin.start().await.is_ok());
        assert_eq!(plugin.get_state(), PluginState::Running);
        
        // Test event handling
        let event = MockEvent::new(
            EventType::System,
            EventPayload::Text("test message".to_string()),
        );
        assert!(plugin.handle_event(&event).await.is_ok());
        
        // Test stopping
        assert!(plugin.stop().await.is_ok());
        assert_eq!(plugin.get_state(), PluginState::Stopped);
        
        // Test error cases
        assert!(plugin.start().await.is_err()); // Can't start when stopped
        assert!(plugin.handle_event(&event).await.is_err()); // Can't handle events when stopped
    }
} 