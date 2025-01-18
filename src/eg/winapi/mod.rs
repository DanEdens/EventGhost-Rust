use windows::Win32::Foundation::{HWND, LPARAM, WPARAM, LRESULT, HINSTANCE};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::PCSTR;
use crate::core::Error;

// Re-export the window functions from win32 module
pub use win32::{
    register_window_class,
    create_window,
    show_window,
    send_message,
    post_message,
    get_window_text,
    set_window_text,
}; 