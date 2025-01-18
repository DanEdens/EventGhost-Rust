use super::base::{ActionBase, ActionInfo};
use crate::core::Error;
use crate::core::event::Event;
use crate::eg::classes::ConfigDialog;
use uuid::Uuid;
use std::sync::Arc;

pub struct ActionItem {
    info: ActionInfo,
    handler: Arc<dyn Fn(Option<&dyn Event>) -> Result<(), Error> + Send + Sync>,
    can_execute: Arc<dyn Fn(Option<&dyn Event>) -> bool + Send + Sync>,
    config_dialog: Option<Arc<dyn Fn() -> ConfigDialog + Send + Sync>>,
}

impl ActionItem {
    pub fn new(
        name: &str,
        description: &str,
        plugin_id: Uuid,
        handler: impl Fn(Option<&dyn Event>) -> Result<(), Error> + Send + Sync + 'static,
    ) -> Self {
        Self {
            info: ActionInfo {
                name: name.to_string(),
                description: description.to_string(),
                id: Uuid::new_v4(),
                plugin_id,
            },
            handler: Arc::new(handler),
            can_execute: Arc::new(|_| true),
            config_dialog: None,
        }
    }

    pub fn with_can_execute(
        mut self,
        can_execute: impl Fn(Option<&dyn Event>) -> bool + Send + Sync + 'static,
    ) -> Self {
        self.can_execute = Arc::new(can_execute);
        self
    }

    pub fn with_config(
        mut self,
        config_dialog: impl Fn() -> ConfigDialog + Send + Sync + 'static,
    ) -> Self {
        self.config_dialog = Some(Arc::new(config_dialog));
        self
    }
}

impl ActionBase for ActionItem {
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
        self.config_dialog.as_ref().map(|f| f())
    }

    fn execute(&mut self, event: Option<&dyn Event>) -> Result<(), Error> {
        (self.handler)(event)
    }

    fn can_execute(&self, event: Option<&dyn Event>) -> bool {
        (self.can_execute)(event)
    }

    fn clone_action(&self) -> Box<dyn ActionBase> {
        Box::new(Self {
            info: self.info.clone(),
            handler: self.handler.clone(),
            can_execute: self.can_execute.clone(),
            config_dialog: self.config_dialog.clone(),
        })
    }
} 