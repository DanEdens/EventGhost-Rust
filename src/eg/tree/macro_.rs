use std::sync::{Arc, RwLock};
use crate::core::Error;
use super::item::{TreeItem, TreeItemInfo};
use crate::core::event::Event;

#[derive(Debug)]
pub struct Macro_ {
    info: TreeItemInfo,
    actions: Vec<Arc<RwLock<dyn TreeItem>>>,
    trigger_event: Option<Box<dyn Event + Send + Sync>>,
}

impl Macro_ {
    pub fn new(name: &str) -> Self {
        Self {
            info: TreeItemInfo {
                id: uuid::Uuid::new_v4(),
                name: name.to_string(),
                description: String::new(),
                enabled: true,
            },
            actions: Vec::new(),
            trigger_event: None,
        }
    }

    pub fn add_action(&mut self, action: Arc<RwLock<dyn TreeItem>>) {
        self.actions.push(action);
    }

    pub fn remove_action(&mut self, id: uuid::Uuid) -> Result<(), Error> {
        if let Some(index) = self.actions.iter().position(|a| {
            if let Ok(action) = a.read() {
                action.get_id() == id
            } else {
                false
            }
        }) {
            self.actions.remove(index);
            Ok(())
        } else {
            Err(Error::Tree(format!("Action with id {} not found", id)))
        }
    }

    pub fn get_actions(&self) -> &[Arc<RwLock<dyn TreeItem>>] {
        &self.actions
    }

    pub fn get_actions_mut(&mut self) -> &mut Vec<Arc<RwLock<dyn TreeItem>>> {
        &mut self.actions
    }

    pub fn set_trigger_event(&mut self, event: Option<Box<dyn Event + Send + Sync>>) {
        self.trigger_event = event;
    }

    pub fn get_trigger_event(&self) -> Option<&(dyn Event + Send + Sync)> {
        self.trigger_event.as_deref()
    }
}

impl TreeItem for Macro_ {
    fn get_id(&self) -> uuid::Uuid {
        self.info.id
    }

    fn get_name(&self) -> &str {
        &self.info.name
    }

    fn set_name(&mut self, name: &str) {
        self.info.name = name.to_string();
    }

    fn get_description(&self) -> &str {
        &self.info.description
    }

    fn set_description(&mut self, description: &str) {
        self.info.description = description.to_string();
    }

    fn is_enabled(&self) -> bool {
        self.info.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.info.enabled = enabled;
    }

    fn execute(&mut self, event: Option<&dyn Event>) -> Result<(), Error> {
        if !self.can_execute(event) {
            return Ok(());
        }

        for action in &self.actions {
            if let Ok(mut action) = action.write() {
                if action.is_enabled() && action.can_execute(event) {
                    action.execute(event)?;
                }
            }
        }
        Ok(())
    }

    fn can_execute(&self, event: Option<&dyn Event>) -> bool {
        if !self.is_enabled() {
            return false;
        }

        if let Some(trigger) = &self.trigger_event {
            if let Some(event) = event {
                // TODO: Implement event matching logic
                // print the unused vars
                println!("Trigger: {:?}", trigger);
                println!("Event: {:?}", event);
                true
            } else {


                false
            }
        } else {
            true
        }
    }

    fn clone_item(&self) -> Arc<RwLock<dyn TreeItem>> {
        Arc::new(RwLock::new(Macro_ {
            info: self.info.clone(),
            actions: self.actions.iter().map(|a| {
                if let Ok(action) = a.read() {
                    action.clone_item()
                } else {
                    panic!("Failed to read action")
                }
            }).collect(),
            trigger_event: self.trigger_event.as_ref().map(|e| e.clone_event()),
        }))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
} 