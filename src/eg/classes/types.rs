use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragEffects {
    NONE = 0,
    COPY = 1,
    MOVE = 2,
    LINK = 4,
    SCROLL = 0x80000000,
}

impl DragEffects {
    pub const ALL: DragEffects = DragEffects::COPY;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    List(Vec<PropertyValue>),
    Map(std::collections::HashMap<String, PropertyValue>),
}

impl fmt::Display for PropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PropertyValue::String(s) => write!(f, "{}", s),
            PropertyValue::Integer(i) => write!(f, "{}", i),
            PropertyValue::Float(fl) => write!(f, "{}", fl),
            PropertyValue::Boolean(b) => write!(f, "{}", b),
            PropertyValue::List(l) => write!(f, "{:?}", l),
            PropertyValue::Map(m) => write!(f, "{:?}", m),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupFlags {
    NONE = 0,
    RIGHT = 0x0002,
    ALIGN_RIGHT = 0x0008,
    ALIGN_BOTTOM = 0x0020,
    RETURN_CMD = 0x0100,
} 
 
 
 