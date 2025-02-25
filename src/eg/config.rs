use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::cell::RefCell;
use log::{debug, error, info};
use quick_xml::de::from_str as xml_from_str;
use quick_xml::se::to_string as xml_to_string;

/// Represents a plugin instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    /// The unique identifier of the plugin.
    pub id: Uuid,
    /// The name of the plugin.
    pub name: String,
    /// The configuration options for the plugin.
    pub config: HashMap<String, String>,
    // Other fields as needed
}

/// Represents a folder that can contain other configuration items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    /// The unique identifier of the folder.
    pub id: Uuid,
    /// The name of the folder.
    pub name: String,
    // Other fields as needed
}

/// Represents a macro.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macro {
    /// The unique identifier of the macro.
    pub id: Uuid,
    /// The name of the macro.
    pub name: String,
    /// The list of events that trigger the macro.
    pub events: Vec<Uuid>,
    /// The list of actions performed by the macro.
    pub actions: Vec<Uuid>,
    // Other fields as needed
}

/// Represents an event that can trigger macros.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// The unique identifier of the event.
    pub id: Uuid,
    /// The name of the event.
    pub name: String,
    /// The parameters for the event.
    pub parameters: HashMap<String, String>,
    // Other fields as needed
}

/// Represents an action that can be performed by a macro.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// The unique identifier of the action.
    pub id: Uuid,
    /// The name of the action.
    pub name: String,
    /// The parameters for the action.
    pub parameters: HashMap<String, String>,
    // Other fields as needed
}

/// Represents a configuration item, which can be a plugin, folder, macro, event, or action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigItem {
    Plugin(Plugin),
    Folder(Folder),
    Macro(Macro),
    Event(Event),
    Action(Action),
}

impl ConfigItem {
    /// Gets the unique identifier of the configuration item.
    pub fn id(&self) -> Uuid {
        match self {
            ConfigItem::Plugin(plugin) => plugin.id,
            ConfigItem::Folder(folder) => folder.id,
            ConfigItem::Macro(macro_) => macro_.id,
            ConfigItem::Event(event) => event.id,
            ConfigItem::Action(action) => action.id,
        }
    }
    
    /// Gets the name of the configuration item.
    pub fn name(&self) -> &str {
        match self {
            ConfigItem::Plugin(plugin) => &plugin.name,
            ConfigItem::Folder(folder) => &folder.name,
            ConfigItem::Macro(macro_) => &macro_.name,
            ConfigItem::Event(event) => &event.name,
            ConfigItem::Action(action) => &action.name,
        }
    }
}

/// Represents the configuration data for EventGhost.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// The list of configuration items.
    pub items: Vec<ConfigItem>,
}

impl Config {
    /// Creates a new empty configuration.
    pub fn new() -> Self {
        Config { items: Vec::new() }
    }
    
    /// Adds a new configuration item.
    pub fn add_item(&mut self, item: ConfigItem) {
        self.items.push(item);
    }
    
    /// Removes a configuration item by its unique identifier.
    pub fn remove_item(&mut self, id: Uuid) {
        self.items.retain(|item| item.id() != id);
    }
    
    /// Finds a configuration item by its unique identifier.
    pub fn find_item(&self, id: Uuid) -> Option<&ConfigItem> {
        self.items.iter().find(|item| item.id() == id)
    }
    
    /// Finds a mutable reference to a configuration item by its unique identifier.
    pub fn find_item_mut(&mut self, id: Uuid) -> Option<&mut ConfigItem> {
        self.items.iter_mut().find(|item| item.id() == id)
    }

    /// Saves the configuration to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
        
        let content = match extension.to_lowercase().as_str() {
            "xml" | "egtree" => {
                debug!("Saving configuration as XML to {}", path.display());
                xml_to_string(&self).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            },
            _ => {
                debug!("Saving configuration as JSON to {}", path.display());
                serde_json::to_string_pretty(&self).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            }
        };
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        
        fs::write(path, content)?;
        info!("Configuration saved to {}", path.display());
        Ok(())
    }

    /// Loads the configuration from a file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
        
        let content = fs::read_to_string(path)?;
        
        let config = match extension.to_lowercase().as_str() {
            "xml" | "egtree" => {
                debug!("Loading configuration from XML file {}", path.display());
                xml_from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            },
            _ => {
                debug!("Loading configuration from JSON file {}", path.display());
                serde_json::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            }
        };
        
        info!("Configuration loaded from {}", path.display());
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_config_crud() {
        let mut config = Config::new();
        
        // Add a plugin
        let plugin = Plugin {
            id: Uuid::new_v4(),
            name: "Test Plugin".to_string(),
            config: HashMap::new(),
        };
        config.add_item(ConfigItem::Plugin(plugin.clone()));
        
        // Add a folder
        let folder = Folder {
            id: Uuid::new_v4(),
            name: "Test Folder".to_string(),
        };
        config.add_item(ConfigItem::Folder(folder.clone()));
        
        // Verify items were added
        assert_eq!(config.items.len(), 2);
        
        // Find the plugin
        let found_plugin = config.find_item(plugin.id).unwrap();
        assert_eq!(found_plugin.name(), plugin.name);
        
        // Remove the folder
        config.remove_item(folder.id);
        assert_eq!(config.items.len(), 1);
    }

    #[test]
    fn test_config_persistence() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.json");

        // Create a test configuration
        let mut config = Config::new();
        
        let plugin = Plugin {
            id: Uuid::new_v4(),
            name: "Test Plugin".to_string(),
            config: HashMap::new(),
        };
        config.add_item(ConfigItem::Plugin(plugin));
        
        let folder = Folder {
            id: Uuid::new_v4(),
            name: "Test Folder".to_string(),
        };
        config.add_item(ConfigItem::Folder(folder));

        // Save to file
        config.save_to_file(&file_path).unwrap();

        // Load from file
        let loaded_config = Config::load_from_file(&file_path).unwrap();

        // Verify loaded configuration
        assert_eq!(loaded_config.items.len(), 2);
        
        // Find the plugin
        let plugin = loaded_config.items.iter().find(|item| {
            matches!(item, ConfigItem::Plugin(_))
        }).unwrap();
        assert_eq!(plugin.name(), "Test Plugin");

        // Find the folder
        let folder = loaded_config.items.iter().find(|item| {
            matches!(item, ConfigItem::Folder(_))
        }).unwrap();
        assert_eq!(folder.name(), "Test Folder");
    }
} 