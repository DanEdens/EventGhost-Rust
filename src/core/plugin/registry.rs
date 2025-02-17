use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
// use crate::core::Error;
use crate::core::config::Config;
use super::traits::{Plugin, PluginInfo, PluginState};
// PluginState
use super::loader::{PluginLoader, LoaderError};
// use crate::core::error::{RegistryError};
// use thiserror::Error;


/// Error type for plugin registry operations
#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("Plugin not found: {0}")]
    NotFound(String),
    #[error("Plugin already exists: {0}")]
    AlreadyExists(Uuid),
    #[error("Plugin is in invalid state: {0}")]
    InvalidState(String),
    #[error("Plugin error: {0}")]
    Plugin(String),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Loader error: {0}")]
    Loader(String),
    #[error("Other error: {0}")]
    Other(String),
}

impl From<LoaderError> for RegistryError {
    fn from(err: LoaderError) -> Self {
        match err {
            LoaderError::LoadFailed(msg) => RegistryError::Loader(msg),
            LoaderError::NotFound(msg) => RegistryError::NotFound(msg),
            LoaderError::Invalid(msg) => RegistryError::Plugin(msg),
            LoaderError::Io(msg) => RegistryError::Io(msg),
            LoaderError::Other(msg) => RegistryError::Other(msg),
        }
    }
}

/// Registry for managing plugin instances
pub struct PluginRegistry {
    /// Loaded plugins
    plugins: Arc<RwLock<Vec<Arc<RwLock<Box<dyn Plugin>>>>>>,
    /// Plugin loader
    loader: PluginLoader,
    /// Plugin configurations
    configs: Arc<RwLock<HashMap<Uuid, Config>>>,
    /// Plugin directory
    plugin_dir: PathBuf,
}

impl PluginRegistry {
    /// Create a new plugin registry
    pub fn new(plugin_dir: PathBuf) -> Result<Self, RegistryError> {
        Ok(Self {
            plugins: Arc::new(RwLock::new(Vec::new())),
            loader: PluginLoader::new(plugin_dir.clone())?,
            configs: Arc::new(RwLock::new(HashMap::new())),
            plugin_dir,
        })
    }

    /// Load all plugins from the plugin directory
    pub async fn load_all(&mut self) -> Result<(), RegistryError> {
        let mut plugins = self.plugins.write().await;
        let mut configs = self.configs.write().await;
        
        // Clear existing plugins
        plugins.clear();
        configs.clear();
        
        // Load plugins from directory
        let entries = tokio::fs::read_dir(&self.plugin_dir)
            .await
            .map_err(|e| RegistryError::Io(e.to_string()))?;
            
        let mut entries = entries.peekable();
        while let Some(entry) = entries.next().await {
            let entry = entry.map_err(|e| RegistryError::Io(e.to_string()))?;
            let path = entry.path();
            
            if let Some(ext) = path.extension() {
                if ext == "dll" || ext == "so" {
                    match self.load_plugin(path).await {
                        Ok(id) => {
                            println!("Loaded plugin: {}", id);
                        }
                        Err(e) => {
                            eprintln!("Failed to load plugin: {}", e);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Unload all plugins
    pub async fn unload_all(&mut self) -> Result<(), RegistryError> {
        let mut plugins = self.plugins.write().await;
        for plugin in plugins.iter() {
            let mut plugin = plugin.write().await;
            if plugin.get_state() == PluginState::Running {
                plugin.stop().await.map_err(|e| RegistryError::Plugin(e.to_string()))?;
            }
        }
        plugins.clear();
        self.configs.write().await.clear();
        Ok(())
    }

    /// Load a plugin from a file
    pub async fn load_plugin(&self, path: PathBuf) -> Result<Uuid, RegistryError> {
        // Load plugin using loader
        let mut plugin = self.loader.load_plugin(&path).await?;
        
        // Initialize plugin
        plugin.initialize().await.map_err(|e| RegistryError::Plugin(e.to_string()))?;
        
        // Get plugin info
        let info = plugin.get_info();
        let id = info.id;
        
        // Check if plugin already exists
        {
            let plugins = self.plugins.read().await;
            for existing in plugins.iter() {
                let existing = existing.read().await;
                if existing.get_info().id == id {
                    return Err(RegistryError::AlreadyExists(id));
                }
            }
        }
        
        // Add plugin to registry
        {
            let mut plugins = self.plugins.write().await;
            plugins.push(Arc::new(RwLock::new(plugin)));
        }
        
        // Add default config if plugin is configurable
        if let Some(config) = plugin.get_config() {
            self.configs.write().await.insert(id, config.clone());
        }
        
        Ok(id)
    }

    /// Unload a plugin
    pub async fn unload_plugin(&self, id: Uuid) -> Result<(), RegistryError> {
        let mut plugins = self.plugins.write().await;
        
        // Find plugin index
        let index = plugins.iter().position(|p| {
            let plugin = p.blocking_read();
            plugin.get_info().id == id
        }).ok_or_else(|| RegistryError::NotFound(id.to_string()))?;
        
        // Stop plugin if running
        {
            let plugin = &plugins[index];
            let mut plugin = plugin.write().await;
            if plugin.get_state() == PluginState::Running {
                plugin.stop().await.map_err(|e| RegistryError::Plugin(e.to_string()))?;
            }
        }
        
        // Remove plugin
        plugins.remove(index);
        self.configs.write().await.remove(&id);
        
        Ok(())
    }

    /// Get a plugin by ID
    pub async fn get_plugin(&self, id: Uuid) -> Result<Arc<RwLock<Box<dyn Plugin>>>, RegistryError> {
        let plugins = self.plugins.read().await;
        for plugin in plugins.iter() {
            let plugin_ref = plugin.read().await;
            if plugin_ref.get_info().id == id {
                return Ok(plugin.clone());
            }
        }
        Err(RegistryError::NotFound(id.to_string()))
    }

    /// Get all loaded plugins
    pub async fn get_plugins(&self) -> Vec<PluginInfo> {
        let plugins = self.plugins.read().await;
        let mut infos = Vec::new();
        for plugin in plugins.iter() {
            let plugin = plugin.read().await;
            infos.push(plugin.get_info());
        }
        infos
    }

    /// Start a plugin
    pub async fn start_plugin(&self, id: Uuid) -> Result<(), RegistryError> {
        let plugin = self.get_plugin(id).await?;
        let mut plugin = plugin.write().await;
        
        if plugin.get_state() != PluginState::Initialized {
            return Err(RegistryError::InvalidState(
                format!("Plugin must be initialized before starting, current state: {:?}", 
                    plugin.get_state())
            ));
        }
        
        plugin.start().await.map_err(|e| RegistryError::Plugin(e.to_string()))
    }

    /// Stop a plugin
    pub async fn stop_plugin(&self, id: Uuid) -> Result<(), RegistryError> {
        let plugin = self.get_plugin(id).await?;
        let mut plugin = plugin.write().await;
        
        if plugin.get_state() != PluginState::Running {
            return Err(RegistryError::InvalidState(
                format!("Plugin must be running before stopping, current state: {:?}", 
                    plugin.get_state())
            ));
        }
        
        plugin.stop().await.map_err(|e| RegistryError::Plugin(e.to_string()))
    }

    /// Update plugin configuration
    pub async fn update_plugin_config(&self, id: Uuid, config: Config) -> Result<(), RegistryError> {
        let plugin = self.get_plugin(id).await?;
        let mut plugin = plugin.write().await;
        
        plugin.update_config(config.clone()).await
            .map_err(|e| RegistryError::Plugin(e.to_string()))?;
            
        self.configs.write().await.insert(id, config);
        
        Ok(())
    }

    /// Get plugin configuration
    pub async fn get_plugin_config(&self, id: Uuid) -> Result<Option<Config>, RegistryError> {
        Ok(self.configs.read().await.get(&id).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::mocks::MockPlugin;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_plugin_registry() {
        let temp_dir = tempdir().unwrap();
        let registry = PluginRegistry::new(temp_dir.path().to_path_buf()).unwrap();
        
        // Test loading all plugins
        registry.load_all().await.unwrap();
        
        // Test getting plugins
        let plugins = registry.get_plugins().await;
        assert!(plugins.is_empty());
        
        // Test unloading all plugins
        registry.unload_all().await.unwrap();
    }
} 