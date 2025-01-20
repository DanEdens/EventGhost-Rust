use super::base::{ActionBase, ActionError};
use super::item::ActionItem;
use crate::core::Error;
use crate::core::event::Event;
use crate::eg::classes::dialog::ConfigDialog;
use uuid::Uuid;
use std::process::Command;
use std::thread;
use std::time::Duration;
use async_trait::async_trait;

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
    let conditional = ConditionalAction::new(name, description, plugin_id, condition, action);
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

/// A conditional action that only executes if a condition is met
pub struct ConditionalAction<F>
where
    F: Fn(&dyn Event) -> bool + Send + Sync,
{
    id: Uuid,
    name: String,
    description: String,
    plugin_id: Uuid,
    condition: F,
    action: Box<dyn ActionBase>,
}

impl<F> ConditionalAction<F>
where
    F: Fn(&dyn Event) -> bool + Send + Sync,
{
    /// Create a new conditional action
    pub fn new(
        name: &str,
        description: &str,
        plugin_id: Uuid,
        condition: F,
        action: Box<dyn ActionBase>,
    ) -> Self {
        ConditionalAction {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: description.to_string(),
            plugin_id,
            condition,
            action,
        }
    }
}

#[async_trait::async_trait]
impl ActionBase for ConditionalAction<F>
where
    F: Fn(&dyn Event) -> bool + Send + Sync,
{
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
        if (self.condition)(event) {
            self.action.execute(event).await
        } else {
            Ok(())
        }
    }
    
    fn configure(&mut self) -> Option<ConfigDialog> {
        self.action.configure()
    }
} 