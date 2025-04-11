use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use log::{debug, error, info};
use quick_xml::de::from_str as xml_from_str;
use quick_xml::se::to_string as xml_to_string;
use quick_xml::events::Event as XmlEvent;
use quick_xml::Reader as XmlReader;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

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

/// Represents the EventGhost configuration including metadata and items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// The list of configuration items
    #[serde(default)]
    pub items: Vec<ConfigItem>,
    /// Version information
    #[serde(default)]
    pub version: String,
    /// GUID for the configuration
    #[serde(default)]
    pub guid: String,
    /// Timestamp
    #[serde(default)]
    pub time: String,
}

impl Config {
    /// Creates a new empty configuration.
    pub fn new() -> Self {
        Config { 
            items: Vec::new(),
            version: "1.0".to_string(),
            guid: Uuid::new_v4().to_string(),
            time: chrono::Local::now().timestamp().to_string(),
        }
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
                Self::parse_egtree_xml(&content).unwrap_or_else(|_| {
                    // Fallback to standard XML parsing
                    match xml_from_str(&content) {
                        Ok(config) => config,
                        Err(e) => {
                            error!("Error parsing XML: {}", e);
                            Config::new() // Return a new empty config instead of returning an error
                        }
                    }
                })
            },
            _ => {
                debug!("Loading configuration from JSON file {}", path.display());
                serde_json::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            }
        };
        
        info!("Configuration loaded from {}", path.display());
        Ok(config)
    }

    /// Parses an EventGhost XML config (.egtree) file
    fn parse_egtree_xml(content: &str) -> Result<Self, io::Error> {
        debug!("Using custom parser for EventGhost XML format");
        
        let mut reader = XmlReader::from_str(content);
        reader.trim_text(true);
        
        let mut config = Config::new();
        let mut buf = Vec::new();
        
        // First pass: extract EventGhost root attributes
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(XmlEvent::Start(ref e)) if e.name().as_ref() == b"EventGhost" => {
                    // Parse root attributes
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or_default();
                            let value = attr.unescape_value().unwrap_or_default().to_string();
                            
                            match key {
                                "Version" => config.version = value,
                                "Guid" => config.guid = value,
                                "Time" => config.time = value,
                                _ => {}
                            }
                        }
                    }
                    break;
                },
                Ok(XmlEvent::Eof) => break,
                Err(e) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Error parsing XML: {}", e)
                    ));
                },
                _ => (), // Skip other elements
            }
            buf.clear();
        }
        
        // Reset reader for second pass
        reader = XmlReader::from_str(content);
        reader.trim_text(true);
        buf.clear();
        
        // Track current parent elements to build hierarchy
        let mut current_path = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(XmlEvent::Start(ref e)) => {
                    let name_obj = e.name();
                    let name_ref = name_obj.as_ref();
                    let name = std::str::from_utf8(name_ref).unwrap_or_default();
                    
                    // Skip the root EventGhost element, we already processed it
                    if name == "EventGhost" {
                        continue;
                    }
                    
                    // Parse the element based on its tag name
                    match name {
                        "Autostart" | "Folder" => {
                            let mut folder = Folder {
                                id: Uuid::new_v4(),
                                name: "Unknown".to_string(),
                            };
                            
                            // Extract attributes
                            for attr in e.attributes() {
                                if let Ok(attr) = attr {
                                    let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or_default();
                                    let value = attr.unescape_value().unwrap_or_default().to_string();
                                    
                                    if key == "Name" {
                                        folder.name = value;
                                    }
                                }
                            }
                            
                            current_path.push(("folder", folder.id));
                            config.add_item(ConfigItem::Folder(folder));
                        },
                        "Macro" => {
                            let mut macro_ = Macro {
                                id: Uuid::new_v4(),
                                name: "Unknown".to_string(),
                                events: Vec::new(),
                                actions: Vec::new(),
                            };
                            
                            // Extract attributes
                            for attr in e.attributes() {
                                if let Ok(attr) = attr {
                                    let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or_default();
                                    let value = attr.unescape_value().unwrap_or_default().to_string();
                                    
                                    if key == "Name" {
                                        macro_.name = value;
                                    }
                                }
                            }
                            
                            current_path.push(("macro", macro_.id));
                            config.add_item(ConfigItem::Macro(macro_));
                        },
                        "Plugin" => {
                            let mut plugin = Plugin {
                                id: Uuid::new_v4(),
                                name: "Unknown".to_string(),
                                config: HashMap::new(),
                            };
                            
                            // Extract attributes
                            for attr in e.attributes() {
                                if let Ok(attr) = attr {
                                    let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or_default();
                                    let value = attr.unescape_value().unwrap_or_default().to_string();
                                    
                                    match key {
                                        "Identifier" => plugin.name = value,
                                        "File" => { plugin.config.insert("File".to_string(), value); },
                                        _ => { plugin.config.insert(key.to_string(), value); }
                                    }
                                }
                            }
                            
                            current_path.push(("plugin", plugin.id));
                            config.add_item(ConfigItem::Plugin(plugin));
                        },
                        "Event" => {
                            let mut event = Event {
                                id: Uuid::new_v4(),
                                name: "Unknown".to_string(),
                                parameters: HashMap::new(),
                            };
                            
                            // Extract attributes
                            for attr in e.attributes() {
                                if let Ok(attr) = attr {
                                    let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or_default();
                                    let value = attr.unescape_value().unwrap_or_default().to_string();
                                    
                                    if key == "Name" {
                                        event.name = value;
                                    } else {
                                        event.parameters.insert(key.to_string(), value);
                                    }
                                }
                            }
                            
                            // Link to parent macro if any
                            if let Some((parent_type, parent_id)) = current_path.last() {
                                if *parent_type == "macro" {
                                    if let Some(ConfigItem::Macro(macro_)) = config.find_item_mut(*parent_id) {
                                        macro_.events.push(event.id);
                                    }
                                }
                            }
                            
                            config.add_item(ConfigItem::Event(event));
                        },
                        "Action" => {
                            let mut action = Action {
                                id: Uuid::new_v4(),
                                name: "Action".to_string(),
                                parameters: HashMap::new(),
                            };
                            
                            // Extract attributes
                            for attr in e.attributes() {
                                if let Ok(attr) = attr {
                                    let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or_default();
                                    let value = attr.unescape_value().unwrap_or_default().to_string();
                                    
                                    if key == "Name" {
                                        action.name = value;
                                    } else {
                                        action.parameters.insert(key.to_string(), value);
                                    }
                                }
                            }
                            
                            // Link to parent macro if any
                            if let Some((parent_type, parent_id)) = current_path.last() {
                                if *parent_type == "macro" {
                                    if let Some(ConfigItem::Macro(macro_)) = config.find_item_mut(*parent_id) {
                                        macro_.actions.push(action.id);
                                    }
                                }
                            }
                            
                            config.add_item(ConfigItem::Action(action));
                        },
                        _ => {}
                    }
                },
                Ok(XmlEvent::End(ref e)) => {
                    let name_obj = e.name();
                    let name_ref = name_obj.as_ref();
                    let name = std::str::from_utf8(name_ref).unwrap_or_default();
                    
                    match name {
                        "Autostart" | "Folder" | "Macro" | "Plugin" => {
                            if !current_path.is_empty() {
                                current_path.pop();
                            }
                        },
                        _ => {}
                    }
                },
                Ok(XmlEvent::Text(ref e)) => {
                    // Handle text content for Plugin elements (base64 encoded data)
                    if let Some((parent_type, parent_id)) = current_path.last() {
                        if *parent_type == "plugin" {
                            if let Some(ConfigItem::Plugin(plugin)) = config.find_item_mut(*parent_id) {
                                let text = e.unescape().unwrap_or_default().to_string();
                                if !text.trim().is_empty() {
                                    // Store encoded data for later processing
                                    plugin.config.insert("EncodedData".to_string(), text.trim().to_string());
                                }
                            }
                        } else if *parent_type == "action" {
                            if let Some(ConfigItem::Action(action)) = config.find_item_mut(*parent_id) {
                                let text = e.unescape().unwrap_or_default().to_string();
                                if !text.trim().is_empty() {
                                    // Store Python script or action code
                                    action.parameters.insert("Script".to_string(), text.trim().to_string());
                                }
                            }
                        }
                    }
                },
                Ok(XmlEvent::Eof) => break,
                Err(e) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Error parsing XML: {}", e)
                    ));
                },
                _ => (), // Skip other elements
            }
            buf.clear();
        }
        
        Ok(config)
    }
    
    /// Attempts to decode base64 encoded plugin data
    pub fn decode_plugin_data(&mut self) -> io::Result<()> {
        for item in &mut self.items {
            if let ConfigItem::Plugin(plugin) = item {
                if let Some(encoded_data) = plugin.config.get("EncodedData") {
                    match BASE64.decode(encoded_data) {
                        Ok(decoded) => {
                            // Store decoded data for debugging
                            if let Ok(decoded_str) = String::from_utf8(decoded.clone()) {
                                plugin.config.insert("DecodedData".to_string(), decoded_str);
                            } else {
                                plugin.config.insert("DecodedDataHex".to_string(), 
                                    decoded.iter().map(|b| format!("{:02x}", b)).collect());
                            }
                        },
                        Err(e) => {
                            error!("Failed to decode base64 data for plugin {}: {}", plugin.name, e);
                        }
                    }
                }
            }
        }
        Ok(())
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
