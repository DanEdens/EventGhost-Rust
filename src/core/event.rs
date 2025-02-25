use chrono::{DateTime, Local};
use std::collections::{VecDeque, HashMap};
use tokio::sync::{RwLock, broadcast};
use std::sync::Arc;
use crate::core::Error;
use std::fmt::Debug;
use std::any::Any;
use async_trait::async_trait;
use futures;

#[cfg(any(test, feature = "testing"))]
use crate::testing::MockEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    System,
    Plugin,
    User,
    Internal,
    KeyPress,
}

#[derive(Debug)]
pub enum EventPayload {
    None,
    Text(String),
    Number(i64),
    Float(f64),
    Boolean(bool),
    Custom(Box<dyn std::any::Any + Send + Sync>),
}

impl Clone for EventPayload {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Text(s) => Self::Text(s.clone()),
            Self::Number(n) => Self::Number(*n),
            Self::Float(f) => Self::Float(*f),
            Self::Boolean(b) => Self::Boolean(*b),
            Self::Custom(_) => Self::None, // Custom data cannot be cloned, fallback to None
        }
    }
}

pub trait Event: Any + Send + Sync + Debug {
    fn get_id(&self) -> &str;
    fn get_type(&self) -> EventType;
    fn get_payload(&self) -> &EventPayload;
    fn get_timestamp(&self) -> DateTime<Local>;
    fn get_source(&self) -> Option<&str>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn clone_event(&self) -> Box<dyn Event + Send + Sync>;
}

impl Clone for Box<dyn Event + Send + Sync> {
    fn clone(&self) -> Self {
        self.clone_event()
    }
}

#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Handle an event
    async fn handle_event(&mut self, event: &dyn Event) -> Result<(), Error>;
    
    /// Get the unique identifier for this handler
    fn get_id(&self) -> &str;
    
    /// Get the event types this handler supports
    fn get_supported_event_types(&self) -> Vec<EventType>;
    
    /// Check if this handler can handle a specific event type
    fn can_handle(&self, event_type: EventType) -> bool {
        self.get_supported_event_types().contains(&event_type)
    }
}

pub struct EventBus {
    sender: broadcast::Sender<Box<dyn Event + Send + Sync>>,
    subscribers: Arc<RwLock<HashMap<EventType, Vec<Arc<RwLock<Box<dyn EventHandler>>>>>>>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self {
            sender,
            subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn subscribe(&self, event_type: EventType, handler: Arc<RwLock<Box<dyn EventHandler>>>) {
        let mut subscribers = self.subscribers.write().await;
        subscribers.entry(event_type)
            .or_insert_with(Vec::new)
            .push(handler);
    }

    pub async fn unsubscribe(&self, event_type: EventType, handler_id: &str) {
        let mut subscribers = self.subscribers.write().await;
        if let Some(handlers) = subscribers.get_mut(&event_type) {
            handlers.retain(|h| {
                let handler = h.blocking_read();
                handler.get_id() != handler_id
            });
        }
    }

    pub async fn publish(&self, event: Box<dyn Event + Send + Sync>) -> Result<(), Error> {
        let subscribers = self.subscribers.read().await;
        if let Some(handlers) = subscribers.get(&event.get_type()) {
            let mut futures = Vec::new();
            for handler in handlers {
                let handler = handler.clone();
                let event = event.clone();
                futures.push(async move {
                    let mut handler = handler.write().await;
                    handler.handle_event(event.as_ref()).await
                });
            }
            
            // Execute all handlers concurrently
            futures::future::join_all(futures).await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;
        }
        
        // Publish event to broadcast channel
        self.sender.send(event).map_err(|e| Error::EventBus(e.to_string()))?;
        Ok(())
    }

    pub fn subscribe_to_events(&self) -> broadcast::Receiver<Box<dyn Event + Send + Sync>> {
        self.sender.subscribe()
    }
}

pub struct EventManager {
    event_bus: EventBus,
    event_queue: Arc<RwLock<VecDeque<Box<dyn Event + Send + Sync>>>>,
    filters: Vec<Box<dyn Fn(&dyn Event) -> bool + Send + Sync>>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            event_bus: EventBus::new(1024), // Buffer size of 1024 events
            event_queue: Arc::new(RwLock::new(VecDeque::new())),
            filters: Vec::new(),
        }
    }

    pub async fn register_handler(&self, handler: Arc<RwLock<Box<dyn EventHandler>>>) {
        let handler_ref = handler.read().await;
        let event_types = handler_ref.get_supported_event_types();
        drop(handler_ref);
        
        for event_type in event_types {
            self.event_bus.subscribe(event_type, handler.clone()).await;
        }
    }

    pub async fn unregister_handler(&self, handler_id: &str, event_type: EventType) {
        self.event_bus.unsubscribe(event_type, handler_id).await;
    }

    pub async fn process_event(&self, event: Box<dyn Event + Send + Sync>) -> Result<(), Error> {
        let subscribers = self.event_bus.subscribers.read().await;
        if let Some(handlers) = subscribers.get(&event.get_type()) {
            let mut futures = Vec::new();
            for handler in handlers {
                let handler = handler.clone();
                let event = event.clone();
                futures.push(async move {
                    let mut handler = handler.write().await;
                    handler.handle_event(event.as_ref()).await
                });
            }
            
            // Execute all handlers concurrently
            futures::future::join_all(futures).await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;
        }
        
        // Publish event to broadcast channel
        self.event_bus.publish(event).await?;
        Ok(())
    }

    pub fn add_filter<F>(&mut self, filter: F)
    where
        F: Fn(&dyn Event) -> bool + Send + Sync + 'static,
    {
        self.filters.push(Box::new(filter));
    }

    pub async fn get_queued_events(&self) -> Vec<Box<dyn Event + Send + Sync>> {
        self.event_queue.read().await.iter().cloned().collect()
    }

    pub fn subscribe_to_events(&self) -> broadcast::Receiver<Box<dyn Event + Send + Sync>> {
        self.event_bus.subscribe_to_events()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_event_routing() {
        let manager = EventManager::new();
        
        // Create a mock event with the correct constructor parameters
        let event = Box::new(MockEvent::new(
            EventType::System,
            EventPayload::Text("test".to_string()),
        ));
        
        // Subscribe to events
        let mut receiver = manager.subscribe_to_events();
        
        // Process event
        manager.process_event(event.clone()).await.unwrap();
        
        // Verify event was received
        let received = tokio::time::timeout(
            Duration::from_secs(1),
            receiver.recv()
        ).await.unwrap().unwrap();
        
        assert_eq!(received.get_id(), event.get_id());
        assert_eq!(received.get_type(), event.get_type());
    }
} 