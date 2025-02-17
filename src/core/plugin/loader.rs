use std::path::{Path, PathBuf};
use tokio::sync::RwLock;
use std::sync::Arc;
use uuid::Uuid;
use libloading::{Library, Symbol};
use notify::{Watcher, RecursiveMode, Event as NotifyEvent, EventKind};
// use crate::core::Error;
use super::Plugin;
use super::traits::PluginState;

#[derive(Debug, thiserror::Error)]
pub enum LoaderError {
    #[error("Failed to load plugin: {0}")]
    LoadFailed(String),
    #[error("Plugin not found: {0}")]
    NotFound(String),
    #[error("Invalid plugin: {0}")]
    Invalid(String),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Library error: {0}")]
    Library(#[from] libloading::Error),
    #[error("Watch error: {0}")]
    Watch(String),
    #[error("Other error: {0}")]
    Other(String),
}

type PluginCreateFn = unsafe fn() -> *mut dyn Plugin;

/// A loaded plugin instance with its library
struct LoadedPlugin {
    plugin: Box<dyn Plugin>,
    library: Arc<Library>,
    path: PathBuf,
}

pub struct PluginLoader {
    path: PathBuf,
    plugins: Arc<RwLock<Vec<LoadedPlugin>>>,
    watcher: Option<notify::RecommendedWatcher>,
}

impl PluginLoader {
    pub fn new(path: PathBuf) -> Result<Self, LoaderError> {
        let plugins = Arc::new(RwLock::new(Vec::new()));
        let plugins_clone = plugins.clone();

        // Create file system watcher
        let mut watcher = notify::recommended_watcher(move |res: Result<NotifyEvent, _>| {
            if let Ok(event) = res {
                if let EventKind::Modify(_) = event.kind {
                    for path in event.paths {
                        if let Some(ext) = path.extension() {
                            if ext == "dll" || ext == "so" {
                                // Get plugin ID from path
                                let plugins = plugins_clone.clone();
                                tokio::spawn(async move {
                                    let plugins = plugins.read().await;
                                    if let Some(plugin) = plugins.iter().find(|p| p.path == path) {
                                        let id = plugin.plugin.get_info().id;
                                        if let Err(e) = Self::reload_plugin_by_id(id, plugins_clone.clone()).await {
                                            eprintln!("Failed to reload plugin {}: {}", id, e);
                                        }
                                    }
                                });
                            }
                        }
                    }
                }
            }
        }).map_err(|e| LoaderError::Watch(e.to_string()))?;

        // Start watching plugin directory
        watcher.watch(&path, RecursiveMode::NonRecursive)
            .map_err(|e| LoaderError::Watch(e.to_string()))?;

        Ok(Self {
            path,
            plugins,
            watcher: Some(watcher),
        })
    }

    pub async fn load(&mut self) -> Result<(), LoaderError> {
        let entries = tokio::fs::read_dir(&self.path)
            .await
            .map_err(|e| LoaderError::Io(e.to_string()))?;

        let mut entries = entries.peekable();
        while let Some(entry) = entries.next().await {
            let entry = entry.map_err(|e| LoaderError::Io(e.to_string()))?;
            let path = entry.path();

            if let Some(ext) = path.extension() {
                if ext == "dll" || ext == "so" {
                    match self.load_plugin(&path).await {
                        Ok(_) => println!("Loaded plugin from: {:?}", path),
                        Err(e) => eprintln!("Failed to load plugin from {:?}: {}", path, e),
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn unload(&mut self) -> Result<(), LoaderError> {
        let mut plugins = self.plugins.write().await;
        for plugin in plugins.iter_mut() {
            if plugin.plugin.get_state() == PluginState::Running {
                plugin.plugin.stop().await.map_err(|e| LoaderError::Other(e.to_string()))?;
            }
        }
        plugins.clear();
        Ok(())
    }

    pub async fn load_plugin(&self, path: &Path) -> Result<Box<dyn Plugin>, LoaderError> {
        // Load the dynamic library
        let library = Arc::new(unsafe {
            Library::new(path).map_err(LoaderError::Library)?
        });

        // Get the plugin creation function
        let create_plugin: Symbol<PluginCreateFn> = unsafe {
            library.get(b"create_plugin")
                .map_err(|e| LoaderError::LoadFailed(format!("Failed to find create_plugin: {}", e)))?
        };

        // Create plugin instance
        let plugin_ptr = unsafe { create_plugin() };
        if plugin_ptr.is_null() {
            return Err(LoaderError::LoadFailed("Plugin creation returned null".into()));
        }

        let plugin = unsafe { Box::from_raw(plugin_ptr) };

        // Initialize plugin
        plugin.initialize().await
            .map_err(|e| LoaderError::LoadFailed(format!("Plugin initialization failed: {}", e)))?;

        // Store loaded plugin
        self.plugins.write().await.push(LoadedPlugin {
            plugin: plugin.clone(),
            library,
            path: path.to_owned(),
        });

        Ok(plugin)
    }

    pub async fn unload_plugin(&self, id: Uuid) -> Result<(), LoaderError> {
        let mut plugins = self.plugins.write().await;
        if let Some(pos) = plugins.iter().position(|p| p.plugin.get_info().id == id) {
            let plugin = &mut plugins[pos];
            if plugin.plugin.get_state() == PluginState::Running {
                plugin.plugin.stop().await
                    .map_err(|e| LoaderError::Other(e.to_string()))?;
            }
            plugins.remove(pos);
            Ok(())
        } else {
            Err(LoaderError::NotFound(id.to_string()))
        }
    }

    pub async fn get_plugin(&self, id: Uuid) -> Option<Box<dyn Plugin>> {
        let plugins = self.plugins.read().await;
        plugins.iter()
            .find(|p| p.plugin.get_info().id == id)
            .map(|p| p.plugin.clone())
    }

    pub async fn reload_plugin(&self, id: Uuid) -> Result<(), LoaderError> {
        let mut plugins = self.plugins.write().await;
        
        // Find the plugin to reload
        let plugin_index = plugins.iter().position(|p| p.plugin.get_info().id == id)
            .ok_or_else(|| LoaderError::NotFound(id.to_string()))?;
        
        let plugin = &plugins[plugin_index];
        let path = plugin.path.clone();
        
        // Stop the plugin if it's running
        if plugin.plugin.get_state() == PluginState::Running {
            plugin.plugin.stop().await
                .map_err(|e| LoaderError::Other(e.to_string()))?;
        }
        
        // Remove old plugin
        plugins.remove(plugin_index);
        
        // Load new version
        drop(plugins); // Release lock before loading
        self.load_plugin(&path).await?;
        
        Ok(())
    }

    async fn reload_plugin_by_id(id: Uuid, plugins: Arc<RwLock<Vec<LoadedPlugin>>>) -> Result<(), LoaderError> {
        let mut plugins = plugins.write().await;
        
        // Find the plugin to reload
        let plugin_index = plugins.iter().position(|p| p.plugin.get_info().id == id)
            .ok_or_else(|| LoaderError::NotFound(id.to_string()))?;
        
        let plugin = &plugins[plugin_index];
        let path = plugin.path.clone();
        let config = plugin.plugin.get_config().cloned();
        
        // Stop the plugin if it's running
        if plugin.plugin.get_state() == PluginState::Running {
            plugin.plugin.stop().await
                .map_err(|e| LoaderError::Other(e.to_string()))?;
        }
        
        // Remove old plugin
        plugins.remove(plugin_index);
        
        // Load new version
        drop(plugins); // Release lock before loading
        let mut new_plugin = Self::load_plugin(&path).await?;
        
        // Restore configuration if available
        if let Some(config) = config {
            new_plugin.update_config(config).await
                .map_err(|e| LoaderError::Other(e.to_string()))?;
        }
        
        // Add new plugin back to registry
        plugins.write().await.push(LoadedPlugin {
            plugin: new_plugin,
            library: Arc::new(unsafe { Library::new(&path).map_err(LoaderError::Library)? }),
            path,
        });
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    use std::time::Duration;

    #[tokio::test]
    async fn test_plugin_hot_reload() {
        let temp_dir = tempdir().unwrap();
        let mut loader = PluginLoader::new(temp_dir.path().to_path_buf()).unwrap();
        
        // Create and load initial plugin
        let plugin_path = temp_dir.path().join("test_plugin.dll");
        fs::write(&plugin_path, b"initial").unwrap();
        loader.load_plugin(&plugin_path).await.unwrap();
        
        // Modify plugin to trigger reload
        fs::write(&plugin_path, b"modified").unwrap();
        
        // Wait for reload
        tokio::time::sleep(Duration::from_secs(1)).await;
        
        // Verify plugin was reloaded
        let plugins = loader.plugins.read().await;
        assert_eq!(plugins.len(), 1);
    }
} 