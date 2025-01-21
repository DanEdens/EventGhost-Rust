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
use crate::core::plugin::traits::{PluginState, PluginCapability};
use async_trait::async_trait;

/// Mock plugin for testing
pub struct MockPlugin {
    info: PluginInfo,
    state: PluginState,
}

impl MockPlugin {
    pub fn new() -> Self {
        MockPlugin {
            info: PluginInfo {
                id: uuid::Uuid::new_v4(),
                name: "Mock Plugin".to_string(),
                description: "A mock plugin for testing".to_string(),
                author: "Test Author".to_string(),
                version: "1.0.0".to_string(),
                capabilities: vec![],
                homepage: None,
                platforms: vec!["all".to_string()],
            },
            state: PluginState::Stopped,
        }
    }
}

#[async_trait]
impl Plugin for MockPlugin {
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
        Ok(())
    }
    
    async fn start(&mut self) -> Result<(), Error> {
        self.state = PluginState::Running;
        Ok(())
    }
    
    async fn stop(&mut self) -> Result<(), Error> {
        self.state = PluginState::Stopped;
        Ok(())
    }
    
    async fn handle_event(&mut self, _event: &dyn Event) -> Result<(), Error> {
        Ok(())
    }
    
    fn get_config(&self) -> Option<&Config> {
        None
    }
    
    async fn update_config(&mut self, _config: Config) -> Result<(), Error> {
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

/// Mock event for testing
#[derive(Debug)]
pub struct MockEvent {
    id: String,
    event_type: EventType,
    payload: EventPayload,
    timestamp: DateTime<Local>,
    source: Option<String>,
}

impl MockEvent {
    pub fn new(event_type: EventType, payload: EventPayload) -> Self {
        MockEvent {
            id: Uuid::new_v4().to_string(),
            event_type,
            payload,
            timestamp: Local::now(),
            source: None,
        }
    }
}

impl Event for MockEvent {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_type(&self) -> EventType {
        self.event_type
    }
    
    fn get_payload(&self) -> &EventPayload {
        &self.payload
    }
    
    fn get_timestamp(&self) -> DateTime<Local> {
        self.timestamp
    }
    
    fn get_source(&self) -> Option<&str> {
        self.source.as_deref()
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn clone_event(&self) -> Box<dyn Event + Send + Sync> {
        Box::new(MockEvent {
            id: self.id.clone(),
            event_type: self.event_type,
            payload: self.payload.clone(),
            timestamp: self.timestamp,
            source: self.source.clone(),
        })
    }
}

/// Mock event handler for testing
pub struct MockEventHandler {
    handled_events: Vec<EventType>,
}

impl MockEventHandler {
    pub fn new() -> Self {
        Self {
            handled_events: Vec::new(),
        }
    }
}

impl EventHandler for MockEventHandler {
    fn handle_event(&mut self, event: &dyn Event) -> Result<(), Error> {
        self.handled_events.push(event.get_type());
        Ok(())
    }

    fn can_handle(&self, event_type: EventType) -> bool {
        println!("Can handle event type: {:?}", event_type);
        true
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
    fn test_mock_window() {
        gtk::init().expect("Failed to initialize GTK");
        
        let window = MockWindow::new();
        assert!(!window.widget.is_visible());
    }
    
    #[test]
    fn test_mock_event_handler() {
        let mut handler = MockEventHandler::new();
        let event = MockEvent::new(
            EventType::KeyPress,
            EventPayload::Text("test".to_string())
        );
        
        assert!(handler.can_handle(EventType::KeyPress));
        handler.handle_event(&event).unwrap();
        assert_eq!(handler.handled_events.len(), 1);
        assert_eq!(handler.handled_events[0], EventType::KeyPress);
    }
} 