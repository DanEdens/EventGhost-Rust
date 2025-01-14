use chrono::{DateTime, Local};
use uuid::Uuid;
use crate::core::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    Command,
    Notification,
    System,
    Plugin,
}

#[derive(Debug, Clone)]
pub struct EventPayload {
    data: Box<dyn std::any::Any + Send + Sync>,
}

pub trait Event {
    fn get_id(&self) -> &str;
    fn get_type(&self) -> EventType;
    fn get_payload(&self) -> &EventPayload;
    fn get_timestamp(&self) -> DateTime<Local>;
}

pub struct EventGhostEvent {
    id: String,
    event_type: EventType,
    payload: EventPayload,
    timestamp: DateTime<Local>,
}

impl Event for EventGhostEvent {
    fn get_id(&self) -> &str { 
        todo!() 
    }
    
    fn get_type(&self) -> EventType { 
        todo!() 
    }
    
    fn get_payload(&self) -> &EventPayload { 
        todo!() 
    }
    
    fn get_timestamp(&self) -> DateTime<Local> { 
        todo!() 
    }
} 