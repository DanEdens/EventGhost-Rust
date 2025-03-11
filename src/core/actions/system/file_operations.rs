use std::any::Any;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::fs;
use std::io;
use uuid::Uuid;
use async_trait::async_trait;
use tokio::task;
use serde::{Serialize, Deserialize};

use crate::core::action::{Action, ActionConfig, ActionResult};
use crate::core::Error;
use crate::core::event::{Event, EventType};
use crate::core::plugin::Plugin;

/// Supported file operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileOperation {
    /// Copy a file from source to destination
    Copy,
    /// Move a file from source to destination
    Move,
    /// Delete a file
    Delete,
    /// Create a new file with the given content
    Create,
    /// Create a new directory
    CreateDirectory,
    /// Check if a file exists
    Exists,
}

impl FileOperation {
    fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "copy" => Some(FileOperation::Copy),
            "move" => Some(FileOperation::Move),
            "delete" => Some(FileOperation::Delete),
            "create" => Some(FileOperation::Create),
            "createdir" | "creatediretcory" | "mkdir" => Some(FileOperation::CreateDirectory),
            "exists" | "check" => Some(FileOperation::Exists),
            _ => None,
        }
    }
}

/// Configuration for the FileOperationsAction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperationsConfig {
    /// The operation to perform
    pub operation: FileOperation,
    /// The source file path
    pub source: PathBuf,
    /// The destination file path (for Copy and Move operations)
    pub destination: Option<PathBuf>,
    /// The content to write (for Create operation)
    pub content: Option<String>,
    /// Whether to overwrite existing files
    pub overwrite: bool,
}

impl Default for FileOperationsConfig {
    fn default() -> Self {
        Self {
            operation: FileOperation::Copy,
            source: PathBuf::new(),
            destination: None,
            content: None,
            overwrite: false,
        }
    }
}

/// An action that performs file operations
#[derive(Debug)]
pub struct FileOperationsAction {
    id: Uuid,
    plugin: Arc<dyn Plugin>,
    config: FileOperationsConfig,
}

impl FileOperationsAction {
    /// Create a new FileOperationsAction
    pub fn new(plugin: Arc<dyn Plugin>) -> Self {
        Self {
            id: Uuid::new_v4(),
            plugin,
            config: FileOperationsConfig::default(),
        }
    }
    
    /// Create a new FileOperationsAction with a specific ID
    pub fn with_id(id: Uuid, plugin: Arc<dyn Plugin>) -> Self {
        Self {
            id,
            plugin,
            config: FileOperationsConfig::default(),
        }
    }
    
    /// Set the operation to perform
    pub fn with_operation(mut self, operation: FileOperation) -> Self {
        self.config.operation = operation;
        self
    }
    
    /// Set the source file path
    pub fn with_source<P: AsRef<Path>>(mut self, source: P) -> Self {
        self.config.source = source.as_ref().to_path_buf();
        self
    }
    
    /// Set the destination file path
    pub fn with_destination<P: AsRef<Path>>(mut self, destination: P) -> Self {
        self.config.destination = Some(destination.as_ref().to_path_buf());
        self
    }
    
    /// Set the content to write
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.config.content = Some(content.into());
        self
    }
    
    /// Set whether to overwrite existing files
    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.config.overwrite = overwrite;
        self
    }
}

#[async_trait]
impl Action for FileOperationsAction {
    fn get_id(&self) -> Uuid {
        self.id
    }
    
    fn get_name(&self) -> &str {
        "File Operations"
    }
    
    fn get_description(&self) -> &str {
        "Performs file operations such as copy, move, delete, and create"
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
        Some("system/file_operations.png".to_string())
    }
    
    fn is_configurable(&self) -> bool {
        true
    }
    
    async fn configure(&mut self, config: ActionConfig) -> Result<(), Error> {
        // Parse operation
        if config.args.len() >= 1 {
            if let Some(op) = FileOperation::from_string(&config.args[0]) {
                self.config.operation = op;
            } else {
                return Err(Error::InvalidArgument(format!("Invalid operation: {}", config.args[0])));
            }
        }
        
        // Parse source path
        if config.args.len() >= 2 {
            self.config.source = PathBuf::from(&config.args[1]);
        }
        
        // Parse destination path (for Copy and Move)
        if config.args.len() >= 3 {
            self.config.destination = Some(PathBuf::from(&config.args[2]));
        }
        
        // Parse content (for Create)
        if config.args.len() >= 4 {
            self.config.content = Some(config.args[3].clone());
        }
        
        // Parse overwrite flag
        if config.args.len() >= 5 {
            self.config.overwrite = config.args[4].parse().unwrap_or(false);
        }
        
        Ok(())
    }
    
    async fn execute(&mut self, _event: &dyn Event) -> Result<ActionResult, Error> {
        // Validate config
        if self.validate().is_err() {
            return Ok(ActionResult::failure("Invalid configuration"));
        }
        
        // Clone values for task
        let operation = self.config.operation.clone();
        let source = self.config.source.clone();
        let destination = self.config.destination.clone();
        let content = self.config.content.clone();
        let overwrite = self.config.overwrite;
        
        // Execute file operation in a separate task to avoid blocking
        let result = task::spawn_blocking(move || {
            match operation {
                FileOperation::Copy => {
                    if let Some(dest) = destination {
                        if dest.exists() && !overwrite {
                            return ActionResult::failure("Destination file already exists");
                        }
                        
                        match fs::copy(&source, &dest) {
                            Ok(_) => ActionResult::success(),
                            Err(e) => ActionResult::failure(format!("Failed to copy file: {}", e)),
                        }
                    } else {
                        ActionResult::failure("Destination path is required for copy operation")
                    }
                },
                FileOperation::Move => {
                    if let Some(dest) = destination {
                        if dest.exists() && !overwrite {
                            return ActionResult::failure("Destination file already exists");
                        }
                        
                        match fs::rename(&source, &dest) {
                            Ok(_) => ActionResult::success(),
                            Err(e) => ActionResult::failure(format!("Failed to move file: {}", e)),
                        }
                    } else {
                        ActionResult::failure("Destination path is required for move operation")
                    }
                },
                FileOperation::Delete => {
                    // Check if the path is a file or directory
                    if source.is_file() {
                        match fs::remove_file(&source) {
                            Ok(_) => ActionResult::success(),
                            Err(e) => ActionResult::failure(format!("Failed to delete file: {}", e)),
                        }
                    } else if source.is_dir() {
                        // Use remove_dir or remove_dir_all based on whether overwrite is set
                        // If overwrite is true, remove directory and all contents (potentially dangerous!)
                        let result = if overwrite {
                            fs::remove_dir_all(&source)
                        } else {
                            fs::remove_dir(&source)
                        };
                        
                        match result {
                            Ok(_) => ActionResult::success(),
                            Err(e) => ActionResult::failure(format!("Failed to delete directory: {}", e)),
                        }
                    } else {
                        ActionResult::failure(format!("Path does not exist: {:?}", source))
                    }
                },
                FileOperation::Create => {
                    // If the file exists and overwrite is false, return error
                    if source.exists() && !overwrite {
                        return ActionResult::failure("File already exists");
                    }
                    
                    // Ensure parent directories exist
                    if let Some(parent) = source.parent() {
                        if !parent.exists() {
                            if let Err(e) = fs::create_dir_all(parent) {
                                return ActionResult::failure(format!("Failed to create parent directories: {}", e));
                            }
                        }
                    }
                    
                    // Write content to file
                    match fs::write(&source, content.unwrap_or_default()) {
                        Ok(_) => ActionResult::success(),
                        Err(e) => ActionResult::failure(format!("Failed to create file: {}", e)),
                    }
                },
                FileOperation::CreateDirectory => {
                    // If the directory exists and overwrite is false, return error
                    if source.exists() && !overwrite {
                        return ActionResult::failure("Directory already exists");
                    }
                    
                    // Create directory (and parents if needed)
                    match fs::create_dir_all(&source) {
                        Ok(_) => ActionResult::success(),
                        Err(e) => ActionResult::failure(format!("Failed to create directory: {}", e)),
                    }
                },
                FileOperation::Exists => {
                    // Check if the file or directory exists
                    let exists = source.exists();
                    ActionResult::success().with_data(exists)
                },
            }
        }).await.map_err(|e| Error::Other(format!("Task error: {}", e)))?;
        
        Ok(result)
    }
    
    fn validate(&self) -> Result<(), Error> {
        // Validate source path
        if self.config.source.as_os_str().is_empty() {
            return Err(Error::InvalidArgument("Source path cannot be empty".to_string()));
        }
        
        // Validate destination path for Copy and Move operations
        if (self.config.operation == FileOperation::Copy || self.config.operation == FileOperation::Move) 
            && self.config.destination.is_none() {
            return Err(Error::InvalidArgument("Destination path is required for copy and move operations".to_string()));
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
    use std::fs;
    use tempfile::tempdir;
    
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
    async fn test_create_file() {
        // Create a temporary directory for testing
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        // Create plugin and action
        let plugin = Arc::new(TestPlugin);
        let mut action = FileOperationsAction::new(plugin)
            .with_operation(FileOperation::Create)
            .with_source(&file_path)
            .with_content("Hello, world!");
        
        // Create event
        let event = TestEvent {
            event_type: EventType::System,
        };
        
        // Execute action
        let result = action.execute(&event).await.unwrap();
        
        // Check result
        assert!(result.success);
        assert!(file_path.exists());
        
        // Check file content
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Hello, world!");
    }
    
    #[tokio::test]
    async fn test_copy_file() {
        // Create a temporary directory for testing
        let temp_dir = tempdir().unwrap();
        let source_path = temp_dir.path().join("source.txt");
        let dest_path = temp_dir.path().join("dest.txt");
        
        // Create source file
        fs::write(&source_path, "Test content").unwrap();
        
        // Create plugin and action
        let plugin = Arc::new(TestPlugin);
        let mut action = FileOperationsAction::new(plugin)
            .with_operation(FileOperation::Copy)
            .with_source(&source_path)
            .with_destination(&dest_path);
        
        // Create event
        let event = TestEvent {
            event_type: EventType::System,
        };
        
        // Execute action
        let result = action.execute(&event).await.unwrap();
        
        // Check result
        assert!(result.success);
        assert!(dest_path.exists());
        
        // Check file content
        let content = fs::read_to_string(&dest_path).unwrap();
        assert_eq!(content, "Test content");
    }
    
    #[tokio::test]
    async fn test_delete_file() {
        // Create a temporary directory for testing
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("to_delete.txt");
        
        // Create file
        fs::write(&file_path, "Delete me").unwrap();
        assert!(file_path.exists());
        
        // Create plugin and action
        let plugin = Arc::new(TestPlugin);
        let mut action = FileOperationsAction::new(plugin)
            .with_operation(FileOperation::Delete)
            .with_source(&file_path);
        
        // Create event
        let event = TestEvent {
            event_type: EventType::System,
        };
        
        // Execute action
        let result = action.execute(&event).await.unwrap();
        
        // Check result
        assert!(result.success);
        assert!(!file_path.exists());
    }
    
    #[tokio::test]
    async fn test_exists_operation() {
        // Create a temporary directory for testing
        let temp_dir = tempdir().unwrap();
        let existing_file = temp_dir.path().join("exists.txt");
        let nonexistent_file = temp_dir.path().join("does_not_exist.txt");
        
        // Create file
        fs::write(&existing_file, "I exist").unwrap();
        
        // Create plugin and action for existing file
        let plugin = Arc::new(TestPlugin);
        let mut action1 = FileOperationsAction::new(plugin.clone())
            .with_operation(FileOperation::Exists)
            .with_source(&existing_file);
        
        // Create plugin and action for non-existent file
        let mut action2 = FileOperationsAction::new(plugin)
            .with_operation(FileOperation::Exists)
            .with_source(&nonexistent_file);
        
        // Create event
        let event = TestEvent {
            event_type: EventType::System,
        };
        
        // Execute action for existing file
        let result1 = action1.execute(&event).await.unwrap();
        
        // Execute action for non-existent file
        let result2 = action2.execute(&event).await.unwrap();
        
        // Check results
        assert!(result1.success);
        assert!(result2.success);
        
        // Check data
        if let Some(data_box) = result1.data {
            if let Some(exists) = data_box.downcast_ref::<bool>() {
                assert!(*exists);
            } else {
                panic!("Expected bool data");
            }
        } else {
            panic!("Expected data");
        }
        
        if let Some(data_box) = result2.data {
            if let Some(exists) = data_box.downcast_ref::<bool>() {
                assert!(!*exists);
            } else {
                panic!("Expected bool data");
            }
        } else {
            panic!("Expected data");
        }
    }
} 