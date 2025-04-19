#[cfg(test)]
mod send_keys_tests {
    use std::sync::Arc;
    use uuid::Uuid;
    use tokio::runtime::Runtime;
    
    use crate::core::actions::system::send_keys_action::{SendKeysAction, KeyOperation};
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
    fn test_send_keys_builder() {
        // Create action with builder pattern
        let plugin = Arc::new(MockPlugin);
        let action = SendKeysAction::new(plugin.clone())
            .with_keys("Hello World")
            .with_operation(KeyOperation::Type)
            .with_natural_typing(true, 100);
        
        // Verify properties
        assert_eq!(action.get_name(), "Send Keys");
        assert_eq!(action.get_description(), "Simulates keyboard input");
        assert!(action.get_supported_event_types().contains(&EventType::System));
    }
    
    #[test]
    fn test_send_keys_configuration() {
        let mut rt = Runtime::new().unwrap();
        
        // Create action
        let plugin = Arc::new(MockPlugin);
        let mut action = SendKeysAction::new(plugin.clone());
        
        // Configure it
        let config = ActionConfig {
            args: vec![
                "Hello World".to_string(),
                "type".to_string(),
                "Notepad".to_string(),
                "true".to_string(),
                "75".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        // Use runtime to run the async configure method
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test with invalid configuration
        let invalid_config = ActionConfig {
            args: vec!["Hello World".to_string()], // Missing operation type
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(invalid_config));
        assert!(result.is_err());
    }
    
    #[test]
    fn test_key_operations_parsing() {
        let mut rt = Runtime::new().unwrap();
        
        // Create action
        let plugin = Arc::new(MockPlugin);
        let mut action = SendKeysAction::new(plugin.clone());
        
        // Test type operation
        let config = ActionConfig {
            args: vec![
                "Hello".to_string(),
                "type".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test keydown operation
        let config = ActionConfig {
            args: vec![
                "{SHIFT}".to_string(),
                "keydown".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test keyup operation
        let config = ActionConfig {
            args: vec![
                "{SHIFT}".to_string(),
                "keyup".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test combo operation
        let config = ActionConfig {
            args: vec![
                "{CTRL}c".to_string(),
                "combo".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_ok());
        
        // Test invalid operation
        let config = ActionConfig {
            args: vec![
                "Hello".to_string(),
                "invalid_op".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = rt.block_on(action.configure(config));
        assert!(result.is_err());
    }
    
    #[test]
    fn test_parse_special_keys() {
        let plugin = Arc::new(MockPlugin);
        let action = SendKeysAction::new(plugin.clone());
        
        // Test with simple text
        let result = action.parse_special_keys("abc");
        assert_eq!(result, vec!["a", "b", "c"]);
        
        // Test with special keys
        let result = action.parse_special_keys("a{ENTER}b");
        assert_eq!(result, vec!["a", "{ENTER}", "b"]);
        
        // Test with multiple special keys
        let result = action.parse_special_keys("{CTRL}{ALT}{DELETE}");
        assert_eq!(result, vec!["{CTRL}", "{ALT}", "{DELETE}"]);
        
        // Test with mixed content
        let result = action.parse_special_keys("Hello{ENTER}World");
        assert_eq!(result, vec!["H", "e", "l", "l", "o", "{ENTER}", "W", "o", "r", "l", "d"]);
    }
} 