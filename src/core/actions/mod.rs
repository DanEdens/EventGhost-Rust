pub mod flow;
pub mod system;

pub use flow::DelayAction;
pub use flow::ConditionalAction;
pub use flow::{WhileLoopAction, ForLoopAction};
pub use system::RunCommandAction;
pub use system::FileOperationsAction;
pub use system::RegistryOperationsAction;
pub use system::WindowActionsAction;
pub use system::SendKeysAction;

#[cfg(test)]
mod tests; 
