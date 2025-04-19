// System actions module
// Contains actions for interacting with the operating system

pub mod run_command;
pub mod file_operations;
pub mod registry_operations;
pub mod window_actions;

pub use run_command::RunCommandAction;
pub use file_operations::FileOperationsAction;
pub use registry_operations::RegistryOperationsAction;
pub use window_actions::WindowActionsAction; 
