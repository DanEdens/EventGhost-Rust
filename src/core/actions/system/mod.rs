// System actions module
// Contains actions for interacting with the operating system

pub mod run_command;
pub mod file_operations;
pub mod registry_operations;
pub mod window_actions;
pub mod send_keys_action;
pub mod mouse_control_action;
pub mod mouse_recorder_action;

pub use run_command::RunCommandAction;
pub use file_operations::FileOperationsAction;
pub use registry_operations::RegistryOperationsAction;
pub use window_actions::WindowActionsAction;
pub use send_keys_action::SendKeysAction;
pub use mouse_control_action::MouseControlAction;
pub use mouse_recorder_action::MouseRecorderAction; 
