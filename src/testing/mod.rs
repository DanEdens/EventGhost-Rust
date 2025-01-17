//! Testing infrastructure for EventGhost
//! 
//! This module provides testing utilities including:
//! - Mock implementations
//! - Test helpers
//! - Integration test framework

pub mod mocks;
pub mod helpers;
pub mod fixtures;

// Re-export common testing utilities
pub use mocks::*;
pub use helpers::*;
pub use fixtures::*;

// Windows API testing
pub mod win32 {
    pub use super::mocks::win32::*;
    pub use super::helpers::win32::*;
} 