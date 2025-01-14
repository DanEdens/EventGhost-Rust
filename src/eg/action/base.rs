use crate::core::Error;
use crate::core::event::Event;
use crate::eg::classes::plugin_config::ConfigDialog;
use uuid::Uuid;

pub trait ActionBase: Send + Sync {
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
    fn get_id(&self) -> Uuid;
    fn get_plugin_id(&self) -> Uuid;
    
    fn configure(&mut self) -> Option<ConfigDialog>;
    fn execute(&mut self, event: Option<&dyn Event>) -> Result<(), Error>;
    fn can_execute(&self, event: Option<&dyn Event>) -> bool;
    
    fn clone_action(&self) -> Box<dyn ActionBase>;
}

#[derive(Debug, Clone)]
pub struct ActionInfo {
    pub name: String,
    pub description: String,
    pub id: Uuid,
    pub plugin_id: Uuid,
} 