pub mod action;
pub mod tree;
pub mod classes;
pub mod winapi;
pub mod bunch;
pub mod globals;

use crate::core::{EventManager, PluginRegistry};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::path::PathBuf;

pub struct EventGhost {
    event_manager: Arc<RwLock<EventManager>>,
    plugins: Arc<RwLock<PluginRegistry>>,
    stop_flag: Arc<RwLock<bool>>,
}

impl EventGhost {
    pub async fn new() -> Result<Self, crate::core::Error> {
        let plugin_dir = PathBuf::from("plugins");  // Default plugin directory
        Ok(Self {
            event_manager: Arc::new(RwLock::new(EventManager::new())),
            plugins: Arc::new(RwLock::new(PluginRegistry::new(plugin_dir)?)),
            stop_flag: Arc::new(RwLock::new(false)),
        })
    }

    pub async fn start(&mut self) -> Result<(), crate::core::Error> {
        // Initialize plugins
        let mut plugins = self.plugins.write().await;
        plugins.load_all().await?;
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), crate::core::Error> {
        // Stop and unload plugins
        let mut plugins = self.plugins.write().await;
        plugins.unload_all().await?;
        Ok(())
    }

    pub async fn should_stop(&self) -> bool {
        *self.stop_flag.read().await
    }
} 