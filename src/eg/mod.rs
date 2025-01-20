pub mod action;
pub mod tree;
pub mod classes;
pub mod winapi;
pub mod bunch;
pub mod globals;

use crate::core::event::EventManager;
use crate::core::PluginRegistry;
use crate::core::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::path::PathBuf;

pub struct EventGhost {
    event_manager: EventManager,
    plugin_registry: PluginRegistry,
    stop_flag: Arc<RwLock<bool>>,
}

impl EventGhost {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            event_manager: EventManager::new(),
            plugin_registry: PluginRegistry::new(PathBuf::from("src\plugins"))?,
            stop_flag: Arc::new(RwLock::new(false)),
        })
    }

    pub fn get_event_manager(&self) -> &EventManager {
        &self.event_manager
    }

    pub fn get_plugin_registry(&self) -> &PluginRegistry {
        &self.plugin_registry
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        // Initialize plugins
        self.plugin_registry.load_all().await?;
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), Error> {
        // Stop and unload plugins
        self.plugin_registry.unload_all().await?;
        Ok(())
    }

    pub async fn should_stop(&self) -> bool {
        *self.stop_flag.read().await
    }
} 