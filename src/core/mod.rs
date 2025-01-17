//! Core functionality for EventGhost
//! 
//! This module provides the core systems including:
//! - Event system
//! - Plugin system with hot-reloading
//! - IPC via named pipes
//! - GUI abstractions
//! - Configuration persistence
//! - Logging system
//! - Error handling

pub mod error;
pub mod event;
pub mod config;
pub mod plugin;
pub mod logging;
pub mod named_pipe;

pub use error::Error;
pub use event::{Event, EventType, EventPayload, EventHandler, EventManager};
pub use plugin::{Plugin, PluginInfo};
pub use crate::eg::classes::PropertySource;

// Re-export commonly used types
pub use plugin::registry::PluginRegistry;
pub use plugin::loader::PluginLoader; 