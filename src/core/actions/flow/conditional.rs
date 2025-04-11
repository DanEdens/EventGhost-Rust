use std::any::Any;
use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::core::Error;
use crate::core::action::{Action, ActionResult, ActionConfig};
use crate::core::event::{Event, EventType, EventPayload};
use crate::core::plugin::{Plugin, PluginInfo, PluginCapability, PluginState, PluginError};
use crate::core::config::Config;

/// Action that provides conditional execution based on event properties or global variables
#[derive(Debug)]
pub struct ConditionalAction {
    id: Uuid,
    plugin: Arc<dyn Plugin>,
    condition_type: ConditionType,
    condition_value: String,
    comparison: Comparison,
    reference_value: String,
    true_actions: Vec<Box<dyn Action>>,
    false_actions: Vec<Box<dyn Action>>,
}

/// Types of condition to evaluate
#[derive(Debug, Clone, PartialEq)]
pub enum ConditionType {
    /// Check event payload
    EventPayload,
    /// Check event type
    EventType,
    /// Check event source
    EventSource,
    /// Check against a variable
    Variable,
    /// Check a constant value
    Constant,
}

impl Default for ConditionType {
    fn default() -> Self {
        Self::EventPayload
    }
}

impl From<&str> for ConditionType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "event" | "eventpayload" | "payload" => Self::EventPayload,
            "type" | "eventtype" => Self::EventType,
            "source" | "eventsource" => Self::EventSource,
            "variable" | "var" => Self::Variable,
            "constant" | "const" => Self::Constant,
            _ => Self::EventPayload,
        }
    }
}

/// Comparison operations for the condition
#[derive(Debug, Clone, PartialEq)]
pub enum Comparison {
    Equal,
    NotEqual,
    Contains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
}

impl Default for Comparison {
    fn default() -> Self {
        Self::Equal
    }
}

impl From<&str> for Comparison {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "==" | "equals" | "equal" | "eq" => Self::Equal,
            "!=" | "notequal" | "ne" => Self::NotEqual,
            "contains" | "has" => Self::Contains,
            "startswith" | "starts" => Self::StartsWith,
            "endswith" | "ends" => Self::EndsWith,
            ">" | "greaterthan" | "gt" => Self::GreaterThan,
            "<" | "lessthan" | "lt" => Self::LessThan,
            ">=" | "greaterorequal" | "ge" => Self::GreaterOrEqual,
            "<=" | "lessorequal" | "le" => Self::LessOrEqual,
            _ => Self::Equal,
        }
    }
}

impl ConditionalAction {
    pub fn new(plugin: Arc<dyn Plugin>) -> Self {
        Self {
            id: Uuid::new_v4(),
            plugin,
            condition_type: ConditionType::default(),
            condition_value: String::new(),
            comparison: Comparison::default(),
            reference_value: String::new(),
            true_actions: Vec::new(),
            false_actions: Vec::new(),
        }
    }
    
    /// Add an action to execute when the condition is true
    pub fn add_true_action(&mut self, action: Box<dyn Action>) {
        self.true_actions.push(action);
    }
    
    /// Add an action to execute when the condition is false
    pub fn add_false_action(&mut self, action: Box<dyn Action>) {
        self.false_actions.push(action);
    }
    
    /// Evaluate a condition against the given event
    fn evaluate_condition(&self, event: &dyn Event) -> bool {
        // Get the left-hand value based on condition type
        let lhs = match self.condition_type {
            ConditionType::EventPayload => self.get_payload_as_string(event),
            ConditionType::EventType => format!("{:?}", event.get_type()),
            ConditionType::EventSource => event.get_source().unwrap_or_default().to_string(),
            ConditionType::Variable => {
                // In a real implementation, we would get the variable value from a context
                // For now, we'll just use the condition_value as the variable name
                self.condition_value.clone()
            },
            ConditionType::Constant => self.condition_value.clone(),
        };
        
        // Perform the comparison
        match self.comparison {
            Comparison::Equal => lhs == self.reference_value,
            Comparison::NotEqual => lhs != self.reference_value,
            Comparison::Contains => lhs.contains(&self.reference_value),
            Comparison::StartsWith => lhs.starts_with(&self.reference_value),
            Comparison::EndsWith => lhs.ends_with(&self.reference_value),
            Comparison::GreaterThan => {
                // Try numeric comparison first
                if let (Ok(lhs_num), Ok(rhs_num)) = (lhs.parse::<f64>(), self.reference_value.parse::<f64>()) {
                    lhs_num > rhs_num
                } else {
                    // Fall back to string comparison
                    lhs > self.reference_value
                }
            },
            Comparison::LessThan => {
                if let (Ok(lhs_num), Ok(rhs_num)) = (lhs.parse::<f64>(), self.reference_value.parse::<f64>()) {
                    lhs_num < rhs_num
                } else {
                    lhs < self.reference_value
                }
            },
            Comparison::GreaterOrEqual => {
                if let (Ok(lhs_num), Ok(rhs_num)) = (lhs.parse::<f64>(), self.reference_value.parse::<f64>()) {
                    lhs_num >= rhs_num
                } else {
                    lhs >= self.reference_value
                }
            },
            Comparison::LessOrEqual => {
                if let (Ok(lhs_num), Ok(rhs_num)) = (lhs.parse::<f64>(), self.reference_value.parse::<f64>()) {
                    lhs_num <= rhs_num
                } else {
                    lhs <= self.reference_value
                }
            },
        }
    }
    
    /// Convert an event payload to a string for comparison
    fn get_payload_as_string(&self, event: &dyn Event) -> String {
        match event.get_payload() {
            EventPayload::None => String::new(),
            EventPayload::Text(s) => s.clone(),
            EventPayload::Number(n) => n.to_string(),
            EventPayload::Float(f) => f.to_string(),
            EventPayload::Boolean(b) => b.to_string(),
            EventPayload::Custom(_) => "[Custom Data]".to_string(),
        }
    }
}

#[async_trait]
impl Action for ConditionalAction {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_name(&self) -> &str {
        "Conditional"
    }

    fn get_description(&self) -> &str {
        "Executes actions based on a condition"
    }

    fn get_supported_event_types(&self) -> Vec<EventType> {
        // Conditional action can be used with any event type
        vec![
            EventType::System,
            EventType::Plugin,
            EventType::User,
            EventType::Internal,
            EventType::KeyPress,
        ]
    }

    fn get_plugin(&self) -> Arc<dyn Plugin> {
        self.plugin.clone()
    }

    async fn configure(&mut self, config: ActionConfig) -> Result<(), Error> {
        if config.args.len() < 3 {
            return Err(Error::InvalidArgument(
                "Conditional action requires at least 3 arguments: condition_type, comparison, reference_value".to_string()
            ));
        }
        
        self.condition_type = ConditionType::from(config.args[0].as_str());
        
        if config.args.len() > 3 {
            self.condition_value = config.args[1].clone();
            self.comparison = Comparison::from(config.args[2].as_str());
            self.reference_value = config.args[3].clone();
        } else {
            // Simplified format: condition_type == reference_value
            self.condition_value = String::new();
            self.comparison = Comparison::from(config.args[1].as_str());
            self.reference_value = config.args[2].clone();
        }
        
        Ok(())
    }

    async fn execute(&mut self, event: &dyn Event) -> Result<ActionResult, Error> {
        let condition_result = self.evaluate_condition(event);
        
        // Execute appropriate action branch
        let actions = if condition_result {
            &mut self.true_actions
        } else {
            &mut self.false_actions
        };
        
        // Execute all actions in the selected branch
        for action in actions.iter_mut() {
            match action.execute(event).await {
                Ok(_) => {}, // Continue to next action
                Err(e) => {
                    // Return early with the error
                    return Err(Error::InvalidOperation(format!(
                        "Error executing conditional branch action: {}", e
                    )));
                }
            }
        }
        
        Ok(ActionResult::success().with_data(condition_result))
    }

    fn validate(&self) -> Result<(), Error> {
        // Ensure we have a valid condition to check
        if self.reference_value.is_empty() {
            return Err(Error::InvalidConfiguration(
                "Conditional reference value cannot be empty".to_string()
            ));
        }
        
        Ok(())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use tokio::test;

    #[derive(Debug, Clone)]
    struct TestPlugin;

    #[async_trait]
    impl Plugin for TestPlugin {
        fn get_name(&self) -> &str { "Test" }
        fn get_description(&self) -> &str { "Test Plugin" }
        fn get_version(&self) -> &str { "1.0.0" }
        fn get_author(&self) -> &str { "Test Author" }

        fn get_info(&self) -> PluginInfo {
            PluginInfo {
                id: Uuid::new_v4(),
                name: self.get_name().to_string(),
                description: self.get_description().to_string(),
                version: self.get_version().to_string(),
                author: self.get_author().to_string(),
                homepage: None,
                platforms: vec!["all".to_string()],
                capabilities: vec![PluginCapability::ActionProvider],
            }
        }

        fn get_capabilities(&self) -> Vec<PluginCapability> {
            vec![PluginCapability::ActionProvider]
        }

        fn get_state(&self) -> PluginState {
            PluginState::Stopped
        }

        async fn initialize(&mut self) -> Result<(), PluginError> {
            Ok(())
        }

        async fn start(&mut self) -> Result<(), PluginError> {
            Ok(())
        }

        async fn stop(&mut self) -> Result<(), PluginError> {
            Ok(())
        }

        async fn handle_event(&mut self, _event: &dyn Event) -> Result<(), PluginError> {
            Ok(())
        }

        fn get_config(&self) -> Option<&Config> {
            None
        }

        async fn update_config(&mut self, _config: Config) -> Result<(), PluginError> {
            Ok(())
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn clone_box(&self) -> Box<dyn Plugin> {
            Box::new(self.clone())
        }
    }

    #[derive(Debug, Clone)]
    struct TestEvent {
        event_type: EventType,
        payload: EventPayload,
        source: Option<String>,
    }
    
    impl TestEvent {
        fn new(event_type: EventType, payload: EventPayload, source: Option<String>) -> Self {
            Self {
                event_type,
                payload,
                source,
            }
        }
    }

    impl Event for TestEvent {
        fn get_id(&self) -> &str {
            "test-event"
        }
        
        fn get_type(&self) -> EventType { 
            self.event_type.clone() 
        }
        
        fn get_payload(&self) -> &EventPayload {
            &self.payload
        }

        fn get_timestamp(&self) -> chrono::DateTime<chrono::Local> {
            chrono::Local::now()
        }

        fn get_source(&self) -> Option<&str> {
            self.source.as_deref()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }

        fn clone_event(&self) -> Box<dyn Event + Send + Sync> {
            Box::new(self.clone())
        }
    }
    
    #[derive(Debug)]
    struct TrackingAction {
        id: Uuid,
        plugin: Arc<dyn Plugin>,
        executed: bool,
    }
    
    impl TrackingAction {
        fn new(plugin: Arc<dyn Plugin>) -> Self {
            Self {
                id: Uuid::new_v4(),
                plugin,
                executed: false,
            }
        }
    }
    
    #[async_trait]
    impl Action for TrackingAction {
        fn get_id(&self) -> Uuid {
            self.id
        }
        
        fn get_name(&self) -> &str {
            "Tracking Action"
        }
        
        fn get_description(&self) -> &str {
            "Action that tracks execution"
        }
        
        fn get_supported_event_types(&self) -> Vec<EventType> {
            vec![EventType::System, EventType::KeyPress]
        }
        
        fn get_plugin(&self) -> Arc<dyn Plugin> {
            self.plugin.clone()
        }
        
        async fn execute(&mut self, _event: &dyn Event) -> Result<ActionResult, Error> {
            self.executed = true;
            Ok(ActionResult::success())
        }
        
        fn validate(&self) -> Result<(), Error> {
            Ok(())
        }
        
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    async fn test_conditional_action_event_payload() {
        let plugin = Arc::new(TestPlugin);
        let mut action = ConditionalAction::new(plugin.clone());
        
        // Configure the condition: EventPayload == "test"
        action.configure(ActionConfig {
            args: vec!["EventPayload".to_string(), "==".to_string(), "test".to_string()],
            enabled: true,
            should_select_on_execute: false,
        }).await.unwrap();
        
        // Add tracking actions to true and false branches
        let mut true_action = TrackingAction::new(plugin.clone());
        let mut false_action = TrackingAction::new(plugin.clone());
        
        action.add_true_action(Box::new(true_action));
        action.add_false_action(Box::new(false_action));
        
        // Create test event with matching payload
        let event_match = TestEvent::new(
            EventType::System,
            EventPayload::Text("test".to_string()),
            None,
        );
        
        // Execute and verify true branch was triggered
        let result = action.execute(&event_match).await.unwrap();
        assert!(result.success);
        
        // Access the executed flag directly since test code has full access to action fields
        assert!(action.true_actions[0].as_any().downcast_ref::<TrackingAction>().unwrap().executed);
        assert!(!action.false_actions[0].as_any().downcast_ref::<TrackingAction>().unwrap().executed);
    }
    
    #[test]
    async fn test_conditional_action_numeric_comparison() {
        let plugin = Arc::new(TestPlugin);
        let mut action = ConditionalAction::new(plugin.clone());
        
        // Configure the condition: EventPayload > 10
        action.configure(ActionConfig {
            args: vec!["EventPayload".to_string(), ">".to_string(), "10".to_string()],
            enabled: true,
            should_select_on_execute: false,
        }).await.unwrap();
        
        // Add tracking actions to true and false branches
        let mut true_action = TrackingAction::new(plugin.clone());
        let mut false_action = TrackingAction::new(plugin.clone());
        
        action.add_true_action(Box::new(true_action));
        action.add_false_action(Box::new(false_action));
        
        // Create test event with payload > 10
        let event_greater = TestEvent::new(
            EventType::System,
            EventPayload::Number(15),
            None,
        );
        
        // Execute and verify true branch was triggered
        let result = action.execute(&event_greater).await.unwrap();
        assert!(result.success);
        assert!(action.true_actions[0].as_any().downcast_ref::<TrackingAction>().unwrap().executed);
        assert!(!action.false_actions[0].as_any().downcast_ref::<TrackingAction>().unwrap().executed);
        
        // Reset the actions
        action.true_actions[0] = Box::new(TrackingAction::new(plugin.clone()));
        action.false_actions[0] = Box::new(TrackingAction::new(plugin.clone()));
        
        // Create test event with payload < 10
        let event_less = TestEvent::new(
            EventType::System,
            EventPayload::Number(5),
            None,
        );
        
        // Execute and verify false branch was triggered
        let result = action.execute(&event_less).await.unwrap();
        assert!(result.success);
        assert!(!action.true_actions[0].as_any().downcast_ref::<TrackingAction>().unwrap().executed);
        assert!(action.false_actions[0].as_any().downcast_ref::<TrackingAction>().unwrap().executed);
    }
    
    #[test]
    async fn test_conditional_action_event_type() {
        let plugin = Arc::new(TestPlugin);
        let mut action = ConditionalAction::new(plugin.clone());
        
        // Configure the condition: EventType == KeyPress
        action.configure(ActionConfig {
            args: vec!["EventType".to_string(), "==".to_string(), "KeyPress".to_string()],
            enabled: true,
            should_select_on_execute: false,
        }).await.unwrap();
        
        // Add tracking actions to true and false branches
        let mut true_action = TrackingAction::new(plugin.clone());
        let mut false_action = TrackingAction::new(plugin.clone());
        
        action.add_true_action(Box::new(true_action));
        action.add_false_action(Box::new(false_action));
        
        // Create test event with KeyPress type
        let keypress_event = TestEvent::new(
            EventType::KeyPress,
            EventPayload::None,
            None,
        );
        
        // Execute and verify true branch was triggered
        let result = action.execute(&keypress_event).await.unwrap();
        assert!(result.success);
        assert!(action.true_actions[0].as_any().downcast_ref::<TrackingAction>().unwrap().executed);
        assert!(!action.false_actions[0].as_any().downcast_ref::<TrackingAction>().unwrap().executed);
    }
    
    #[test]
    async fn test_conditional_action_source() {
        let plugin = Arc::new(TestPlugin);
        let mut action = ConditionalAction::new(plugin.clone());
        
        // Configure the condition: EventSource contains "keyboard"
        action.configure(ActionConfig {
            args: vec!["EventSource".to_string(), "contains".to_string(), "keyboard".to_string()],
            enabled: true,
            should_select_on_execute: false,
        }).await.unwrap();
        
        // Add tracking actions to true and false branches
        let mut true_action = TrackingAction::new(plugin.clone());
        let mut false_action = TrackingAction::new(plugin.clone());
        
        action.add_true_action(Box::new(true_action));
        action.add_false_action(Box::new(false_action));
        
        // Create test event with matching source
        let event_match = TestEvent::new(
            EventType::KeyPress,
            EventPayload::None,
            Some("usb-keyboard".to_string()),
        );
        
        // Execute and verify true branch was triggered
        let result = action.execute(&event_match).await.unwrap();
        assert!(result.success);
        assert!(action.true_actions[0].as_any().downcast_ref::<TrackingAction>().unwrap().executed);
        assert!(!action.false_actions[0].as_any().downcast_ref::<TrackingAction>().unwrap().executed);
    }
} 
