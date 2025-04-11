use super::base::{ActionBase, ActionError};
use super::item::ActionItem;
use crate::core::Error;
use crate::core::event::Event;
// use crate::eg::classes::dialog::ConfigDialog;
use uuid::Uuid;
use std::process::Command;
use std::thread;
use std::time::Duration;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::runtime::Runtime;

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
            // print the unused var
            println!("Event: {:?}", event);
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
    let conditional = ConditionalAction::new(name.to_string(), description.to_string(), plugin_id, condition, action);
    ActionItem::new(
        name,
        description,
        plugin_id,
        move |event| {
            let mut action = conditional.clone_action();
            if action.can_execute(Some(event)) {
                let rt = Runtime::new().map_err(|e| Error::Other(e.to_string()))?;
                rt.block_on(action.execute(event))
            } else {
                Ok(())
            }
        }
    )
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
            let mut action = action.clone_action();
            let rt = Runtime::new().map_err(|e| Error::Other(e.to_string()))?;
            for _ in 0..count {
                rt.block_on(action.execute(event))?;
            }
            Ok(())
        }
    )
}

/// A conditional action that only executes if a condition is met
pub struct ConditionalAction<F>
where
    F: Fn(Option<&dyn Event>) -> bool + Send + Sync + 'static,
{
    id: Uuid,
    name: String,
    description: String,
    plugin_id: Uuid,
    condition: Arc<F>,
    action: Box<dyn ActionBase>,
}

impl<F> ConditionalAction<F>
where
    F: Fn(Option<&dyn Event>) -> bool + Send + Sync + 'static,
{
    /// Create a new conditional action
    pub fn new(
        name: String,
        description: String,
        plugin_id: Uuid,
        condition: F,
        action: Box<dyn ActionBase>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            plugin_id,
            condition: Arc::new(condition),
            action,
        }
    }
}

#[async_trait]
impl<F> ActionBase for ConditionalAction<F>
where
    F: Fn(Option<&dyn Event>) -> bool + Send + Sync + 'static,
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
    
    fn can_execute(&self, event: Option<&dyn Event>) -> bool {
        (self.condition)(event)
    }
    
    fn clone_action(&self) -> Box<dyn ActionBase> {
        Box::new(ConditionalAction {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            plugin_id: self.plugin_id,
            condition: self.condition.clone(),
            action: self.action.clone_action(),
        })
    }
    
    async fn execute(&mut self, event: &dyn Event) -> Result<(), Error> {
        if self.can_execute(Some(event)) {
            self.action.execute(event).await
        } else {
            Ok(())
        }
    }
    
    fn configure(&mut self) -> Result<bool, Error> {
        self.action.configure()
    }
} 
