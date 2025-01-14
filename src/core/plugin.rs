use uuid::Uuid;
use crate::core::Error;
use crate::core::event::Event;
use crate::eg::classes::plugin_config::ConfigDialog;
use crate::eg::action::base::ActionBase;

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub guid: Uuid,
}

pub trait PropertySource {
    fn get_properties(&self) -> Vec<Property>;
    fn set_property(&mut self, name: &str, value: PropertyValue) -> Result<(), Error>;
    fn validate_property(&self, name: &str, value: &PropertyValue) -> Result<(), String>;
}

pub trait Plugin: PropertySource + Send + Sync {
    fn get_info(&self) -> PluginInfo;
    fn initialize(&mut self) -> Result<(), Error>;
    fn start(&mut self) -> Result<(), Error>;
    fn stop(&mut self) -> Result<(), Error>;
    fn configure(&mut self) -> Option<ConfigDialog>;
    fn handle_event(&mut self, event: &Event) -> Result<(), Error>;
    fn add_action(&mut self, action: Box<dyn ActionBase>);
    fn get_actions(&self) -> &[Box<dyn ActionBase>];
}

pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) -> Result<(), Error> {
        // TODO: Implement plugin registration
        Ok(())
    }

    pub fn unregister(&mut self, guid: Uuid) -> Result<(), Error> {
        // TODO: Implement plugin unregistration
        Ok(())
    }

    pub fn get_plugin(&self, guid: Uuid) -> Option<&dyn Plugin> {
        // TODO: Implement plugin lookup
        None
    }

    pub fn get_plugin_mut(&mut self, guid: Uuid) -> Option<&mut dyn Plugin> {
        // TODO: Implement mutable plugin lookup
        None
    }
} 