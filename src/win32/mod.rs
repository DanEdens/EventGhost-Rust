use windows::Win32::Foundation::{HWND, LPARAM, WPARAM, LRESULT, HINSTANCE};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::PCSTR;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Windows API error: {0}")]
    Win32(String),
}

pub mod window;
pub mod utils;

pub use window::*;
pub use utils::*; 