use std::path::PathBuf;
// , 
use serde::{Serialize, Deserialize};
// use crate::core::Error;
use thiserror::Error;
use std::fmt::Debug;
use std::collections::HashMap;
use serde::de::DeserializeOwned;
use serde_json::Value;

/// Error type for configuration operations
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Invalid configuration: {0}")]
    Invalid(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Other error: {0}")]
    Other(String),
}

impl From<String> for ConfigError {
    fn from(s: String) -> Self {
        ConfigError::Other(s)
    }
}

impl From<&str> for ConfigError {
    fn from(s: &str) -> Self {
        ConfigError::Other(s.to_string())
    }
}

/// Base trait for configuration storage
pub trait ConfigStore: Send + Sync + Debug {
    /// Load configuration from storage
    fn load(&self) -> Result<Config, ConfigError>;
    /// Save configuration to storage
    fn save(&self, config: &Config) -> Result<(), ConfigError>;
}

/// Configuration data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Global settings
    pub global: GlobalConfig,
    /// Plugin-specific settings
    pub plugins: Vec<PluginConfig>,
    /// Generic settings map
    pub settings: HashMap<String, Value>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            global: GlobalConfig::default(),
            plugins: Vec::new(),
            settings: HashMap::new(),
        }
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.settings.get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    pub fn set<T: Serialize>(&mut self, key: &str, value: T) -> Result<(), ConfigError> {
        let value = serde_json::to_value(value)
            .map_err(|e| ConfigError::Invalid(format!("Failed to serialize value: {}", e)))?;
        self.settings.insert(key.to_string(), value);
        Ok(())
    }
}

/// Global configuration settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalConfig {
    /// Plugin directory path
    pub plugin_dir: PathBuf,
    /// Log level
    pub log_level: String,
    /// UI theme
    pub theme: String,
}

/// Plugin-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    /// Plugin ID
    pub id: String,
    /// Plugin settings
    pub settings: HashMap<String, Value>,
    /// Plugin enabled state
    pub enabled: bool,
}

/// Configuration manager
pub struct ConfigManager {
    store: Box<dyn ConfigStore>,
    config: Config,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new(store: Box<dyn ConfigStore>) -> Result<Self, ConfigError> {
        // TODO: Implement config manager creation
        
        println!("Creating config manager with store: {:?}", store);
        unimplemented!()
    }



    /// Get the current configuration
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    /// Update the configuration
    pub fn update_config(&mut self, config: Config) -> Result<(), ConfigError> {
        // TODO: Implement config update
        println!("Updating config: {:?}", config);
        unimplemented!()
    }


    /// Get plugin configuration
    pub fn get_plugin_config(&self, id: &str) -> Option<&PluginConfig> {
        // TODO: Implement plugin config retrieval
        println!("Getting plugin config for: {}", id);
        unimplemented!()
    }


    /// Update plugin configuration
    pub fn update_plugin_config(&mut self, config: PluginConfig) -> Result<(), ConfigError> {
        // TODO: Implement plugin config update
        println!("Updating plugin config: {:?}", config);
        unimplemented!()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     struct MockConfigStore;

//     impl ConfigStore for MockConfigStore {
//         fn load(&self) -> Result<Config, ConfigError> {
//             // TODO: Implement mock load
//             unimplemented!()
//         }

//         fn save(&self, _config: &Config) -> Result<(), ConfigError> {
//             // TODO: Implement mock save
//             unimplemented!()
//         }
//     }

//     #[test]
//     fn test_config_manager() {
//         // TODO: Implement config manager tests
//         unimplemented!()
//     }
// } 
