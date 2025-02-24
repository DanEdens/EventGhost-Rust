use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::core::Error;
use crate::core::event::{Event, EventType};
use crate::core::plugin::Plugin;

/// Represents the result of an action execution
#[derive(Debug, Clone)]
pub struct ActionResult {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<Box<dyn Any + Send + Sync>>,
}

impl ActionResult {
    pub fn success() -> Self {
        Self {
            success: true,
            message: None,
            data: None,
        }
    }

    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: Some(message.into()),
            data: None,
        }
    }

    pub fn with_data(mut self, data: impl Any + Send + Sync) -> Self {
        self.data = Some(Box::new(data));
        self
    }
}

/// Configuration for an action
#[derive(Debug, Clone)]
pub struct ActionConfig {
    pub args: Vec<String>,
    pub enabled: bool,
    pub should_select_on_execute: bool,
}

impl Default for ActionConfig {
    fn default() -> Self {
        Self {
            args: Vec::new(),
            enabled: true,
            should_select_on_execute: false,
        }
    }
}

/// Trait that defines the interface for all actions
#[async_trait]
pub trait Action: Send + Sync + Debug {
    /// Get the unique identifier for this action
    fn get_id(&self) -> Uuid;

    /// Get a human-readable name for this action
    fn get_name(&self) -> &str;

    /// Get a description of what this action does
    fn get_description(&self) -> &str;

    /// Get the event types this action can respond to
    fn get_supported_event_types(&self) -> Vec<EventType>;

    /// Get the plugin that owns this action
    fn get_plugin(&self) -> Arc<dyn Plugin>;

    /// Get the icon file path for this action (if any)
    fn get_icon_path(&self) -> Option<String> {
        None
    }

    /// Get the help URL for this action (if any)
    fn get_help_url(&self) -> Option<String> {
        None
    }

    /// Get whether this action is configurable
    fn is_configurable(&self) -> bool {
        true
    }

    /// Get whether this action is executable
    fn is_executable(&self) -> bool {
        true
    }

    /// Configure the action with the given arguments
    async fn configure(&mut self, config: ActionConfig) -> Result<(), Error> {
        Ok(())
    }

    /// Compile/prepare the action for execution (if needed)
    async fn compile(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /// Execute the action in response to an event
    async fn execute(&mut self, event: &dyn Event) -> Result<ActionResult, Error>;

    /// Validate that the action can be executed with the current configuration
    fn validate(&self) -> Result<(), Error>;
}

/// Represents a group of related actions
#[derive(Debug)]
pub struct ActionGroup {
    pub name: String,
    pub description: Option<String>,
    pub icon_path: Option<String>,
    pub plugin: Arc<dyn Plugin>,
    pub actions: Vec<Box<dyn Action>>,
    pub subgroups: Vec<ActionGroup>,
    pub expanded: bool,
}

impl ActionGroup {
    pub fn new(
        name: impl Into<String>,
        plugin: Arc<dyn Plugin>,
        description: Option<String>,
        icon_path: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description,
            icon_path,
            plugin,
            actions: Vec::new(),
            subgroups: Vec::new(),
            expanded: false,
        }
    }

    pub fn add_action(&mut self, action: Box<dyn Action>) -> Result<(), Error> {
        action.validate()?;
        self.actions.push(action);
        Ok(())
    }

    pub fn add_subgroup(&mut self, group: ActionGroup) {
        self.subgroups.push(group);
    }

    pub fn find_action(&self, id: Uuid) -> Option<&Box<dyn Action>> {
        // First search in this group's actions
        if let Some(action) = self.actions.iter().find(|a| a.get_id() == id) {
            return Some(action);
        }

        // Then recursively search subgroups
        for subgroup in &self.subgroups {
            if let Some(action) = subgroup.find_action(id) {
                return Some(action);
            }
        }

        None
    }

    pub fn find_action_mut(&mut self, id: Uuid) -> Option<&mut Box<dyn Action>> {
        // First search in this group's actions
        if let Some(action) = self.actions.iter_mut().find(|a| a.get_id() == id) {
            return Some(action);
        }

        // Then recursively search subgroups
        for subgroup in &mut self.subgroups {
            if let Some(action) = subgroup.find_action_mut(id) {
                return Some(action);
            }
        }

        None
    }
}

/// Action manager that handles registration and execution of actions
#[derive(Default)]
pub struct ActionManager {
    root_group: ActionGroup,
}

impl ActionManager {
    pub fn new() -> Self {
        Self {
            root_group: ActionGroup::new(
                "Root",
                Arc::new(DummyPlugin {}),
                None,
                None,
            ),
        }
    }

    pub fn register_group(&mut self, group: ActionGroup) {
        self.root_group.add_subgroup(group);
    }

    pub async fn execute_action(&mut self, action_id: Uuid, event: &dyn Event) -> Result<ActionResult, Error> {
        let action = self.root_group
            .find_action_mut(action_id)
            .ok_or_else(|| Error::NotFound(format!("Action with ID {} not found", action_id)))?;

        // Check if the action supports this event type
        if !action.get_supported_event_types().contains(&event.get_type()) {
            return Err(Error::InvalidOperation(format!(
                "Action {} does not support event type {:?}",
                action.get_name(),
                event.get_type()
            )));
        }

        // Compile the action if needed
        action.compile().await?;

        // Execute the action
        action.execute(event).await
    }

    pub fn get_actions_for_event_type(&self, event_type: EventType) -> Vec<&Box<dyn Action>> {
        let mut actions = Vec::new();
        self.collect_actions_for_event_type(&self.root_group, event_type, &mut actions);
        actions
    }

    fn collect_actions_for_event_type<'a>(
        &'a self,
        group: &'a ActionGroup,
        event_type: EventType,
        actions: &mut Vec<&'a Box<dyn Action>>,
    ) {
        // Add matching actions from this group
        actions.extend(
            group.actions
                .iter()
                .filter(|action| action.get_supported_event_types().contains(&event_type))
        );

        // Recursively collect from subgroups
        for subgroup in &group.subgroups {
            self.collect_actions_for_event_type(subgroup, event_type, actions);
        }
    }
}

// Dummy plugin implementation for the root group
#[derive(Debug)]
struct DummyPlugin;

#[async_trait]
impl Plugin for DummyPlugin {
    fn get_name(&self) -> &str {
        "Root"
    }

    fn get_description(&self) -> &str {
        "Root action group"
    }

    fn get_version(&self) -> &str {
        "1.0.0"
    }

    fn get_author(&self) -> &str {
        "System"
    }

    async fn initialize(&mut self) -> Result<(), Error> {
        Ok(())
    }

    async fn terminate(&mut self) -> Result<(), Error> {
        Ok(())
    }
} 