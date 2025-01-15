//! Plugin system for EventGhost
//! 
//! This module provides the core plugin functionality including:
//! - Plugin traits and types
//! - Plugin registry for management
//! - Plugin loading and unloading
//! - Plugin configuration
//! - Plugin state management

mod traits;
mod registry;
mod loader;

pub use traits::{
    Plugin, PluginInfo, PluginState, PluginCapability,
    EventGenerator, Configurable, Stateful,
};
pub use registry::{PluginRegistry, RegistryError};

// Re-export from plugin_loader
pub use crate::core::plugin_loader::PluginLoader; 