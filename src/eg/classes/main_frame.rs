use windows::Win32::Foundation::{HWND, LPARAM, WPARAM, LRESULT, HINSTANCE, RECT};
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
        // Get the client area dimensions
        let mut client_rect = RECT::default();
        unsafe {
            GetClientRect(self.hwnd, &mut client_rect);
        }

        let width = client_rect.right - client_rect.left;
        let height = client_rect.bottom - client_rect.top;

        // Toolbar height (typical height is around 30 pixels)
        let toolbar_height = 30;

        // Status bar height (typical height is around 22 pixels)
        let status_bar_height = 22;

        // Layout toolbar at the top
        unsafe {
            MoveWindow(
                self.toolbar.get_hwnd(), 
                0, 
                0, 
                width, 
                toolbar_height, 
                true
            );
        }

        // Layout status bar at the bottom
        unsafe {
            MoveWindow(
                self.status_bar.get_hwnd(), 
                0, 
                height - status_bar_height, 
                width, 
                status_bar_height, 
                true
            );
        }

        // Layout tree control on the left side (1/3 of the width)
        let tree_width = width / 3;
        unsafe {
            MoveWindow(
                self.tree_ctrl.get_hwnd(), 
                0, 
                toolbar_height, 
                tree_width, 
                height - toolbar_height - status_bar_height, 
                true
            );
        }

        // Layout log control on the right side (2/3 of the width)
        unsafe {
            MoveWindow(
                self.log_ctrl.get_hwnd(), 
                tree_width, 
                toolbar_height, 
                width - tree_width, 
                height - toolbar_height - status_bar_height, 
                true
            );
        }
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