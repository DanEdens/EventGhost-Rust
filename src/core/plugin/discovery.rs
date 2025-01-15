use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::core::Error;
use super::traits::{Plugin, PluginInfo, PluginCapability};

/// Error type for plugin discovery operations
#[derive(Debug, thiserror::Error)]
pub enum DiscoveryError {
    #[error("Invalid plugin path: {0}")]
    InvalidPath(PathBuf),
    #[error("Failed to read plugin metadata: {0}")]
    MetadataRead(String),
    #[error("Invalid plugin metadata: {0}")]
    InvalidMetadata(String),
    #[error("Plugin scan error: {0}")]
    ScanError(String),
    #[error("Plugin dependency error: {0}")]
    DependencyError(String),
}

/// Plugin dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    /// Name of required plugin
    pub name: String,
    /// Version requirement
    pub version_req: String,
    /// Whether this is an optional dependency
    pub optional: bool,
}

/// Plugin metadata from discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Basic plugin info
    pub info: PluginInfo,
    /// Path to plugin file
    pub path: PathBuf,
    /// Plugin dependencies
    pub dependencies: Vec<PluginDependency>,
    /// Plugin capabilities
    pub capabilities: Vec<PluginCapability>,
    /// Additional metadata
    pub extra: serde_json::Value,
}

/// Plugin discovery service
pub struct PluginDiscovery {
    /// Plugin directories to scan
    directories: Vec<PathBuf>,
    /// Discovered plugin metadata
    plugins: Arc<RwLock<HashMap<Uuid, PluginMetadata>>>,
    /// Plugin dependency graph
    dependencies: Arc<RwLock<petgraph::Graph<Uuid, ()>>>,
}

impl PluginDiscovery {
    /// Create a new plugin discovery service
    pub fn new() -> Self {
        Self {
            directories: Vec::new(),
            plugins: Arc::new(RwLock::new(HashMap::new())),
            dependencies: Arc::new(RwLock::new(petgraph::Graph::new())),
        }
    }

    /// Add a directory to scan for plugins
    pub fn add_directory(&mut self, path: PathBuf) -> Result<(), DiscoveryError> {
        if !path.exists() || !path.is_dir() {
            return Err(DiscoveryError::InvalidPath(path));
        }
        if !self.directories.contains(&path) {
            self.directories.push(path);
        }
        Ok(())
    }

    /// Scan for plugins in registered directories
    pub async fn scan_plugins(&mut self) -> Result<Vec<PluginMetadata>, DiscoveryError> {
        let mut discovered = Vec::new();
        
        for dir in &self.directories {
            let entries = tokio::fs::read_dir(dir)
                .await
                .map_err(|e| DiscoveryError::ScanError(e.to_string()))?;

            let mut entries = entries.peekable();
            while let Some(entry) = entries.next().await {
                let entry = entry.map_err(|e| DiscoveryError::ScanError(e.to_string()))?;
                let path = entry.path();

                // Skip non-plugin files (implement proper plugin file detection later)
                if !Self::is_plugin_file(&path) {
                    continue;
                }

                if let Ok(metadata) = self.read_plugin_metadata(&path).await {
                    let mut plugins = self.plugins.write().await;
                    plugins.insert(metadata.info.id, metadata.clone());
                    discovered.push(metadata);
                }
            }
        }

        Ok(discovered)
    }

    /// Check if a file is a potential plugin
    fn is_plugin_file(path: &Path) -> bool {
        // TODO: Implement proper plugin detection
        // For now, just check if it's a .dll or .so file
        if let Some(ext) = path.extension() {
            matches!(ext.to_str(), Some("dll") | Some("so"))
        } else {
            false
        }
    }

    /// Read plugin metadata from a file
    async fn read_plugin_metadata(&self, path: &Path) -> Result<PluginMetadata, DiscoveryError> {
        // TODO: Implement actual metadata reading from plugin files
        // For now, return dummy metadata for testing
        Ok(PluginMetadata {
            info: PluginInfo {
                id: Uuid::new_v4(),
                name: path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string(),
                description: String::new(),
                version: String::from("0.1.0"),
                author: String::new(),
                homepage: None,
                supported_platforms: Vec::new(),
                capabilities: Vec::new(),
            },
            path: path.to_path_buf(),
            dependencies: Vec::new(),
            capabilities: Vec::new(),
            extra: serde_json::Value::Null,
        })
    }

    /// Get metadata for a specific plugin
    pub async fn get_plugin_metadata(&self, id: Uuid) -> Option<PluginMetadata> {
        self.plugins.read().await.get(&id).cloned()
    }

    /// Get all discovered plugins
    pub async fn get_discovered_plugins(&self) -> Vec<PluginMetadata> {
        self.plugins.read().await.values().cloned().collect()
    }

    /// Calculate plugin load order based on dependencies
    pub async fn calculate_load_order(&self) -> Result<Vec<Uuid>, DiscoveryError> {
        let plugins = self.plugins.read().await;
        let mut graph = petgraph::Graph::new();
        let mut node_indices = HashMap::new();

        // Create nodes for all plugins
        for (id, metadata) in plugins.iter() {
            let idx = graph.add_node(*id);
            node_indices.insert(id, idx);
        }

        // Add edges for dependencies
        for (id, metadata) in plugins.iter() {
            let from_idx = node_indices[id];
            for dep in &metadata.dependencies {
                // Find the plugin that satisfies this dependency
                if let Some((dep_id, _)) = plugins.iter().find(|(_, p)| p.info.name == dep.name) {
                    let to_idx = node_indices[dep_id];
                    graph.add_edge(from_idx, to_idx, ());
                } else if !dep.optional {
                    return Err(DiscoveryError::DependencyError(
                        format!("Required dependency {} not found", dep.name)
                    ));
                }
            }
        }

        // Perform topological sort
        match petgraph::algo::toposort(&graph, None) {
            Ok(order) => Ok(order.into_iter().map(|idx| graph[idx]).collect()),
            Err(_) => Err(DiscoveryError::DependencyError(
                "Circular dependency detected".to_string()
            )),
        }
    }

    /// Validate plugin dependencies
    pub async fn validate_dependencies(&self, plugin: &PluginMetadata) -> Result<(), DiscoveryError> {
        let plugins = self.plugins.read().await;
        
        for dep in &plugin.dependencies {
            if let Some(dep_plugin) = plugins.values().find(|p| p.info.name == dep.name) {
                // TODO: Implement version requirement checking
                continue;
            } else if !dep.optional {
                return Err(DiscoveryError::DependencyError(
                    format!("Required dependency {} not found", dep.name)
                ));
            }
        }
        
        Ok(())
    }

    /// Watch for plugin changes
    pub async fn watch_for_changes(&self) -> Result<(), DiscoveryError> {
        // TODO: Implement file system watching using notify crate
        unimplemented!("File system watching not yet implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[tokio::test]
    async fn test_plugin_discovery() {
        let temp = tempdir().unwrap();
        let plugin_dir = temp.path().to_path_buf();
        
        // Create dummy plugin files
        fs::write(plugin_dir.join("test1.dll"), "dummy").unwrap();
        fs::write(plugin_dir.join("test2.dll"), "dummy").unwrap();
        fs::write(plugin_dir.join("not_a_plugin.txt"), "dummy").unwrap();

        let mut discovery = PluginDiscovery::new();
        discovery.add_directory(plugin_dir).unwrap();

        let plugins = discovery.scan_plugins().await.unwrap();
        assert_eq!(plugins.len(), 2);
        
        let all_plugins = discovery.get_discovered_plugins().await;
        assert_eq!(all_plugins.len(), 2);
    }

    #[tokio::test]
    async fn test_dependency_resolution() {
        let mut discovery = PluginDiscovery::new();
        let plugins = discovery.plugins.write().await;
        
        // Create test plugins with dependencies
        let plugin1 = PluginMetadata {
            info: PluginInfo {
                id: Uuid::new_v4(),
                name: "plugin1".to_string(),
                description: String::new(),
                version: "1.0.0".to_string(),
                author: String::new(),
                homepage: None,
                supported_platforms: Vec::new(),
                capabilities: Vec::new(),
            },
            path: PathBuf::from("plugin1.dll"),
            dependencies: vec![],
            capabilities: vec![],
            extra: serde_json::Value::Null,
        };

        let plugin2 = PluginMetadata {
            info: PluginInfo {
                id: Uuid::new_v4(),
                name: "plugin2".to_string(),
                description: String::new(),
                version: "1.0.0".to_string(),
                author: String::new(),
                homepage: None,
                supported_platforms: Vec::new(),
                capabilities: Vec::new(),
            },
            path: PathBuf::from("plugin2.dll"),
            dependencies: vec![PluginDependency {
                name: "plugin1".to_string(),
                version_req: "1.0.0".to_string(),
                optional: false,
            }],
            capabilities: vec![],
            extra: serde_json::Value::Null,
        };

        plugins.insert(plugin1.info.id, plugin1.clone());
        plugins.insert(plugin2.info.id, plugin2.clone());
        drop(plugins);

        let load_order = discovery.calculate_load_order().await.unwrap();
        assert_eq!(load_order.len(), 2);
        assert_eq!(load_order[0], plugin1.info.id);
        assert_eq!(load_order[1], plugin2.info.id);
    }

    #[tokio::test]
    async fn test_plugin_validation() {
        let mut discovery = PluginDiscovery::new();
        
        // Test plugin with missing dependency
        let plugin = PluginMetadata {
            info: PluginInfo {
                id: Uuid::new_v4(),
                name: "test".to_string(),
                description: String::new(),
                version: "1.0.0".to_string(),
                author: String::new(),
                homepage: None,
                supported_platforms: Vec::new(),
                capabilities: Vec::new(),
            },
            path: PathBuf::from("test.dll"),
            dependencies: vec![PluginDependency {
                name: "missing".to_string(),
                version_req: "1.0.0".to_string(),
                optional: false,
            }],
            capabilities: vec![],
            extra: serde_json::Value::Null,
        };

        let result = discovery.validate_dependencies(&plugin).await;
        assert!(matches!(result, Err(DiscoveryError::DependencyError(_))));

        // Test plugin with optional missing dependency
        let plugin = PluginMetadata {
            info: plugin.info.clone(),
            path: plugin.path.clone(),
            dependencies: vec![PluginDependency {
                name: "missing".to_string(),
                version_req: "1.0.0".to_string(),
                optional: true,
            }],
            capabilities: vec![],
            extra: serde_json::Value::Null,
        };

        let result = discovery.validate_dependencies(&plugin).await;
        assert!(result.is_ok());
    }
} 