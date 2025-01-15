use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::core::{
    Plugin, PluginInfo, Event, EventHandler, Error,
    Config, ConfigStore, ConfigError,
    Window, WindowConfig,
};

/// Mock plugin for testing
pub struct MockPlugin {
    info: PluginInfo,
    events: Arc<Mutex<Vec<Box<dyn Event>>>>,
}

impl Plugin for MockPlugin {
    fn get_info(&self) -> PluginInfo {
        // TODO: Implement mock plugin info
        unimplemented!()
    }

    fn initialize(&mut self) -> Result<(), Error> {
        // TODO: Implement mock initialization
        unimplemented!()
    }

    fn start(&mut self) -> Result<(), Error> {
        // TODO: Implement mock start
        unimplemented!()
    }

    fn stop(&mut self) -> Result<(), Error> {
        // TODO: Implement mock stop
        unimplemented!()
    }
}

/// Mock event for testing
pub struct MockEvent {
    id: String,
    payload: Vec<u8>,
}

impl Event for MockEvent {
    fn get_id(&self) -> &str {
        // TODO: Implement mock event id
        unimplemented!()
    }

    fn get_payload(&self) -> &[u8] {
        // TODO: Implement mock event payload
        unimplemented!()
    }
}

/// Mock event handler for testing
pub struct MockEventHandler {
    handled_events: Arc<Mutex<Vec<Box<dyn Event>>>>,
}

impl EventHandler for MockEventHandler {
    fn handle_event(&mut self, event: &dyn Event) -> Result<(), Error> {
        // TODO: Implement mock event handling
        unimplemented!()
    }
}

/// Mock config store for testing
pub struct MockConfigStore {
    config: Arc<Mutex<Config>>,
}

impl ConfigStore for MockConfigStore {
    fn load(&self) -> Result<Config, ConfigError> {
        // TODO: Implement mock config loading
        unimplemented!()
    }

    fn save(&self, config: &Config) -> Result<(), ConfigError> {
        // TODO: Implement mock config saving
        unimplemented!()
    }
}

/// Mock window for testing
pub struct MockWindow {
    config: WindowConfig,
    events: Arc<Mutex<Vec<String>>>,
}

impl Window for MockWindow {
    fn show(&mut self) {
        // TODO: Implement mock window show
        unimplemented!()
    }

    fn hide(&mut self) {
        // TODO: Implement mock window hide
        unimplemented!()
    }

    fn close(&mut self) {
        // TODO: Implement mock window close
        unimplemented!()
    }
} 