pub mod error;
pub mod event;
pub mod plugin;
pub mod gui;
pub mod init;
pub mod named_pipe;
pub mod utils;

pub use error::Error;
pub use event::{Event, EventType, EventHandler};
pub use plugin::{Plugin, PluginInfo, PropertySource};
pub use gui::{Window, WindowConfig};
pub use named_pipe::{NamedPipeServer, NamedPipeClient, PipeError}; 