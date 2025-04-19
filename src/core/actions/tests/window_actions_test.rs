#[cfg(test)]
mod window_actions_tests {
    use std::sync::Arc;
    use uuid::Uuid;
    use tokio::runtime::Runtime;
    
    use crate::core::actions::system::window_actions::{WindowActionsAction, WindowOperation};
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
    #[cfg(target_os = "windows")]
    fn test_window_actions_builder() {
        // Create a window action with the builder pattern
        let plugin = Arc::new(MockPlugin);
        let action = WindowActionsAction::new(plugin.clone())
            .with_window_identifier("Notepad", true)
            .with_operation(WindowOperation::Activate)
            .with_wait(true);
        
        // Verify properties
        assert_eq!(action.get_name(), "Window Actions");
        assert_eq!(
            action.get_description(),
            "Manipulates windows (minimize, maximize, activate, etc.)"
        );
        assert!(action.get_supported_event_types().contains(&EventType::System));
    }
    
    #[test]
    #[cfg(target_os = "windows")]
    fn test_window_actions_configuration() {
        let mut rt = Runtime::new().unwrap();
        
        // Create a window action
        let plugin = Arc::new(MockPlugin);
        let mut action = WindowActionsAction::new(plugin.clone());
        
        // Configure it
        let config = ActionConfig {
            args: vec![
                "Notepad".to_string(),
                "activate".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        // Use runtime to run the async configure method
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test with invalid configuration
        let invalid_config = ActionConfig {
            args: vec!["Notepad".to_string()], // Missing operation
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(invalid_config));
        assert!(result.is_err());
    }
    
    #[test]
    #[cfg(target_os = "windows")]
    fn test_window_operations_parsing() {
        let mut rt = Runtime::new().unwrap();
        
        // Create a window action
        let plugin = Arc::new(MockPlugin);
        let mut action = WindowActionsAction::new(plugin.clone());
        
        // Test activate operation
        let config = ActionConfig {
            args: vec![
                "Notepad".to_string(),
                "activate".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test minimize operation
        let config = ActionConfig {
            args: vec![
                "Notepad".to_string(),
                "minimize".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test maximize operation
        let config = ActionConfig {
            args: vec![
                "Notepad".to_string(),
                "maximize".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test setposition operation
        let config = ActionConfig {
            args: vec![
                "Notepad".to_string(),
                "setposition".to_string(),
                "100".to_string(),
                "200".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test invalid operation
        let config = ActionConfig {
            args: vec![
                "Notepad".to_string(),
                "invalid_op".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_err());
    }
} 