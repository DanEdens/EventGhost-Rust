use uuid::Uuid;
use async_trait::async_trait;
use std::any::Any;
use crate::core::plugin::traits::{Plugin, PluginInfo, PluginState, PluginCapability};
use crate::core::Error;
use crate::core::config::Config;
use crate::core::event::Event;

#[derive(Debug)]
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
        ]
    }

    fn get_state(&self) -> PluginState {
        self.state
    }

    async fn initialize(&mut self) -> Result<(), Error> {
        println!("[Logger] Initializing...");
        self.state = PluginState::Initialized;
        Ok(())
    }

    async fn start(&mut self) -> Result<(), Error> {
        println!("[Logger] Starting...");
        self.state = PluginState::Running;
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), Error> {
        println!("[Logger] Stopping...");
        self.state = PluginState::Stopped;
        Ok(())
    }

    async fn handle_event(&mut self, event: &dyn Event) -> Result<(), Error> {
        println!("[Logger] Received event: {:?}", event);
        Ok(())
    }

    fn get_config(&self) -> Option<&Config> {
        self.config.as_ref()
    }

    async fn update_config(&mut self, config: Config) -> Result<(), Error> {
        println!("[Logger] Updating config...");
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
}

#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    let plugin = Box::new(LoggerPlugin::new());
    Box::into_raw(plugin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_logger_plugin() {
        let mut plugin = LoggerPlugin::new();
        
        // Test initialization
        plugin.initialize().await.unwrap();
        assert_eq!(plugin.get_state(), PluginState::Initialized);
        
        // Test starting
        plugin.start().await.unwrap();
        assert_eq!(plugin.get_state(), PluginState::Running);
        
        // Test stopping
        plugin.stop().await.unwrap();
        assert_eq!(plugin.get_state(), PluginState::Stopped);
    }
} 