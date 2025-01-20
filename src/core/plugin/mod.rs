//! Plugin system for EventGhost
//! 
//! This module provides the core plugin functionality including:
//! - Plugin traits and types
//! - Plugin registry for management
//! - Plugin loading and unloading
//! - Plugin configuration
//! - Plugin state management

pub mod loader;
pub mod registry;
pub mod traits;

pub use self::traits::*;
pub use registry::PluginRegistry;
pub use loader::PluginLoader;

// Re-export common types
pub use registry::RegistryError;
pub use loader::LoaderError; 