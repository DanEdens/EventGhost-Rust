use windows::Win32::Foundation::{HWND, LPARAM, WPARAM, LRESULT, HINSTANCE};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::PCSTR;

use crate::win32::{self, Error as Win32Error};
use super::tree_ctrl::TreeCtrl;
use super::log_ctrl::LogCtrl;
use super::status_bar::StatusBar;
use super::toolbar::Toolbar;

const MAIN_WINDOW_CLASS: &[u8] = b"EventGhostMainWindow\0";

pub struct MainFrame {
    hwnd: HWND,
    tree_ctrl: TreeCtrl,
    log_ctrl: LogCtrl,
    status_bar: StatusBar,
    toolbar: Toolbar,
    is_visible: bool,
}

impl MainFrame {
    pub fn new(instance: HINSTANCE) -> Result<Self, Win32Error> {
        // Register window class
        win32::register_window_class(
            PCSTR::from_raw(MAIN_WINDOW_CLASS.as_ptr()),
            Some(Self::window_proc),
            instance,
        )?;

        // Create main window
        let hwnd = win32::create_window(
            PCSTR::from_raw(MAIN_WINDOW_CLASS.as_ptr()),
            PCSTR::from_raw(b"EventGhost\0".as_ptr()),
            WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            600,
            None,
            instance,
        )?;

        let mut frame = Self {
            hwnd,
            tree_ctrl: TreeCtrl::new(hwnd, instance)?,
            log_ctrl: LogCtrl::new(hwnd, instance)?,
            status_bar: StatusBar::new(hwnd, instance)?,
            toolbar: Toolbar::new(hwnd, instance)?,
            is_visible: false,
        };

        frame.initialize()?;
        Ok(frame)
    }

    fn initialize(&mut self) -> Result<(), Win32Error> {
        // Initialize child controls
        self.tree_ctrl.initialize()?;
        self.log_ctrl.initialize()?;
        self.status_bar.initialize()?;
        self.toolbar.initialize()?;

        // Layout controls
        self.layout_controls();

        Ok(())
    }

    fn layout_controls(&mut self) {
        // TODO: Implement control layout
    }

    pub fn show(&mut self) {
        win32::show_window(self.hwnd, SW_SHOW);
        self.is_visible = true;
    }

    pub fn hide(&mut self) {
        win32::show_window(self.hwnd, SW_HIDE);
        self.is_visible = false;
    }

    pub fn set_title(&self, title: &str) -> Result<(), Win32Error> {
        win32::set_window_text(self.hwnd, title)
    }

    unsafe extern "system" fn window_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_DESTROY => {
                win32::post_message(hwnd, WM_QUIT, WPARAM(0), LPARAM(0))
                    .expect("Failed to post quit message");
                LRESULT(0)
            }
            WM_SIZE => {
                // TODO: Handle window resizing
                LRESULT(0)
            }
            _ => unsafe { DefWindowProcA(hwnd, msg, wparam, lparam) }
        }
    }
}

impl Drop for MainFrame {
    fn drop(&mut self) {
        // Child controls will be destroyed automatically when parent window is destroyed
        unsafe {
            DestroyWindow(self.hwnd);
        }
    }
} 