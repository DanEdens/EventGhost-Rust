use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use libloading::{Library, Symbol};
use notify::{Watcher, RecursiveMode, Event as NotifyEvent};
use crate::core::Error;
use crate::core::plugin::{Plugin, PluginInfo};
use uuid::Uuid;

/// Error type for plugin loading operations
#[derive(Debug)]
pub enum PluginLoadError {
    /// Failed to load library
    LibraryLoad(String),
    /// Failed to find plugin entry point
    EntryPoint(String),
    /// Failed to create plugin instance
    Creation(String),
    /// Failed to watch plugin file
    Watch(String),
}

impl std::fmt::Display for PluginLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginLoadError::LibraryLoad(msg) => write!(f, "Failed to load plugin library: {}", msg),
            PluginLoadError::EntryPoint(msg) => write!(f, "Failed to find plugin entry point: {}", msg),
            PluginLoadError::Creation(msg) => write!(f, "Failed to create plugin instance: {}", msg),
            PluginLoadError::Watch(msg) => write!(f, "Failed to watch plugin file: {}", msg),
        }
    }
}

impl std::error::Error for PluginLoadError {}

impl From<PluginLoadError> for Error {
    fn from(err: PluginLoadError) -> Self {
        Error::Plugin(err.to_string())
    }
}

type PluginCreateFn = unsafe fn() -> *mut dyn Plugin;

/// A loaded plugin instance
pub struct LoadedPlugin {
    /// The plugin instance
    pub plugin: Box<dyn Plugin>,
    /// The library handle
    library: Arc<Library>,
    /// Path to the plugin file
    path: PathBuf,
    /// Plugin ID
    id: Uuid,
}

impl LoadedPlugin {
    /// Get the plugin's ID
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Get the plugin's path
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// Manages plugin loading and reloading
pub struct PluginLoader {
    /// Currently loaded plugins
    plugins: Arc<RwLock<Vec<LoadedPlugin>>>,
    /// File system watcher for plugin changes
    watcher: notify::RecommendedWatcher,
    /// Plugin directory path
    plugin_dir: PathBuf,
}

impl PluginLoader {
    /// Create a new plugin loader
    pub fn new(plugin_dir: PathBuf) -> Result<Self, PluginLoadError> {
        let plugins = Arc::new(RwLock::new(Vec::new()));
        let plugins_clone = plugins.clone();

        // Create file system watcher
        let mut watcher = notify::recommended_watcher(move |res: Result<NotifyEvent, _>| {
            if let Ok(event) = res {
                if let notify::EventKind::Modify(_) = event.kind {
                    for path in event.paths {
                        if let Some(ext) = path.extension() {
                            if ext == "dll" {
                                // Reload plugin
                                let plugins = plugins_clone.clone();
                                tokio::spawn(async move {
                                    if let Err(e) = Self::reload_plugin(&path, plugins).await {
                                        eprintln!("Failed to reload plugin: {}", e);
                                    }
                                });
                            }
                        }
                    }
                }
            }
        }).map_err(|e| PluginLoadError::Watch(e.to_string()))?;

        // Start watching plugin directory
        watcher.watch(&plugin_dir, RecursiveMode::NonRecursive)
            .map_err(|e| PluginLoadError::Watch(e.to_string()))?;

        Ok(Self {
            plugins,
            watcher,
            plugin_dir,
        })
    }

    /// Load a plugin from a file
    pub async fn load_plugin(&self, path: &Path) -> Result<(), PluginLoadError> {
        // Load the dynamic library
        let library = Arc::new(unsafe {
            Library::new(path)
                .map_err(|e| PluginLoadError::LibraryLoad(e.to_string()))?
        });

        // Get the plugin creation function
        let create_fn: Symbol<PluginCreateFn> = unsafe {
            library.get(b"create_plugin")
                .map_err(|e| PluginLoadError::EntryPoint(e.to_string()))?
        };

        // Create the plugin instance
        let plugin_ptr = unsafe {
            create_fn()
        };

        if plugin_ptr.is_null() {
            return Err(PluginLoadError::Creation("Plugin creation returned null".into()));
        }

        let plugin = unsafe {
            Box::from_raw(plugin_ptr)
        };

        let loaded_plugin = LoadedPlugin {
            plugin,
            library,
            path: path.to_owned(),
            id: Uuid::new_v4(),
        };

        // Add to loaded plugins
        self.plugins.write().await.push(loaded_plugin);

        Ok(())
    }

    /// Reload a plugin
    async fn reload_plugin(path: &Path, plugins: Arc<RwLock<Vec<LoadedPlugin>>>) -> Result<(), PluginLoadError> {
        let mut plugins = plugins.write().await;
        
        // Find the plugin to reload
        if let Some(index) = plugins.iter().position(|p| p.path == path) {
            // Remove old plugin
            plugins.remove(index);
            
            // Load new version
            // Note: We create a new PluginLoader just for loading to avoid self-reference
            let temp_loader = Self::new(path.parent().unwrap().to_owned())?;
            temp_loader.load_plugin(path).await?;
            
            // Transfer the loaded plugin
            if let Some(loaded) = temp_loader.plugins.write().await.pop() {
                plugins.push(loaded);
            }
        }

        Ok(())
    }

    /// Get all loaded plugins
    pub async fn get_plugins(&self) -> Vec<PluginInfo> {
        self.plugins.read().await
            .iter()
            .map(|p| p.plugin.get_info())
            .collect()
    }

    /// Get a specific plugin by ID
    pub async fn get_plugin(&self, id: Uuid) -> Option<Box<dyn Plugin>> {
        self.plugins.read().await
            .iter()
            .find(|p| p.id == id)
            .map(|p| Box::new(p.plugin.clone()))
    }

    /// Unload a plugin
    pub async fn unload_plugin(&self, id: Uuid) -> Result<(), PluginLoadError> {
        let mut plugins = self.plugins.write().await;
        if let Some(index) = plugins.iter().position(|p| p.id == id) {
            plugins.remove(index);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[tokio::test]
    async fn test_plugin_loading() {
        // Create temporary plugin directory
        let dir = tempdir().unwrap();
        let loader = PluginLoader::new(dir.path().to_owned()).unwrap();

        // Copy test plugin to temp directory
        let plugin_path = dir.path().join("test_plugin.dll");
        fs::copy("path/to/test_plugin.dll", &plugin_path).unwrap();

        // Load plugin
        loader.load_plugin(&plugin_path).await.unwrap();

        // Verify plugin is loaded
        let plugins = loader.get_plugins().await;
        assert_eq!(plugins.len(), 1);

        // Modify plugin to trigger reload
        fs::write(&plugin_path, b"modified").unwrap();

        // Wait for reload
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // Verify plugin was reloaded
        let plugins = loader.get_plugins().await;
        assert_eq!(plugins.len(), 1);
    }
} 
