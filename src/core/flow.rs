use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;
use crate::core::Error;
use crate::core::macro_::{MacroEngine, ExecutionState};

/// Represents a condition for flow control
#[derive(Debug, Clone)]
pub enum Condition {
    /// Always true
    Always,
    /// Last action was successful
    LastActionSuccessful,
    /// Last action failed
    LastActionFailed,
    /// Custom condition (evaluates a boolean expression)
    Custom(String),
}

/// Flow control actions for macros
pub struct FlowControl {
    engine: MacroEngine,
}

impl FlowControl {
    pub fn new(engine: MacroEngine) -> Self {
        Self { engine }
    }

    /// Execute a conditional jump
    pub async fn jump_if(
        &self,
        condition: Condition,
        from_macro_id: Uuid,
        to_macro_id: Uuid,
        should_return: bool,
    ) -> Result<(), Error> {
        match condition {
            Condition::Always => {
                self.engine.jump_to_macro(from_macro_id, to_macro_id, should_return).await
            }
            Condition::LastActionSuccessful => {
                // TODO: Check last action result
                Ok(())
            }
            Condition::LastActionFailed => {
                // TODO: Check last action result
                Ok(())
            }
            Condition::Custom(expr) => {
                // TODO: Evaluate custom expression
                println!("Custom condition: {}", expr);
                Ok(())
            }
        }
    }

    /// Execute a conditional jump with else branch
    pub async fn jump_if_else(
        &self,
        condition: Condition,
        from_macro_id: Uuid,
        true_macro_id: Uuid,
        false_macro_id: Uuid,
        should_return: bool,
    ) -> Result<(), Error> {
        match condition {
            Condition::Always => {
                self.engine.jump_to_macro(from_macro_id, true_macro_id, should_return).await
            }
            Condition::LastActionSuccessful => {
                // TODO: Check last action result and jump accordingly
                Ok(())
            }
            Condition::LastActionFailed => {
                // TODO: Check last action result and jump accordingly
                Ok(())
            }
            Condition::Custom(expr) => {
                // TODO: Evaluate custom expression and jump accordingly
                println!("Custom condition: {}", expr);
                Ok(())
            }
        }
    }

    /// Wait for a specified duration
    pub async fn wait(&self, duration: Duration) -> Result<(), Error> {
        sleep(duration).await;
        Ok(())
    }

    /// Wait until a condition is met or timeout occurs
    pub async fn wait_until(
        &self,
        condition: Condition,
        timeout: Option<Duration>,
    ) -> Result<(), Error> {
        let start = std::time::Instant::now();
        
        loop {
            match condition.clone() {
                Condition::Always => break,
                Condition::LastActionSuccessful => {
                    // TODO: Check last action result
                    break;
                }
                Condition::LastActionFailed => {
                    // TODO: Check last action result
                    break;
                }
                Condition::Custom(expr) => {
                    // TODO: Evaluate custom expression
                    println!("Custom condition: {}", expr);
                    break;
                }
            }

            if let Some(timeout) = timeout {
                if start.elapsed() >= timeout {
                    return Err(Error::Timeout("Wait condition timed out".to_string()));
                }
            }

            sleep(Duration::from_millis(100)).await;
        }

        Ok(())
    }

    /// Repeat a macro a specified number of times
    pub async fn repeat(
        &self,
        macro_id: Uuid,
        count: usize,
        delay: Option<Duration>,
    ) -> Result<(), Error> {
        for _ in 0..count {
            self.engine.execute_macro(macro_id, None).await?;
            
            if let Some(delay) = delay {
                sleep(delay).await;
            }
        }
        Ok(())
    }

    /// Repeat a macro while a condition is true
    pub async fn repeat_while(
        &self,
        macro_id: Uuid,
        condition: Condition,
        delay: Option<Duration>,
        max_iterations: Option<usize>,
    ) -> Result<(), Error> {
        let mut iterations = 0;
        
        loop {
            match condition.clone() {
                Condition::Always => {}
                Condition::LastActionSuccessful => {
                    // TODO: Check last action result
                    break;
                }
                Condition::LastActionFailed => {
                    // TODO: Check last action result
                    break;
                }
                Condition::Custom(expr) => {
                    // TODO: Evaluate custom expression
                    println!("Custom condition: {}", expr);
                    break;
                }
            }

            if let Some(max) = max_iterations {
                if iterations >= max {
                    break;
                }
            }

            self.engine.execute_macro(macro_id, None).await?;
            iterations += 1;

            if let Some(delay) = delay {
                sleep(delay).await;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_wait() {
        let engine = MacroEngine::new(1024);
        let flow = FlowControl::new(engine);
        
        // Test basic wait
        let start = std::time::Instant::now();
        flow.wait(Duration::from_millis(100)).await.unwrap();
        assert!(start.elapsed() >= Duration::from_millis(100));
    }

    #[tokio::test]
    async fn test_repeat() {
        let engine = MacroEngine::new(1024);
        let flow = FlowControl::new(engine);
        
        let macro_id = Uuid::new_v4();
        flow.repeat(macro_id, 3, Some(Duration::from_millis(50))).await.unwrap();
        
        let contexts = engine.contexts.read().await;
        assert_eq!(contexts.len(), 1);
    }
} 
