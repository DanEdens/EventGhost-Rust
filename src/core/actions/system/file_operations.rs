use std::any::Any;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::fs;
use std::io;
use std::io::Write;
use uuid::Uuid;
use async_trait::async_trait;
use tokio::task;
use serde::{Serialize, Deserialize};
use serde_json;

// Add path utilities
use crate::utils::path;
use crate::core::action::{Action, ActionConfig, ActionResult};
use crate::core::Error;
use crate::core::event::{Event, EventType};
use crate::core::plugin::Plugin;

/// Supported file operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileOperation {
    /// Copy a file or directory from source to destination
    Copy { source: String, destination: String },
    /// Move a file or directory from source to destination
    Move { source: String, destination: String },
    /// Delete a file or directory
    Delete { path: String },
    /// Create a file with the given content
    Create { path: String, content: String },
    /// Read the contents of a file
    Read { path: String },
    /// Create a directory
    CreateDirectory { path: String },
    /// Check if a file or directory exists
    Exists { path: String },
}

impl FileOperation {
    fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "copy" => Some(FileOperation::Copy { source: String::new(), destination: String::new() }),
            "move" => Some(FileOperation::Move { source: String::new(), destination: String::new() }),
            "delete" => Some(FileOperation::Delete { path: String::new() }),
            "create" => Some(FileOperation::Create { path: String::new(), content: String::new() }),
            "createdir" | "creatediretcory" | "mkdir" => Some(FileOperation::CreateDirectory { path: String::new() }),
            "exists" | "check" => Some(FileOperation::Exists { path: String::new() }),
            "read" | "readfile" | "getcontents" => Some(FileOperation::Read { path: String::new() }),
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
            operation: FileOperation::Copy { source: String::new(), destination: String::new() },
            source: PathBuf::new(),
            destination: None,
            content: None,
            overwrite: false,
        }
    }
}

/// Configuration file types supported by EventGhost
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConfigFileType {
    /// JSON configuration file (.json)
    Json,
    /// XML configuration file (.xml)
    Xml,
    /// EventGhost tree file (.egtree)
    EgTree,
    /// Unknown file type
    Unknown,
}

impl ConfigFileType {
    /// Get the file type from a path's extension
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            match ext.to_lowercase().as_str() {
                "json" => ConfigFileType::Json,
                "xml" => ConfigFileType::Xml,
                "egtree" => ConfigFileType::EgTree,
                _ => ConfigFileType::Unknown,
            }
        } else {
            ConfigFileType::Unknown
        }
    }
    
    /// Get the file extension for this config type
    pub fn extension(&self) -> &'static str {
        match self {
            ConfigFileType::Json => "json",
            ConfigFileType::Xml => "xml",
            ConfigFileType::EgTree => "egtree",
            ConfigFileType::Unknown => "",
        }
    }
    
    /// Get the MIME type for this config type
    pub fn mime_type(&self) -> &'static str {
        match self {
            ConfigFileType::Json => "application/json",
            ConfigFileType::Xml => "application/xml",
            ConfigFileType::EgTree => "application/xml",
            ConfigFileType::Unknown => "application/octet-stream",
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
    
    /// Load a configuration file and parse it based on its extension
    pub async fn load_config<P: AsRef<Path>>(&self, path: P) -> Result<serde_json::Value, Error> {
        let path = path.as_ref();
        
        // Check if the file exists
        if !path.exists() {
            return Err(Error::InvalidArgument(format!("File does not exist: {:?}", path)));
        }
        
        // Read the file content
        let content = fs::read_to_string(path)
            .map_err(|e| Error::InvalidOperation(format!("Failed to read file: {}", e)))?;
        
        // Parse the content based on the file type
        match ConfigFileType::from_path(path) {
            ConfigFileType::Json => {
                serde_json::from_str(&content)
                    .map_err(|e| Error::InvalidOperation(format!("Failed to parse JSON: {}", e)))
            },
            ConfigFileType::Xml | ConfigFileType::EgTree => {
                // For XML files, we need to parse them to a JSON value for consistency
                // This would require xml-to-json conversion which is more complex
                // For now, we'll return an error
                Err(Error::InvalidOperation("XML parsing not implemented yet".to_string()))
            },
            ConfigFileType::Unknown => {
                Err(Error::InvalidOperation(format!("Unsupported file type: {:?}", path)))
            },
        }
    }
    
    /// Save a configuration file in the specified format
    pub async fn save_config<P: AsRef<Path>>(&self, data: &serde_json::Value, destination: P, overwrite: bool) -> Result<(), Error> {
        let destination_path = path::to_path_buf(destination);
        
        // Check if the file exists and we're not allowed to overwrite
        if path::exists(&destination_path) && !overwrite {
            return Err(Error::FileExists(path::to_string_lossy(&destination_path)));
        }
        
        // Ensure parent directories exist
        if let Some(parent) = path::parent(&destination_path) {
            if !path::exists(&parent) {
                fs::create_dir_all(&parent)
                    .map_err(|e| Error::InvalidOperation(format!("Failed to create parent directories: {}", e)))?;
            }
        }
        
        // Serialize and write the data based on the file type
        match ConfigFileType::from_path(&destination_path) {
            ConfigFileType::Json => {
                let json_string = serde_json::to_string_pretty(data)
                    .map_err(|e| Error::InvalidOperation(format!("Failed to serialize to JSON: {}", e)))?;
                
                fs::write(&destination_path, json_string)
                    .map_err(|e| Error::InvalidOperation(format!("Failed to write file: {}", e)))?;
            },
            ConfigFileType::Xml | ConfigFileType::EgTree => {
                // For XML files, we need to convert from JSON to XML
                // This would require json-to-xml conversion which is more complex
                // For now, we'll return an error
                return Err(Error::InvalidOperation("XML saving not implemented yet".to_string()));
            },
            ConfigFileType::Unknown => {
                return Err(Error::InvalidOperation(format!("Unsupported file type: {}", path::to_string_lossy(&destination_path))));
            },
        }
        
        Ok(())
    }
    
    /// Create a backup of a configuration file
    pub async fn backup_config<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf, Error> {
        let file_path = path::to_path_buf(path);
        
        // Check if the file exists
        if !path::exists(&file_path) {
            return Err(Error::InvalidArgument(format!("File does not exist: {}", path::to_string_lossy(&file_path))));
        }
        
        // Generate backup file path with timestamp
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let file_name = path::file_name(&file_path)
            .ok_or_else(|| Error::InvalidArgument(format!("Invalid file name: {}", path::to_string_lossy(&file_path))))?;
        
        let mut backup_path = file_path.clone();
        if let Some(parent_dir) = path::parent(&file_path) {
            backup_path = path::join(parent_dir, format!("{}_{}.bak", file_name, timestamp));
        } else {
            backup_path = path::with_extension(&file_path, format!("{}_{}.bak", file_name, timestamp));
        }
        
        // Copy the file to the backup path
        fs::copy(&file_path, &backup_path)
            .map_err(|e| Error::InvalidOperation(format!("Failed to create backup: {}", e)))?;
        
        Ok(backup_path)
    }
    
    /// Get the last modified time of a file
    pub fn get_file_modified_time<P: AsRef<Path>>(&self, path: P) -> Result<chrono::DateTime<chrono::Local>, Error> {
        let path = path.as_ref();
        
        // Check if the file exists
        if !path.exists() {
            return Err(Error::InvalidArgument(format!("File does not exist: {:?}", path)));
        }
        
        // Get the file metadata
        let metadata = fs::metadata(path)
            .map_err(|e| Error::InvalidOperation(format!("Failed to get file metadata: {}", e)))?;
        
        // Get the modified time
        let modified = metadata.modified()
            .map_err(|e| Error::InvalidOperation(format!("Failed to get modified time: {}", e)))?;
        
        // Convert to DateTime
        let datetime = chrono::DateTime::<chrono::Local>::from(modified);
        
        Ok(datetime)
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
            if create_backup && path.exists() {
                self.backup_config(&path)?;
            }

            // Result is the return type of the execute function
            Ok(match operation {
                FileOperation::Copy { source, destination } => {
                    if std::path::Path::new(&destination).exists() && !overwrite {
                        return Err(Error::InvalidOperation(format!("Destination already exists: {}", destination)));
                    }
                    
                    // Check if source exists
                    if !std::path::Path::new(&source).exists() {
                        ActionResult::failure(format!("File or directory does not exist: {:?}", source))
                    } else {
                        // Execute the copy operation
                        self.copy_file(&source, &destination, overwrite)?
                    }
                },
                FileOperation::Move { source, destination } => {
                    if std::path::Path::new(&destination).exists() && !overwrite {
                        return Err(Error::InvalidOperation(format!("Destination already exists: {}", destination)));
                    }
                    
                    // Check if source exists
                    if !std::path::Path::new(&source).exists() {
                        ActionResult::failure(format!("File or directory does not exist: {:?}", source))
                    } else {
                        // Execute the move operation
                        self.move_file(&source, &destination, overwrite)?
                    }
                },
                FileOperation::Delete { path } => {
                    if !std::path::Path::new(&path).exists() {
                        ActionResult::failure(format!("File or directory does not exist: {:?}", path))
                    } else {
                        self.delete_file(&path)?
                    }
                },
                FileOperation::Create { path, content } => {
                    let path = Path::new(&path);
                    if path.exists() && !overwrite {
                        return ActionResult::failure("File already exists");
                    }
                    
                    if let Some(parent) = path.parent() {
                        if !parent.exists() {
                            if let Err(e) = fs::create_dir_all(parent) {
                                return ActionResult::failure(format!("Failed to create parent directories: {}", e));
                            }
                        }
                    }
                    
                    match fs::write(path, content) {
                        Ok(_) => ActionResult::success(),
                        Err(e) => ActionResult::failure(format!("Failed to create file: {}", e)),
                    }
                },
                FileOperation::CreateDirectory { path } => {
                    let path = Path::new(&path);
                    if path.exists() && !overwrite {
                        return ActionResult::failure("Directory already exists");
                    }
                    
                    match fs::create_dir_all(path) {
                        Ok(_) => ActionResult::success(),
                        Err(e) => ActionResult::failure(format!("Failed to create directory: {}", e)),
                    }
                },
                FileOperation::Exists { path } => {
                    let path = Path::new(&path);
                    let exists = path.exists();
                    ActionResult::success().with_data(exists)
                },
                FileOperation::Read { path } => {
                    let path = Path::new(&path);
                    if !path.exists() {
                        return ActionResult::failure(format!("File does not exist: {:?}", path));
                    }
                    
                    if !path.is_file() {
                        return ActionResult::failure(format!("Path is not a file: {:?}", path));
                    }
                    
                    let content = fs::read_to_string(path)
                        .map_err(|e| Error::InvalidOperation(format!("Failed to read file: {}", e)))?;
                        
                    ActionResult::success().with_data(content)
                },
            })
        }).await.map_err(|e| Error::Other(format!("Task error: {}", e)))?;
        
        Ok(result)
    }
    
    fn validate(&self) -> Result<(), Error> {
        // Validate source path
        if self.config.source.as_os_str().is_empty() {
            return Err(Error::InvalidArgument("Source path cannot be empty".to_string()));
        }
        
        // Validate destination path for Copy and Move operations
        if (matches!(self.config.operation, FileOperation::Copy { .. }) || matches!(self.config.operation, FileOperation::Move { .. })) 
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
            .with_operation(FileOperation::Create { path: file_path.to_string_lossy().to_string(), content: "Hello, world!".to_string() });
        
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
            .with_operation(FileOperation::Copy { source: source_path.to_string_lossy().to_string(), destination: dest_path.to_string_lossy().to_string() });
        
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
            .with_operation(FileOperation::Delete { path: file_path.to_string_lossy().to_string() });
        
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
            .with_operation(FileOperation::Exists { path: existing_file.to_string_lossy().to_string() });
        
        // Create plugin and action for non-existent file
        let mut action2 = FileOperationsAction::new(plugin)
            .with_operation(FileOperation::Exists { path: nonexistent_file.to_string_lossy().to_string() });
        
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
    
    #[tokio::test]
    async fn test_read_file() {
        // Create a temporary directory for testing
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        // Create file
        fs::write(&file_path, "Hello, world!").unwrap();
        
        // Create plugin and action
        let plugin = Arc::new(TestPlugin);
        let mut action = FileOperationsAction::new(plugin)
            .with_operation(FileOperation::Read { path: file_path.to_string_lossy().to_string() });
        
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
        if let Some(data_box) = result.data {
            if let Some(content) = data_box.downcast_ref::<String>() {
                assert_eq!(*content, "Hello, world!");
            } else {
                panic!("Expected string data");
            }
        } else {
            panic!("Expected data");
        }
    }
} 
