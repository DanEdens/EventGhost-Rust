use windows::Win32::Foundation::{HWND, LPARAM, WPARAM, LRESULT, HINSTANCE};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::PCSTR;
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
        if RegisterClassA(&wc) == 0 {
            return Err(Error::Win32("Failed to register window class".into()));
        }
    }

    Ok(())
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
        return Err(Error::Win32("Failed to create window".into()));
    }

    Ok(hwnd)
}

pub fn show_window(hwnd: HWND, cmd_show: SHOW_WINDOW_CMD) {
    unsafe {
        ShowWindow(hwnd, cmd_show);
        UpdateWindow(hwnd);
    }
} 