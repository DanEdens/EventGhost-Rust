use super::base::{ActionBase, ActionInfo};
use crate::core::Error;
use crate::core::event::Event;
use crate::eg::classes::plugin_config::ConfigDialog;
use uuid::Uuid;

pub struct ActionGroup {
    info: ActionInfo,
    actions: Vec<Box<dyn ActionBase>>,
}

impl ActionGroup {
    pub fn new(name: &str, description: &str, plugin_id: Uuid) -> Self {
        Self {
            info: ActionInfo {
                name: name.to_string(),
                description: description.to_string(),
                id: Uuid::new_v4(),
                plugin_id,
            },
            actions: Vec::new(),
        }
    }

    pub fn add_action(&mut self, action: Box<dyn ActionBase>) {
        self.actions.push(action);
    }

    pub fn remove_action(&mut self, id: Uuid) -> Option<Box<dyn ActionBase>> {
        if let Some(index) = self.actions.iter().position(|a| a.get_id() == id) {
            Some(self.actions.remove(index))
        } else {
            None
        }
    }

    pub fn get_actions(&self) -> &[Box<dyn ActionBase>] {
        &self.actions
    }

    pub fn get_actions_mut(&mut self) -> &mut [Box<dyn ActionBase>] {
        &mut self.actions
    }
}

impl ActionBase for ActionGroup {
    fn get_name(&self) -> &str {
        &self.info.name
    }

    fn get_description(&self) -> &str {
        &self.info.description
    }

    fn get_id(&self) -> Uuid {
        self.info.id
    }

    fn get_plugin_id(&self) -> Uuid {
        self.info.plugin_id
    }

    fn configure(&mut self) -> Option<ConfigDialog> {
        None // Groups don't have configuration
    }

    fn execute(&mut self, event: Option<&dyn Event>) -> Result<(), Error> {
        for action in &mut self.actions {
            action.execute(event)?;
        }
        Ok(())
    }

    fn can_execute(&self, event: Option<&dyn Event>) -> bool {
        self.actions.iter().any(|action| action.can_execute(event))
    }

    fn clone_action(&self) -> Box<dyn ActionBase> {
        let mut group = ActionGroup::new(
            &self.info.name,
            &self.info.description,
            self.info.plugin_id,
        );
        group.info.id = self.info.id;
        
        for action in &self.actions {
            group.add_action(action.clone_action());
        }
        
        Box::new(group)
    }
} 