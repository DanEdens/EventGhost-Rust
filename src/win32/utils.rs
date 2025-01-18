use windows::Win32::Foundation::{HWND, LPARAM, WPARAM, LRESULT};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::{PCSTR, Error as WindowsError};
use super::Error;

pub fn send_message(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> Result<LRESULT, Error> {
    unsafe {
        SendMessageA(hwnd, msg, wparam, lparam).map_err(|e| Error::Win32(format!("Failed to send message: {}", e)))
    }
}

pub fn post_message(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> Result<(), Error> {
    unsafe {
        PostMessageA(hwnd, msg, wparam, lparam).map_err(|e| Error::Win32(format!("Failed to post message: {}", e)))
    }
}

pub fn get_window_text(hwnd: HWND) -> Result<String, Error> {
    let mut text = [0u8; 512];
    let len = unsafe {
        GetWindowTextA(hwnd, &mut text).map_err(|e| Error::Win32(format!("Failed to get window text: {}", e)))?
    };
    
    if len == 0 {
        return Err(Error::Win32("Window text is empty".into()));
    }

    Ok(String::from_utf8_lossy(&text[..len as usize]).into_owned())
}

pub fn set_window_text(hwnd: HWND, text: &str) -> Result<(), Error> {
    // Convert &str to null-terminated string
    let text = format!("{}\0", text);
    unsafe {
        SetWindowTextA(hwnd, PCSTR::from_raw(text.as_ptr()))
            .map_err(|e| Error::Win32(format!("Failed to set window text: {}", e)))
    }
} 