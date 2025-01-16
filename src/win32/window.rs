use std::mem;
use std::ptr;
use std::ffi::c_void;
use windows::Win32::Foundation::{
    HWND, BOOL, LPARAM, WPARAM, RECT, POINT,
    GetLastError, HANDLE,
};
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, FindWindowExW,
    GetWindowTextW, GetClassNameW,
    GetWindowRect, IsWindowVisible, IsWindowEnabled,
    SetForegroundWindow, ShowWindow, MoveWindow,
    SendMessageW, PostMessageW,
    GetForegroundWindow, GetDesktopWindow,
    EnumWindows, EnumChildWindows,
    SW_SHOW, SW_HIDE, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE,
};
use windows::Win32::System::Threading::{
    GetWindowThreadProcessId,
};
use windows::core::{PCWSTR, Error as WinError};

use super::error::Error;
use super::traits::{WindowInfo, WindowOps, WindowManager};

pub struct RealWindow {
    handle: HWND,
}

impl RealWindow {
    pub fn new(handle: HWND) -> Self {
        Self { handle }
    }

    fn get_window_text(&self) -> Result<String, Error> {
        let mut text = [0u16; 512];
        unsafe {
            let len = GetWindowTextW(self.handle, &mut text);
            if len == 0 && GetLastError() != 0 {
                return Err(Error::Windows(io::Error::last_os_error()));
            }
            Ok(String::from_utf16_lossy(&text[..len as usize]))
        }
    }

    fn get_class_name(&self) -> Result<String, Error> {
        let mut class_name = [0u16; 256];
        unsafe {
            let len = GetClassNameW(self.handle, &mut class_name);
            if len == 0 {
                return Err(Error::Windows(io::Error::last_os_error()));
            }
            Ok(String::from_utf16_lossy(&class_name[..len as usize]))
        }
    }
}

impl WindowOps for RealWindow {
    fn get_info(&self) -> Result<WindowInfo, Error> {
        let mut rect = RECT::default();
        let mut pid = 0u32;

        unsafe {
            if !GetWindowRect(self.handle, &mut rect).as_bool() {
                return Err(Error::Windows(io::Error::last_os_error()));
            }
            GetWindowThreadProcessId(self.handle, Some(&mut pid));
        }

        Ok(WindowInfo {
            handle: self.handle,
            title: self.get_window_text()?,
            class_name: self.get_class_name()?,
            rect,
            visible: unsafe { IsWindowVisible(self.handle).as_bool() },
            enabled: unsafe { IsWindowEnabled(self.handle).as_bool() },
            pid,
        })
    }

    fn set_foreground(&self) -> Result<(), Error> {
        unsafe {
            if !SetForegroundWindow(self.handle).as_bool() {
                return Err(Error::Windows(io::Error::last_os_error()));
            }
        }
        Ok(())
    }

    fn show(&self) -> Result<(), Error> {
        unsafe {
            ShowWindow(self.handle, SW_SHOW);
        }
        Ok(())
    }

    fn hide(&self) -> Result<(), Error> {
        unsafe {
            ShowWindow(self.handle, SW_HIDE);
        }
        Ok(())
    }

    fn maximize(&self) -> Result<(), Error> {
        unsafe {
            ShowWindow(self.handle, SW_MAXIMIZE);
        }
        Ok(())
    }

    fn minimize(&self) -> Result<(), Error> {
        unsafe {
            ShowWindow(self.handle, SW_MINIMIZE);
        }
        Ok(())
    }

    fn restore(&self) -> Result<(), Error> {
        unsafe {
            ShowWindow(self.handle, SW_RESTORE);
        }
        Ok(())
    }

    fn move_window(&self, x: i32, y: i32, width: i32, height: i32) -> Result<(), Error> {
        unsafe {
            if !MoveWindow(self.handle, x, y, width, height, true).as_bool() {
                return Err(Error::Windows(io::Error::last_os_error()));
            }
        }
        Ok(())
    }

    fn send_message(&self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> Result<isize, Error> {
        unsafe {
            Ok(SendMessageW(self.handle, msg, wparam, lparam))
        }
    }

    fn post_message(&self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> Result<(), Error> {
        unsafe {
            if !PostMessageW(self.handle, msg, wparam, lparam).as_bool() {
                return Err(Error::Windows(io::Error::last_os_error()));
            }
        }
        Ok(())
    }
}

pub struct RealWindowManager;

impl RealWindowManager {
    pub fn new() -> Self {
        Self
    }
}

impl WindowManager for RealWindowManager {
    type Window = RealWindow;

    fn find_window(&self, class_name: Option<&str>, window_name: Option<&str>) -> Result<Option<Self::Window>, Error> {
        unsafe {
            let class = class_name.map(|s| s.encode_utf16().collect::<Vec<_>>());
            let title = window_name.map(|s| s.encode_utf16().collect::<Vec<_>>());
            
            let hwnd = FindWindowW(
                class.as_ref().map(|s| PCWSTR::from_raw(s.as_ptr())).unwrap_or(PCWSTR::null()),
                title.as_ref().map(|s| PCWSTR::from_raw(s.as_ptr())).unwrap_or(PCWSTR::null()),
            );
            
            if hwnd.0 == 0 {
                Ok(None)
            } else {
                Ok(Some(RealWindow::new(hwnd)))
            }
        }
    }

    fn find_window_ex(&self, parent: Option<HWND>, child: Option<HWND>, class: Option<&str>, title: Option<&str>) -> Result<Option<Self::Window>, Error> {
        unsafe {
            let class_str = class.map(|s| s.encode_utf16().collect::<Vec<_>>());
            let title_str = title.map(|s| s.encode_utf16().collect::<Vec<_>>());
            
            let hwnd = FindWindowExW(
                parent.unwrap_or(HWND(0)),
                child.unwrap_or(HWND(0)),
                class_str.as_ref().map(|s| PCWSTR::from_raw(s.as_ptr())).unwrap_or(PCWSTR::null()),
                title_str.as_ref().map(|s| PCWSTR::from_raw(s.as_ptr())).unwrap_or(PCWSTR::null()),
            );
            
            if hwnd.0 == 0 {
                Ok(None)
            } else {
                Ok(Some(RealWindow::new(hwnd)))
            }
        }
    }

    fn enum_windows(&self) -> Result<Vec<Self::Window>, Error> {
        unsafe {
            let mut windows = Vec::new();
            let windows_ptr = &mut windows as *mut Vec<RealWindow> as *mut c_void;
            
            EnumWindows(Some(enum_window_proc), LPARAM(windows_ptr as isize));
            
            Ok(windows)
        }
    }

    fn enum_child_windows(&self, parent: HWND) -> Result<Vec<Self::Window>, Error> {
        unsafe {
            let mut windows = Vec::new();
            let windows_ptr = &mut windows as *mut Vec<RealWindow> as *mut c_void;
            
            EnumChildWindows(parent, Some(enum_window_proc), LPARAM(windows_ptr as isize));
            
            Ok(windows)
        }
    }

    fn get_foreground_window(&self) -> Result<Option<Self::Window>, Error> {
        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd.0 == 0 {
                Ok(None)
            } else {
                Ok(Some(RealWindow::new(hwnd)))
            }
        }
    }

    fn get_desktop_window(&self) -> Result<Self::Window, Error> {
        unsafe {
            Ok(RealWindow::new(GetDesktopWindow()))
        }
    }
}

unsafe extern "system" fn enum_window_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let windows = &mut *(lparam.0 as *mut Vec<RealWindow>);
    windows.push(RealWindow::new(hwnd));
    BOOL(1)
} 