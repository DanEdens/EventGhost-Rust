//! Testing infrastructure for EventGhost
//! 
//! This module provides testing utilities including:
//! - Mock implementations
//! - Test helpers
//! - Integration test framework

pub mod fixtures;
pub mod helpers;
pub mod mocks;

#[cfg(test)]
pub mod test {
    pub use super::fixtures::*;
    pub use super::helpers::*;
    pub use super::mocks::*;
}

// Re-export common testing utilities
pub use mocks::*;
pub use helpers::*;
pub use fixtures::*; 
