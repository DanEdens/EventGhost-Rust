use std::path::PathBuf;
use tokio::sync::RwLock;
use std::sync::Arc;
use uuid::Uuid;
// use crate::core::Error;
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
        let entries = tokio::fs::read_dir(&self.path)
            .await
            .map_err(|e| LoaderError::Io(e.to_string()))?;

        let mut entries = entries.peekable();
        while let Some(entry) = entries.next().await {
            let entry = entry.map_err(|e| LoaderError::Io(e.to_string()))?;
            let path = entry.path();

            if let Some(ext) = path.extension() {
                if ext == "dll" || ext == "so" {
                    // TODO: Implement actual plugin loading
                    println!("Would load plugin from: {:?}", path);
                }
            }
        }

        Ok(())
    }

    pub async fn unload(&mut self) -> Result<(), LoaderError> {
        self.plugins.write().await.clear();
        Ok(())
    }

    pub async fn load_plugin(&self, path: &PathBuf) -> Result<Box<dyn Plugin>, LoaderError> {
        // TODO: Implement actual plugin loading
        Err(LoaderError::Other("Not implemented".to_string()))
    }

    pub async fn unload_plugin(&self, id: Uuid) -> Result<(), LoaderError> {
        let mut plugins = self.plugins.write().await;
        if let Some(pos) = plugins.iter().position(|p| p.get_info().id == id) {
            plugins.remove(pos);
            Ok(())
        } else {
            Err(LoaderError::NotFound(id.to_string()))
        }
    }

    pub async fn get_plugin(&self, id: Uuid) -> Option<Box<dyn Plugin>> {
        let plugins = self.plugins.read().await;
        plugins.iter()
            .find(|p| p.get_info().id == id)
            .map(|p| p.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_plugin_loader() {
        let temp_dir = tempdir().unwrap();
        let mut loader = PluginLoader::new(temp_dir.path().to_path_buf()).unwrap();
        
        // Test basic loading
        loader.load().await.unwrap();
        
        // Test unloading
        loader.unload().await.unwrap();
    }
} 