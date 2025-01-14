pub mod event;
pub mod plugin;
pub mod gui;
pub mod init;
pub mod named_pipe;
pub mod utils;
pub mod error;

pub use error::Error;
pub use event::{Event, EventHandler, EventManager, EventType, EventPayload};
pub use plugin::{Plugin, PluginInfo, PluginRegistry};
pub use gui::{UIComponent, Dialog, DialogResult}; 