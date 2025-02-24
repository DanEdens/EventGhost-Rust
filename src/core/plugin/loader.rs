use std::path::{Path, PathBuf};
use tokio::sync::RwLock;
use std::sync::Arc;
use uuid::Uuid;
use libloading::{Library, Symbol};
use notify::{Watcher, RecursiveMode, Event as NotifyEvent, EventKind};
// use crate::core::Error;
use super::Plugin;
use super::traits::{PluginState, PluginInfo};
use crate::core::config::Config;
use tokio::fs;
use futures::executor;

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

impl From<std::io::Error> for LoaderError {
    fn from(err: std::io::Error) -> Self {
        LoaderError::Io(err.to_string())
    }
}

impl From<notify::Error> for LoaderError {
    fn from(err: notify::Error) -> Self {
        LoaderError::Watch(err.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for LoaderError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        LoaderError::Other(err.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for LoaderError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        LoaderError::Other(err.to_string())
    }
}

impl From<&dyn std::error::Error> for LoaderError {
    fn from(err: &dyn std::error::Error) -> Self {
        LoaderError::Other(err.to_string())
    }
}

type PluginCreateFn = unsafe fn() -> *mut dyn Plugin;

#[derive(Clone)]
pub struct LoadedPlugin {
    pub path: PathBuf,
    pub plugin: Arc<RwLock<Box<dyn Plugin + Send + Sync>>>,
}

impl LoadedPlugin {
    pub async fn get_state(&self) -> Result<PluginState, LoaderError> {
        Ok(self.plugin.read().await.get_state())
    }

    pub async fn stop(&mut self) -> Result<(), LoaderError> {
        match self.plugin.write().await.stop().await {
            Ok(_) => Ok(()),
            Err(e) => Err(LoaderError::Other(e.to_string()))
        }
    }

    pub async fn get_info(&self) -> Result<PluginInfo, LoaderError> {
        Ok(self.plugin.read().await.get_info())
    }

    pub async fn get_config(&self) -> Result<Option<Config>, LoaderError> {
        Ok(self.plugin.read().await.get_config().cloned())
    }
}

pub struct PluginLoader {
    path: PathBuf,
    plugins: Arc<RwLock<Vec<LoadedPlugin>>>,
    watcher: Option<notify::RecommendedWatcher>,
}

impl PluginLoader {
    async fn handle_plugin_change(plugins: Arc<RwLock<Vec<LoadedPlugin>>>, path: PathBuf) {
        let plugins_guard = plugins.read().await;
        if let Some(plugin) = plugins_guard.iter().find(|p| p.path == path) {
            match plugin.get_info().await {
                Ok(info) => {
                    let id = info.id;
                    if let Err(e) = Self::reload_plugin_by_id(id, plugins.clone()).await {
                        eprintln!("Failed to reload plugin {}: {}", id, e);
                    }
                }
                Err(e) => eprintln!("Failed to get plugin info: {}", e),
            }
        }
    }

    pub fn new(path: PathBuf) -> Result<Self, LoaderError> {
        let plugins: Arc<RwLock<Vec<LoadedPlugin>>> = Arc::new(RwLock::new(Vec::new()));
        let plugins_clone = plugins.clone();

        // Create file system watcher
        let mut watcher = notify::recommended_watcher({
            let plugins = plugins_clone.clone();
            move |res: Result<NotifyEvent, _>| {
                if let Ok(event) = res {
                    if let EventKind::Modify(_) = event.kind {
                        for path_buf in event.paths {
                            if let Some(ext) = path_buf.extension() {
                                if ext == "dll" || ext == "so" {
                                    let plugins = plugins.clone();
                                    let path = path_buf.clone();
                                    tokio::spawn(async move {
                                        Self::handle_plugin_change(plugins, path).await;
                                    });
                                }
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
        let mut entries = fs::read_dir(&self.path).await?;
        
        while let Some(entry) = entries.next_entry().await? {
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
            let state = plugin.get_state().await?;
            if state == PluginState::Running {
                if let Err(e) = plugin.stop().await {
                    return Err(LoaderError::Other(e.to_string()));
                }
            }
        }
        plugins.clear();
        Ok(())
    }

    pub async fn scan_directory(&self, dir: &Path) -> Result<(), LoaderError> {
        let mut entries = fs::read_dir(dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
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

        let mut plugin = unsafe { Box::from_raw(plugin_ptr) };

        // Initialize plugin
        plugin.initialize().await
            .map_err(|e| LoaderError::LoadFailed(format!("Plugin initialization failed: {}", e)))?;

        // Store loaded plugin
        self.plugins.write().await.push(LoadedPlugin {
            path: path.to_path_buf(),
            plugin: Arc::new(RwLock::new(plugin.clone())),
        });

        Ok(plugin)
    }

    pub async fn unload_plugin(&self, id: Uuid) -> Result<(), LoaderError> {
        let mut plugins = self.plugins.write().await;
        if let Some(pos) = plugins.iter().position(|p| {
            let info = executor::block_on(p.get_info());
            info.map(|info| info.id == id).unwrap_or(false)
        }) {
            let plugin = &mut plugins[pos];
            let state = plugin.get_state().await?;
            if state == PluginState::Running {
                let mut plugin_guard = plugin.plugin.write().await;
                if let Err(e) = plugin_guard.stop().await {
                    return Err(LoaderError::Other(e.to_string()));
                }
            }
            plugins.remove(pos);
            Ok(())
        } else {
            Err(LoaderError::NotFound(id.to_string()))
        }
    }

    pub async fn get_plugin(&self, id: Uuid) -> Option<Arc<RwLock<Box<dyn Plugin + Send + Sync>>>> {
        let plugins = self.plugins.read().await;
        for p in plugins.iter() {
            if let Ok(info) = p.get_info().await {
                if info.id == id {
                    return Some(p.plugin.clone());
                }
            }
        }
        None
    }

    pub async fn reload_plugin(&self, id: Uuid) -> Result<(), LoaderError> {
        let mut plugins = self.plugins.write().await;
        
        // Find the plugin to reload
        let plugin_index = plugins.iter().position(|p| {
            let info = executor::block_on(p.get_info());
            info.map(|info| info.id == id).unwrap_or(false)
        }).ok_or_else(|| LoaderError::NotFound(id.to_string()))?;
        
        let plugin = &plugins[plugin_index];
        let path = plugin.path.clone();
        
        // Stop the plugin if it's running
        let state = plugin.get_state().await?;
        if state == PluginState::Running {
            let mut plugin_guard = plugin.plugin.write().await;
            if let Err(e) = plugin_guard.stop().await {
                return Err(LoaderError::Other(e.to_string()));
            }
        }
        
        // Remove old plugin
        plugins.remove(plugin_index);
        
        // Load new version
        drop(plugins); // Release lock before loading
        self.load_plugin(&path).await?;
        
        Ok(())
    }

    async fn reload_plugin_by_id(id: Uuid, plugins: Arc<RwLock<Vec<LoadedPlugin>>>) -> Result<(), LoaderError> {
        let mut plugins_guard = plugins.write().await;
        
        // Find the plugin to reload
        let plugin_index = plugins_guard.iter().position(|p| {
            let info = executor::block_on(p.get_info());
            info.map(|info| info.id == id).unwrap_or(false)
        }).ok_or_else(|| LoaderError::NotFound(id.to_string()))?;
        
        let plugin = &plugins_guard[plugin_index];
        let path = plugin.path.clone();
        let config = plugin.get_config().await?;
        
        // Stop the plugin if it's running
        let state = plugin.get_state().await?;
        if state == PluginState::Running {
            let mut plugin_guard = plugin.plugin.write().await;
            if let Err(e) = plugin_guard.stop().await {
                return Err(LoaderError::Other(e.to_string()));
            }
        }
        
        // Remove old plugin
        plugins_guard.remove(plugin_index);
        
        // Load new version
        drop(plugins_guard); // Release lock before loading
        let loader = PluginLoader::new(path.parent().unwrap_or_else(|| Path::new("")).to_path_buf())?;
        let mut new_plugin = loader.load_plugin(&path).await?;
        
        // Restore configuration if available
        if let Some(config) = config {
            if let Err(e) = new_plugin.update_config(config).await {
                return Err(LoaderError::Other(e.to_string()));
            }
        }
        
        // Add new plugin back to registry
        let mut plugins_guard = plugins.write().await;
        plugins_guard.push(LoadedPlugin {
            path,
            plugin: Arc::new(RwLock::new(new_plugin)),
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