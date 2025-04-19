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

/// Types of window operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowOperation {
    /// Activate and bring window to front
    Activate,
    /// Minimize window
    Minimize,
    /// Maximize window
    Maximize,
    /// Restore window to normal size
    Restore,
    /// Close window
    Close,
    /// Set window position
    SetPosition { x: i32, y: i32 },
    /// Set window size
    SetSize { width: i32, height: i32 },
    /// Set window title
    SetTitle { title: String },
    /// Find window by title
    FindByTitle { title: String, exact_match: bool },
    /// Find window by class
    FindByClass { class_name: String },
}

impl Default for WindowOperation {
    fn default() -> Self {
        WindowOperation::Activate
    }
}

/// Configuration for the WindowActionsAction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowActionsConfig {
    /// The window title or class to target
    pub window_identifier: String,
    /// Whether to use exact matching for the window identifier
    pub exact_match: bool,
    /// The operation to perform on the window
    pub operation: WindowOperation,
    /// Whether to wait for the operation to complete
    pub wait: bool,
}

impl Default for WindowActionsConfig {
    fn default() -> Self {
        Self {
            window_identifier: String::new(),
            exact_match: false,
            operation: WindowOperation::default(),
            wait: true,
        }
    }
}

/// An action that manipulates windows
#[derive(Debug)]
pub struct WindowActionsAction {
    id: Uuid,
    plugin: Arc<dyn Plugin>,
    config: WindowActionsConfig,
}

impl WindowActionsAction {
    /// Create a new WindowActionsAction
    pub fn new(plugin: Arc<dyn Plugin>) -> Self {
        Self {
            id: Uuid::new_v4(),
            plugin,
            config: WindowActionsConfig::default(),
        }
    }
    
    /// Create a new WindowActionsAction with a specific ID
    pub fn with_id(id: Uuid, plugin: Arc<dyn Plugin>) -> Self {
        Self {
            id,
            plugin,
            config: WindowActionsConfig::default(),
        }
    }
    
    /// Set the window identifier
    pub fn with_window_identifier(mut self, identifier: impl Into<String>, exact_match: bool) -> Self {
        self.config.window_identifier = identifier.into();
        self.config.exact_match = exact_match;
        self
    }
    
    /// Set the window operation
    pub fn with_operation(mut self, operation: WindowOperation) -> Self {
        self.config.operation = operation;
        self
    }
    
    /// Set whether to wait for the operation to complete
    pub fn with_wait(mut self, wait: bool) -> Self {
        self.config.wait = wait;
        self
    }
    
    /// Find window by title
    #[cfg(target_os = "windows")]
    fn find_window_by_title(&self) -> Result<winapi::shared::windef::HWND, Error> {
        use std::ffi::CString;
        use std::ptr::null_mut;
        use winapi::um::winuser::FindWindowA;
        
        let title = if self.config.exact_match {
            CString::new(self.config.window_identifier.clone())
                .map_err(|e| Error::InvalidArgument(format!("Invalid window title: {}", e)))?
        } else {
            CString::new("")
                .map_err(|e| Error::InvalidArgument(format!("Invalid empty string: {}", e)))?
        };
        
        unsafe {
            let hwnd = FindWindowA(null_mut(), title.as_ptr());
            if hwnd.is_null() {
                return Err(Error::NotFound(format!("Window not found: {}", self.config.window_identifier)));
            }
            Ok(hwnd)
        }
    }
    
    /// Find window by class name
    #[cfg(target_os = "windows")]
    fn find_window_by_class(&self) -> Result<winapi::shared::windef::HWND, Error> {
        use std::ffi::CString;
        use std::ptr::null_mut;
        use winapi::um::winuser::FindWindowA;
        
        let class_name = CString::new(self.config.window_identifier.clone())
            .map_err(|e| Error::InvalidArgument(format!("Invalid class name: {}", e)))?;
        
        unsafe {
            let hwnd = FindWindowA(class_name.as_ptr(), null_mut());
            if hwnd.is_null() {
                return Err(Error::NotFound(format!("Window with class not found: {}", self.config.window_identifier)));
            }
            Ok(hwnd)
        }
    }
    
    /// Execute window operation
    #[cfg(target_os = "windows")]
    async fn execute_window_operation(&self) -> Result<ActionResult, Error> {
        use winapi::um::winuser::{
            SetForegroundWindow, ShowWindow, SendMessageA, 
            SetWindowPos, SetWindowTextA, FindWindowExA
        };
        use winapi::um::winuser::{
            SW_MINIMIZE, SW_MAXIMIZE, SW_RESTORE, SW_SHOW,
            WM_CLOSE, SWP_NOSIZE, SWP_NOMOVE, SWP_NOZORDER
        };
        use std::ffi::CString;
        use std::ptr::null_mut;
        
        let hwnd = match self.config.operation {
            WindowOperation::FindByTitle { ref title, exact_match } => {
                let title_cstr = CString::new(title.clone())
                    .map_err(|e| Error::InvalidArgument(format!("Invalid window title: {}", e)))?;
                
                let hwnd = if exact_match {
                    unsafe { FindWindowA(null_mut(), title_cstr.as_ptr()) }
                } else {
                    // When not exact_match, we need to enumerate windows and check titles
                    // TODO: Implement window enumeration for partial title matches
                    unsafe { FindWindowA(null_mut(), title_cstr.as_ptr()) }
                };
                
                if hwnd.is_null() {
                    return Ok(ActionResult::failure(format!("Window not found: {}", title)));
                }
                
                hwnd
            },
            WindowOperation::FindByClass { ref class_name } => {
                let class_cstr = CString::new(class_name.clone())
                    .map_err(|e| Error::InvalidArgument(format!("Invalid class name: {}", e)))?;
                
                let hwnd = unsafe { FindWindowA(class_cstr.as_ptr(), null_mut()) };
                
                if hwnd.is_null() {
                    return Ok(ActionResult::failure(format!("Window with class not found: {}", class_name)));
                }
                
                hwnd
            },
            _ => {
                // For other operations, use the window identifier to find the window
                if self.config.window_identifier.is_empty() {
                    return Ok(ActionResult::failure("Window identifier cannot be empty"));
                }
                
                let title_cstr = CString::new(self.config.window_identifier.clone())
                    .map_err(|e| Error::InvalidArgument(format!("Invalid window identifier: {}", e)))?;
                
                let hwnd = unsafe { FindWindowA(null_mut(), title_cstr.as_ptr()) };
                
                if hwnd.is_null() {
                    return Ok(ActionResult::failure(format!("Window not found: {}", self.config.window_identifier)));
                }
                
                hwnd
            }
        };
        
        // Execute the operation
        match self.config.operation {
            WindowOperation::Activate => {
                let result = unsafe {
                    SetForegroundWindow(hwnd);
                    ShowWindow(hwnd, SW_SHOW)
                };
                
                if result == 0 {
                    Ok(ActionResult::failure("Failed to activate window"))
                } else {
                    Ok(ActionResult::success())
                }
            },
            WindowOperation::Minimize => {
                let result = unsafe { ShowWindow(hwnd, SW_MINIMIZE) };
                
                if result == 0 {
                    Ok(ActionResult::failure("Failed to minimize window"))
                } else {
                    Ok(ActionResult::success())
                }
            },
            WindowOperation::Maximize => {
                let result = unsafe { ShowWindow(hwnd, SW_MAXIMIZE) };
                
                if result == 0 {
                    Ok(ActionResult::failure("Failed to maximize window"))
                } else {
                    Ok(ActionResult::success())
                }
            },
            WindowOperation::Restore => {
                let result = unsafe { ShowWindow(hwnd, SW_RESTORE) };
                
                if result == 0 {
                    Ok(ActionResult::failure("Failed to restore window"))
                } else {
                    Ok(ActionResult::success())
                }
            },
            WindowOperation::Close => {
                let result = unsafe { SendMessageA(hwnd, WM_CLOSE, 0, 0) };
                
                if result == 0 {
                    Ok(ActionResult::success())
                } else {
                    Ok(ActionResult::failure("Failed to close window"))
                }
            },
            WindowOperation::SetPosition { x, y } => {
                let result = unsafe { 
                    SetWindowPos(
                        hwnd,
                        null_mut(),
                        x,
                        y,
                        0,
                        0,
                        SWP_NOSIZE | SWP_NOZORDER
                    )
                };
                
                if result == 0 {
                    Ok(ActionResult::failure("Failed to set window position"))
                } else {
                    Ok(ActionResult::success())
                }
            },
            WindowOperation::SetSize { width, height } => {
                let result = unsafe { 
                    SetWindowPos(
                        hwnd,
                        null_mut(),
                        0,
                        0,
                        width,
                        height,
                        SWP_NOMOVE | SWP_NOZORDER
                    )
                };
                
                if result == 0 {
                    Ok(ActionResult::failure("Failed to set window size"))
                } else {
                    Ok(ActionResult::success())
                }
            },
            WindowOperation::SetTitle { ref title } => {
                let title_cstr = CString::new(title.clone())
                    .map_err(|e| Error::InvalidArgument(format!("Invalid window title: {}", e)))?;
                
                let result = unsafe { SetWindowTextA(hwnd, title_cstr.as_ptr()) };
                
                if result == 0 {
                    Ok(ActionResult::failure("Failed to set window title"))
                } else {
                    Ok(ActionResult::success())
                }
            },
            // FindByTitle and FindByClass are handled in the match above
            _ => Ok(ActionResult::success()),
        }
    }

    /// Execute window operation (non-Windows implementation)
    #[cfg(not(target_os = "windows"))]
    async fn execute_window_operation(&self) -> Result<ActionResult, Error> {
        // On non-Windows platforms, return a platform-specific error
        Ok(ActionResult::failure("Window manipulation is only supported on Windows"))
    }
}

#[async_trait]
impl Action for WindowActionsAction {
    fn get_id(&self) -> Uuid {
        self.id
    }
    
    fn get_name(&self) -> &str {
        "Window Actions"
    }
    
    fn get_description(&self) -> &str {
        "Manipulates windows (minimize, maximize, activate, etc.)"
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
        Some("system/window_actions.png".to_string())
    }
    
    fn is_configurable(&self) -> bool {
        true
    }
    
    async fn configure(&mut self, config: ActionConfig) -> Result<(), Error> {
        // Parse the configuration from the args
        // Example format: 
        // args[0] = window identifier
        // args[1] = operation type
        // args[2] = additional parameters based on operation
        if config.args.len() >= 2 {
            self.config.window_identifier = config.args[0].clone();
            
            // Parse operation
            let operation = match config.args[1].as_str() {
                "activate" => WindowOperation::Activate,
                "minimize" => WindowOperation::Minimize,
                "maximize" => WindowOperation::Maximize,
                "restore" => WindowOperation::Restore,
                "close" => WindowOperation::Close,
                "setposition" => {
                    if config.args.len() >= 4 {
                        let x = config.args[2].parse().unwrap_or(0);
                        let y = config.args[3].parse().unwrap_or(0);
                        WindowOperation::SetPosition { x, y }
                    } else {
                        return Err(Error::InvalidArgument("Missing position coordinates".to_string()));
                    }
                },
                "setsize" => {
                    if config.args.len() >= 4 {
                        let width = config.args[2].parse().unwrap_or(0);
                        let height = config.args[3].parse().unwrap_or(0);
                        WindowOperation::SetSize { width, height }
                    } else {
                        return Err(Error::InvalidArgument("Missing size dimensions".to_string()));
                    }
                },
                "settitle" => {
                    if config.args.len() >= 3 {
                        WindowOperation::SetTitle { title: config.args[2].clone() }
                    } else {
                        return Err(Error::InvalidArgument("Missing window title".to_string()));
                    }
                },
                "findbytitle" => {
                    if config.args.len() >= 3 {
                        let exact_match = if config.args.len() >= 4 {
                            config.args[3].parse().unwrap_or(false)
                        } else {
                            false
                        };
                        WindowOperation::FindByTitle { 
                            title: config.args[2].clone(),
                            exact_match,
                        }
                    } else {
                        return Err(Error::InvalidArgument("Missing title to find".to_string()));
                    }
                },
                "findbyclass" => {
                    if config.args.len() >= 3 {
                        WindowOperation::FindByClass { class_name: config.args[2].clone() }
                    } else {
                        return Err(Error::InvalidArgument("Missing class name to find".to_string()));
                    }
                },
                _ => return Err(Error::InvalidArgument(format!("Unknown window operation: {}", config.args[1]))),
            };
            
            self.config.operation = operation;
            
            // Parse exact match flag if present
            if config.args.len() >= 5 && !matches!(self.config.operation, 
                WindowOperation::SetPosition { .. } | 
                WindowOperation::SetSize { .. } |
                WindowOperation::FindByTitle { .. }
            ) {
                self.config.exact_match = config.args[4].parse().unwrap_or(false);
            }
            
            // Parse wait flag if present
            if config.args.len() >= 6 {
                self.config.wait = config.args[5].parse().unwrap_or(true);
            }
        } else {
            return Err(Error::InvalidArgument("Window action requires at least window identifier and operation arguments".to_string()));
        }
        
        Ok(())
    }
    
    async fn execute(&mut self, _event: &dyn Event) -> Result<ActionResult, Error> {
        // Clone values for task
        let config = self.config.clone();
        
        // Execute window operation in a separate task to avoid blocking
        if config.wait {
            // If we need to wait, we'll execute directly in this task
            self.execute_window_operation().await
        } else {
            // If we don't need to wait, we'll spawn a task and return immediately
            let _ = task::spawn(async move {
                let action = WindowActionsAction {
                    id: Uuid::new_v4(),
                    plugin: Arc::new(DummyPlugin {}), // We need a dummy plugin for the spawned task
                    config,
                };
                
                let _ = action.execute_window_operation().await;
            });
            
            Ok(ActionResult::success())
        }
    }
    
    fn validate(&self) -> Result<(), Error> {
        if self.config.window_identifier.is_empty() && !matches!(self.config.operation, 
            WindowOperation::FindByTitle { .. } | 
            WindowOperation::FindByClass { .. }
        ) {
            return Err(Error::InvalidArgument("Window identifier cannot be empty".to_string()));
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
        "Dummy plugin for window actions"
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
    
    #[tokio::test]
    #[cfg(target_os = "windows")]
    async fn test_window_action_configuration() {
        // Create plugin and action
        let plugin = Arc::new(TestPlugin);
        let mut action = WindowActionsAction::new(plugin);
        
        // Configure action
        let config = ActionConfig {
            args: vec![
                "Notepad".to_string(),
                "activate".to_string(),
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = action.configure(config).await;
        assert!(result.is_ok());
        
        // Check configuration
        assert_eq!(action.config.window_identifier, "Notepad");
        match action.config.operation {
            WindowOperation::Activate => {},
            _ => panic!("Expected Activate operation"),
        }
    }
    
    #[tokio::test]
    #[cfg(target_os = "windows")]
    async fn test_window_action_invalid_configuration() {
        // Create plugin and action
        let plugin = Arc::new(TestPlugin);
        let mut action = WindowActionsAction::new(plugin);
        
        // Configure action with invalid args
        let config = ActionConfig {
            args: vec!["Notepad".to_string()],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = action.configure(config).await;
        assert!(result.is_err());
    }
} 