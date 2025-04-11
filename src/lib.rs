pub mod core;
pub mod eg;
pub mod utils;
pub mod cli;

// Re-exports of common types
pub use core::{Error, Event, Plugin, PluginRegistry};
pub use eg::classes::main_frame::MainFrame;
pub use eg::classes::log_ctrl::LogCtrl;
pub use eg::config::Config;

// Export the prelude module
pub use eg::prelude;

// Test utilities
#[cfg(any(test, feature = "testing"))]
pub mod testing; 
