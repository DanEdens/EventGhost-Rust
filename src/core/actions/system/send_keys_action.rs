use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;
use tokio::task;
use serde::{Serialize, Deserialize};

use crate::core::action::{Action, ActionConfig, ActionResult};
use crate::core::Error;
use crate::core::event::{Event, EventType};
use crate::core::plugin::Plugin;

/// Types of key operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyOperation {
    /// Press and release keys in sequence
    Type,
    /// Press and hold keys
    KeyDown,
    /// Release previously held keys
    KeyUp,
    /// Special key combinations (e.g., CTRL+ALT+DELETE)
    KeyCombo,
}

impl Default for KeyOperation {
    fn default() -> Self {
        KeyOperation::Type
    }
}

/// Configuration for the SendKeysAction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendKeysConfig {
    /// The keys to send
    pub keys: String,
    /// The operation to perform
    pub operation: KeyOperation,
    /// The target window (optional, if not provided, the active window is used)
    pub target_window: Option<String>,
    /// Whether to pause between key presses for a more natural typing rhythm
    pub natural_typing: bool,
    /// Typing speed in milliseconds per keystroke (when natural_typing is true)
    pub typing_speed_ms: u64,
}

impl Default for SendKeysConfig {
    fn default() -> Self {
        Self {
            keys: String::new(),
            operation: KeyOperation::default(),
            target_window: None,
            natural_typing: false,
            typing_speed_ms: 50, // Default to 50ms between keystrokes
        }
    }
}

/// An action that simulates keyboard input
#[derive(Debug)]
pub struct SendKeysAction {
    id: Uuid,
    plugin: Arc<dyn Plugin>,
    config: SendKeysConfig,
}

impl SendKeysAction {
    /// Create a new SendKeysAction
    pub fn new(plugin: Arc<dyn Plugin>) -> Self {
        Self {
            id: Uuid::new_v4(),
            plugin,
            config: SendKeysConfig::default(),
        }
    }
    
    /// Create a new SendKeysAction with a specific ID
    pub fn with_id(id: Uuid, plugin: Arc<dyn Plugin>) -> Self {
        Self {
            id,
            plugin,
            config: SendKeysConfig::default(),
        }
    }
    
    /// Set the keys to send
    pub fn with_keys(mut self, keys: impl Into<String>) -> Self {
        self.config.keys = keys.into();
        self
    }
    
    /// Set the operation type
    pub fn with_operation(mut self, operation: KeyOperation) -> Self {
        self.config.operation = operation;
        self
    }
    
    /// Set the target window
    pub fn with_target_window(mut self, window: impl Into<String>) -> Self {
        self.config.target_window = Some(window.into());
        self
    }
    
    /// Set the natural typing option and speed
    pub fn with_natural_typing(mut self, enabled: bool, speed_ms: u64) -> Self {
        self.config.natural_typing = enabled;
        self.config.typing_speed_ms = speed_ms;
        self
    }
    
    /// Parse special key codes in the input string
    fn parse_special_keys(&self, input: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut in_special = false;
        
        for c in input.chars() {
            if c == '{' && !in_special {
                // Start of special key
                if !current.is_empty() {
                    result.push(current);
                    current = String::new();
                }
                in_special = true;
                current.push(c);
            } else if c == '}' && in_special {
                // End of special key
                current.push(c);
                result.push(current);
                current = String::new();
                in_special = false;
            } else {
                current.push(c);
                if !in_special && current.len() == 1 {
                    // For regular characters, add them individually
                    result.push(current);
                    current = String::new();
                }
            }
        }
        
        // Add any remaining text
        if !current.is_empty() {
            result.push(current);
        }
        
        result
    }
    
    /// Execute send keys operation
    #[cfg(target_os = "windows")]
    async fn execute_send_keys(&self) -> Result<ActionResult, Error> {
        use winapi::um::winuser::{
            SendInput, INPUT, INPUT_KEYBOARD, KEYEVENTF_KEYUP, 
            VK_CONTROL, VK_MENU, VK_SHIFT, VK_RETURN, VK_TAB, VK_ESCAPE, 
            VK_BACK, VK_DELETE, VK_F1, VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, 
            VK_F7, VK_F8, VK_F9, VK_F10, VK_F11, VK_F12, VK_HOME, VK_END,
            VK_LEFT, VK_RIGHT, VK_UP, VK_DOWN, VK_PRIOR, VK_NEXT,
            MapVirtualKeyA, MAPVK_VK_TO_VSC
        };
        use std::mem::size_of;
        use std::ptr::null_mut;
        use tokio::time::sleep;
        use tokio::time::Duration;
        
        // If a target window is specified, activate it first
        if let Some(window) = &self.config.target_window {
            // Create a temporary WindowActionsAction to activate the window
            use crate::core::actions::system::window_actions::{WindowActionsAction, WindowOperation};
            
            let window_action = WindowActionsAction::new(self.plugin.clone())
                .with_window_identifier(window, false)
                .with_operation(WindowOperation::Activate);
            
            // Wait a moment for the window to activate
            sleep(Duration::from_millis(300)).await;
        }
        
        // Parse the keys string into individual keys and special key codes
        let key_sequence = self.parse_special_keys(&self.config.keys);
        
        // Create a mapping of special key codes to virtual key codes
        let special_keys: std::collections::HashMap<&str, i32> = [
            ("{ENTER}", VK_RETURN as i32),
            ("{TAB}", VK_TAB as i32),
            ("{ESC}", VK_ESCAPE as i32),
            ("{BACKSPACE}", VK_BACK as i32),
            ("{DELETE}", VK_DELETE as i32),
            ("{F1}", VK_F1 as i32),
            ("{F2}", VK_F2 as i32),
            ("{F3}", VK_F3 as i32),
            ("{F4}", VK_F4 as i32),
            ("{F5}", VK_F5 as i32),
            ("{F6}", VK_F6 as i32),
            ("{F7}", VK_F7 as i32),
            ("{F8}", VK_F8 as i32),
            ("{F9}", VK_F9 as i32),
            ("{F10}", VK_F10 as i32),
            ("{F11}", VK_F11 as i32),
            ("{F12}", VK_F12 as i32),
            ("{HOME}", VK_HOME as i32),
            ("{END}", VK_END as i32),
            ("{LEFT}", VK_LEFT as i32),
            ("{RIGHT}", VK_RIGHT as i32),
            ("{UP}", VK_UP as i32),
            ("{DOWN}", VK_DOWN as i32),
            ("{PGUP}", VK_PRIOR as i32),
            ("{PGDN}", VK_NEXT as i32),
            ("{CTRL}", VK_CONTROL as i32),
            ("{ALT}", VK_MENU as i32),
            ("{SHIFT}", VK_SHIFT as i32),
        ].iter().cloned().collect();
        
        // Track which modifier keys are held down
        let mut ctrl_down = false;
        let mut alt_down = false;
        let mut shift_down = false;
        
        // Function to send a single key event
        let send_key_event = |key_code: i32, flags: u32| -> Result<(), Error> {
            unsafe {
                let mut input = INPUT {
                    type_: INPUT_KEYBOARD,
                    u: std::mem::zeroed(),
                };
                
                // Set up keyboard input
                *input.u.ki_mut() = std::mem::zeroed();
                input.u.ki_mut().wVk = key_code as u16;
                input.u.ki_mut().wScan = MapVirtualKeyA(key_code as u32, MAPVK_VK_TO_VSC) as u16;
                input.u.ki_mut().dwFlags = flags;
                
                // Send the input
                let result = SendInput(1, &mut input, size_of::<INPUT>() as i32);
                
                if result != 1 {
                    return Err(Error::Other(format!("Failed to send key event: {}", key_code)));
                }
                
                Ok(())
            }
        };
        
        // Send keys based on the operation type
        match self.config.operation {
            KeyOperation::Type => {
                for key in key_sequence {
                    if key.starts_with('{') && key.ends_with('}') {
                        // Handle special keys
                        if let Some(&vk) = special_keys.get(key.as_str()) {
                            // Press the key
                            send_key_event(vk, 0)?;
                            
                            // If natural typing is enabled, add a delay
                            if self.config.natural_typing {
                                sleep(Duration::from_millis(self.config.typing_speed_ms)).await;
                            }
                            
                            // Release the key
                            send_key_event(vk, KEYEVENTF_KEYUP)?;
                        }
                    } else if key.len() == 1 {
                        // Handle regular character
                        let c = key.chars().next().unwrap();
                        let vk = c as i32;
                        
                        // Press the key
                        send_key_event(vk, 0)?;
                        
                        // If natural typing is enabled, add a delay
                        if self.config.natural_typing {
                            sleep(Duration::from_millis(self.config.typing_speed_ms)).await;
                        }
                        
                        // Release the key
                        send_key_event(vk, KEYEVENTF_KEYUP)?;
                    }
                }
            },
            KeyOperation::KeyDown => {
                for key in key_sequence {
                    if key.starts_with('{') && key.ends_with('}') {
                        // Handle special keys
                        if let Some(&vk) = special_keys.get(key.as_str()) {
                            // Press and hold the key
                            send_key_event(vk, 0)?;
                            
                            // Track modifier keys
                            match vk {
                                ctrl => if ctrl == VK_CONTROL as i32 { ctrl_down = true },
                                alt => if alt == VK_MENU as i32 { alt_down = true },
                                shift => if shift == VK_SHIFT as i32 { shift_down = true },
                                _ => {}
                            }
                        }
                    } else if key.len() == 1 {
                        // Handle regular character
                        let c = key.chars().next().unwrap();
                        let vk = c as i32;
                        
                        // Press and hold the key
                        send_key_event(vk, 0)?;
                    }
                }
            },
            KeyOperation::KeyUp => {
                for key in key_sequence {
                    if key.starts_with('{') && key.ends_with('}') {
                        // Handle special keys
                        if let Some(&vk) = special_keys.get(key.as_str()) {
                            // Release the key
                            send_key_event(vk, KEYEVENTF_KEYUP)?;
                            
                            // Track modifier keys
                            match vk {
                                ctrl => if ctrl == VK_CONTROL as i32 { ctrl_down = false },
                                alt => if alt == VK_MENU as i32 { alt_down = false },
                                shift => if shift == VK_SHIFT as i32 { shift_down = false },
                                _ => {}
                            }
                        }
                    } else if key.len() == 1 {
                        // Handle regular character
                        let c = key.chars().next().unwrap();
                        let vk = c as i32;
                        
                        // Release the key
                        send_key_event(vk, KEYEVENTF_KEYUP)?;
                    }
                }
            },
            KeyOperation::KeyCombo => {
                // For key combinations, we press all keys first, then release them in reverse order
                let mut pressed_keys = Vec::new();
                
                // Press all keys
                for key in &key_sequence {
                    let vk = if key.starts_with('{') && key.ends_with('}') {
                        // Handle special keys
                        if let Some(&vk) = special_keys.get(key.as_str()) {
                            vk
                        } else {
                            continue;
                        }
                    } else if key.len() == 1 {
                        // Handle regular character
                        let c = key.chars().next().unwrap();
                        c as i32
                    } else {
                        continue;
                    };
                    
                    // Press the key
                    send_key_event(vk, 0)?;
                    pressed_keys.push(vk);
                    
                    // Track modifier keys
                    match vk {
                        ctrl => if ctrl == VK_CONTROL as i32 { ctrl_down = true },
                        alt => if alt == VK_MENU as i32 { alt_down = true },
                        shift => if shift == VK_SHIFT as i32 { shift_down = true },
                        _ => {}
                    }
                }
                
                // Wait a moment for the combination to register
                sleep(Duration::from_millis(50)).await;
                
                // Release all keys in reverse order
                for &vk in pressed_keys.iter().rev() {
                    send_key_event(vk, KEYEVENTF_KEYUP)?;
                    
                    // Track modifier keys
                    match vk {
                        ctrl => if ctrl == VK_CONTROL as i32 { ctrl_down = false },
                        alt => if alt == VK_MENU as i32 { alt_down = false },
                        shift => if shift == VK_SHIFT as i32 { shift_down = false },
                        _ => {}
                    }
                }
            },
        }
        
        // Ensure all modifier keys are released
        if ctrl_down {
            send_key_event(VK_CONTROL as i32, KEYEVENTF_KEYUP)?;
        }
        if alt_down {
            send_key_event(VK_MENU as i32, KEYEVENTF_KEYUP)?;
        }
        if shift_down {
            send_key_event(VK_SHIFT as i32, KEYEVENTF_KEYUP)?;
        }
        
        Ok(ActionResult::success())
    }
    
    /// Execute send keys operation (non-Windows implementation)
    #[cfg(not(target_os = "windows"))]
    async fn execute_send_keys(&self) -> Result<ActionResult, Error> {
        // On non-Windows platforms, return a platform-specific error
        Ok(ActionResult::failure("Send keys is only supported on Windows"))
    }
}

#[async_trait]
impl Action for SendKeysAction {
    fn get_id(&self) -> Uuid {
        self.id
    }
    
    fn get_name(&self) -> &str {
        "Send Keys"
    }
    
    fn get_description(&self) -> &str {
        "Simulates keyboard input"
    }
    
    fn get_supported_event_types(&self) -> Vec<EventType> {
        // This action can be triggered by any event type
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
    
    fn get_icon_path(&self) -> Option<String> {
        Some("system/send_keys.png".to_string())
    }
    
    fn is_configurable(&self) -> bool {
        true
    }
    
    async fn configure(&mut self, config: ActionConfig) -> Result<(), Error> {
        // Parse the configuration from the args
        // Example format: 
        // args[0] = keys to send
        // args[1] = operation type ("type", "keydown", "keyup", "combo")
        // args[2] = target window (optional)
        // args[3] = natural typing flag (optional)
        // args[4] = typing speed in ms (optional)
        if config.args.len() >= 2 {
            self.config.keys = config.args[0].clone();
            
            // Parse operation
            let operation = match config.args[1].to_lowercase().as_str() {
                "type" => KeyOperation::Type,
                "keydown" => KeyOperation::KeyDown,
                "keyup" => KeyOperation::KeyUp,
                "combo" => KeyOperation::KeyCombo,
                _ => return Err(Error::InvalidArgument(format!("Unknown key operation: {}", config.args[1]))),
            };
            
            self.config.operation = operation;
            
            // Parse target window if provided
            if config.args.len() >= 3 && !config.args[2].is_empty() {
                self.config.target_window = Some(config.args[2].clone());
            }
            
            // Parse natural typing flag if provided
            if config.args.len() >= 4 {
                self.config.natural_typing = config.args[3].parse().unwrap_or(false);
            }
            
            // Parse typing speed if provided
            if config.args.len() >= 5 {
                self.config.typing_speed_ms = config.args[4].parse().unwrap_or(50);
            }
        } else {
            return Err(Error::InvalidArgument("Send keys action requires at least keys and operation type arguments".to_string()));
        }
        
        Ok(())
    }
    
    async fn execute(&mut self, _event: &dyn Event) -> Result<ActionResult, Error> {
        // Clone config for task
        let config = self.config.clone();
        
        // Execute in a separate task to avoid blocking
        let result = task::spawn_blocking(move || {
            // We need to create a runtime inside the blocking task to execute async code
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| Error::Other(format!("Failed to create runtime: {}", e)))?;
            
            // Create a new SendKeysAction with the cloned config
            let action = SendKeysAction {
                id: Uuid::new_v4(),
                plugin: Arc::new(DummyPlugin {}),
                config,
            };
            
            // Execute the send keys operation
            rt.block_on(action.execute_send_keys())
        }).await.map_err(|e| Error::Other(format!("Task error: {}", e)))??;
        
        Ok(result)
    }
    
    fn validate(&self) -> Result<(), Error> {
        if self.config.keys.is_empty() {
            return Err(Error::InvalidArgument("Keys to send cannot be empty".to_string()));
        }
        
        Ok(())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Dummy plugin for the spawned task
#[derive(Debug)]
struct DummyPlugin {}

impl Plugin for DummyPlugin {
    fn get_id(&self) -> Uuid {
        Uuid::nil()
    }

    fn get_name(&self) -> &str {
        "DummyPlugin"
    }

    fn get_description(&self) -> &str {
        "Dummy plugin for send keys actions"
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::plugin::{PluginInfo, PluginCapability, PluginError, PluginState};
    use crate::core::config::Config;
    use crate::core::event::EventType;
    use std::sync::Arc;
    
    #[derive(Debug)]
    struct TestPlugin;
    
    impl Plugin for TestPlugin {
        fn get_id(&self) -> Uuid {
            Uuid::new_v4()
        }
        
        fn get_name(&self) -> &str {
            "Test Plugin"
        }
        
        fn get_description(&self) -> &str {
            "Test Plugin Description"
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
    
    struct TestEvent {
        event_type: EventType,
    }
    
    impl Debug for TestEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TestEvent")
                .field("event_type", &self.event_type)
                .finish()
        }
    }
    
    impl Event for TestEvent {
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
        let plugin = Arc::new(TestPlugin);
        let action = SendKeysAction::new(plugin.clone())
            .with_keys("Hello World")
            .with_operation(KeyOperation::Type)
            .with_natural_typing(true, 100);
        
        // Verify properties
        assert_eq!(action.get_name(), "Send Keys");
        assert_eq!(action.get_description(), "Simulates keyboard input");
        assert!(action.get_supported_event_types().contains(&EventType::System));
        assert_eq!(action.config.keys, "Hello World");
        assert_eq!(action.config.typing_speed_ms, 100);
        assert!(action.config.natural_typing);
    }
    
    #[tokio::test]
    async fn test_send_keys_configuration() {
        // Create action
        let plugin = Arc::new(TestPlugin);
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
        
        let result = action.configure(config).await;
        assert!(result.is_ok());
        
        // Verify configuration
        assert_eq!(action.config.keys, "Hello World");
        match action.config.operation {
            KeyOperation::Type => {},
            _ => panic!("Expected Type operation"),
        }
        assert_eq!(action.config.target_window, Some("Notepad".to_string()));
        assert!(action.config.natural_typing);
        assert_eq!(action.config.typing_speed_ms, 75);
        
        // Test with invalid args
        let invalid_config = ActionConfig {
            args: vec!["Hello World".to_string()],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = action.configure(invalid_config).await;
        assert!(result.is_err());
    }
    
    #[test]
    fn test_parse_special_keys() {
        let plugin = Arc::new(TestPlugin);
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