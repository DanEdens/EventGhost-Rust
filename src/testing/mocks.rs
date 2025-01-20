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

/// Mock plugin for testing
pub struct MockPlugin {
    pub info: PluginInfo,
    pub enabled: bool,
}

impl Plugin for MockPlugin {
    fn get_info(&self) -> &PluginInfo {
        &self.info
    }
}

/// Mock event for testing
#[derive(Debug)]
pub struct MockEvent {
    id: String,
    payload: Vec<u8>,
}

impl Event for MockEvent {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_payload(&self) -> &[u8] {
        &self.payload
    }
}

/// Mock event handler for testing
pub struct MockEventHandler {
    pub received_events: Vec<Box<dyn Event>>,
}

impl EventHandler for MockEventHandler {
    fn handle_event(&mut self, event: Box<dyn Event>) {
        self.received_events.push(event);
    }
}

/// Mock config store for testing
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
        let info = PluginInfo {
            id: "mock".into(),
            name: "Mock Plugin".into(),
            description: "A mock plugin for testing".into(),
            version: "1.0.0".into(),
            author: "Test Author".into(),
        };
        
        let plugin = MockPlugin {
            info,
            enabled: false,
        };
        
        assert_eq!(plugin.get_info().id, "mock");
    }
    
    #[test]
    fn test_mock_window() {
        gtk::init().expect("Failed to initialize GTK");
        
        let window = MockWindow::new();
        assert!(!window.widget.is_visible());
    }
    
    #[test]
    fn test_mock_event_handler() {
        let mut handler = MockEventHandler {
            received_events: Vec::new(),
        };
        
        let event = MockEvent {
            id: "test.event".into(),
            payload: vec![],
        };
        handler.handle_event(Box::new(event));
        
        assert_eq!(handler.received_events.len(), 1);
    }
} 