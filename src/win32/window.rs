use windows::Win32::Foundation::{HWND, LPARAM, WPARAM, LRESULT, HINSTANCE};
use windows::Win32::UI::WindowsAndMessaging::{WNDCLASSA, *};
use windows::core::{PCSTR, Error as WindowsError};
use super::Error;

pub fn register_window_class(
    class_name: PCSTR,
    window_proc: WNDPROC,
    instance: HINSTANCE,
) -> Result<(), Error> {
    let wc = WNDCLASSA {
        lpfnWndProc: window_proc,
        hInstance: instance,
        lpszClassName: class_name,
        style: CS_HREDRAW | CS_VREDRAW,
        ..Default::default()
    };

    unsafe {
        RegisterClassA(&wc).map_err(|e| Error::Win32(format!("Failed to register window class: {}", e)))
    }
}

pub fn create_window(
    class_name: PCSTR,
    window_name: PCSTR,
    style: WINDOW_STYLE,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    parent: Option<HWND>,
    instance: HINSTANCE,
) -> Result<HWND, Error> {
    let hwnd = unsafe {
        CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            class_name,
            window_name,
            style,
            x,
            y,
            width,
            height,
            parent.unwrap_or(HWND(0)),
            None,
            instance,
            None,
        )
    };

    if hwnd.0 == 0 {
        Err(Error::Win32("Failed to create window".into()))
    } else {
        Ok(hwnd)
    }
}

pub fn show_window(hwnd: HWND, cmd_show: SHOW_WINDOW_CMD) -> Result<(), Error> {
    unsafe {
        ShowWindow(hwnd, cmd_show);
        UpdateWindow(hwnd).map_err(|e| Error::Win32(format!("Failed to update window: {}", e)))
    }
} 