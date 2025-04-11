use std::any::Any;
use std::fmt::Debug;
use std::process::Command;
use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;
use tokio::task;
use serde::{Serialize, Deserialize};

use crate::core::action::{Action, ActionConfig, ActionResult};
use crate::core::Error;
use crate::core::event::{Event, EventType};
use crate::core::plugin::Plugin;

/// Configuration for the RunCommandAction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunCommandConfig {
    /// The command to execute
    pub command: String,
    /// The working directory for the command (optional)
    pub working_dir: Option<String>,
    /// Whether to wait for the command to complete
    pub wait: bool,
    /// Whether to show the command window
    pub show_window: bool,
}

impl Default for RunCommandConfig {
    fn default() -> Self {
        Self {
            command: String::new(),
            working_dir: None,
            wait: true,
            show_window: true,
        }
    }
}

/// An action that executes a system command
#[derive(Debug)]
pub struct RunCommandAction {
    id: Uuid,
    plugin: Arc<dyn Plugin>,
    config: RunCommandConfig,
}

impl RunCommandAction {
    /// Create a new RunCommandAction
    pub fn new(plugin: Arc<dyn Plugin>) -> Self {
        Self {
            id: Uuid::new_v4(),
            plugin,
            config: RunCommandConfig::default(),
        }
    }
    
    /// Create a new RunCommandAction with a specific ID
    pub fn with_id(id: Uuid, plugin: Arc<dyn Plugin>) -> Self {
        Self {
            id,
            plugin,
            config: RunCommandConfig::default(),
        }
    }
    
    /// Set the command to run
    pub fn with_command(mut self, command: impl Into<String>) -> Self {
        self.config.command = command.into();
        self
    }
    
    /// Set the working directory
    pub fn with_working_dir(mut self, dir: impl Into<String>) -> Self {
        self.config.working_dir = Some(dir.into());
        self
    }
    
    /// Set whether to wait for the command to complete
    pub fn with_wait(mut self, wait: bool) -> Self {
        self.config.wait = wait;
        self
    }
    
    /// Set whether to show the command window
    pub fn with_show_window(mut self, show: bool) -> Self {
        self.config.show_window = show;
        self
    }
}

#[async_trait]
impl Action for RunCommandAction {
    fn get_id(&self) -> Uuid {
        self.id
    }
    
    fn get_name(&self) -> &str {
        "Run Command"
    }
    
    fn get_description(&self) -> &str {
        "Executes a system command"
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
        Some("system/run_command.png".to_string())
    }
    
    fn is_configurable(&self) -> bool {
        true
    }
    
    async fn configure(&mut self, config: ActionConfig) -> Result<(), Error> {
        if config.args.len() >= 1 {
            self.config.command = config.args[0].clone();
        }
        
        if config.args.len() >= 2 {
            self.config.working_dir = Some(config.args[1].clone());
        }
        
        if config.args.len() >= 3 {
            self.config.wait = config.args[2].parse().unwrap_or(true);
        }
        
        if config.args.len() >= 4 {
            self.config.show_window = config.args[3].parse().unwrap_or(true);
        }
        
        Ok(())
    }
    
    async fn execute(&mut self, _event: &dyn Event) -> Result<ActionResult, Error> {
        // Validate command is not empty
        if self.config.command.is_empty() {
            return Ok(ActionResult::failure("Command cannot be empty"));
        }
        
        // Clone values for task
        let command_str = self.config.command.clone();
        let working_dir = self.config.working_dir.clone();
        let wait = self.config.wait;
        let show_window = self.config.show_window;
        
        // Execute command in a separate task to avoid blocking
        let result = task::spawn_blocking(move || {
            // Split command into program and args
            let mut parts = command_str.split_whitespace();
            let program = parts.next().unwrap_or("");
            let args: Vec<&str> = parts.collect();
            
            // Create command
            let mut command = Command::new(program);
            command.args(args);
            
            // Set working directory if specified
            if let Some(dir) = working_dir {
                command.current_dir(dir);
            }
            
            // Configure whether to show window (Windows-specific)
            #[cfg(target_os = "windows")]
            {
                use std::os::windows::process::CommandExt;
                const CREATE_NO_WINDOW: u32 = 0x08000000;
                if !show_window {
                    command.creation_flags(CREATE_NO_WINDOW);
                }
            }
            
            // Execute command
            if wait {
                // Wait for command to complete and get status
                match command.output() {
                    Ok(output) => {
                        let success = output.status.success();
                        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                        
                        if success {
                            ActionResult::success()
                                .with_data((stdout, stderr))
                        } else {
                            let exit_code = output.status.code().map_or_else(
                                || "terminated by signal".to_string(),
                                |code| format!("exit code {}", code)
                            );
                            ActionResult::failure(format!("Command failed with {}: {}", exit_code, stderr))
                                .with_data((stdout, stderr))
                        }
                    },
                    Err(e) => {
                        ActionResult::failure(format!("Failed to execute command: {}", e))
                    }
                }
            } else {
                // Just spawn and don't wait
                match command.spawn() {
                    Ok(_) => ActionResult::success(),
                    Err(e) => ActionResult::failure(format!("Failed to spawn command: {}", e)),
                }
            }
        }).await.map_err(|e| Error::Other(format!("Task error: {}", e)))?;
        
        Ok(result)
    }
    
    fn validate(&self) -> Result<(), Error> {
        if self.config.command.is_empty() {
            return Err(Error::InvalidArgument("Command cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::event::EventPayload;
    
    struct TestPlugin;
    
    impl std::fmt::Debug for TestPlugin {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TestPlugin").finish()
        }
    }
    
    #[async_trait]
    impl Plugin for TestPlugin {
        fn get_name(&self) -> &str {
            "Test Plugin"
        }
        
        fn get_description(&self) -> &str {
            "Test plugin for unit tests"
        }
        
        fn get_version(&self) -> &str {
            "1.0.0"
        }
        
        fn get_author(&self) -> &str {
            "Test Author"
        }
        
        fn get_info(&self) -> crate::core::plugin::PluginInfo {
            use crate::core::plugin::{PluginInfo, PluginCapability};
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
        
        fn get_capabilities(&self) -> Vec<crate::core::plugin::PluginCapability> {
            vec![crate::core::plugin::PluginCapability::ActionProvider]
        }
        
        fn get_state(&self) -> crate::core::plugin::PluginState {
            crate::core::plugin::PluginState::Running
        }
        
        async fn initialize(&mut self) -> Result<(), crate::core::plugin::PluginError> {
            Ok(())
        }
        
        async fn start(&mut self) -> Result<(), crate::core::plugin::PluginError> {
            Ok(())
        }
        
        async fn stop(&mut self) -> Result<(), crate::core::plugin::PluginError> {
            Ok(())
        }
        
        async fn handle_event(&mut self, _event: &dyn Event) -> Result<(), crate::core::plugin::PluginError> {
            Ok(())
        }
        
        fn get_config(&self) -> Option<&crate::core::config::Config> {
            None
        }
        
        async fn update_config(&mut self, _config: crate::core::config::Config) -> Result<(), crate::core::plugin::PluginError> {
            Ok(())
        }
        
        fn as_any(&self) -> &dyn Any {
            self
        }
        
        fn clone_box(&self) -> Box<dyn Plugin> {
            Box::new(Self)
        }
    }
    
    #[derive(Debug)]
    struct TestEvent {
        event_type: EventType,
    }
    
    impl Event for TestEvent {
        fn get_id(&self) -> &str {
            "test-event"
        }
        
        fn get_type(&self) -> EventType {
            self.event_type
        }
        
        fn get_payload(&self) -> &crate::core::event::EventPayload {
            &EventPayload::None
        }
        
        fn get_timestamp(&self) -> chrono::DateTime<chrono::Local> {
            chrono::Local::now()
        }
        
        fn get_source(&self) -> Option<&str> {
            Some("test")
        }
        
        fn as_any(&self) -> &dyn Any {
            self
        }
        
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        
        fn clone_event(&self) -> Box<dyn Event + Send + Sync> {
            Box::new(Self {
                event_type: self.event_type,
            })
        }
    }
    
    #[tokio::test]
    async fn test_run_command() {
        // Create plugin and action
        let plugin = Arc::new(TestPlugin);
        let mut action = RunCommandAction::new(plugin)
            .with_command("echo Hello World");
        
        // Create event
        let event = TestEvent {
            event_type: EventType::System,
        };
        
        // Execute action
        let result = action.execute(&event).await.unwrap();
        
        // Check result
        assert!(result.success);
    }
    
    #[tokio::test]
    async fn test_invalid_command() {
        // Create plugin and action
        let plugin = Arc::new(TestPlugin);
        let mut action = RunCommandAction::new(plugin)
            .with_command("nonexistentcommand");
        
        // Create event
        let event = TestEvent {
            event_type: EventType::System,
        };
        
        // Execute action
        let result = action.execute(&event).await.unwrap();
        
        // Check result
        assert!(!result.success);
    }
} 
