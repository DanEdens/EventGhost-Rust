use eventghost::{
    Plugin, PluginState, EventHandler, Event, EventType, EventPayload,
    Config, Error,
};
use async_trait::async_trait;
use uuid::Uuid;
use log::{info, warn, error};

#[derive(Default)]
pub struct LoggerPlugin {
    id: Uuid,
    state: PluginState,
    config: Config,
}

impl LoggerPlugin {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            state: PluginState::Initialized,
            config: Config::new(),
        }
    }
}

#[async_trait]
impl Plugin for LoggerPlugin {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_name(&self) -> String {
        "Logger".to_string()
    }

    fn get_description(&self) -> String {
        "Logs events and system information".to_string()
    }

    fn get_state(&self) -> PluginState {
        self.state.clone()
    }

    async fn initialize(&mut self) -> Result<(), Error> {
        info!("Initializing Logger plugin");
        self.state = PluginState::Initialized;
        Ok(())
    }

    async fn start(&mut self) -> Result<(), Error> {
        info!("Starting Logger plugin");
        self.state = PluginState::Running;
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), Error> {
        info!("Stopping Logger plugin");
        self.state = PluginState::Stopped;
        Ok(())
    }

    fn get_config(&self) -> Config {
        self.config.clone()
    }

    fn update_config(&mut self, config: Config) -> Result<(), Error> {
        info!("Updating Logger plugin configuration");
        self.config = config;
        Ok(())
    }
}

#[async_trait]
impl EventHandler for LoggerPlugin {
    async fn handle_event(&mut self, event: Box<dyn Event + Send + Sync>) -> Result<(), Error> {
        match event.get_type() {
            EventType::System => {
                info!("System event: {:?} from {}", event.get_payload(), event.get_source());
            }
            EventType::Plugin => {
                info!("Plugin event: {:?} from {}", event.get_payload(), event.get_source());
            }
            EventType::User => {
                info!("User event: {:?} from {}", event.get_payload(), event.get_source());
            }
            EventType::Internal => {
                info!("Internal event: {:?} from {}", event.get_payload(), event.get_source());
            }
            EventType::KeyPress => {
                info!("KeyPress event: {:?} from {}", event.get_payload(), event.get_source());
            }
        }
        Ok(())
    }

    fn get_supported_event_types(&self) -> Vec<EventType> {
        vec![
            EventType::System,
            EventType::Plugin,
            EventType::User,
            EventType::Internal,
            EventType::KeyPress,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_logger_plugin() {
        let mut plugin = LoggerPlugin::new();
        
        // Test initialization
        assert!(plugin.initialize().await.is_ok());
        assert_eq!(plugin.get_state(), PluginState::Initialized);
        
        // Test starting
        assert!(plugin.start().await.is_ok());
        assert_eq!(plugin.get_state(), PluginState::Running);
        
        // Test stopping
        assert!(plugin.stop().await.is_ok());
        assert_eq!(plugin.get_state(), PluginState::Stopped);
    }

    #[tokio::test]
    async fn test_event_handling() {
        let mut plugin = LoggerPlugin::new();
        
        // Create a test event
        let event = TestEvent {
            event_type: EventType::System,
            payload: EventPayload::Text("test message".to_string()),
            source: "test".to_string(),
        };
        
        // Test event handling
        assert!(plugin.handle_event(Box::new(event)).await.is_ok());
    }
}

// Test event implementation
#[derive(Clone)]
struct TestEvent {
    event_type: EventType,
    payload: EventPayload,
    source: String,
}

impl Event for TestEvent {
    fn get_type(&self) -> EventType {
        self.event_type.clone()
    }

    fn get_payload(&self) -> EventPayload {
        self.payload.clone()
    }

    fn get_source(&self) -> String {
        self.source.clone()
    }
} 