pub mod main_frame;
pub mod tree_ctrl;
pub mod log_ctrl;
pub mod guid;
pub mod status_bar;
pub mod toolbar;

pub use main_frame::MainFrame;
pub use tree_ctrl::TreeCtrl;
pub use log_ctrl::LogCtrl;
pub use guid::GUID;
pub use status_bar::StatusBar;
pub use toolbar::Toolbar;

use crate::core::Error;
use windows::Win32::Foundation::HWND;

/// Base trait for UI components
pub trait UIComponent {
    fn get_hwnd(&self) -> HWND;
    fn show(&mut self) -> Result<(), Error>;
    fn hide(&mut self) -> Result<(), Error>;
    fn is_visible(&self) -> bool;
} 