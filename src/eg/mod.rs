pub mod action;
pub mod tree;
pub mod classes;
pub mod winapi;
pub mod bunch;
pub mod globals;

use crate::core::{EventManager, PluginRegistry};
use crate::core::Error;
use std::sync::{Arc, RwLock};

pub struct Globals {
    events: Arc<RwLock<EventManager>>,
    plugins: Arc<RwLock<PluginRegistry>>,
    document: Arc<RwLock<tree::Document>>,
}

impl Globals {
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(EventManager::new())),
            plugins: Arc::new(RwLock::new(PluginRegistry::new())),
            document: Arc::new(RwLock::new(tree::Document::new())),
        }
    }

    pub fn events(&self) -> Arc<RwLock<EventManager>> {
        self.events.clone()
    }

    pub fn plugins(&self) -> Arc<RwLock<PluginRegistry>> {
        self.plugins.clone()
    }

    pub fn document(&self) -> Arc<RwLock<tree::Document>> {
        self.document.clone()
    }
} 