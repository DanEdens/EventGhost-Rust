use super::base::{ActionBase, ActionInfo, ActionError};
use super::item::ActionItem;
use crate::core::Error;
use crate::core::event::Event;
use crate::eg::classes::plugin_config::ConfigDialog;
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
    F: Fn(Option<&dyn Event>) -> bool + Send + Sync + 'static,
{
    ActionItem::new(
        name,
        description,
        plugin_id,
        move |event| {
            if condition(event) {
                action.clone_action().execute(event)
            } else {
                Ok(())
            }
        },
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