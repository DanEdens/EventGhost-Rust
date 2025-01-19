use windows::Win32::Foundation::{HWND, HINSTANCE};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::UI::Controls::*;
use crate::core::Error;
use crate::win32;
use super::UIComponent;

#[derive(Debug, Clone)]
pub struct StatusPart {
    pub text: String,
    pub width: i32,
    pub style: StatusPartStyle,
}

#[derive(Debug, Clone, Copy)]
pub enum StatusPartStyle {
    Normal,
    Progress,
    Owner,
}

pub struct StatusBar {
    hwnd: HWND,
    parent: HWND,
    instance: HINSTANCE,
    is_visible: bool,
    parts: Vec<StatusPart>,
}

impl StatusBar {
    pub fn new(parent: HWND, instance: HINSTANCE) -> Result<Self, Error> {
        Ok(Self {
            hwnd: HWND::default(),
            parent,
            instance,
            is_visible: false,
            parts: Vec::new(),
        })
    }

    pub fn initialize(&mut self) -> Result<(), Error> {
        // Create the status bar window
        let hwnd = win32::create_window(
            "msctls_statusbar32\0",
            "",
            WS_CHILD | WS_VISIBLE | SBARS_SIZEGRIP,
            0,
            0,
            0,
            0,
            Some(self.parent),
            self.instance,
        )?;

        self.hwnd = hwnd;
        self.is_visible = true;

        // Set default parts
        let parts = [200, 400, -1]; // -1 means extend to the end
        unsafe {
            SendMessageA(
                self.hwnd,
                SB_SETPARTS,
                WPARAM(parts.len()),
                LPARAM(parts.as_ptr() as isize),
            );

            // Set initial text for each part
            SendMessageA(
                self.hwnd,
                SB_SETTEXTA,
                WPARAM(0),
                LPARAM("Ready\0".as_ptr() as isize),
            );
        }

        Ok(())
    }

    /// Set the text for a specific part
    pub fn set_text(&mut self, part_index: usize, text: &str) -> Result<(), Error> {
        todo!()
    }

    /// Set progress value (0-100) for a progress-style part
    pub fn set_progress(&mut self, part_index: usize, progress: i32) -> Result<(), Error> {
        todo!()
    }

    /// Add a new part to the status bar
    pub fn add_part(&mut self, part: StatusPart) -> Result<(), Error> {
        todo!()
    }

    /// Remove a part from the status bar
    pub fn remove_part(&mut self, part_index: usize) -> Result<(), Error> {
        todo!()
    }

    /// Get the text of a specific part
    pub fn get_text(&self, part_index: usize) -> Option<String> {
        todo!()
    }

    /// Set the minimum height of the status bar
    pub fn set_min_height(&mut self, height: i32) -> Result<(), Error> {
        todo!()
    }

    /// Get the rect of a specific part
    pub fn get_part_rect(&self, part_index: usize) -> Result<Option<windows::Win32::Foundation::RECT>, Error> {
        todo!()
    }
}

impl UIComponent for StatusBar {
    fn get_hwnd(&self) -> HWND {
        self.hwnd
    }

    fn show(&mut self) -> Result<(), Error> {
        unsafe {
            ShowWindow(self.hwnd, SW_SHOW);
        }
        self.is_visible = true;
        Ok(())
    }

    fn hide(&mut self) -> Result<(), Error> {
        unsafe {
            ShowWindow(self.hwnd, SW_HIDE);
        }
        self.is_visible = false;
        Ok(())
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }
} 