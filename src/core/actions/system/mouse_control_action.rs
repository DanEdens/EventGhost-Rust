use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;
use tokio::task;
use serde::{Serialize, Deserialize};
use crate::core::actions::{Action, ActionBuilder, ActionMode, ActionResult, ActionValidationError};
use crate::core::Error;
use crate::core::event::{Event, EventType};
use crate::core::plugin::Plugin;
use std::time::Duration;
use tokio::time::sleep;
use windows::Win32::UI::WindowsAndMessaging::{
    SetCursorPos, GetCursorPos, 
    MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, 
    MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
    MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP,
    MOUSEEVENTF_MOVE, MOUSEEVENTF_ABSOLUTE, 
    MOUSEEVENTF_WHEEL, mouse_event, 
    POINT, INPUT, INPUT_MOUSE, MOUSEEVENTF_HWHEEL,
    MOUSEINPUT, SendInput, GetSystemMetrics,
    SM_CXSCREEN, SM_CYSCREEN
};
use std::str::FromStr;
use thiserror::Error;

/// Types of mouse operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MouseOperation {
    /// Move the mouse cursor to specified coordinates
    Move,
    /// Click at current or specified position
    Click,
    /// Double-click at current or specified position
    DoubleClick,
    /// Right-click at current or specified position
    RightClick,
    /// Middle-click at current or specified position
    MiddleClick,
    /// Press and hold mouse button
    MouseDown,
    /// Release a previously held mouse button
    MouseUp,
    /// Scroll the mouse wheel
    Scroll,
    /// Drag from one position to another
    Drag,
    /// Restore the mouse position
    RestorePosition,
}

/// Types of mouse buttons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl Default for MouseOperation {
    fn default() -> Self {
        MouseOperation::Move
    }
}

impl Default for MouseButton {
    fn default() -> Self {
        MouseButton::Left
    }
}

/// Configuration for the MouseControlAction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseControlConfig {
    /// The operation to perform
    pub operation: MouseOperation,
    /// X coordinate (screen coordinates)
    pub x: Option<i32>,
    /// Y coordinate (screen coordinates)
    pub y: Option<i32>,
    /// Target X coordinate for drag operations
    pub target_x: Option<i32>,
    /// Target Y coordinate for drag operations
    pub target_y: Option<i32>,
    /// Mouse button to use
    pub button: MouseButton,
    /// Scroll amount (positive = up, negative = down)
    pub scroll_amount: i32,
    /// Whether to use smooth movement for mouse operations
    pub smooth_movement: bool,
    /// Whether to scroll horizontally instead of vertically
    pub horizontal_scroll: Option<bool>,
    /// The target window (optional, if not provided, uses screen coordinates)
    pub target_window: Option<String>,
    /// Whether to restore the mouse position after the operation
    pub restore_position: bool,
}

impl Default for MouseControlConfig {
    fn default() -> Self {
        Self {
            operation: MouseOperation::default(),
            x: None,
            y: None,
            target_x: None,
            target_y: None,
            button: MouseButton::default(),
            scroll_amount: 0,
            smooth_movement: false,
            horizontal_scroll: None,
            target_window: None,
            restore_position: false,
        }
    }
}

/// An action that simulates mouse input
#[derive(Debug)]
pub struct MouseControlAction {
    id: Uuid,
    plugin: Arc<dyn Plugin>,
    config: MouseControlConfig,
}

impl MouseControlAction {
    /// Create a new MouseControlAction
    pub fn new(plugin: Arc<dyn Plugin>) -> Self {
        Self {
            id: Uuid::new_v4(),
            plugin,
            config: MouseControlConfig::default(),
        }
    }
    
    /// Create a new MouseControlAction with a specific ID
    pub fn with_id(id: Uuid, plugin: Arc<dyn Plugin>) -> Self {
        Self {
            id,
            plugin,
            config: MouseControlConfig::default(),
        }
    }
    
    /// Set the mouse operation
    pub fn with_operation(mut self, operation: MouseOperation) -> Self {
        self.config.operation = operation;
        self
    }
    
    /// Set the mouse coordinates
    pub fn with_position(mut self, x: i32, y: i32) -> Self {
        self.config.x = Some(x);
        self.config.y = Some(y);
        self
    }
    
    /// Set the target position for drag operations
    pub fn with_target_position(mut self, x: i32, y: i32) -> Self {
        self.config.target_x = Some(x);
        self.config.target_y = Some(y);
        self
    }
    
    /// Set the mouse button
    pub fn with_button(mut self, button: MouseButton) -> Self {
        self.config.button = button;
        self
    }
    
    /// Set the scroll amount
    pub fn with_scroll_amount(mut self, amount: i32) -> Self {
        self.config.scroll_amount = amount;
        self
    }
    
    /// Set smooth movement option
    pub fn with_smooth_movement(mut self, enabled: bool) -> Self {
        self.config.smooth_movement = enabled;
        self
    }
    
    /// Set the target window
    pub fn with_target_window(mut self, window: impl Into<String>) -> Self {
        self.config.target_window = Some(window.into());
        self
    }
    
    /// Set whether to restore the original mouse position
    pub fn with_restore_position(mut self, restore: bool) -> Self {
        self.config.restore_position = restore;
        self
    }
    
    /// Get the current mouse position
    #[cfg(target_os = "windows")]
    fn get_current_position(&self) -> Result<(i32, i32), Error> {
        let mut point = POINT { x: 0, y: 0 };
        
        unsafe {
            if GetCursorPos(&mut point).as_bool() {
                Ok((point.x, point.y))
            } else {
                Err(Error::Other("Failed to get cursor position".to_string()))
            }
        }
    }
    
    /// Move the mouse to a specific position
    #[cfg(target_os = "windows")]
    fn move_mouse(&self, x: i32, y: i32, smooth: bool) -> Result<(), Error> {
        if !smooth {
            // For direct movement, we'll set the cursor position
            // This is more reliable than sending relative mouse moves
            self.set_cursor_position(x, y)?;
        } else {
            // For smooth movement, we use set_cursor_position with incremental steps
            // Get current position
            let (start_x, start_y) = self.get_current_position()?;
            
            // Calculate steps for smooth movement (use 30 steps for smoother movement)
            let steps = 30;
            let step_x = (x - start_x) as f64 / steps as f64;
            let step_y = (y - start_y) as f64 / steps as f64;
            
            // Move in steps
            for i in 1..=steps {
                let new_x = start_x + (step_x * i as f64) as i32;
                let new_y = start_y + (step_y * i as f64) as i32;
                
                self.set_cursor_position(new_x, new_y)?;
                
                // Small delay between steps - note this will block the thread
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            
            // Ensure we end up exactly at the target position
            self.set_cursor_position(x, y)?;
        }
        
        Ok(())
    }
    
    #[cfg(target_os = "windows")]
    fn set_cursor_position(&self, x: i32, y: i32) -> Result<(), Error> {
        unsafe {
            if SetCursorPos(x, y).as_bool() {
                Ok(())
            } else {
                Err(Error::Other("Failed to set cursor position".to_string()))
            }
        }
    }
    
    /// Perform a mouse click
    #[cfg(target_os = "windows")]
    fn click_mouse(&self, button: MouseButton) -> Result<(), Error> {
        // Press and release the button
        self.mouse_down(button.clone())?;
        std::thread::sleep(std::time::Duration::from_millis(10));
        self.mouse_up(button)?;
        
        Ok(())
    }
    
    /// Press a mouse button down
    #[cfg(target_os = "windows")]
    fn mouse_down(&self, button: MouseButton) -> Result<(), Error> {
        let mut input = INPUT { r#type: INPUT_MOUSE, Anonymous: Default::default() };
        
        let flag = match button {
            MouseButton::Left => MOUSEEVENTF_LEFTDOWN,
            MouseButton::Right => MOUSEEVENTF_RIGHTDOWN,
            MouseButton::Middle => MOUSEEVENTF_MIDDLEDOWN,
        };
        
        *input.Anonymous.Mouse_mut() = MOUSEINPUT {
            dx: 0,
            dy: 0,
            mouseData: 0,
            dwFlags: flag,
            time: 0,
            dwExtraInfo: 0,
        };
        
        unsafe {
            SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
        }
        
        Ok(())
    }
    
    /// Release a mouse button
    #[cfg(target_os = "windows")]
    fn mouse_up(&self, button: MouseButton) -> Result<(), Error> {
        let mut input = INPUT { r#type: INPUT_MOUSE, Anonymous: Default::default() };
        
        let flag = match button {
            MouseButton::Left => MOUSEEVENTF_LEFTUP,
            MouseButton::Right => MOUSEEVENTF_RIGHTUP,
            MouseButton::Middle => MOUSEEVENTF_MIDDLEUP,
        };
        
        *input.Anonymous.Mouse_mut() = MOUSEINPUT {
            dx: 0,
            dy: 0,
            mouseData: 0,
            dwFlags: flag,
            time: 0,
            dwExtraInfo: 0,
        };
        
        unsafe {
            SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
        }
        
        Ok(())
    }
    
    /// Scroll the mouse wheel
    #[cfg(target_os = "windows")]
    fn scroll_mouse(&self, amount: Option<i32>) -> Result<(), Error> {
        let scroll_amount = amount.unwrap_or(120); // Default scroll amount, positive for up, negative for down
        
        let mut input = INPUT { r#type: INPUT_MOUSE, Anonymous: Default::default() };
        
        let flag = if self.config.horizontal_scroll.unwrap_or(false) {
            MOUSEEVENTF_HWHEEL
        } else {
            MOUSEEVENTF_WHEEL
        };
        
        *input.Anonymous.Mouse_mut() = MOUSEINPUT {
            dx: 0,
            dy: 0,
            mouseData: scroll_amount as u32,
            dwFlags: flag,
            time: 0,
            dwExtraInfo: 0,
        };
        
        unsafe {
            SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
        }
        
        Ok(())
    }
    
    /// Execute the mouse control action
    #[cfg(target_os = "windows")]
    fn execute_mouse_control(&self) -> Result<ActionResult, Error> {
        // Get the current mouse position if we need to restore it later
        let original_position = if self.config.restore_position {
            Some(self.get_current_position()?)
        } else {
            None
        };
        
        match self.config.operation {
            MouseOperation::Move => {
                if let (Some(x), Some(y)) = (self.config.x, self.config.y) {
                    self.move_mouse(x, y, self.config.smooth_movement)?;
                } else {
                    return Err(Error::Other("Missing coordinates for mouse move operation".to_string()));
                }
            },
            
            MouseOperation::Click => {
                // Move to position if specified
                if let (Some(x), Some(y)) = (self.config.x, self.config.y) {
                    self.move_mouse(x, y, self.config.smooth_movement)?;
                }
                
                // Perform click
                self.click_mouse(self.config.button.clone())?;
            },
            
            MouseOperation::DoubleClick => {
                // Move to position if specified
                if let (Some(x), Some(y)) = (self.config.x, self.config.y) {
                    self.move_mouse(x, y, self.config.smooth_movement)?;
                }
                
                // Perform double click (two clicks with a small delay)
                self.click_mouse(self.config.button.clone())?;
                std::thread::sleep(std::time::Duration::from_millis(10));
                self.click_mouse(self.config.button.clone())?;
            },
            
            MouseOperation::RightClick => {
                // Move to position if specified
                if let (Some(x), Some(y)) = (self.config.x, self.config.y) {
                    self.move_mouse(x, y, self.config.smooth_movement)?;
                }
                
                // Perform right click
                self.click_mouse(MouseButton::Right)?;
            },
            
            MouseOperation::MiddleClick => {
                // Move to position if specified
                if let (Some(x), Some(y)) = (self.config.x, self.config.y) {
                    self.move_mouse(x, y, self.config.smooth_movement)?;
                }
                
                // Perform middle click
                self.click_mouse(MouseButton::Middle)?;
            },
            
            MouseOperation::MouseDown => {
                // Move to position if specified
                if let (Some(x), Some(y)) = (self.config.x, self.config.y) {
                    self.move_mouse(x, y, self.config.smooth_movement)?;
                }
                
                // Press mouse button
                self.mouse_down(self.config.button.clone())?;
            },
            
            MouseOperation::MouseUp => {
                // Move to position if specified
                // Move the mouse if position is provided
                if self.config.x.is_some() && self.config.y.is_some() {
                    self.move_mouse(self.config.x.unwrap(), self.config.y.unwrap(), self.config.smooth_movement)?;
                }
                
                // Perform mouse up
                self.mouse_up(self.config.button.clone())?;
            },
            MouseOperation::Scroll => {
                // Move the mouse if position is provided
                if self.config.x.is_some() && self.config.y.is_some() {
                    self.move_mouse(self.config.x.unwrap(), self.config.y.unwrap(), self.config.smooth_movement)?;
                }
                
                // Perform scroll
                self.scroll_mouse(Some(self.config.scroll_amount))?;
            },
            MouseOperation::Drag => {
                // Get target position
                let target = if let (Some(x), Some(y)) = (self.config.target_x, self.config.target_y) {
                    (x, y)
                } else {
                    return Err(Error::InvalidArgument("Target position is required for drag operations".to_string()));
                };
                
                // Move to start position
                self.move_mouse(self.config.x.unwrap(), self.config.y.unwrap(), self.config.smooth_movement)?;
                
                // Press mouse button
                self.mouse_down(self.config.button.clone())?;
                
                // Wait a moment
                std::thread::sleep(std::time::Duration::from_millis(100));
                
                // Move to target position
                self.move_mouse(target.0, target.1, self.config.smooth_movement)?;
                
                // Wait a moment
                std::thread::sleep(std::time::Duration::from_millis(100));
                
                // Release mouse button
                self.mouse_up(self.config.button.clone())?;
            },
        }
        
        // Restore original position if requested
        if let Some((x, y)) = original_position {
            self.move_mouse(x, y, self.config.smooth_movement)?;
        }
        
        Ok(ActionResult::success())
    }
    
    /// Execute mouse control operation (non-Windows implementation)
    #[cfg(not(target_os = "windows"))]
    async fn execute_mouse_control(&self) -> Result<ActionResult, Error> {
        // On non-Windows platforms, return a platform-specific error
        Ok(ActionResult::failure("Mouse control is only supported on Windows"))
    }

    /// Check if the mouse cursor is at the expected position
    #[cfg(target_os = "windows")]
    fn is_cursor_at_position(&self, expected_x: i32, expected_y: i32, tolerance: i32) -> Result<bool, Error> {
        let (x, y) = self.get_current_position()?;
        
        // Check if cursor is within tolerance range of expected position
        let x_diff = (x - expected_x).abs();
        let y_diff = (y - expected_y).abs();
        
        Ok(x_diff <= tolerance && y_diff <= tolerance)
    }

    #[cfg(target_os = "windows")]
    fn move_mouse_absolute(&self, x: i32, y: i32) -> Result<(), Error> {
        // For absolute positioning, we need to convert screen coordinates to normalized coordinates
        // The coordinates need to be in the range 0-65535
        
        // Get screen dimensions
        let screen_width = unsafe { windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics(windows::Win32::UI::WindowsAndMessaging::SM_CXSCREEN) };
        let screen_height = unsafe { windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics(windows::Win32::UI::WindowsAndMessaging::SM_CYSCREEN) };
        
        // Convert to normalized coordinates (0-65535)
        let normalized_x = (x * 65535) / screen_width;
        let normalized_y = (y * 65535) / screen_height;
        
        let mut input = INPUT { r#type: INPUT_MOUSE, Anonymous: Default::default() };
        
        *input.Anonymous.Mouse_mut() = MOUSEINPUT {
            dx: normalized_x,
            dy: normalized_y,
            mouseData: 0,
            dwFlags: MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE,
            time: 0,
            dwExtraInfo: 0,
        };
        
        unsafe {
            SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
        }
        
        Ok(())
    }
}

#[async_trait]
impl Action for MouseControlAction {
    fn get_id(&self) -> Uuid {
        self.id
    }
    
    fn get_name(&self) -> String {
        "MouseControlAction".to_string()
    }
    
    fn get_description(&self) -> String {
        match self.config.operation {
            MouseOperation::Move => "Move mouse cursor".to_string(),
            MouseOperation::Click => "Click mouse button".to_string(),
            MouseOperation::DoubleClick => "Double-click mouse button".to_string(),
            MouseOperation::RightClick => "Right-click mouse button".to_string(),
            MouseOperation::MiddleClick => "Middle-click mouse button".to_string(),
            MouseOperation::MouseDown => "Press mouse button down".to_string(),
            MouseOperation::MouseUp => "Release mouse button".to_string(),
            MouseOperation::Scroll => "Scroll mouse wheel".to_string(),
            MouseOperation::Drag => "Drag with mouse".to_string(),
            MouseOperation::RestorePosition => "Restore mouse position".to_string(),
        }
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
        Some("system/mouse_control.png".to_string())
    }
    
    fn is_configurable(&self) -> bool {
        true
    }
    
    async fn configure(&mut self, config: ActionConfig) -> Result<(), Error> {
        // Parse the configuration from the args
        // Example format: 
        // args[0] = operation type ("move", "click", "doubleclick", "rightclick", "middleclick", "mousedown", "mouseup", "scroll", "drag")
        // args[1] = x coordinate (optional, blank for current)
        // args[2] = y coordinate (optional, blank for current)
        // args[3] = button ("left", "right", "middle") or scroll amount for scroll operations or target x for drag operations
        // args[4] = smooth movement flag or target y for drag operations
        // args[5] = target window (optional)
        // args[6] = restore position flag (optional)
        if config.args.len() >= 1 {
            // Parse operation
            let operation = match config.args[0].to_lowercase().as_str() {
                "move" => MouseOperation::Move,
                "click" => MouseOperation::Click,
                "doubleclick" => MouseOperation::DoubleClick,
                "rightclick" => MouseOperation::RightClick,
                "middleclick" => MouseOperation::MiddleClick,
                "mousedown" => MouseOperation::MouseDown,
                "mouseup" => MouseOperation::MouseUp,
                "scroll" => MouseOperation::Scroll,
                "drag" => MouseOperation::Drag,
                _ => return Err(Error::InvalidArgument(format!("Unknown mouse operation: {}", config.args[0]))),
            };
            
            self.config.operation = operation;
            
            // Parse x coordinate if provided
            if config.args.len() >= 2 && !config.args[1].is_empty() {
                self.config.x = Some(config.args[1].parse().map_err(|_| {
                    Error::InvalidArgument(format!("Invalid x coordinate: {}", config.args[1]))
                })?);
            }
            
            // Parse y coordinate if provided
            if config.args.len() >= 3 && !config.args[2].is_empty() {
                self.config.y = Some(config.args[2].parse().map_err(|_| {
                    Error::InvalidArgument(format!("Invalid y coordinate: {}", config.args[2]))
                })?);
            }
            
            // Parse button or scroll amount or target x based on operation
            if config.args.len() >= 4 && !config.args[3].is_empty() {
                match self.config.operation {
                    MouseOperation::Scroll => {
                        self.config.scroll_amount = config.args[3].parse().map_err(|_| {
                            Error::InvalidArgument(format!("Invalid scroll amount: {}", config.args[3]))
                        })?;
                    },
                    MouseOperation::Drag => {
                        self.config.target_x = Some(config.args[3].parse().map_err(|_| {
                            Error::InvalidArgument(format!("Invalid target x coordinate: {}", config.args[3]))
                        })?);
                    },
                    _ => {
                        // Parse button
                        self.config.button = match config.args[3].to_lowercase().as_str() {
                            "left" => MouseButton::Left,
                            "right" => MouseButton::Right,
                            "middle" => MouseButton::Middle,
                            _ => return Err(Error::InvalidArgument(format!("Unknown mouse button: {}", config.args[3]))),
                        };
                    },
                }
            }
            
            // Parse smooth movement flag or target y
            if config.args.len() >= 5 && !config.args[4].is_empty() {
                match self.config.operation {
                    MouseOperation::Drag => {
                        self.config.target_y = Some(config.args[4].parse().map_err(|_| {
                            Error::InvalidArgument(format!("Invalid target y coordinate: {}", config.args[4]))
                        })?);
                    },
                    _ => {
                        self.config.smooth_movement = config.args[4].parse().unwrap_or(false);
                    },
                }
            }
            
            // Parse target window if provided
            if config.args.len() >= 6 && !config.args[5].is_empty() {
                self.config.target_window = Some(config.args[5].clone());
            }
            
            // Parse restore position flag if provided
            if config.args.len() >= 7 {
                self.config.restore_position = config.args[6].parse().unwrap_or(false);
            }
        } else {
            return Err(Error::InvalidArgument("Mouse control action requires at least an operation type argument".to_string()));
        }
        
        Ok(())
    }
    
    async fn execute(&mut self, _event: &dyn Event) -> Result<ActionResult, Error> {
        // Execute the mouse control action
        let result = self.execute_mouse_control();
        
        match result {
            Ok(action_result) => Ok(action_result),
            Err(e) => {
                // Log the error
                log::error!("Mouse control action failed: {}", e);
                
                // Return error result
                Ok(ActionResult::Error(e.to_string()))
            }
        }
    }
    
    fn validate(&self) -> Result<(), Error> {
        // Validate based on operation type
        match self.config.operation {
            MouseOperation::Move => {
                if self.config.x.is_none() || self.config.y.is_none() {
                    return Err(Error::ValidationError("Move operation requires x and y coordinates".to_string()));
                }
            },
            MouseOperation::Drag => {
                if self.config.target_x.is_none() || self.config.target_y.is_none() {
                    return Err(Error::ValidationError("Drag operation requires target coordinates".to_string()));
                }
            },
            MouseOperation::Scroll => {
                if self.config.scroll_amount == 0 {
                    return Err(Error::ValidationError("Scroll operation requires non-zero scroll amount".to_string()));
                }
            },
            _ => {}
        }
        
        Ok(())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn get_config(&self) -> Box<dyn std::any::Any> {
        Box::new(self.config.clone())
    }
    
    fn set_config(&mut self, config: Box<dyn std::any::Any>) -> Result<(), Error> {
        if let Ok(cfg) = config.downcast::<MouseControlConfig>() {
            self.config = *cfg;
            Ok(())
        } else {
            Err(Error::ValidationError("Invalid configuration type".to_string()))
        }
    }
}

#[cfg(not(target_os = "windows"))]
impl Action for MouseControlAction {
    fn get_name(&self) -> String {
        "MouseControlAction".to_string()
    }
    
    fn get_description(&self) -> String {
        "Mouse control (not supported on this platform)".to_string()
    }
    
    async fn execute(&mut self, _event: &dyn Event) -> Result<ActionResult, Error> {
        Err(Error::Other("Mouse control is only supported on Windows".to_string()))
    }
    
    fn validate(&self) -> Result<(), Error> {
        Err(Error::ValidationError("Mouse control is only supported on Windows".to_string()))
    }
    
    fn is_configurable(&self) -> bool {
        false
    }
    
    fn get_config(&self) -> Box<dyn std::any::Any> {
        Box::new(self.config.clone())
    }
    
    fn set_config(&mut self, config: Box<dyn std::any::Any>) -> Result<(), Error> {
        if let Ok(cfg) = config.downcast::<MouseControlConfig>() {
            self.config = *cfg;
            Ok(())
        } else {
            Err(Error::ValidationError("Invalid configuration type".to_string()))
        }
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
        "Dummy plugin for mouse control actions"
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
    fn test_mouse_control_builder() {
        // Create action with builder pattern
        let plugin = Arc::new(TestPlugin);
        let action = MouseControlAction::new(plugin.clone())
            .with_operation(MouseOperation::Click)
            .with_position(100, 200)
            .with_button(MouseButton::Left)
            .with_smooth_movement(true);
        
        // Verify properties
        assert_eq!(action.get_name(), "Mouse Control");
        assert_eq!(action.get_description(), "Simulates mouse input (move, click, drag, scroll)");
        assert!(action.get_supported_event_types().contains(&EventType::System));
        assert_eq!(action.config.x, Some(100));
        assert_eq!(action.config.y, Some(200));
        assert!(action.config.smooth_movement);
        
        match action.config.button {
            MouseButton::Left => {},
            _ => panic!("Expected Left button"),
        }
        
        match action.config.operation {
            MouseOperation::Click => {},
            _ => panic!("Expected Click operation"),
        }
    }
    
    #[tokio::test]
    async fn test_mouse_control_configuration() {
        // Create action
        let plugin = Arc::new(TestPlugin);
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
        
        let result = action.configure(config).await;
        assert!(result.is_ok());
        
        // Verify configuration
        assert_eq!(action.config.x, Some(100));
        assert_eq!(action.config.y, Some(200));
        assert!(action.config.smooth_movement);
        assert_eq!(action.config.target_window, Some("Notepad".to_string()));
        assert!(action.config.restore_position);
        
        match action.config.button {
            MouseButton::Left => {},
            _ => panic!("Expected Left button"),
        }
        
        match action.config.operation {
            MouseOperation::Click => {},
            _ => panic!("Expected Click operation"),
        }
        
        // Configure it for scroll operation
        let config = ActionConfig {
            args: vec![
                "scroll".to_string(),
                "100".to_string(),
                "200".to_string(),
                "10".to_string(),     // scroll amount
                "true".to_string(),   // smooth movement
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = action.configure(config).await;
        assert!(result.is_ok());
        
        // Verify configuration
        assert_eq!(action.config.scroll_amount, 10);
        
        match action.config.operation {
            MouseOperation::Scroll => {},
            _ => panic!("Expected Scroll operation"),
        }
        
        // Configure it for drag operation
        let config = ActionConfig {
            args: vec![
                "drag".to_string(),
                "100".to_string(),     // start x
                "200".to_string(),     // start y
                "300".to_string(),     // target x
                "400".to_string(),     // target y
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = action.configure(config).await;
        assert!(result.is_ok());
        
        // Verify configuration
        assert_eq!(action.config.x, Some(100));
        assert_eq!(action.config.y, Some(200));
        assert_eq!(action.config.target_x, Some(300));
        assert_eq!(action.config.target_y, Some(400));
        
        match action.config.operation {
            MouseOperation::Drag => {},
            _ => panic!("Expected Drag operation"),
        }
        
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
    async fn test_validate_drag_operation() {
        // Create action
        let plugin = Arc::new(TestPlugin);
        let mut action = MouseControlAction::new(plugin.clone());
        
        // Configure it for drag operation with missing target coordinates
        let config = ActionConfig {
            args: vec![
                "drag".to_string(),
                "100".to_string(),     // start x
                "200".to_string(),     // start y
            ],
            enabled: true,
            should_select_on_execute: false,
        };
        
        let result = action.configure(config).await;
        assert!(result.is_ok());
        
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