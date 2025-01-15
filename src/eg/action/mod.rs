//! Action system for EventGhost
//! 
//! This module provides the core action system functionality including:
//! - Base action traits and types
//! - Action groups for organizing actions
//! - Common action implementations
//! - Action item builder

pub mod base;
pub mod group;
pub mod item;
pub mod common;

pub use base::{ActionBase, ActionInfo, ActionError};
pub use group::ActionGroup;
pub use item::ActionItem;
pub use common::{
    shell_command_action,
    delay_action,
    generate_event_action,
    conditional_action,
    repeat_action,
};

#[cfg(test)]
mod tests; 