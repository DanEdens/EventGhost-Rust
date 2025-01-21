use chrono::{DateTime, Local};
use std::collections::VecDeque;
use crate::core::Error;
use std::fmt::Debug;
use std::any::Any;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub trait EventHandler: Send + Sync {
    fn handle_event(&mut self, event: &dyn Event) -> Result<(), Error>;
    fn can_handle(&self, event_type: EventType) -> bool;
}

pub struct EventManager {
    handlers: Vec<Box<dyn EventHandler>>,
    event_queue: VecDeque<Box<dyn Event>>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            event_queue: VecDeque::new(),
        }
    }

    pub fn register_handler(&mut self, handler: Box<dyn EventHandler>) {
        self.handlers.push(handler);
    }

    pub fn unregister_handler(&mut self, id: &str) {
        // TODO: Implement handler removal
        println!("Unregistering handler: {}", id);
    }


    pub fn process_event(&mut self, event: Box<dyn Event>) -> Result<(), Error> {
        // TODO: Implement event processing
        println!("Processing event: {:?}", event);
        Ok(())
    }
} 