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
pub mod plugin;
pub mod gui;
pub mod init;
pub mod named_pipe;
pub mod plugin_loader;
pub mod config;
pub mod logging;
pub mod utils;

pub use error::Error;
pub use event::{Event, EventType, EventHandler};
pub use plugin::{Plugin, PluginInfo, PropertySource};
pub use gui::{Window, WindowConfig};
pub use named_pipe::{NamedPipeServer, NamedPipeClient, PipeError};
pub use plugin_loader::{PluginLoader, PluginLoadError};
pub use config::{Config, ConfigManager, ConfigStore, ConfigError};
pub use logging::{Logger, LogConfig, LogTarget, LogEntry}; 