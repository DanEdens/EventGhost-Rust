pub mod core;
pub mod eg;
pub mod win32;

// Re-exports of common types
pub use core::Error;

// Test utilities
#[cfg(test)]
pub mod testing; 