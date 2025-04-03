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
//! - Globals system with MQTT and Redis backends

pub mod config;
pub mod event;
pub mod plugin;
pub mod error;
pub mod logging;
pub mod named_pipe;
pub mod action;
pub mod actions;
pub mod utils;
pub mod globals;
pub mod config_manager;

pub use error::Error;
pub use config::Config;

// Plugin types - Basic functionality only for now
pub use plugin::registry::PluginRegistry;
pub use plugin::loader::PluginLoader;
pub use plugin::Plugin;
pub use plugin::PluginInfo;

// Event types
pub use event::Event;
pub use event::EventManager;
pub use event::EventHandler;

// Action types
pub use action::Action;
pub use action::ActionManager;
pub use action::ActionResult;

// Action implementations
pub use actions::DelayAction;

// Globals types
pub use globals::GlobalsStore;
pub use globals::GlobalsConfig;
pub use globals::GlobalsBackendType;
pub use globals::GlobalValue;

// Re-export config types
pub use config::ConfigStore;
pub use config::ConfigError;

// TODO: Phase 2 - Advanced plugin functionality
// pub use plugin::manager::PluginManager;
// pub use plugin::config::PluginConfig;
// pub use plugin::config::GlobalConfig; 