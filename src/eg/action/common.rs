use super::base::{ActionBase, ActionInfo, ActionError};
use super::item::ActionItem;
use crate::core::Error;
use crate::core::event::Event;
use crate::eg::classes::ConfigDialog;
use uuid::Uuid;
use std::process::Command;
use std::thread;
use std::time::Duration;

/// Creates an action that executes a shell command
pub fn shell_command_action(
    name: &str,
    description: &str,
    plugin_id: Uuid,
    command: String,
    args: Vec<String>,
) -> ActionItem {
    ActionItem::new(
        name,
        description,
        plugin_id,
        move |_| {
            Command::new(&command)
                .args(&args)
                .output()
                .map_err(|e| ActionError::ExecutionFailed(e.to_string()))?;
            Ok(())
        },
    )
}

/// Creates an action that delays execution for a specified duration
pub fn delay_action(
    name: &str,
    description: &str,
    plugin_id: Uuid,
    duration_ms: u64,
) -> ActionItem {
    ActionItem::new(
        name,
        description,
        plugin_id,
        move |_| {
            thread::sleep(Duration::from_millis(duration_ms));
            Ok(())
        },
    )
}

/// Creates an action that generates a new event
pub fn generate_event_action<F>(
    name: &str,
    description: &str,
    plugin_id: Uuid,
    event_generator: F,
) -> ActionItem 
where
    F: Fn() -> Box<dyn Event> + Send + Sync + 'static,
{
    ActionItem::new(
        name,
        description,
        plugin_id,
        move |_| {
            let event = event_generator();
            // TODO: Send event through event system
            Ok(())
        },
    )
}

/// Creates an action that executes only if a condition is met
pub fn conditional_action<F>(
    name: &str,
    description: &str,
    plugin_id: Uuid,
    condition: F,
    action: Box<dyn ActionBase>,
) -> ActionItem 
where
    F: Fn(Option<&dyn Event>) -> bool + Send + Sync + Clone + 'static,
{
    let conditional = ConditionalAction::new(action, condition.clone());
    ActionItem::new(
        name,
        description,
        plugin_id,
        move |event| conditional.clone_action().execute(event)
    ).with_can_execute(condition)
}

/// Creates an action that repeats another action a specified number of times
pub fn repeat_action(
    name: &str,
    description: &str,
    plugin_id: Uuid,
    action: Box<dyn ActionBase>,
    count: u32,
) -> ActionItem {
    ActionItem::new(
        name,
        description,
        plugin_id,
        move |event| {
            for _ in 0..count {
                action.clone_action().execute(event)?;
            }
            Ok(())
        },
    )
}

pub struct ConditionalAction<F> {
    action: Box<dyn ActionBase>,
    condition: F,
}

impl<F> ConditionalAction<F>
where
    F: Fn(Option<&dyn Event>) -> bool + Send + Sync + 'static,
{
    pub fn new(mut action: Box<dyn ActionBase>, condition: F) -> Self {
        Self { action, condition }
    }

    pub fn get_action(&self) -> &Box<dyn ActionBase> {
        &self.action
    }

    pub fn get_condition(&self) -> &F {
        &self.condition
    }
}

impl<F> ActionBase for ConditionalAction<F>
where
    F: Fn(Option<&dyn Event>) -> bool + Send + Sync + Clone + 'static,
{
    fn get_name(&self) -> &str {
        self.action.get_name()
    }

    fn get_description(&self) -> &str {
        self.action.get_description()
    }

    fn get_id(&self) -> Uuid {
        self.action.get_id()
    }

    fn get_plugin_id(&self) -> Uuid {
        self.action.get_plugin_id()
    }

    fn configure(&mut self) -> Option<ConfigDialog> {
        self.action.configure()
    }

    fn can_execute(&self, event: Option<&dyn Event>) -> bool {
        (self.condition)(event) && self.action.can_execute(event)
    }

    fn clone_action(&self) -> Box<dyn ActionBase> {
        Box::new(ConditionalAction {
            action: self.action.clone_action(),
            condition: self.condition.clone(),
        })
    }

    fn execute(&mut self, event: Option<&dyn Event>) -> Result<(), Error> {
        if (self.condition)(event) {
            self.action.execute(event)
        } else {
            Ok(())
        }
    }
} 