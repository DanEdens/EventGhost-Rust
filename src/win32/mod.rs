use windows::Win32::Foundation::{HWND, LPARAM, WPARAM, LRESULT, HINSTANCE};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::{PCSTR, Error as WindowsError};
use crate::core;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Windows API error: {0}")]
    Win32(String),
}

impl From<WindowsError> for Error {
    fn from(err: WindowsError) -> Self {
        Error::Win32(err.to_string())
    }
}

impl From<core::Error> for Error {
    fn from(err: core::Error) -> Self {
        match err {
            core::Error::Win32(err) => Error::Win32(err.to_string()),
            _ => Error::Win32(err.to_string()),
        }
    }
}

pub mod window;
pub mod utils;

pub use window::*;
pub use utils::*; 