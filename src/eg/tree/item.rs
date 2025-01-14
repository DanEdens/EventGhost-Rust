use uuid::Uuid;
use crate::core::Error;
use crate::core::event::Event;
use std::any::Any;

pub trait TreeItem: Any + Send + Sync {
    fn get_id(&self) -> Uuid;
    fn get_name(&self) -> &str;
    fn set_name(&mut self, name: &str);
    fn get_description(&self) -> &str;
    fn set_description(&mut self, description: &str);
    fn is_enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);
    
    fn execute(&mut self, event: Option<&dyn Event>) -> Result<(), Error>;
    fn can_execute(&self, event: Option<&dyn Event>) -> bool;
    
    fn clone_item(&self) -> Box<dyn TreeItem>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug, Clone)]
pub struct TreeItemInfo {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub enabled: bool,
} 