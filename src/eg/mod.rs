pub mod action;
pub mod tree;
pub mod classes;
pub mod winapi;
pub mod bunch;
pub mod globals;

use crate::core::{EventManager, PluginRegistry};
use crate::core::Error;
use std::sync::{Arc, RwLock};
use std::path::PathBuf;
use tokio::sync::RwLock;

pub struct EventGhost {
    event_manager: Arc<RwLock<EventManager>>,
    plugins: Arc<RwLock<PluginRegistry>>,
}

impl EventGhost {
    pub async fn new() -> Result<Self, crate::core::Error> {
        let plugin_dir = PathBuf::from("plugins");  // Default plugin directory
        Ok(Self {
            event_manager: Arc::new(RwLock::new(EventManager::new())),
            plugins: Arc::new(RwLock::new(PluginRegistry::new(plugin_dir)?)),
        })
    }

    pub async fn start(&mut self) -> Result<(), crate::core::Error> {
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), crate::core::Error> {
        Ok(())
    }
} 