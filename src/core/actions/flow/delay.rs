use std::any::Any;
use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use uuid::Uuid;
use crate::core::Error;
use crate::core::action::{Action, ActionResult, ActionConfig};
use crate::core::event::{Event, EventType, EventPayload};
use crate::core::plugin::{Plugin, PluginInfo, PluginCapability, PluginState, PluginError};
use crate::core::config::Config;
use chrono::{DateTime, Local};

/// Action that introduces a delay in the execution flow
#[derive(Debug)]
pub struct DelayAction {
    id: Uuid,
    plugin: Arc<dyn Plugin>,
    duration_ms: u64,
}

impl DelayAction {
    pub fn new(plugin: Arc<dyn Plugin>) -> Self {
        Self {
            id: Uuid::new_v4(),
            plugin,
            duration_ms: 1000, // Default 1 second delay
        }
    }
}

#[async_trait]
impl Action for DelayAction {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_name(&self) -> &str {
        "Delay"
    }

    fn get_description(&self) -> &str {
        "Pauses execution for a specified duration"
    }

    fn get_supported_event_types(&self) -> Vec<EventType> {
        // Delay action can be used with any event type
        vec![
            EventType::System,
            EventType::Plugin,
            EventType::User,
            EventType::Internal,
            EventType::KeyPress,
        ]
    }

    fn get_plugin(&self) -> Arc<dyn Plugin> {
        self.plugin.clone()
    }

    async fn configure(&mut self, config: ActionConfig) -> Result<(), Error> {
        // Parse duration from args
        if let Some(duration_str) = config.args.first() {
            match duration_str.parse::<u64>() {
                Ok(duration) => {
                    self.duration_ms = duration;
                    Ok(())
                }
                Err(_) => Err(Error::InvalidArgument(format!(
                    "Invalid duration value: {}",
                    duration_str
                )))
            }
        } else {
            Ok(()) // Use default if no args provided
        }
    }

    async fn execute(&mut self, _event: &dyn Event) -> Result<ActionResult, Error> {
        // Sleep for the specified duration
        tokio::time::sleep(Duration::from_millis(self.duration_ms)).await;
        
        Ok(ActionResult::success().with_data(self.duration_ms))
    }

    fn validate(&self) -> Result<(), Error> {
        // Any positive duration is valid
        if self.duration_ms == 0 {
            Err(Error::InvalidConfiguration(
                "Delay duration must be greater than 0".to_string()
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use tokio::test;

    #[derive(Debug, Clone)]
    struct TestPlugin;

    #[async_trait]
    impl Plugin for TestPlugin {
        fn get_name(&self) -> &str { "Test" }
        fn get_description(&self) -> &str { "Test Plugin" }
        fn get_version(&self) -> &str { "1.0.0" }
        fn get_author(&self) -> &str { "Test Author" }

        fn get_info(&self) -> PluginInfo {
            PluginInfo {
                id: Uuid::new_v4(),
                name: self.get_name().to_string(),
                description: self.get_description().to_string(),
                version: self.get_version().to_string(),
                author: self.get_author().to_string(),
                homepage: None,
                platforms: vec!["all".to_string()],
                capabilities: vec![PluginCapability::ActionProvider],
            }
        }

        fn get_capabilities(&self) -> Vec<PluginCapability> {
            vec![PluginCapability::ActionProvider]
        }

        fn get_state(&self) -> PluginState {
            PluginState::Stopped
        }

        async fn initialize(&mut self) -> Result<(), PluginError> {
            Ok(())
        }

        async fn start(&mut self) -> Result<(), PluginError> {
            Ok(())
        }

        async fn stop(&mut self) -> Result<(), PluginError> {
            Ok(())
        }

        async fn handle_event(&mut self, _event: &dyn Event) -> Result<(), PluginError> {
            Ok(())
        }

        fn get_config(&self) -> Option<&Config> {
            None
        }

        async fn update_config(&mut self, _config: Config) -> Result<(), PluginError> {
            Ok(())
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn clone_box(&self) -> Box<dyn Plugin> {
            Box::new(self.clone())
        }
    }

    #[derive(Debug, Clone)]
    struct TestEvent {
        event_type: EventType,
    }

    impl Event for TestEvent {
        fn get_id(&self) -> &str {
            "test-event"
        }
        
        fn get_type(&self) -> EventType { 
            self.event_type.clone() 
        }
        
        fn get_payload(&self) -> &EventPayload {
            // Using static payload for testing
            static NONE_PAYLOAD: EventPayload = EventPayload::None;
            &NONE_PAYLOAD
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

    #[test]
    async fn test_delay_action_default() {
        let plugin = Arc::new(TestPlugin);
        let mut action = DelayAction::new(plugin);
        let event = TestEvent { event_type: EventType::System };

        // Test default delay (1000ms)
        let start = Instant::now();
        let result = action.execute(&event).await.unwrap();
        let elapsed = start.elapsed();

        assert!(result.success);
        assert!(elapsed.as_millis() >= 1000);
        assert!(elapsed.as_millis() < 1100); // Allow some overhead
    }

    #[test]
    async fn test_delay_action_custom_duration() {
        let plugin = Arc::new(TestPlugin);
        let mut action = DelayAction::new(plugin);
        let event = TestEvent { event_type: EventType::System };

        // Configure for 500ms delay
        action.configure(ActionConfig {
            args: vec!["500".to_string()],
            enabled: true,
            should_select_on_execute: false,
        }).await.unwrap();

        let start = Instant::now();
        let result = action.execute(&event).await.unwrap();
        let elapsed = start.elapsed();

        assert!(result.success);
        assert!(elapsed.as_millis() >= 500);
        assert!(elapsed.as_millis() < 600); // Allow some overhead
    }

    #[test]
    async fn test_delay_action_invalid_config() {
        let plugin = Arc::new(TestPlugin);
        let mut action = DelayAction::new(plugin);

        // Test invalid duration string
        let result = action.configure(ActionConfig {
            args: vec!["invalid".to_string()],
            enabled: true,
            should_select_on_execute: false,
        }).await;

        assert!(result.is_err());
        if let Err(Error::InvalidArgument(_)) = result {
            // Expected error
        } else {
            panic!("Expected InvalidArgument error");
        }
    }

    #[test]
    async fn test_delay_action_validation() {
        let plugin = Arc::new(TestPlugin);
        let mut action = DelayAction::new(plugin);

        // Valid by default
        assert!(action.validate().is_ok());

        // Set invalid duration
        action.duration_ms = 0;
        assert!(action.validate().is_err());
    }
} 