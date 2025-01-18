use windows::Win32::Foundation::{HWND, LPARAM, WPARAM, LRESULT, RECT, POINT};
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

pub fn get_window_rect(hwnd: HWND) -> Result<RECT, Error> {
    let mut rect = RECT::default();
    unsafe {
        GetWindowRect(hwnd, &mut rect)
            .map_err(|e| Error::Win32(format!("Failed to get window rect: {}", e)))?;
    }
    Ok(rect)
}

pub fn get_client_rect(hwnd: HWND) -> Result<RECT, Error> {
    let mut rect = RECT::default();
    unsafe {
        GetClientRect(hwnd, &mut rect)
            .map_err(|e| Error::Win32(format!("Failed to get client rect: {}", e)))?;
    }
    Ok(rect)
}

pub fn screen_to_client(hwnd: HWND, point: POINT) -> Result<POINT, Error> {
    let mut pt = point;
    unsafe {
        ScreenToClient(hwnd, &mut pt)
            .map_err(|e| Error::Win32(format!("Failed to convert screen to client coordinates: {}", e)))?;
    }
    Ok(pt)
}

pub fn client_to_screen(hwnd: HWND, point: POINT) -> Result<POINT, Error> {
    let mut pt = point;
    unsafe {
        ClientToScreen(hwnd, &mut pt)
            .map_err(|e| Error::Win32(format!("Failed to convert client to screen coordinates: {}", e)))?;
    }
    Ok(pt)
}

pub fn is_window_visible(hwnd: HWND) -> bool {
    unsafe { IsWindowVisible(hwnd).as_bool() }
}

pub fn get_parent(hwnd: HWND) -> Option<HWND> {
    unsafe {
        let parent = GetParent(hwnd);
        if parent.0 == 0 {
            None
        } else {
            Some(parent)
        }
    }
}

pub fn get_class_name(hwnd: HWND) -> Result<String, Error> {
    let mut name = [0u8; 256];
    let len = unsafe {
        GetClassNameA(hwnd, &mut name)
            .map_err(|e| Error::Win32(format!("Failed to get class name: {}", e)))?
    };
    
    if len == 0 {
        return Err(Error::Win32("Class name is empty".into()));
    }

    Ok(String::from_utf8_lossy(&name[..len as usize]).into_owned())
}

pub fn find_window(class_name: Option<&str>, window_name: Option<&str>) -> Result<Option<HWND>, Error> {
    let class_name = class_name.map(|s| format!("{}\0", s));
    let window_name = window_name.map(|s| format!("{}\0", s));
    
    let hwnd = unsafe {
        FindWindowA(
            class_name.as_ref().map(|s| PCSTR::from_raw(s.as_ptr())).unwrap_or(PCSTR::null()),
            window_name.as_ref().map(|s| PCSTR::from_raw(s.as_ptr())).unwrap_or(PCSTR::null()),
        )
    };

    if hwnd.0 == 0 {
        Ok(None)
    } else {
        Ok(Some(hwnd))
    }
}

pub fn enumerate_child_windows(
    parent: HWND,
    mut callback: impl FnMut(HWND) -> bool
) -> Result<(), Error> {
    unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let callback = &mut *(lparam.0 as *mut Box<dyn FnMut(HWND) -> bool>);
        BOOL(if callback(hwnd) { 1 } else { 0 })
    }

    unsafe {
        let mut boxed_callback = Box::new(Box::new(callback) as Box<dyn FnMut(HWND) -> bool>);
        let lparam = LPARAM((&mut boxed_callback as *mut Box<dyn FnMut(HWND) -> bool>) as isize);
        
        EnumChildWindows(parent, Some(enum_proc), lparam)
            .map_err(|e| Error::Win32(format!("Failed to enumerate child windows: {}", e)))?;
    }
    
    Ok(())
} 