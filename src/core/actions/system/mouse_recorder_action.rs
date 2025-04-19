use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;
use tokio::task;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::{Duration, Instant};
use std::thread::sleep;

use crate::core::action::{Action, ActionConfig, ActionResult};
use crate::core::Error;
use crate::core::event::{Event, EventType};
use crate::core::plugin::Plugin;
use crate::core::actions::system::mouse_control_action::{MouseButton, MouseOperation, MouseControlAction};

/// Types of mouse recorder operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MouseRecorderOperation {
    /// Start recording mouse actions
    Record,
    /// Stop recording
    Stop,
    /// Play recorded sequence
    Play,
    /// Save recorded sequence to file
    Save,
    /// Load recorded sequence from file
    Load,
}

impl Default for MouseRecorderOperation {
    fn default() -> Self {
        MouseRecorderOperation::Record
    }
}

/// A single recorded mouse action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseAction {
    /// The operation type
    pub operation: MouseOperation,
    /// X coordinate
    pub x: i32,
    /// Y coordinate
    pub y: i32,
    /// Target X coordinate (for drag operations)
    pub target_x: Option<i32>,
    /// Target Y coordinate (for drag operations)
    pub target_y: Option<i32>,
    /// Mouse button used
    pub button: MouseButton,
    /// Delay before this action (milliseconds)
    pub delay_ms: u64,
}

/// A sequence of recorded mouse actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseSequence {
    /// Name of the sequence
    pub name: String,
    /// The recorded actions
    pub actions: Vec<MouseAction>,
    /// Total duration of the sequence in milliseconds
    pub total_duration_ms: u64,
}

impl MouseSequence {
    pub fn new(name: String) -> Self {
        Self {
            name,
            actions: Vec::new(),
            total_duration_ms: 0,
        }
    }

    pub fn add_action(&mut self, action: MouseAction) {
        self.total_duration_ms += action.delay_ms;
        self.actions.push(action);
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), Error> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| Error::Other(format!("Failed to serialize sequence: {}", e)))?;
        
        let mut file = File::create(path)
            .map_err(|e| Error::Other(format!("Failed to create file: {}", e)))?;
        
        file.write_all(json.as_bytes())
            .map_err(|e| Error::Other(format!("Failed to write to file: {}", e)))?;
        
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> Result<Self, Error> {
        let mut file = File::open(path)
            .map_err(|e| Error::Other(format!("Failed to open file: {}", e)))?;
        
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| Error::Other(format!("Failed to read file: {}", e)))?;
        
        let sequence = serde_json::from_str(&contents)
            .map_err(|e| Error::Other(format!("Failed to deserialize sequence: {}", e)))?;
        
        Ok(sequence)
    }
}

/// Configuration for the MouseRecorderAction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseRecorderConfig {
    /// The operation to perform
    pub operation: MouseRecorderOperation,
    /// Name of the sequence
    pub sequence_name: String,
    /// Path to save/load the sequence
    pub file_path: Option<String>,
    /// Whether to use smooth movement for playback
    pub smooth_movement: bool,
    /// Playback speed multiplier (1.0 = normal speed)
    pub speed_multiplier: f64,
    /// Maximum recording time in seconds (0 = unlimited)
    pub max_recording_time: u64,
    /// Whether to capture keyboard events during recording
    pub capture_keyboard: bool,
    /// Whether to capture mouse move events or just clicks
    pub capture_mouse_moves: bool,
    /// Minimum movement distance to record (pixels)
    pub min_move_distance: i32,
    /// Whether to restore the mouse position after playback
    pub restore_position: bool,
}

impl Default for MouseRecorderConfig {
    fn default() -> Self {
        Self {
            operation: MouseRecorderOperation::default(),
            sequence_name: "Untitled Sequence".to_string(),
            file_path: None,
            smooth_movement: true,
            speed_multiplier: 1.0,
            max_recording_time: 0, // unlimited
            capture_keyboard: false, // not implemented yet
            capture_mouse_moves: true,
            min_move_distance: 5, // pixels
            restore_position: true,
        }
    }
}

/// A recorder for mouse actions that can play them back
#[derive(Debug)]
pub struct MouseRecorderAction {
    id: Uuid,
    plugin: Arc<dyn Plugin>,
    config: MouseRecorderConfig,
    sequence: MouseSequence,
    recording: bool,
    last_position: Option<(i32, i32)>,
    last_time: Option<Instant>,
}

impl MouseRecorderAction {
    /// Create a new MouseRecorderAction
    pub fn new(plugin: Arc<dyn Plugin>) -> Self {
        Self {
            id: Uuid::new_v4(),
            plugin,
            config: MouseRecorderConfig::default(),
            sequence: MouseSequence::new("Untitled Sequence".to_string()),
            recording: false,
            last_position: None,
            last_time: None,
        }
    }
    
    /// Create a new MouseRecorderAction with a specific ID
    pub fn with_id(id: Uuid, plugin: Arc<dyn Plugin>) -> Self {
        Self {
            id,
            plugin,
            config: MouseRecorderConfig::default(),
            sequence: MouseSequence::new("Untitled Sequence".to_string()),
            recording: false,
            last_position: None,
            last_time: None,
        }
    }
    
    /// Set the mouse recorder operation
    pub fn with_operation(mut self, operation: MouseRecorderOperation) -> Self {
        self.config.operation = operation;
        self
    }
    
    /// Set the sequence name
    pub fn with_sequence_name(mut self, name: impl Into<String>) -> Self {
        let name = name.into();
        self.config.sequence_name = name.clone();
        self.sequence = MouseSequence::new(name);
        self
    }
    
    /// Set the file path for saving/loading
    pub fn with_file_path(mut self, path: impl Into<String>) -> Self {
        self.config.file_path = Some(path.into());
        self
    }
    
    /// Set the smooth movement option
    pub fn with_smooth_movement(mut self, enabled: bool) -> Self {
        self.config.smooth_movement = enabled;
        self
    }
    
    /// Set the playback speed multiplier
    pub fn with_speed_multiplier(mut self, multiplier: f64) -> Self {
        self.config.speed_multiplier = multiplier;
        self
    }
    
    /// Set whether to restore the mouse position after playback
    pub fn with_restore_position(mut self, restore: bool) -> Self {
        self.config.restore_position = restore;
        self
    }
    
    /// Start recording mouse actions
    fn start_recording(&mut self) -> Result<(), Error> {
        if self.recording {
            return Err(Error::Other("Already recording".to_string()));
        }
        
        // Create a new sequence
        self.sequence = MouseSequence::new(self.config.sequence_name.clone());
        self.recording = true;
        self.last_time = Some(Instant::now());
        
        // Get initial mouse position
        let position = self.get_current_position()?;
        self.last_position = Some(position);
        
        log::info!("Started recording mouse sequence: {}", self.config.sequence_name);
        
        Ok(())
    }
    
    /// Stop recording mouse actions
    fn stop_recording(&mut self) -> Result<(), Error> {
        if !self.recording {
            return Err(Error::Other("Not recording".to_string()));
        }
        
        self.recording = false;
        self.last_time = None;
        self.last_position = None;
        
        log::info!("Stopped recording mouse sequence: {}. Recorded {} actions over {} ms", 
            self.config.sequence_name, 
            self.sequence.actions.len(),
            self.sequence.total_duration_ms);
        
        Ok(())
    }
    
    /// Play back the recorded sequence
    fn play_sequence(&self) -> Result<(), Error> {
        if self.sequence.actions.is_empty() {
            return Err(Error::Other("No actions to play".to_string()));
        }
        
        log::info!("Playing mouse sequence: {} ({} actions)", 
            self.config.sequence_name, 
            self.sequence.actions.len());
        
        // Get current mouse position if we need to restore it later
        let original_position = if self.config.restore_position {
            Some(self.get_current_position()?)
        } else {
            None
        };
        
        // Create a mouse control action to execute the sequence
        let mouse_control = MouseControlAction::new(self.plugin.clone());
        
        // Play each action in the sequence
        for action in &self.sequence.actions {
            // Apply speed multiplier to delay
            let adjusted_delay = (action.delay_ms as f64 / self.config.speed_multiplier) as u64;
            
            // Wait for the specified delay
            if adjusted_delay > 0 {
                sleep(Duration::from_millis(adjusted_delay));
            }
            
            // Convert MouseAction to appropriate MouseControlAction call
            match action.operation {
                MouseOperation::Move => {
                    mouse_control.with_operation(MouseOperation::Move)
                        .with_position(action.x, action.y)
                        .with_smooth_movement(self.config.smooth_movement)
                        .execute_mouse_control()?;
                },
                MouseOperation::Click => {
                    mouse_control.with_operation(MouseOperation::Click)
                        .with_position(action.x, action.y)
                        .with_button(action.button.clone())
                        .with_smooth_movement(self.config.smooth_movement)
                        .execute_mouse_control()?;
                },
                MouseOperation::DoubleClick => {
                    mouse_control.with_operation(MouseOperation::DoubleClick)
                        .with_position(action.x, action.y)
                        .with_button(action.button.clone())
                        .with_smooth_movement(self.config.smooth_movement)
                        .execute_mouse_control()?;
                },
                MouseOperation::RightClick => {
                    mouse_control.with_operation(MouseOperation::RightClick)
                        .with_position(action.x, action.y)
                        .with_smooth_movement(self.config.smooth_movement)
                        .execute_mouse_control()?;
                },
                MouseOperation::MiddleClick => {
                    mouse_control.with_operation(MouseOperation::MiddleClick)
                        .with_position(action.x, action.y)
                        .with_smooth_movement(self.config.smooth_movement)
                        .execute_mouse_control()?;
                },
                MouseOperation::MouseDown => {
                    mouse_control.with_operation(MouseOperation::MouseDown)
                        .with_position(action.x, action.y)
                        .with_button(action.button.clone())
                        .with_smooth_movement(self.config.smooth_movement)
                        .execute_mouse_control()?;
                },
                MouseOperation::MouseUp => {
                    mouse_control.with_operation(MouseOperation::MouseUp)
                        .with_position(action.x, action.y)
                        .with_button(action.button.clone())
                        .with_smooth_movement(self.config.smooth_movement)
                        .execute_mouse_control()?;
                },
                MouseOperation::Scroll => {
                    // Create a proper scroll action
                    // Scroll amount is not directly stored in MouseAction,
                    // so we use a default value
                    let scroll_amount = 120; // Default scroll amount
                    
                    mouse_control.with_operation(MouseOperation::Scroll)
                        .with_position(action.x, action.y)
                        .with_scroll_amount(scroll_amount)
                        .with_smooth_movement(self.config.smooth_movement)
                        .execute_mouse_control()?;
                },
                MouseOperation::Drag => {
                    // For drag, we need start and target position
                    if let (Some(target_x), Some(target_y)) = (action.target_x, action.target_y) {
                        mouse_control.with_operation(MouseOperation::Drag)
                            .with_position(action.x, action.y)
                            .with_target_position(target_x, target_y)
                            .with_button(action.button.clone())
                            .with_smooth_movement(self.config.smooth_movement)
                            .execute_mouse_control()?;
                    } else {
                        log::warn!("Skipping drag action with missing target coordinates");
                    }
                },
                _ => {
                    // Not supported or not applicable
                    log::warn!("Skipping unsupported operation: {:?}", action.operation);
                }
            }
        }
        
        // Restore original position if requested
        if let Some((x, y)) = original_position {
            mouse_control.with_operation(MouseOperation::Move)
                .with_position(x, y)
                .with_smooth_movement(self.config.smooth_movement)
                .execute_mouse_control()?;
        }
        
        log::info!("Finished playing mouse sequence: {}", self.config.sequence_name);
        
        Ok(())
    }
    
    /// Save the recorded sequence to a file
    fn save_sequence(&self) -> Result<(), Error> {
        if self.sequence.actions.is_empty() {
            return Err(Error::Other("No actions to save".to_string()));
        }
        
        let path = if let Some(file_path) = &self.config.file_path {
            Path::new(file_path)
        } else {
            return Err(Error::Other("No file path specified".to_string()));
        };
        
        self.sequence.save_to_file(path)?;
        
        log::info!("Saved mouse sequence to: {}", path.display());
        
        Ok(())
    }
    
    /// Load a recorded sequence from a file
    fn load_sequence(&mut self) -> Result<(), Error> {
        let path = if let Some(file_path) = &self.config.file_path {
            Path::new(file_path)
        } else {
            return Err(Error::Other("No file path specified".to_string()));
        };
        
        self.sequence = MouseSequence::load_from_file(path)?;
        self.config.sequence_name = self.sequence.name.clone();
        
        log::info!("Loaded mouse sequence from: {}. Sequence contains {} actions over {} ms", 
            path.display(), 
            self.sequence.actions.len(),
            self.sequence.total_duration_ms);
        
        Ok(())
    }
    
    /// Record a mouse action
    fn record_action(&mut self, action: MouseAction) -> Result<(), Error> {
        if !self.recording {
            return Err(Error::Other("Not recording".to_string()));
        }
        
        self.sequence.add_action(action);
        
        Ok(())
    }
    
    /// Get the current mouse position
    #[cfg(target_os = "windows")]
    fn get_current_position(&self) -> Result<(i32, i32), Error> {
        use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, POINT};
        
        let mut point = POINT { x: 0, y: 0 };
        
        unsafe {
            if GetCursorPos(&mut point).as_bool() {
                Ok((point.x, point.y))
            } else {
                Err(Error::Other("Failed to get cursor position".to_string()))
            }
        }
    }
    
    /// Get the current mouse position (non-Windows implementation)
    #[cfg(not(target_os = "windows"))]
    fn get_current_position(&self) -> Result<(i32, i32), Error> {
        Err(Error::Other("Mouse position is only supported on Windows".to_string()))
    }
    
    /// Execute the recorder action
    fn execute_recorder(&mut self) -> Result<ActionResult, Error> {
        match self.config.operation {
            MouseRecorderOperation::Record => {
                self.start_recording()?;
            },
            MouseRecorderOperation::Stop => {
                self.stop_recording()?;
            },
            MouseRecorderOperation::Play => {
                self.play_sequence()?;
            },
            MouseRecorderOperation::Save => {
                self.save_sequence()?;
            },
            MouseRecorderOperation::Load => {
                self.load_sequence()?;
            },
        }
        
        Ok(ActionResult::success())
    }
}

#[async_trait]
impl Action for MouseRecorderAction {
    fn get_id(&self) -> Uuid {
        self.id
    }
    
    fn get_name(&self) -> &str {
        "Mouse Recorder"
    }
    
    fn get_description(&self) -> &str {
        "Records and plays back mouse actions"
    }
    
    fn get_supported_event_types(&self) -> Vec<EventType> {
        // This action can be triggered by various events
        vec![
            EventType::System,
            EventType::Plugin,
            EventType::User,
            EventType::Internal,
            EventType::KeyPress,
            EventType::MouseEvent,
        ]
    }
    
    fn get_plugin(&self) -> Arc<dyn Plugin> {
        self.plugin.clone()
    }
    
    fn get_icon_path(&self) -> Option<String> {
        Some("system/mouse_recorder.png".to_string())
    }
    
    fn is_configurable(&self) -> bool {
        true
    }
    
    async fn configure(&mut self, config: ActionConfig) -> Result<(), Error> {
        // Parse the configuration from the args
        // Example format: 
        // args[0] = operation ("record", "stop", "play", "save", "load")
        // args[1] = sequence name
        // args[2] = file path (for save/load)
        // args[3] = smooth movement flag (optional)
        // args[4] = speed multiplier (optional)
        // args[5] = restore position flag (optional)
        if config.args.len() >= 1 {
            // Parse operation
            let operation = match config.args[0].to_lowercase().as_str() {
                "record" => MouseRecorderOperation::Record,
                "stop" => MouseRecorderOperation::Stop,
                "play" => MouseRecorderOperation::Play,
                "save" => MouseRecorderOperation::Save,
                "load" => MouseRecorderOperation::Load,
                _ => return Err(Error::InvalidArgument(format!("Unknown mouse recorder operation: {}", config.args[0]))),
            };
            
            self.config.operation = operation;
            
            // Parse sequence name if provided
            if config.args.len() >= 2 && !config.args[1].is_empty() {
                self.config.sequence_name = config.args[1].clone();
                self.sequence = MouseSequence::new(self.config.sequence_name.clone());
            }
            
            // Parse file path if provided
            if config.args.len() >= 3 && !config.args[2].is_empty() {
                self.config.file_path = Some(config.args[2].clone());
            }
            
            // Parse smooth movement flag if provided
            if config.args.len() >= 4 {
                self.config.smooth_movement = config.args[3].parse().unwrap_or(true);
            }
            
            // Parse speed multiplier if provided
            if config.args.len() >= 5 {
                self.config.speed_multiplier = config.args[4].parse().unwrap_or(1.0);
            }
            
            // Parse restore position flag if provided
            if config.args.len() >= 6 {
                self.config.restore_position = config.args[5].parse().unwrap_or(true);
            }
        } else {
            return Err(Error::InvalidArgument("Mouse recorder action requires at least an operation type argument".to_string()));
        }
        
        Ok(())
    }
    
    async fn execute(&mut self, event: &dyn Event) -> Result<ActionResult, Error> {
        // Handle record mode specially to capture mouse events
        if self.recording && event.get_type() == EventType::MouseEvent {
            if let Some(payload) = event.get_payload() {
                // Parse mouse event payload
                // Expected format: "button,x,y,action"
                // Example: "left,100,200,click"
                let parts: Vec<&str> = payload.split(',').collect();
                if parts.len() >= 4 {
                    let button_str = parts[0];
                    let x = parts[1].parse::<i32>().unwrap_or(0);
                    let y = parts[2].parse::<i32>().unwrap_or(0);
                    let action_str = parts[3];
                    
                    // Parse mouse button
                    let button = match button_str.to_lowercase().as_str() {
                        "left" => MouseButton::Left,
                        "right" => MouseButton::Right,
                        "middle" => MouseButton::Middle,
                        _ => MouseButton::Left, // Default to left
                    };
                    
                    // Parse action type
                    let operation = match action_str.to_lowercase().as_str() {
                        "move" => {
                            // For move events, check minimum distance
                            if let Some((last_x, last_y)) = self.last_position {
                                let dx = x - last_x;
                                let dy = y - last_y;
                                let distance = (dx * dx + dy * dy).sqrt();
                                
                                if distance < self.config.min_move_distance as f64 {
                                    // Skip this move, too small
                                    return Ok(ActionResult::success());
                                }
                            }
                            
                            MouseOperation::Move
                        },
                        "down" => MouseOperation::MouseDown,
                        "up" => MouseOperation::MouseUp,
                        "click" => MouseOperation::Click,
                        "doubleclick" => MouseOperation::DoubleClick,
                        "rightclick" => MouseOperation::RightClick,
                        "middleclick" => MouseOperation::MiddleClick,
                        "scroll" => MouseOperation::Scroll,
                        "drag" => {
                            // For drag, we need target coordinates
                            if parts.len() >= 6 {
                                let target_x = parts[4].parse::<i32>().unwrap_or(0);
                                let target_y = parts[5].parse::<i32>().unwrap_or(0);
                                
                                // Create drag action
                                let mut action = MouseAction {
                                    operation: MouseOperation::Drag,
                                    x,
                                    y,
                                    target_x: Some(target_x),
                                    target_y: Some(target_y),
                                    button,
                                    delay_ms: self.calculate_delay(),
                                };
                                
                                self.record_action(action)?;
                                return Ok(ActionResult::success());
                            } else {
                                // Not enough data for drag
                                return Ok(ActionResult::success());
                            }
                        },
                        _ => {
                            // Unknown action type
                            return Ok(ActionResult::success());
                        }
                    };
                    
                    // Create and record the action
                    let action = MouseAction {
                        operation,
                        x,
                        y,
                        target_x: None,
                        target_y: None,
                        button,
                        delay_ms: self.calculate_delay(),
                    };
                    
                    // Update last position and time
                    self.last_position = Some((x, y));
                    
                    self.record_action(action)?;
                }
            }
            
            return Ok(ActionResult::success());
        }
        
        // Execute the recorder action
        let result = self.execute_recorder();
        
        match result {
            Ok(action_result) => Ok(action_result),
            Err(e) => {
                // Log the error
                log::error!("Mouse recorder action failed: {}", e);
                
                // Return error result
                Ok(ActionResult::Error(e.to_string()))
            }
        }
    }
    
    fn validate(&self) -> Result<(), Error> {
        match self.config.operation {
            MouseRecorderOperation::Save | MouseRecorderOperation::Load => {
                if self.config.file_path.is_none() {
                    return Err(Error::ValidationError("File path is required for save/load operations".to_string()));
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
        if let Ok(cfg) = config.downcast::<MouseRecorderConfig>() {
            self.config = *cfg;
            Ok(())
        } else {
            Err(Error::ValidationError("Invalid configuration type".to_string()))
        }
    }
}

impl MouseRecorderAction {
    /// Calculate the delay since the last recorded action
    fn calculate_delay(&mut self) -> u64 {
        if let Some(last_time) = self.last_time {
            let now = Instant::now();
            let duration = now.duration_since(last_time);
            self.last_time = Some(now);
            duration.as_millis() as u64
        } else {
            self.last_time = Some(Instant::now());
            0 // First action has no delay
        }
    }
}

#[cfg(not(target_os = "windows"))]
impl Action for MouseRecorderAction {
    fn get_name(&self) -> String {
        "Mouse Recorder".to_string()
    }
    
    fn get_description(&self) -> String {
        "Mouse recorder (not supported on this platform)".to_string()
    }
    
    async fn execute(&mut self, _event: &dyn Event) -> Result<ActionResult, Error> {
        Err(Error::Other("Mouse recorder is only supported on Windows".to_string()))
    }
    
    fn validate(&self) -> Result<(), Error> {
        Err(Error::ValidationError("Mouse recorder is only supported on Windows".to_string()))
    }
    
    fn is_configurable(&self) -> bool {
        false
    }
}

// Dummy plugin for testing
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
        "Dummy plugin for mouse recorder actions"
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
        payload: Option<String>,
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
        let plugin = Arc::new(TestPlugin);
        let action = MouseRecorderAction::new(plugin.clone())
            .with_operation(MouseRecorderOperation::Record)
            .with_sequence_name("Test Recording")
            .with_smooth_movement(true)
            .with_speed_multiplier(1.5);
        
        // Verify properties
        assert_eq!(action.get_name(), "Mouse Recorder");
        assert_eq!(action.get_description(), "Records and plays back mouse actions");
        assert!(action.get_supported_event_types().contains(&EventType::MouseEvent));
        assert_eq!(action.config.sequence_name, "Test Recording");
        assert_eq!(action.config.speed_multiplier, 1.5);
        assert!(action.config.smooth_movement);
    }
    
    #[tokio::test]
    async fn test_mouse_recorder_configuration() {
        // Create action
        let plugin = Arc::new(TestPlugin);
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
        
        // Verify configuration
        assert_eq!(action.config.sequence_name, "Test Recording");
        match action.config.operation {
            MouseRecorderOperation::Record => {},
            _ => panic!("Expected Record operation"),
        }
        assert_eq!(action.config.file_path, Some("test.json".to_string()));
        assert!(action.config.smooth_movement);
        assert_eq!(action.config.speed_multiplier, 1.5);
        assert!(action.config.restore_position);
        
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
    async fn test_mouse_sequence_serialization() {
        // Create a sequence
        let mut sequence = MouseSequence::new("Test Sequence".to_string());
        
        // Add some actions
        sequence.add_action(MouseAction {
            operation: MouseOperation::Move,
            x: 100,
            y: 200,
            target_x: None,
            target_y: None,
            button: MouseButton::Left,
            delay_ms: 100,
        });
        
        sequence.add_action(MouseAction {
            operation: MouseOperation::Click,
            x: 100,
            y: 200,
            target_x: None,
            target_y: None,
            button: MouseButton::Left,
            delay_ms: 200,
        });
        
        // Serialize to JSON
        let json = serde_json::to_string_pretty(&sequence).unwrap();
        
        // Deserialize from JSON
        let deserialized: MouseSequence = serde_json::from_str(&json).unwrap();
        
        // Verify
        assert_eq!(deserialized.name, "Test Sequence");
        assert_eq!(deserialized.actions.len(), 2);
        assert_eq!(deserialized.total_duration_ms, 300);
    }
    
    #[tokio::test]
    async fn test_record_mouse_event() {
        // Create action
        let plugin = Arc::new(TestPlugin);
        let mut action = MouseRecorderAction::new(plugin.clone())
            .with_operation(MouseRecorderOperation::Record);
        
        // Start recording
        let result = action.execute_recorder();
        assert!(result.is_ok());
        
        // Create a mouse event
        let event = TestEvent {
            event_type: EventType::MouseEvent,
            payload: Some("left,100,200,click".to_string()),
        };
        
        // Process the event
        let result = action.execute(&event).await;
        assert!(result.is_ok());
        
        // Verify recording
        assert_eq!(action.sequence.actions.len(), 1);
        
        // Stop recording
        action.config.operation = MouseRecorderOperation::Stop;
        let result = action.execute_recorder();
        assert!(result.is_ok());
    }
} 