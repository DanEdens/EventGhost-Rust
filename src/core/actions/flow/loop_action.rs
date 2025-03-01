use std::any::Any;
use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::core::Error;
use crate::core::action::{Action, ActionResult, ActionConfig};
use crate::core::event::{Event, EventType, EventPayload};
use crate::core::plugin::Plugin;
use super::conditional::{ConditionType, Comparison};

/// Maximum number of iterations to prevent infinite loops
const MAX_ITERATIONS: usize = 10000;

/// Action that executes a set of actions in a while loop
#[derive(Debug)]
pub struct WhileLoopAction {
    id: Uuid,
    plugin: Arc<dyn Plugin>,
    condition_type: ConditionType,
    condition_value: String,
    comparison: Comparison,
    reference_value: String,
    actions: Vec<Box<dyn Action>>,
    max_iterations: usize,
}

impl WhileLoopAction {
    pub fn new(plugin: Arc<dyn Plugin>) -> Self {
        Self {
            id: Uuid::new_v4(),
            plugin,
            condition_type: ConditionType::default(),
            condition_value: String::new(),
            comparison: Comparison::default(),
            reference_value: String::new(),
            actions: Vec::new(),
            max_iterations: MAX_ITERATIONS,
        }
    }
    
    /// Add an action to execute in the loop
    pub fn add_action(&mut self, action: Box<dyn Action>) {
        self.actions.push(action);
    }
    
    /// Set maximum iterations to prevent infinite loops
    pub fn set_max_iterations(&mut self, max: usize) {
        self.max_iterations = max;
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
impl Action for WhileLoopAction {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_name(&self) -> &str {
        "While Loop"
    }

    fn get_description(&self) -> &str {
        "Executes actions repeatedly while a condition is true"
    }

    fn get_supported_event_types(&self) -> Vec<EventType> {
        // While loop action can be used with any event type
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
                "While loop action requires at least 3 arguments: condition_type, comparison, reference_value".to_string()
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
        
        // If max iterations is specified
        if config.args.len() > 4 {
            if let Ok(max) = config.args[4].parse::<usize>() {
                self.max_iterations = max;
            }
        }
        
        Ok(())
    }

    async fn execute(&mut self, event: &dyn Event) -> Result<ActionResult, Error> {
        let mut iterations = 0;
        let mut last_result = ActionResult::success();
        
        // Loop while condition is true and we haven't exceeded max iterations
        while self.evaluate_condition(event) && iterations < self.max_iterations {
            iterations += 1;
            
            // Execute all actions in the loop
            for action in self.actions.iter_mut() {
                match action.execute(event).await {
                    Ok(result) => {
                        last_result = result;
                    },
                    Err(e) => {
                        return Err(Error::InvalidOperation(format!(
                            "Error executing loop action (iteration {}): {}", iterations, e
                        )));
                    }
                }
            }
        }
        
        // Check if we hit the max iteration limit
        if iterations >= self.max_iterations {
            Ok(ActionResult::failure(format!(
                "Loop terminated after reaching maximum iterations ({})", self.max_iterations
            )).with_data(iterations))
        } else {
            Ok(ActionResult::success().with_data(iterations))
        }
    }

    fn validate(&self) -> Result<(), Error> {
        // Ensure we have a valid condition to check
        if self.reference_value.is_empty() {
            return Err(Error::InvalidConfiguration(
                "While loop reference value cannot be empty".to_string()
            ));
        }
        
        Ok(())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Action that executes a set of actions a specified number of times
#[derive(Debug)]
pub struct ForLoopAction {
    id: Uuid,
    plugin: Arc<dyn Plugin>,
    start: i64,
    end: i64,
    step: i64,
    variable_name: String,
    actions: Vec<Box<dyn Action>>,
}

impl ForLoopAction {
    pub fn new(plugin: Arc<dyn Plugin>) -> Self {
        Self {
            id: Uuid::new_v4(),
            plugin,
            start: 0,
            end: 10,
            step: 1,
            variable_name: "i".to_string(),
            actions: Vec::new(),
        }
    }
    
    /// Add an action to execute in the loop
    pub fn add_action(&mut self, action: Box<dyn Action>) {
        self.actions.push(action);
    }
}

#[async_trait]
impl Action for ForLoopAction {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_name(&self) -> &str {
        "For Loop"
    }

    fn get_description(&self) -> &str {
        "Executes actions a specified number of times"
    }

    fn get_supported_event_types(&self) -> Vec<EventType> {
        // For loop action can be used with any event type
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
        // Different configuration formats:
        // 1. ForLoop(end) - Loop from 0 to end-1 with step 1
        // 2. ForLoop(start, end) - Loop from start to end-1 with step 1
        // 3. ForLoop(start, end, step) - Loop from start to end-1 with given step
        // 4. ForLoop(start, end, step, variable_name) - Same as above with custom variable name
        
        if config.args.is_empty() {
            return Err(Error::InvalidArgument(
                "For loop action requires at least one argument: the end value".to_string()
            ));
        }
        
        // Parse end value (required)
        match config.args[0].parse::<i64>() {
            Ok(val) => {
                self.end = val;
            },
            Err(_) => {
                return Err(Error::InvalidArgument(
                    format!("Invalid end value: {}", config.args[0])
                ));
            }
        }
        
        // Parse start value (optional)
        if config.args.len() > 1 {
            match config.args[1].parse::<i64>() {
                Ok(val) => {
                    self.start = val;
                },
                Err(_) => {
                    return Err(Error::InvalidArgument(
                        format!("Invalid start value: {}", config.args[1])
                    ));
                }
            }
        }
        
        // Parse step value (optional)
        if config.args.len() > 2 {
            match config.args[2].parse::<i64>() {
                Ok(val) => {
                    if val == 0 {
                        return Err(Error::InvalidArgument(
                            "Step value cannot be zero".to_string()
                        ));
                    }
                    self.step = val;
                },
                Err(_) => {
                    return Err(Error::InvalidArgument(
                        format!("Invalid step value: {}", config.args[2])
                    ));
                }
            }
        }
        
        // Parse variable name (optional)
        if config.args.len() > 3 {
            self.variable_name = config.args[3].clone();
        }
        
        Ok(())
    }

    async fn execute(&mut self, event: &dyn Event) -> Result<ActionResult, Error> {
        let mut iterations = 0;
        let direction = if self.step > 0 { 1 } else { -1 };
        
        // In a real implementation, we would need a context to store the loop variable
        // For now, we just execute the actions without setting the variable
        
        // Define the loop condition based on the direction
        let should_continue = |current: i64| -> bool {
            if direction > 0 {
                current < self.end
            } else {
                current > self.end
            }
        };
        
        // Execute the loop
        let mut current = self.start;
        while should_continue(current) {
            iterations += 1;
            
            // TODO: In a full implementation, we would set the loop variable here
            // context.set_variable(self.variable_name.clone(), current);
            
            // Execute all actions in the loop
            for action in self.actions.iter_mut() {
                match action.execute(event).await {
                    Ok(_) => {},
                    Err(e) => {
                        return Err(Error::InvalidOperation(format!(
                            "Error executing for loop action (iteration {}): {}", iterations, e
                        )));
                    }
                }
            }
            
            // Increment the loop variable
            current += self.step;
            
            // Safety check to prevent potential infinite loops
            if iterations > MAX_ITERATIONS {
                return Ok(ActionResult::failure(format!(
                    "For loop terminated after reaching maximum iterations ({})", MAX_ITERATIONS
                )).with_data(iterations));
            }
        }
        
        Ok(ActionResult::success().with_data(iterations))
    }

    fn validate(&self) -> Result<(), Error> {
        // Ensure step is not zero
        if self.step == 0 {
            return Err(Error::InvalidConfiguration(
                "For loop step cannot be zero".to_string()
            ));
        }
        
        // Check that the loop will eventually terminate
        let will_terminate = if self.step > 0 {
            self.start < self.end
        } else {
            self.start > self.end
        };
        
        if !will_terminate {
            return Err(Error::InvalidConfiguration(
                format!("For loop will never execute with start={}, end={}, step={}",
                       self.start, self.end, self.step)
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
    
    #[cfg(any(test, feature = "testing"))]
    use crate::core::plugin::MockPlugin;
    
    #[cfg(any(test, feature = "testing"))]
    use crate::core::event::MockEvent;
    
    use std::sync::atomic::{AtomicI32, Ordering};
    
    #[tokio::test]
    async fn test_while_loop_execution() {
        let plugin = Arc::new(MockPlugin::new());
        let mut action = WhileLoopAction::new(plugin.clone());
        
        // Configure the loop for a simpler test
        // We'll set a condition that's never true, but use max_iterations
        // to control the execution
        action.condition_type = ConditionType::Constant;
        action.condition_value = "true".to_string(); // This is just for documentary purposes
        action.comparison = Comparison::Equal;
        action.reference_value = "true".to_string();
        
        // Set a very small max iterations so the test will always complete
        action.max_iterations = 5;
        
        // Create a counter that will be incremented by the test action
        let counter = Arc::new(AtomicI32::new(0));
        
        // Create a test action that increments the counter
        #[derive(Debug)]
        struct CounterAction {
            id: Uuid,
            plugin: Arc<dyn Plugin>,
            counter: Arc<AtomicI32>,
        }
        
        #[async_trait]
        impl Action for CounterAction {
            fn get_id(&self) -> Uuid {
                self.id
            }
            
            fn get_name(&self) -> &str {
                "Counter Action"
            }
            
            fn get_description(&self) -> &str {
                "Increments a counter"
            }
            
            fn get_supported_event_types(&self) -> Vec<EventType> {
                vec![EventType::System]
            }
            
            fn get_plugin(&self) -> Arc<dyn Plugin> {
                self.plugin.clone()
            }
            
            async fn execute(&mut self, _event: &dyn Event) -> Result<ActionResult, Error> {
                self.counter.fetch_add(1, Ordering::SeqCst);
                Ok(ActionResult::success())
            }
            
            fn validate(&self) -> Result<(), Error> {
                Ok(())
            }
            
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
        
        // Add the counter action to the loop
        let counter_action = CounterAction {
            id: Uuid::new_v4(),
            plugin: plugin.clone(),
            counter: counter.clone(),
        };
        action.add_action(Box::new(counter_action));
        
        // Create a test event
        let event = MockEvent::new(
            EventType::System,
            EventPayload::None,
        );
        
        // Execute the loop - this should run until max_iterations is reached
        let result = action.execute(&event).await.unwrap();
        
        // Since we set condition to be always true, it should hit the max iterations
        // and return failure
        assert!(!result.success);
        
        // Check that the loop executed 5 times (our max_iterations value)
        assert_eq!(counter.load(Ordering::SeqCst), 5);
    }
    
    #[tokio::test]
    async fn test_for_loop_execution() {
        let plugin = Arc::new(MockPlugin::new());
        let mut action = ForLoopAction::new(plugin.clone());
        
        // Configure to loop from 0 to 5
        action.configure(ActionConfig {
            args: vec!["5".to_string()],
            enabled: true,
            should_select_on_execute: false,
        }).await.unwrap();
        
        // Create a counter that will track the number of executions
        let counter = Arc::new(AtomicI32::new(0));
        
        // Create a test action that increments the counter
        #[derive(Debug)]
        struct CounterAction {
            id: Uuid,
            plugin: Arc<dyn Plugin>,
            counter: Arc<AtomicI32>,
        }
        
        #[async_trait]
        impl Action for CounterAction {
            fn get_id(&self) -> Uuid {
                self.id
            }
            
            fn get_name(&self) -> &str {
                "Counter Action"
            }
            
            fn get_description(&self) -> &str {
                "Increments a counter"
            }
            
            fn get_supported_event_types(&self) -> Vec<EventType> {
                vec![EventType::System]
            }
            
            fn get_plugin(&self) -> Arc<dyn Plugin> {
                self.plugin.clone()
            }
            
            async fn execute(&mut self, _event: &dyn Event) -> Result<ActionResult, Error> {
                self.counter.fetch_add(1, Ordering::SeqCst);
                Ok(ActionResult::success())
            }
            
            fn validate(&self) -> Result<(), Error> {
                Ok(())
            }
            
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
        
        // Add the counter action to the loop
        let counter_action = CounterAction {
            id: Uuid::new_v4(),
            plugin: plugin.clone(),
            counter: counter.clone(),
        };
        action.add_action(Box::new(counter_action));
        
        // Create a test event
        let event = MockEvent::new(
            EventType::System,
            EventPayload::None,
        );
        
        // Execute the loop
        let result = action.execute(&event).await.unwrap();
        
        // Check that the loop executed 5 times (from 0 to 4)
        assert!(result.success);
        assert_eq!(counter.load(Ordering::SeqCst), 5);
    }
    
    #[tokio::test]
    async fn test_for_loop_custom_range() {
        let plugin = Arc::new(MockPlugin::new());
        let mut action = ForLoopAction::new(plugin.clone());
        
        // Configure to loop from 2 to 7 with step 2
        action.configure(ActionConfig {
            args: vec!["7".to_string(), "2".to_string(), "2".to_string()],
            enabled: true,
            should_select_on_execute: false,
        }).await.unwrap();
        
        // Create a counter that will track the number of executions
        let counter = Arc::new(AtomicI32::new(0));
        
        // Create a test action that increments the counter
        #[derive(Debug)]
        struct CounterAction {
            id: Uuid,
            plugin: Arc<dyn Plugin>,
            counter: Arc<AtomicI32>,
        }
        
        #[async_trait]
        impl Action for CounterAction {
            fn get_id(&self) -> Uuid {
                self.id
            }
            
            fn get_name(&self) -> &str {
                "Counter Action"
            }
            
            fn get_description(&self) -> &str {
                "Increments a counter"
            }
            
            fn get_supported_event_types(&self) -> Vec<EventType> {
                vec![EventType::System]
            }
            
            fn get_plugin(&self) -> Arc<dyn Plugin> {
                self.plugin.clone()
            }
            
            async fn execute(&mut self, _event: &dyn Event) -> Result<ActionResult, Error> {
                self.counter.fetch_add(1, Ordering::SeqCst);
                Ok(ActionResult::success())
            }
            
            fn validate(&self) -> Result<(), Error> {
                Ok(())
            }
            
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
        
        // Add the counter action to the loop
        let counter_action = CounterAction {
            id: Uuid::new_v4(),
            plugin: plugin.clone(),
            counter: counter.clone(),
        };
        action.add_action(Box::new(counter_action));
        
        // Create a test event
        let event = MockEvent::new(
            EventType::System,
            EventPayload::None,
        );
        
        // Execute the loop
        let result = action.execute(&event).await.unwrap();
        
        // Should execute for i=2, i=4, i=6 (3 times)
        assert!(result.success);
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }
    
    #[tokio::test]
    async fn test_for_loop_negative_step() {
        let plugin = Arc::new(MockPlugin::new());
        let mut action = ForLoopAction::new(plugin.clone());
        
        // Configure to loop from 10 to 0 with step -2
        action.configure(ActionConfig {
            args: vec!["0".to_string(), "10".to_string(), "-2".to_string()],
            enabled: true,
            should_select_on_execute: false,
        }).await.unwrap();
        
        // Create a counter that will track the number of executions
        let counter = Arc::new(AtomicI32::new(0));
        
        // Create a test action that increments the counter
        #[derive(Debug)]
        struct CounterAction {
            id: Uuid,
            plugin: Arc<dyn Plugin>,
            counter: Arc<AtomicI32>,
        }
        
        #[async_trait]
        impl Action for CounterAction {
            fn get_id(&self) -> Uuid {
                self.id
            }
            
            fn get_name(&self) -> &str {
                "Counter Action"
            }
            
            fn get_description(&self) -> &str {
                "Increments a counter"
            }
            
            fn get_supported_event_types(&self) -> Vec<EventType> {
                vec![EventType::System]
            }
            
            fn get_plugin(&self) -> Arc<dyn Plugin> {
                self.plugin.clone()
            }
            
            async fn execute(&mut self, _event: &dyn Event) -> Result<ActionResult, Error> {
                self.counter.fetch_add(1, Ordering::SeqCst);
                Ok(ActionResult::success())
            }
            
            fn validate(&self) -> Result<(), Error> {
                Ok(())
            }
            
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
        
        // Add the counter action to the loop
        let counter_action = CounterAction {
            id: Uuid::new_v4(),
            plugin: plugin.clone(),
            counter: counter.clone(),
        };
        action.add_action(Box::new(counter_action));
        
        // Create a test event
        let event = MockEvent::new(
            EventType::System,
            EventPayload::None,
        );
        
        // Execute the loop
        let result = action.execute(&event).await.unwrap();
        
        // Should execute for i=10, i=8, i=6, i=4, i=2 (5 times)
        assert!(result.success);
        assert_eq!(counter.load(Ordering::SeqCst), 5);
    }
} 