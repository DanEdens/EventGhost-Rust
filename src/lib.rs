pub mod core;
pub mod eg;
pub mod cli;

// Re-exports of common types
pub use core::Error;

// Test utilities
#[cfg(any(test, feature = "testing"))]
pub mod testing; 