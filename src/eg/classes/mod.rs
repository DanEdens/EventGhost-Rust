pub mod drag_drop;
pub mod guid;
pub mod log_ctrl;
pub mod main_frame;
pub mod menu;
pub mod plugin_config;
pub mod property_grid;
pub mod tree_ctrl;
pub mod types;
pub mod status_bar;
pub mod toolbar;
pub mod dialog;

// Re-export common types
pub use types::{DragEffects, PropertyValue, PopupFlags};

// Re-export UI components
pub use main_frame::MainFrame;
pub use tree_ctrl::TreeCtrl;
pub use log_ctrl::LogCtrl;
pub use guid::GUID;
pub use status_bar::StatusBar;
pub use toolbar::Toolbar;
pub use dialog::{Dialog, DialogResult, CommonDialogs, FileDialogOptions, MessageBoxStyle};
pub use menu::{Menu, MenuItem, MenuItemKind};
pub use drag_drop::{DragDropManager, DragData, DragSource, DropTarget};
pub use property_grid::{PropertyGrid, Property, PropertySource};
pub use plugin_config::ConfigDialog;

use crate::core::Error;
use windows::Win32::Foundation::HWND;

/// Base trait for UI components
pub trait UIComponent {
    fn get_hwnd(&self) -> HWND;
    fn show(&mut self) -> Result<(), Error>;
    fn hide(&mut self) -> Result<(), Error>;
    fn is_visible(&self) -> bool;
} 