use windows::Win32::Foundation::{HWND, LPARAM, WPARAM, LRESULT, HINSTANCE};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::PCSTR;
use crate::core::Error;

pub mod utils;

/// Window procedure callback type
pub type WindowProc = unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT;

/// Register a window class with the given name and window procedure
pub fn register_window_class(class_name: &str, window_proc: WindowProc) -> Result<(), Error> {
    let class_name = format!("{}\0", class_name);
    let wc = WNDCLASSA {
        lpfnWndProc: Some(window_proc),
        hInstance: HINSTANCE(0), // Current module
        lpszClassName: PCSTR::from_raw(class_name.as_ptr()),
        style: CS_HREDRAW | CS_VREDRAW,
        ..Default::default()
    };

    unsafe {
        if RegisterClassA(&wc).0 == 0 {
            Err(Error::Win32("Failed to register window class".into()))
        } else {
            Ok(())
        }
    }
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
    let class_name = format!("{}\0", class_name);
    let window_name = format!("{}\0", window_name);

    unsafe {
        let hwnd = CreateWindowExA(
            WINDOW_EX_STYLE(0),
            PCSTR::from_raw(class_name.as_ptr()),
            PCSTR::from_raw(window_name.as_ptr()),
            WINDOW_STYLE(style),
            x,
            y,
            width,
            height,
            parent.unwrap_or(HWND(0)),
            None,
            HINSTANCE(0),
            None,
        );

        if hwnd.0 == 0 {
            Err(Error::Win32("Failed to create window".into()))
        } else {
            Ok(hwnd)
        }
    }
}

/// Send a message to a window
pub fn send_message(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    todo!()
}

/// Post a message to a window's message queue
pub fn post_message(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> Result<(), Error> {
    todo!()
} 