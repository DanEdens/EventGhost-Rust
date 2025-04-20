#[cfg(test)]
mod mouse_control_tests {
    use std::sync::Arc;
    use uuid::Uuid;
    use tokio::runtime::Runtime;
    
    use crate::core::actions::system::mouse_control_action::{MouseControlAction, MouseOperation, MouseButton};
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
            None
        }
        
        fn get_timestamp(&self) -> chrono::DateTime<chrono::Utc> {
            chrono::Utc::now()
        }
        
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn test_mouse_control_builder() {
        // Create action with builder pattern
        let plugin = Arc::new(MockPlugin);
        let action = MouseControlAction::new(plugin.clone())
            .with_operation(MouseOperation::Click)
            .with_position(100, 200)
            .with_button(MouseButton::Left)
            .with_smooth_movement(true);
        
        // Verify properties
        assert_eq!(action.get_name(), "Mouse Control");
        assert_eq!(action.get_description(), "Simulates mouse input (move, click, drag, scroll)");
        assert!(action.get_supported_event_types().contains(&EventType::System));
    }
    
    #[test]
    fn test_mouse_control_configuration() {
        let mut rt = Runtime::new().unwrap();
        
        // Create action
        let plugin = Arc::new(MockPlugin);
        let mut action = MouseControlAction::new(plugin.clone());
        
        // Configure it for click operation
        let config = ActionConfig {
            args: vec![
                "click".to_string(),
                "100".to_string(),
                "200".to_string(),
                "left".to_string(),
                "true".to_string(),
                "Notepad".to_string(),
                "true".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        // Use runtime to run the async configure method
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test with invalid configuration
        let invalid_config = ActionConfig {
            args: vec![], // Missing operation type
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(invalid_config));
        assert!(result.is_err());
    }
    
    #[test]
    fn test_mouse_operations_parsing() {
        let mut rt = Runtime::new().unwrap();
        
        // Create action
        let plugin = Arc::new(MockPlugin);
        let mut action = MouseControlAction::new(plugin.clone());
        
        // Test move operation
        let config = ActionConfig {
            args: vec![
                "move".to_string(),
                "100".to_string(),
                "200".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test click operation
        let config = ActionConfig {
            args: vec![
                "click".to_string(),
                "100".to_string(),
                "200".to_string(),
                "left".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test doubleclick operation
        let config = ActionConfig {
            args: vec![
                "doubleclick".to_string(),
                "100".to_string(),
                "200".to_string(),
                "left".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test rightclick operation
        let config = ActionConfig {
            args: vec![
                "rightclick".to_string(),
                "100".to_string(),
                "200".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test middleclick operation
        let config = ActionConfig {
            args: vec![
                "middleclick".to_string(),
                "100".to_string(),
                "200".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test mousedown operation
        let config = ActionConfig {
            args: vec![
                "mousedown".to_string(),
                "100".to_string(),
                "200".to_string(),
                "left".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test mouseup operation
        let config = ActionConfig {
            args: vec![
                "mouseup".to_string(),
                "100".to_string(),
                "200".to_string(),
                "left".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test scroll operation
        let config = ActionConfig {
            args: vec![
                "scroll".to_string(),
                "100".to_string(),
                "200".to_string(),
                "10".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test drag operation
        let config = ActionConfig {
            args: vec![
                "drag".to_string(),
                "100".to_string(),
                "200".to_string(),
                "300".to_string(),
                "400".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test invalid operation
        let config = ActionConfig {
            args: vec![
                "invalid_op".to_string(),
                "100".to_string(),
                "200".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validate_drag_operation() {
        // Create action
        let plugin = Arc::new(MockPlugin);
        let mut action = MouseControlAction::new(plugin.clone());
        
        // Configure it for drag operation with missing target coordinates
        action.config.operation = MouseOperation::Drag;
        action.config.x = Some(100);
        action.config.y = Some(200);
        action.config.target_x = None;
        action.config.target_y = None;
        
        // Validation should fail
        let result = action.validate();
        assert!(result.is_err());
        
        // Now set the target coordinates
        action.config.target_x = Some(300);
        action.config.target_y = Some(400);
        
        // Validation should now pass
        let result = action.validate();
        assert!(result.is_ok());
    }
} 