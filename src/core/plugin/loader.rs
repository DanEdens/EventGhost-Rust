use std::path::PathBuf;
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::core::Error;
use super::Plugin;

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
    #[error("Other error: {0}")]
    Other(String),
}

pub struct PluginLoader {
    path: PathBuf,
    plugins: Arc<RwLock<Vec<Box<dyn Plugin>>>>,
}

impl PluginLoader {
    pub fn new(path: PathBuf) -> Result<Self, LoaderError> {
        Ok(Self {
            path,
            plugins: Arc::new(RwLock::new(Vec::new())),
        })
    }

    pub async fn load(&mut self) -> Result<(), LoaderError> {
        // Phase 1: Basic loading structure
        // TODO: Implement actual plugin loading
        Ok(())
    }

    pub async fn unload(&mut self) -> Result<(), LoaderError> {
        // Phase 1: Basic unloading structure
        self.plugins.write().await.clear();
        Ok(())
    }
} 