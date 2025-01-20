use crate::core::Error;
use crate::core::event::Event;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use super::base::ActionBase;
use async_trait::async_trait;

/// A group of actions that can be executed together
pub struct ActionGroup {
    id: Uuid,
    name: String,
    description: String,
    plugin_id: Uuid,
    actions: Vec<Arc<Mutex<Box<dyn ActionBase>>>>,
}

impl ActionGroup {
    /// Create a new action group
    pub fn new(name: &str, description: &str, plugin_id: Uuid) -> Self {
        ActionGroup {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: description.to_string(),
            plugin_id,
            actions: Vec::new(),
        }
    }
    
    /// Add an action to the group
    pub fn add_action(&mut self, action: Box<dyn ActionBase>) {
        let action = Arc::new(Mutex::new(action));
        self.actions.push(action);
    }
    
    /// Remove an action from the group
    pub fn remove_action(&mut self, id: Uuid) {
        self.actions.retain(|a| {
            let action = a.lock().unwrap();
            action.get_id() != id
        });
    }
    
    /// Get all actions in the group
    pub fn get_actions(&self) -> &[Arc<Mutex<Box<dyn ActionBase>>>] {
        &self.actions
    }
}

#[async_trait::async_trait]
impl ActionBase for ActionGroup {
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
        for action in &self.actions {
            let mut action = action.lock().unwrap();
            action.execute(event).await?;
        }
        Ok(())
    }
    
    fn can_execute(&self, event: Option<&dyn Event>) -> bool {
        // Groups can always execute
        // print the unused var
        println!("Event: {:?}", event);
        true
    }
    

    fn clone_action(&self) -> Box<dyn ActionBase> {
        Box::new(ActionGroup {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            plugin_id: self.plugin_id,
            actions: self.actions.clone(),
        })
    }
} 