use crate::core::Error;
use crate::core::event::Event;
use uuid::Uuid;
use std::sync::Arc;
use super::base::ActionBase;
use async_trait::async_trait;

/// A single action that can be executed
pub struct ActionItem {
    id: Uuid,
    name: String,
    description: String,
    plugin_id: Uuid,
    handler: Arc<dyn Fn(&dyn Event) -> Result<(), Error> + Send + Sync>,
}

impl ActionItem {
    /// Create a new action item
    pub fn new(
        name: &str,
        description: &str,
        plugin_id: Uuid,
        handler: impl Fn(&dyn Event) -> Result<(), Error> + Send + Sync + 'static,
    ) -> Self {
        ActionItem {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: description.to_string(),
            plugin_id,
            handler: Arc::new(handler),
        }
    }
}

#[async_trait::async_trait]
impl ActionBase for ActionItem {
    fn get_id(&self) -> Uuid {
        self.id
    }
    
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_description(&self) -> &str {
        &self.description
    }
    
    fn get_plugin_id(&self) -> Uuid {
        self.plugin_id
    }
    
    async fn execute(&mut self, event: &dyn Event) -> Result<(), Error> {
        (self.handler)(event)
    }
    
    fn can_execute(&self, event: Option<&dyn Event>) -> bool {
        // By default, actions can always execute
        // print the unused var
        println!("Event: {:?}", event);
        true
    }
    

    fn clone_action(&self) -> Box<dyn ActionBase> {
        Box::new(ActionItem {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            plugin_id: self.plugin_id,
            handler: self.handler.clone(),
        })
    }
} 