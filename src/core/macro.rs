use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use uuid::Uuid;
use crate::core::Error;
use crate::core::event::{Event, EventType};
use crate::core::config::Config;
use std::collections::VecDeque;

/// Represents the execution state of a macro
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionState {
    /// Macro is not running
    Idle,
    /// Macro is running
    Running,
    /// Macro is paused
    Paused,
    /// Macro has completed
    Completed,
    /// Macro execution failed
    Failed(String),
}

/// Represents a program counter for macro execution
#[derive(Debug, Clone)]
pub struct ProgramCounter {
    /// The ID of the current action
    pub action_id: Uuid,
    /// The index within the current macro
    pub index: usize,
}

/// Represents a macro execution context
#[derive(Debug)]
pub struct MacroContext {
    /// The macro ID being executed
    pub macro_id: Uuid,
    /// The event that triggered the macro
    pub trigger_event: Option<Box<dyn Event + Send + Sync>>,
    /// The current program counter
    pub program_counter: Option<ProgramCounter>,
    /// The return stack for nested macro calls
    pub return_stack: VecDeque<ProgramCounter>,
    /// Local variables for this macro execution
    pub variables: Config,
    /// The execution state
    pub state: ExecutionState,
}

impl MacroContext {
    pub fn new(macro_id: Uuid, trigger_event: Option<Box<dyn Event + Send + Sync>>) -> Self {
        Self {
            macro_id,
            trigger_event,
            program_counter: None,
            return_stack: VecDeque::new(),
            variables: Config::new(),
            state: ExecutionState::Idle,
        }
    }
}

/// The macro execution engine
pub struct MacroEngine {
    /// Active macro execution contexts
    contexts: Arc<RwLock<Vec<MacroContext>>>,
    /// Event sender for macro execution events
    event_sender: broadcast::Sender<Box<dyn Event + Send + Sync>>,
}

impl MacroEngine {
    pub fn new(event_capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(event_capacity);
        Self {
            contexts: Arc::new(RwLock::new(Vec::new())),
            event_sender: sender,
        }
    }

    /// Start executing a macro
    pub async fn execute_macro(
        &self,
        macro_id: Uuid,
        trigger_event: Option<Box<dyn Event + Send + Sync>>,
    ) -> Result<(), Error> {
        let mut contexts = self.contexts.write().await;
        
        // Create new execution context
        let context = MacroContext::new(macro_id, trigger_event);
        contexts.push(context);
        
        Ok(())
    }

    /// Stop a running macro
    pub async fn stop_macro(&self, macro_id: Uuid) -> Result<(), Error> {
        let mut contexts = self.contexts.write().await;
        
        // Find and remove the context
        if let Some(pos) = contexts.iter().position(|ctx| ctx.macro_id == macro_id) {
            contexts.remove(pos);
        }
        
        Ok(())
    }

    /// Pause a running macro
    pub async fn pause_macro(&self, macro_id: Uuid) -> Result<(), Error> {
        let mut contexts = self.contexts.write().await;
        
        // Find and pause the context
        if let Some(context) = contexts.iter_mut().find(|ctx| ctx.macro_id == macro_id) {
            context.state = ExecutionState::Paused;
        }
        
        Ok(())
    }

    /// Resume a paused macro
    pub async fn resume_macro(&self, macro_id: Uuid) -> Result<(), Error> {
        let mut contexts = self.contexts.write().await;
        
        // Find and resume the context
        if let Some(context) = contexts.iter_mut().find(|ctx| ctx.macro_id == macro_id) {
            context.state = ExecutionState::Running;
        }
        
        Ok(())
    }

    /// Jump to another macro
    pub async fn jump_to_macro(
        &self,
        from_macro_id: Uuid,
        to_macro_id: Uuid,
        should_return: bool,
    ) -> Result<(), Error> {
        let mut contexts = self.contexts.write().await;
        
        // Find the source context
        if let Some(context) = contexts.iter_mut().find(|ctx| ctx.macro_id == from_macro_id) {
            // Save return point if needed
            if should_return {
                if let Some(pc) = &context.program_counter {
                    context.return_stack.push_back(pc.clone());
                }
            }
            
            // Update program counter to new macro
            context.program_counter = Some(ProgramCounter {
                action_id: to_macro_id,
                index: 0,
            });
        }
        
        Ok(())
    }

    /// Return from a macro to the calling macro
    pub async fn return_from_macro(&self, macro_id: Uuid) -> Result<(), Error> {
        let mut contexts = self.contexts.write().await;
        
        // Find the context
        if let Some(context) = contexts.iter_mut().find(|ctx| ctx.macro_id == macro_id) {
            // Restore return point
            if let Some(return_pc) = context.return_stack.pop_back() {
                context.program_counter = Some(return_pc);
            } else {
                context.state = ExecutionState::Completed;
            }
        }
        
        Ok(())
    }

    /// Get a subscription to macro execution events
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<Box<dyn Event + Send + Sync>> {
        self.event_sender.subscribe()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::mocks::MockEvent;

    #[tokio::test]
    async fn test_macro_execution() {
        let engine = MacroEngine::new(1024);
        
        // Create mock event
        let event = Box::new(MockEvent::new(
            "test_event",
            EventType::System,
            crate::core::event::EventPayload::Text("test".to_string()),
        ));
        
        // Start macro execution
        let macro_id = Uuid::new_v4();
        engine.execute_macro(macro_id, Some(event)).await.unwrap();
        
        // Verify context was created
        let contexts = engine.contexts.read().await;
        assert_eq!(contexts.len(), 1);
        assert_eq!(contexts[0].macro_id, macro_id);
        assert_eq!(contexts[0].state, ExecutionState::Idle);
        
        // Test stopping macro
        drop(contexts);
        engine.stop_macro(macro_id).await.unwrap();
        
        let contexts = engine.contexts.read().await;
        assert_eq!(contexts.len(), 0);
    }
} 