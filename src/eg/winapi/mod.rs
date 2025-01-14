use windows::Win32::Foundation::{HWND, LPARAM, WPARAM, LRESULT};
use windows::Win32::UI::WindowsAndMessaging::*;
use crate::core::Error;

pub mod utils;

/// Window procedure callback type
pub type WindowProc = unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT;

/// Register a window class with the given name and window procedure
pub fn register_window_class(class_name: &str, window_proc: WindowProc) -> Result<(), Error> {
    todo!()
}

/// Create a window with the given class name and window name
pub fn create_window(
    class_name: &str,
    window_name: &str,
    style: u32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    parent: Option<HWND>,
) -> Result<HWND, Error> {
    todo!()
}

/// Send a message to a window
pub fn send_message(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    todo!()
}

/// Post a message to a window's message queue
pub fn post_message(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> Result<(), Error> {
    todo!()
} 