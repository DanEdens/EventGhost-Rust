use std::sync::Arc;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM, HMODULE, BOOL};
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowsHookExW, UnhookWindowsHookEx, CallNextHookEx,
    WH_CALLWNDPROC, WH_SHELL, WH_CBT,
    HHOOK, CWPSTRUCT, CBT_CREATEWND, HSHELL_WINDOWCREATED,
    HCBT_ACTIVATE, HCBT_DESTROYWND,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

use super::error::Error;
use super::traits::{WindowEventHook, WindowEventCallback, WindowHookFactory};

pub struct RealWindowHook {
    hook: HHOOK,
    callback: Arc<WindowEventCallback>,
}

impl RealWindowHook {
    fn new(hook_id: i32, callback: WindowEventCallback) -> Result<Self, Error> {
        unsafe {
            let module = GetModuleHandleW(None)?;
            let callback = Arc::new(callback);
            let callback_ptr = Arc::clone(&callback);
            
            let hook = SetWindowsHookExW(
                hook_id,
                Some(hook_proc),
                HMODULE(0),
                0,
            );

            if hook.0 == 0 {
                return Err(Error::Windows(std::io::Error::last_os_error()));
            }

            Ok(Self {
                hook,
                callback: callback_ptr,
            })
        }
    }
}

impl WindowEventHook for RealWindowHook {
    fn start(&mut self) -> Result<(), Error> {
        // Hook is started on creation
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Error> {
        unsafe {
            if !UnhookWindowsHookEx(self.hook).as_bool() {
                return Err(Error::Windows(std::io::Error::last_os_error()));
            }
        }
        Ok(())
    }

    fn is_active(&self) -> bool {
        self.hook.0 != 0
    }
}

impl Drop for RealWindowHook {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

pub struct RealHookFactory;

impl RealHookFactory {
    pub fn new() -> Self {
        Self
    }
}

impl WindowHookFactory for RealHookFactory {
    type Hook = RealWindowHook;

    fn create_wnd_proc_hook(&self, callback: WindowEventCallback) -> Result<Self::Hook, Error> {
        RealWindowHook::new(WH_CALLWNDPROC, callback)
    }

    fn create_shell_hook(&self, callback: WindowEventCallback) -> Result<Self::Hook, Error> {
        RealWindowHook::new(WH_SHELL, callback)
    }

    fn create_foreground_hook(&self, callback: WindowEventCallback) -> Result<Self::Hook, Error> {
        RealWindowHook::new(WH_CBT, callback)
    }
}

unsafe extern "system" fn hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LPARAM {
    // Process hook based on type
    match code {
        // Window procedure hooks
        WH_CALLWNDPROC => {
            let cwp = &*(lparam.0 as *const CWPSTRUCT);
            // Handle window procedure message
        }
        
        // Shell hooks
        WH_SHELL => {
            match wparam.0 as u32 {
                HSHELL_WINDOWCREATED => {
                    let hwnd = HWND(lparam.0);
                    // Handle window creation
                }
                _ => {}
            }
        }
        
        // CBT hooks
        WH_CBT => {
            match wparam.0 as i32 {
                HCBT_ACTIVATE => {
                    let hwnd = HWND(lparam.0);
                    // Handle window activation
                }
                HCBT_DESTROYWND => {
                    let hwnd = HWND(lparam.0);
                    // Handle window destruction
                }
                _ => {}
            }
        }
        
        _ => {}
    }

    // Call next hook in chain
    CallNextHookEx(HHOOK(0), code, wparam, lparam)
} 