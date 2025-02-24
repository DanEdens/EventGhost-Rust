use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::core::{
    Plugin, PluginInfo, Event,
    Error, Config, ConfigStore, ConfigError,
};
use crate::core::event::EventHandler;
use gtk::prelude::*;
use gtk::{self, Window};
use std::any::Any;
use chrono::{DateTime, Local};
use crate::core::event::{EventType, EventPayload};
use crate::core::plugin::traits::{PluginState, PluginCapability, PluginError};
use async_trait::async_trait;

/// Mock plugin for testing
#[derive(Debug, Clone)]
pub struct MockPlugin {
    state: PluginState,
    name: String,
    description: String,
    author: String,
    version: String,
}

impl MockPlugin {
    pub fn new() -> Self {
        Self {
            state: PluginState::Created,
            name: "Mock Plugin".to_string(),
            description: "A mock plugin for testing".to_string(),
            author: "Test Author".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}

#[async_trait]
impl Plugin for MockPlugin {
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

    fn get_capabilities(&self) -> Vec<PluginCapability> {
        vec![PluginCapability::EventHandler]
    }

    fn get_info(&self) -> PluginInfo {
        PluginInfo {
            id: Uuid::new_v4(),
            name: self.name.clone(),
            description: self.description.clone(),
            version: self.version.clone(),
            author: self.author.clone(),
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
        Box::new(Self {
            state: self.state.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            author: self.author.clone(),
            version: self.version.clone(),
        })
    }
}

/// Mock event for testing
#[derive(Debug, Clone)]
pub struct MockEvent {
    event_type: EventType,
    payload: EventPayload,
}

impl MockEvent {
    pub fn new(event_type: EventType, payload: EventPayload) -> Self {
        Self {
            event_type,
            payload,
        }
    }
}

impl Event for MockEvent {
    fn get_id(&self) -> &str {
        "mock-event"
    }
    
    fn get_type(&self) -> EventType {
        self.event_type.clone()
    }

    fn get_payload(&self) -> &EventPayload {
        &self.payload
    }

    fn get_timestamp(&self) -> DateTime<Local> {
        chrono::Local::now()
    }

    fn get_source(&self) -> Option<&str> {
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn clone_event(&self) -> Box<dyn Event + Send + Sync> {
        Box::new(self.clone())
    }
}

/// Mock event handler for testing
pub struct MockEventHandler {
    id: String,
    supported_types: Vec<EventType>,
}

impl MockEventHandler {
    pub fn new(id: String, supported_types: Vec<EventType>) -> Self {
        Self {
            id,
            supported_types,
        }
    }
}

#[async_trait]
impl EventHandler for MockEventHandler {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_supported_event_types(&self) -> Vec<EventType> {
        self.supported_types.clone()
    }

    async fn handle_event(&mut self, _event: &dyn Event) -> Result<(), Error> {
        Ok(())
    }
}

/// Mock config store for testing
#[derive(Debug)]
pub struct MockConfigStore {
    config: Arc<Mutex<Config>>,
}


impl ConfigStore for MockConfigStore {
    fn load(&self) -> Result<Config, ConfigError> {
        todo!("Implement mock config loading")
    }

    fn save(&self, _config: &Config) -> Result<(), ConfigError> {
        todo!("Implement mock config saving")
    }
}

/// Mock window for testing
pub struct MockWindow {
    pub widget: Window,
}

impl MockWindow {
    pub fn new() -> Self {
        let widget = Window::new();
        MockWindow { widget }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mock_plugin() {
        let plugin = MockPlugin::new();
        assert_eq!(plugin.get_name(), "Mock Plugin");
        assert_eq!(plugin.get_description(), "A mock plugin for testing");
        assert_eq!(plugin.get_author(), "Test Author");
        assert_eq!(plugin.get_version(), "1.0.0");
    }
    
    #[test]
    #[ignore] // Temporarily ignoring GTK-related test
    fn test_mock_window() {
        gtk::init().expect("Failed to initialize GTK");
        
        let window = MockWindow::new();
        assert!(!window.widget.is_visible());
    }
    
    #[tokio::test]
    async fn test_mock_event_handler() {
        let mut handler = MockEventHandler::new(
            "test_handler".to_string(),
            vec![EventType::KeyPress]
        );
        let event = MockEvent::new(
            EventType::KeyPress,
            EventPayload::Text("test".to_string())
        );
        
        assert_eq!(handler.get_supported_event_types(), vec![EventType::KeyPress]);
        let result = handler.handle_event(&event).await;
        assert!(result.is_ok());
    }
} 