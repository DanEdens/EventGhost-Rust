//! Testing infrastructure for EventGhost
//! 
//! This module provides testing utilities including:
//! - Mock implementations
//! - Test helpers
//! - Integration test framework

pub mod mocks;
pub mod helpers;
pub mod fixtures;

pub use mocks::*;
pub use helpers::*;
pub use fixtures::*; 