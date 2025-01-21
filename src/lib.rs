pub mod core;
pub mod eg;

// Re-exports of common types
pub use core::Error;

// Test utilities
#[cfg(test)]
pub mod testing; 