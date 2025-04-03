use std::path::{Path, PathBuf};
use std::fs;
use std::sync::{Arc, Mutex};
use serde_json::{Value as JsonValue, json};
use chrono::{DateTime, Local};
use log::{debug, info, warn, error};

use crate::core::actions::system::file_operations::{FileOperationsAction, ConfigFileType, Error};
use crate::core::plugin::Plugin;

/// Configuration change event type
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigChangeEvent {
    /// Configuration was loaded from a file
    Loaded { path: PathBuf },
    /// Configuration was saved to a file
    Saved { path: PathBuf },
    /// Configuration was modified
    Modified,
    /// Configuration was reset to defaults
    Reset,
}

struct DummyConfigPlugin;

impl Plugin for DummyConfigPlugin {
    fn name(&self) -> &str {
        "ConfigPlugin"
    }

    fn description(&self) -> &str {
        "Plugin for configuration file operations"
    }
}

/// Configuration manager for EventGhost
/// 
/// Handles loading, saving, and managing configuration files
#[derive(Debug)]
pub struct ConfigManager {
    /// Current configuration data
    config: Arc<Mutex<JsonValue>>,
    /// Path to the current configuration file
    config_path: Option<PathBuf>,
    /// Whether the configuration has been modified since last save
    modified: bool,
    /// Last time the configuration was saved
    last_saved: Option<DateTime<Local>>,
    /// File operations action for file I/O
    file_operations: FileOperationsAction,
    /// Configuration change listeners
    change_listeners: Vec<Box<dyn Fn(ConfigChangeEvent) + Send + Sync>>,
}

impl ConfigManager {
    /// Create a new configuration manager with default settings
    pub fn new() -> Self {
        let plugin = Arc::new(DummyConfigPlugin);
        Self {
            config: Arc::new(Mutex::new(json!({}))),
            config_path: None,
            modified: false,
            last_saved: None,
            file_operations: FileOperationsAction::new(plugin),
            change_listeners: Vec::new(),
        }
    }
    
    /// Create a new configuration manager with the given configuration
    pub fn with_config(config: JsonValue) -> Self {
        let plugin = Arc::new(DummyConfigPlugin);
        Self {
            config: Arc::new(Mutex::new(config)),
            config_path: None,
            modified: false,
            last_saved: None,
            file_operations: FileOperationsAction::new(plugin),
            change_listeners: Vec::new(),
        }
    }
    
    /// Load a configuration from the given path
    pub async fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Error> {
        let path = path.as_ref();
        
        debug!("Loading configuration from {:?}", path);
        
        // Load the configuration
        let config = self.file_operations.load_config(path).await?;
        
        // Update the configuration
        let mut config_lock = self.config.lock().unwrap();
        *config_lock = config;
        drop(config_lock);
        
        // Update the path and modified flag
        self.config_path = Some(path.to_path_buf());
        self.modified = false;
        
        // Notify listeners
        self.notify_listeners(ConfigChangeEvent::Loaded { path: path.to_path_buf() });
        
        info!("Configuration loaded from {:?}", path);
        
        Ok(())
    }
    
    /// Save the current configuration to the given path
    pub async fn save<P: AsRef<Path>>(&mut self, path: P, overwrite: bool) -> Result<(), Error> {
        let path = path.as_ref();
        
        debug!("Saving configuration to {:?}", path);
        
        // Create a backup if the file exists
        if path.exists() {
            debug!("Creating backup of existing configuration file");
            let backup_path = self.file_operations.backup_config(path).await?;
            debug!("Backup created at {:?}", backup_path);
        }
        
        // Get the configuration
        let config = self.config.lock().unwrap().clone();
        
        // Save the configuration
        self.file_operations.save_config(path, &config, overwrite).await?;
        
        // Update the path and modified flag
        self.config_path = Some(path.to_path_buf());
        self.modified = false;
        self.last_saved = Some(Local::now());
        
        // Notify listeners
        self.notify_listeners(ConfigChangeEvent::Saved { path: path.to_path_buf() });
        
        info!("Configuration saved to {:?}", path);
        
        Ok(())
    }
    
    /// Save the current configuration to the previously loaded path
    pub async fn save_current(&mut self) -> Result<(), Error> {
        if let Some(path) = &self.config_path {
            self.save(path, true).await
        } else {
            Err(Error::InvalidArgument("No current configuration path".to_string()))
        }
    }
    
    /// Check if the configuration has been modified since last save
    pub fn is_modified(&self) -> bool {
        self.modified
    }
    
    /// Get the current configuration path
    pub fn config_path(&self) -> Option<&PathBuf> {
        self.config_path.as_ref()
    }
    
    /// Get the last time the configuration was saved
    pub fn last_saved(&self) -> Option<DateTime<Local>> {
        self.last_saved
    }
    
    /// Reset the configuration to defaults
    pub fn reset(&mut self) {
        debug!("Resetting configuration to defaults");
        
        // Update the configuration
        let mut config_lock = self.config.lock().unwrap();
        *config_lock = json!({});
        drop(config_lock);
        
        // Update the path and modified flag
        self.config_path = None;
        self.modified = false;
        
        // Notify listeners
        self.notify_listeners(ConfigChangeEvent::Reset);
        
        info!("Configuration reset to defaults");
    }
    
    /// Get a value from the configuration
    pub fn get(&self, path: &str) -> Option<JsonValue> {
        let config = self.config.lock().unwrap();
        
        if path.is_empty() {
            return Some(config.clone());
        }
        
        // Split the path and traverse the JSON
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = &*config;
        
        for part in parts {
            if let Some(value) = current.get(part) {
                current = value;
            } else {
                return None;
            }
        }
        
        Some(current.clone())
    }
    
    /// Set a value in the configuration
    pub fn set(&mut self, path: &str, value: JsonValue) -> Result<(), Error> {
        let mut config = self.config.lock().unwrap();
        
        if path.is_empty() {
            *config = value;
            self.modified = true;
            
            // Notify listeners
            self.notify_listeners(ConfigChangeEvent::Modified);
            
            return Ok(());
        }
        
        // Split the path and traverse/create the JSON
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = &mut *config;
        
        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                // Last part, set the value
                if let Some(obj) = current.as_object_mut() {
                    obj.insert(part.to_string(), value.clone());
                    self.modified = true;
                    
                    // Notify listeners
                    self.notify_listeners(ConfigChangeEvent::Modified);
                    
                    return Ok(());
                } else {
                    return Err(Error::InvalidArgument(format!("Path {} is not an object", path)));
                }
            } else {
                // Create object if it doesn't exist
                if let Some(obj) = current.as_object_mut() {
                    if !obj.contains_key(*part) {
                        obj.insert(part.to_string(), json!({}));
                    }
                    current = obj.get_mut(*part).unwrap();
                } else {
                    return Err(Error::InvalidArgument(format!("Path {} is not an object", path)));
                }
            }
        }
        
        Err(Error::InvalidArgument(format!("Invalid path: {}", path)))
    }
    
    /// Get the entire configuration
    pub fn get_config(&self) -> JsonValue {
        self.config.lock().unwrap().clone()
    }
    
    /// Set the entire configuration
    pub fn set_config(&mut self, config: JsonValue) {
        let mut config_lock = self.config.lock().unwrap();
        *config_lock = config;
        drop(config_lock);
        
        self.modified = true;
        
        // Notify listeners
        self.notify_listeners(ConfigChangeEvent::Modified);
    }
    
    /// Add a configuration change listener
    pub fn add_change_listener<F>(&mut self, listener: F)
    where
        F: Fn(ConfigChangeEvent) + Send + Sync + 'static,
    {
        self.change_listeners.push(Box::new(listener));
    }
    
    /// Notify all listeners of a configuration change
    fn notify_listeners(&self, event: ConfigChangeEvent) {
        for listener in &self.change_listeners {
            listener(event.clone());
        }
    }
    
    /// Get recently used configuration files
    pub fn get_recent_configs(&self, max_count: usize) -> Vec<(PathBuf, DateTime<Local>)> {
        // This would typically read from a persistent store of recent files
        // For now, we'll just return the current config if it exists
        let mut recent = Vec::new();
        
        if let Some(path) = &self.config_path {
            if let Ok(metadata) = fs::metadata(path) {
                if let Ok(modified) = metadata.modified() {
                    let datetime = DateTime::<Local>::from(modified);
                    recent.push((path.clone(), datetime));
                }
            }
        }
        
        // Sort by most recent
        recent.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Limit to max_count
        if recent.len() > max_count {
            recent.truncate(max_count);
        }
        
        recent
    }
    
    /// Determine if a configuration can be safely closed
    /// Returns true if the configuration is not modified or has no path
    pub fn can_close(&self) -> bool {
        !self.modified || self.config_path.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_load_save_config() {
        // Create a temporary directory for testing
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.json");
        
        // Create a config manager
        let mut config_manager = ConfigManager::new();
        
        // Set some configuration values
        config_manager.set("app.name", json!("EventGhost")).unwrap();
        config_manager.set("app.version", json!("1.0.0")).unwrap();
        config_manager.set("app.debug", json!(true)).unwrap();
        
        // Save the configuration
        config_manager.save(&config_path, true).await.unwrap();
        
        // Reset the configuration
        config_manager.reset();
        
        // Load the configuration
        config_manager.load(&config_path).await.unwrap();
        
        // Check the values
        assert_eq!(config_manager.get("app.name").unwrap().as_str().unwrap(), "EventGhost");
        assert_eq!(config_manager.get("app.version").unwrap().as_str().unwrap(), "1.0.0");
        assert_eq!(config_manager.get("app.debug").unwrap().as_bool().unwrap(), true);
    }
    
    #[test]
    fn test_get_set_config() {
        // Create a config manager
        let mut config_manager = ConfigManager::new();
        
        // Set some configuration values
        config_manager.set("app.name", json!("EventGhost")).unwrap();
        config_manager.set("app.version", json!("1.0.0")).unwrap();
        config_manager.set("app.debug", json!(true)).unwrap();
        
        // Get the entire configuration
        let config = config_manager.get_config();
        
        // Check the values
        assert_eq!(config["app"]["name"].as_str().unwrap(), "EventGhost");
        assert_eq!(config["app"]["version"].as_str().unwrap(), "1.0.0");
        assert_eq!(config["app"]["debug"].as_bool().unwrap(), true);
        
        // Set a new configuration
        let new_config = json!({
            "app": {
                "name": "EventGhost",
                "version": "2.0.0",
                "debug": false
            }
        });
        
        config_manager.set_config(new_config);
        
        // Check the values
        assert_eq!(config_manager.get("app.version").unwrap().as_str().unwrap(), "2.0.0");
        assert_eq!(config_manager.get("app.debug").unwrap().as_bool().unwrap(), false);
    }
    
    #[test]
    fn test_change_listeners() {
        // Create a config manager
        let mut config_manager = ConfigManager::new();
        
        // Add a change listener
        let changes = Arc::new(Mutex::new(Vec::new()));
        let changes_clone = changes.clone();
        
        config_manager.add_change_listener(move |event| {
            let mut changes = changes_clone.lock().unwrap();
            changes.push(event);
        });
        
        // Make changes
        config_manager.set("app.name", json!("EventGhost")).unwrap();
        config_manager.reset();
        
        // Check the events
        let changes = changes.lock().unwrap();
        assert_eq!(changes.len(), 2);
        
        match &changes[0] {
            ConfigChangeEvent::Modified => {},
            _ => panic!("Expected Modified event"),
        }
        
        match &changes[1] {
            ConfigChangeEvent::Reset => {},
            _ => panic!("Expected Reset event"),
        }
    }
} 