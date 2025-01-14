use chrono::{DateTime, Local};
use std::collections::VecDeque;
use crate::core::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    System,
    Plugin,
    User,
    Internal,
}

#[derive(Debug, Clone)]
pub enum EventPayload {
    None,
    Text(String),
    Number(i64),
    Float(f64),
    Boolean(bool),
    Custom(Box<dyn std::any::Any + Send + Sync>),
}

pub trait Event: Send + Sync {
    fn get_id(&self) -> &str;
    fn get_type(&self) -> EventType;
    fn get_payload(&self) -> &EventPayload;
    fn get_timestamp(&self) -> DateTime<Local>;
    fn get_source(&self) -> Option<&str>;
}

pub trait EventHandler: Send + Sync {
    fn handle_event(&mut self, event: &Event) -> Result<(), Error>;
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
    }

    pub fn process_event(&mut self, event: Box<dyn Event>) -> Result<(), Error> {
        // TODO: Implement event processing
        Ok(())
    }
} 