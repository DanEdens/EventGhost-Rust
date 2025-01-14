use std::sync::{Arc, RwLock};
use crate::core::Error;
use super::item::TreeItem;

pub trait TreeLink: TreeItem {
    fn get_target(&self) -> Option<Arc<RwLock<dyn TreeItem>>>;
    fn set_target(&mut self, target: Option<Arc<RwLock<dyn TreeItem>>>) -> Result<(), Error>;
    fn get_target_id(&self) -> Option<uuid::Uuid>;
    fn resolve_target(&mut self, items: &[Arc<RwLock<dyn TreeItem>>]) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct Link {
    info: super::item::TreeItemInfo,
    target: Option<Arc<RwLock<dyn TreeItem>>>,
    target_id: Option<uuid::Uuid>,
}

impl Link {
    pub fn new(name: &str) -> Self {
        Self {
            info: super::item::TreeItemInfo {
                id: uuid::Uuid::new_v4(),
                name: name.to_string(),
                description: String::new(),
                enabled: true,
            },
            target: None,
            target_id: None,
        }
    }
} 