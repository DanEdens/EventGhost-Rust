#[cfg(test)]
mod mouse_recorder_tests {
    use std::sync::Arc;
    use uuid::Uuid;
    use tokio::runtime::Runtime;
    
    use crate::core::actions::system::mouse_recorder_action::{MouseRecorderAction, MouseRecorderOperation};
    use crate::core::action::{Action, ActionConfig};
    use crate::core::event::{Event, EventType};
    use crate::core::plugin::{Plugin, PluginInfo, PluginState, PluginCapability, PluginError};
    use crate::core::config::Config;
    use std::any::Any;
    use std::fmt::Debug;
    
    // Mock plugin for testing
    #[derive(Debug)]
    struct MockPlugin;
    
    impl Plugin for MockPlugin {
        fn get_id(&self) -> Uuid {
            Uuid::new_v4()
        }
        
        fn get_name(&self) -> &str {
            "Mock Plugin"
        }
        
        fn get_description(&self) -> &str {
            "Mock Plugin for Testing"
        }
        
        fn get_info(&self) -> Arc<dyn PluginInfo> {
            unimplemented!()
        }
        
        fn get_state(&self) -> PluginState {
            PluginState::Running
        }
        
        fn get_capabilities(&self) -> Vec<PluginCapability> {
            vec![]
        }
        
        async fn start(&self) -> Result<(), PluginError> {
            Ok(())
        }
        
        async fn stop(&self) -> Result<(), PluginError> {
            Ok(())
        }
        
        async fn restart(&self) -> Result<(), PluginError> {
            Ok(())
        }
        
        fn handle_event(&self, _event: &dyn Event) -> Result<(), PluginError> {
            Ok(())
        }
        
        fn configure(&self, _config: Config) -> Result<(), PluginError> {
            Ok(())
        }
    }
    
    // Mock event for testing
    struct MockEvent {
        event_type: EventType,
        payload: Option<String>,
    }
    
    impl Debug for MockEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("MockEvent")
                .field("event_type", &self.event_type)
                .finish()
        }
    }
    
    impl Event for MockEvent {
        fn get_id(&self) -> Uuid {
            Uuid::new_v4()
        }
        
        fn get_type(&self) -> EventType {
            self.event_type
        }
        
        fn get_source(&self) -> Option<String> {
            None
        }
        
        fn get_payload(&self) -> Option<String> {
            self.payload.clone()
        }
        
        fn get_timestamp(&self) -> chrono::DateTime<chrono::Utc> {
            chrono::Utc::now()
        }
        
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn test_mouse_recorder_builder() {
        // Create action with builder pattern
        let plugin = Arc::new(MockPlugin);
        let action = MouseRecorderAction::new(plugin.clone())
            .with_operation(MouseRecorderOperation::Record)
            .with_sequence_name("Test Recording")
            .with_smooth_movement(true)
            .with_speed_multiplier(1.5);
        
        // Verify properties
        assert_eq!(action.get_name(), "Mouse Recorder");
        assert_eq!(action.get_description(), "Records and plays back mouse actions");
        assert!(action.get_supported_event_types().contains(&EventType::MouseEvent));
    }
    
    #[tokio::test]
    async fn test_mouse_recorder_configuration() {
        // Create action
        let plugin = Arc::new(MockPlugin);
        let mut action = MouseRecorderAction::new(plugin.clone());
        
        // Configure it
        let config = ActionConfig {
            args: vec![
                "record".to_string(),
                "Test Recording".to_string(),
                "test.json".to_string(),
                "true".to_string(),
                "1.5".to_string(),
                "true".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = action.configure(config).await;
        assert!(result.is_ok());
        
        // Test with invalid args
        let invalid_config = ActionConfig {
            args: vec![],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = action.configure(invalid_config).await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_record_mouse_event() {
        // Create action
        let plugin = Arc::new(MockPlugin);
        let mut action = MouseRecorderAction::new(plugin.clone())
            .with_operation(MouseRecorderOperation::Record);
        
        // Start recording (by executing the action)
        let event = MockEvent {
            event_type: EventType::System,
            payload: None,
        };
        let result = action.execute(&event).await;
        assert!(result.is_ok());
        
        // Create a mouse event
        let mouse_event = MockEvent {
            event_type: EventType::MouseEvent,
            payload: Some("left,100,200,click".to_string()),
        };
        
        // Process the mouse event
        let result = action.execute(&mouse_event).await;
        assert!(result.is_ok());
        
        // Stop recording
        action.config.operation = MouseRecorderOperation::Stop;
        let result = action.execute(&event).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_mouse_recorder_operations() {
        let plugin = Arc::new(MockPlugin);
        let mut action = MouseRecorderAction::new(plugin.clone());
        
        // Test record operation
        let config = ActionConfig {
            args: vec![
                "record".to_string(),
                "Test Recording".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = action.configure(config).await;
        assert!(result.is_ok());
        
        // Test stop operation
        let config = ActionConfig {
            args: vec![
                "stop".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = action.configure(config).await;
        assert!(result.is_ok());
        
        // Test play operation
        let config = ActionConfig {
            args: vec![
                "play".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = action.configure(config).await;
        assert!(result.is_ok());
        
        // Test save operation with file path
        let config = ActionConfig {
            args: vec![
                "save".to_string(),
                "Test Recording".to_string(),
                "test.json".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = action.configure(config).await;
        assert!(result.is_ok());
        
        // Test load operation with file path
        let config = ActionConfig {
            args: vec![
                "load".to_string(),
                "".to_string(), // Empty sequence name, will be taken from the loaded file
                "test.json".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = action.configure(config).await;
        assert!(result.is_ok());
        
        // Test invalid operation
        let config = ActionConfig {
            args: vec![
                "invalid_op".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = action.configure(config).await;
        assert!(result.is_err());
    }
} 