pub mod core;
pub mod eg;
pub mod utils;
pub mod cli;

// Re-exports of common types
pub use core::{Error, Event, Plugin, PluginRegistry};
pub use eg::prelude;

// Convenience import
pub mod prelude {
    pub use crate::eg::prelude::*;
}

// Test utilities
#[cfg(any(test, feature = "testing"))]
pub mod testing; 