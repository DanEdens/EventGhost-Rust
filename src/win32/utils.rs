use windows::Win32::Foundation::{HWND, LPARAM, WPARAM, LRESULT};
use windows::Win32::UI::WindowsAndMessaging::*;
use super::Error;

pub fn send_message(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        SendMessageA(hwnd, msg, wparam, lparam)
    }
}

pub fn post_message(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> Result<(), Error> {
    unsafe {
        if PostMessageA(hwnd, msg, wparam, lparam).0 == 0 {
            return Err(Error::Win32("Failed to post message".into()));
        }
    }
    Ok(())
}

pub fn get_window_text(hwnd: HWND) -> Result<String, Error> {
    let mut text = [0u8; 512];
    let len = unsafe {
        GetWindowTextA(hwnd, &mut text)
    };
    
    if len == 0 {
        return Err(Error::Win32("Failed to get window text".into()));
    }

    Ok(String::from_utf8_lossy(&text[..len as usize]).into_owned())
}

pub fn set_window_text(hwnd: HWND, text: &str) -> Result<(), Error> {
    unsafe {
        if SetWindowTextA(hwnd, text).0 == 0 {
            return Err(Error::Win32("Failed to set window text".into()));
        }
    }
    Ok(())
} 