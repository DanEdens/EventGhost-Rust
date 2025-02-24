use std::collections::HashMap;
use std::path::{Path, PathBuf};
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
use super::PluginCapability;
use tokio::fs;
use futures::executor;


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
    #[error("Plugin not supported: {0}")]
    NotSupported(String),
}

impl From<LoaderError> for RegistryError {
    fn from(err: LoaderError) -> Self {
        match err {
            LoaderError::LoadFailed(e) => RegistryError::Loader(e),
            LoaderError::NotFound(e) => RegistryError::NotFound(e),
            LoaderError::Invalid(e) => RegistryError::Other(e),
            LoaderError::Io(e) => RegistryError::Io(e),
            LoaderError::Library(e) => RegistryError::Loader(e.to_string()),
            LoaderError::Watch(e) => RegistryError::Other(e),
            LoaderError::Other(e) => RegistryError::Other(e),
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
        let mut entries = fs::read_dir(&self.plugin_dir)
            .await
            .map_err(|e| RegistryError::Io(e.to_string()))?;
            
        while let Some(entry) = entries.next_entry().await.map_err(|e| RegistryError::Io(e.to_string()))? {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "dll" || ext == "so" {
                    match self.load_plugin(&path).await {
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
    pub async fn load_plugin(&self, path: &Path) -> Result<Uuid, RegistryError> {
        // Load plugin using loader
        let plugin = self.loader.load_plugin(path).await?;
        let plugin = Arc::new(RwLock::new(plugin));
        
        // Get plugin info
        let info = {
            let plugin_ref = plugin.read().await;
            plugin_ref.get_info()
        };
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
            plugins.push(plugin.clone());
        }
        
        // Add default config if plugin is configurable
        {
            let plugin_ref = plugin.read().await;
            if let Some(config) = plugin_ref.get_config() {
                self.configs.write().await.insert(id, config.clone());
            }
        }
        
        Ok(id)
    }

    /// Unload a plugin
    pub async fn unload_plugin(&self, id: Uuid) -> Result<(), RegistryError> {
        let mut plugins = self.plugins.write().await;
        let index = plugins.iter().position(|p| {
            let plugin = futures::executor::block_on(p.read());
            plugin.get_info().map(|info| info.id == id).unwrap_or(false)
        });
        
        if index.is_none() {
            return Err(RegistryError::NotFound(id.to_string()));
        }
        
        // Stop plugin if running
        {
            let plugin = &plugins[index.unwrap()];
            let mut plugin = futures::executor::block_on(plugin.write());
            if plugin.get_state() == PluginState::Running {
                plugin.stop().await.map_err(|e| RegistryError::Plugin(e.to_string()))?;
            }
        }
        
        // Remove plugin
        plugins.remove(index.unwrap());
        self.configs.write().await.remove(&id);
        
        Ok(())
    }

    /// Get a plugin by ID
    pub async fn get_plugin(&self, id: Uuid) -> Result<Arc<RwLock<Box<dyn Plugin>>>, RegistryError> {
        let plugins = self.plugins.read().await;
        let index = plugins.iter().position(|p| {
            let plugin = futures::executor::block_on(p.read());
            plugin.get_info().map(|info| info.id == id).unwrap_or(false)
        });
        
        if index.is_none() {
            return Err(RegistryError::NotFound(id.to_string()));
        }
        
        Ok(plugins[index.unwrap()].clone())
    }

    /// Get all loaded plugins
    pub async fn get_plugins(&self) -> Vec<PluginInfo> {
        let plugins = self.plugins.read().await;
        let mut infos = Vec::new();
        for plugin in plugins.iter() {
            let plugin = futures::executor::block_on(plugin.read());
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
        let plugins = self.plugins.read().await;
        if let Some(plugin) = plugins.iter().find(|p| {
            let plugin_ref = futures::executor::block_on(p.read());
            plugin_ref.get_info().map(|info| info.id == id).unwrap_or(false)
        }) {
            let mut plugin = futures::executor::block_on(plugin.write());
            plugin.stop().await.map_err(|e| RegistryError::Plugin(e.to_string()))?;
        }
        Ok(())
    }

    /// Update plugin configuration
    pub async fn update_plugin_config(&self, id: Uuid, config: Config) -> Result<(), RegistryError> {
        let plugin = self.get_plugin(id).await?;
        let mut plugin = plugin.write().await;
        
        plugin.update_config(config.clone()).await
            .map_err(|e| RegistryError::Plugin(e.to_string()))?;
            
        self.configs.write().await.insert(id, config.clone());
        
        Ok(())
    }

    /// Get plugin configuration
    pub async fn get_plugin_config(&self, id: Uuid) -> Result<Option<Config>, RegistryError> {
        Ok(self.configs.read().await.get(&id).cloned())
    }

    /// Reload a plugin by ID
    pub async fn reload_plugin(&self, id: Uuid) -> Result<(), RegistryError> {
        let plugin = self.get_plugin(id).await?;
        let plugin = futures::executor::block_on(plugin.read());
        
        // Get plugin info and config before reload
        let info = plugin.get_info();
        let config = plugin.get_config().cloned();
        let path = self.plugin_dir.join(format!("{}.{}", info.name.to_lowercase(), if cfg!(windows) { "dll" } else { "so" }));
        
        // Stop plugin if running
        if plugin.get_state() == PluginState::Running {
            futures::executor::block_on(plugin.write()).stop().await.map_err(|e| RegistryError::Plugin(e.to_string()))?;
        }
        
        // Drop read lock
        drop(plugin);
        
        // Remove old plugin
        self.unload_plugin(id).await?;
        
        // Load new version
        let new_id = futures::executor::block_on(self.load_plugin(&path))?;
        
        // Restore configuration if available
        if let Some(config) = config {
            self.update_plugin_config(new_id, config).await?;
        }
        
        Ok(())
    }

    /// Enable hot-reloading for a plugin
    pub async fn enable_hot_reload(&self, id: Uuid) -> Result<(), RegistryError> {
        let plugin = self.get_plugin(id).await?;
        let plugin = futures::executor::block_on(plugin.read());
        
        // Check if plugin supports hot-reloading
        if !plugin.get_capabilities().contains(&PluginCapability::HotReload) {
            return Err(RegistryError::NotSupported(format!("Plugin {} does not support hot-reloading", id)));
        }
        
        Ok(())
    }

    /// Disable hot-reloading for a plugin
    pub async fn disable_hot_reload(&self, id: Uuid) -> Result<(), RegistryError> {
        let plugin = self.get_plugin(id).await?;
        let plugin = futures::executor::block_on(plugin.read());
        
        // Check if plugin supports hot-reloading
        if !plugin.get_capabilities().contains(&PluginCapability::HotReload) {
            return Err(RegistryError::NotSupported(format!("Plugin {} does not support hot-reloading", id)));
        }
        
        Ok(())
    }

    pub async fn scan_directory(&self, dir: &Path) -> Result<(), RegistryError> {
        let mut entries = fs::read_dir(dir).await.map_err(|e| RegistryError::Io(e.to_string()))?;
        
        while let Some(entry) = entries.next_entry().await.map_err(|e| RegistryError::Io(e.to_string()))? {
            if let Ok(file_type) = entry.file_type().await {
                if file_type.is_file() {
                    let path = entry.path();
                    if let Some(ext) = path.extension() {
                        if ext == "dll" || ext == "so" || ext == "dylib" {
                            self.load_plugin(&path).await?;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::mocks::MockPlugin;
    use tempfile::tempdir;
    use std::fs;
    use std::time::Duration;
    
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

    #[tokio::test]
    async fn test_plugin_hot_reload() {
        let temp_dir = tempdir().unwrap();
        let mut registry = PluginRegistry::new(temp_dir.path().to_path_buf()).unwrap();
        
        // Create and load initial plugin
        let plugin_path = temp_dir.path().join("test_plugin.dll");
        fs::write(&plugin_path, b"initial").unwrap();
        let id = registry.load_plugin(plugin_path.clone()).await.unwrap();
        
        // Enable hot-reloading
        registry.enable_hot_reload(id).await.unwrap();
        
        // Modify plugin
        fs::write(&plugin_path, b"modified").unwrap();
        
        // Wait for reload
        tokio::time::sleep(Duration::from_secs(1)).await;
        
        // Verify plugin was reloaded
        let plugin = registry.get_plugin(id).await.unwrap();
        let plugin = futures::executor::block_on(plugin.read());
        assert_eq!(plugin.get_state(), PluginState::Initialized);
    }
} 