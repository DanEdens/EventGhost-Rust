use windows::Win32::Foundation::{HWND, HINSTANCE, WPARAM, LPARAM};
use windows::Win32::UI::Controls::{
    LVS_EX_DOUBLEBUFFER, LVS_EX_FULLROWSELECT, LVS_EX_GRIDLINES,
    LVCF_TEXT, LVCF_WIDTH, LVM_INSERTCOLUMNA,
};
use windows::Win32::UI::WindowsAndMessaging::{WS_CHILD, WS_VISIBLE, ShowWindow, SW_SHOW, SW_HIDE, SendMessageA};
use chrono::{DateTime, Local};
use crate::core::Error;
use crate::win32;
use super::UIComponent;

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Local>,
    pub level: LogLevel,
    pub message: String,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

pub struct LogCtrl {
    hwnd: HWND,
    parent: HWND,
    instance: HINSTANCE,
    is_visible: bool,
    max_entries: usize,
}

impl LogCtrl {
    pub fn new(parent: HWND, instance: HINSTANCE) -> Result<Self, Error> {
        Ok(Self {
            hwnd: HWND::default(),
            parent,
            instance,
            is_visible: false,
            max_entries: 1000, // Default max entries
        })
    }

    pub fn initialize(&mut self) -> Result<(), Error> {
        // Create the list view control window
        let hwnd = win32::create_window(
            "SysListView32\0",
            "",
            WS_CHILD | WS_VISIBLE | LVS_REPORT | LVS_NOSORTHEADER | LVS_SHOWSELALWAYS,
            0,
            0,
            0,
            0,
            Some(self.parent),
            self.instance,
        )?;

        self.hwnd = hwnd;
        self.is_visible = true;

        // Set extended list view styles
        unsafe {
            SendMessageA(
                self.hwnd,
                LVM_SETEXTENDEDLISTVIEWSTYLE,
                WPARAM(0),
                LPARAM((LVS_EX_FULLROWSELECT | LVS_EX_GRIDLINES | LVS_EX_DOUBLEBUFFER) as isize),
            );

            // Add columns
            let mut lvc = LVCOLUMNA::default();
            lvc.mask = LVCF_TEXT | LVCF_WIDTH;
            
            // Time column
            lvc.pszText = "Time\0".as_ptr() as *mut i8;
            lvc.cx = 100;
            SendMessageA(self.hwnd, LVM_INSERTCOLUMNA, WPARAM(0), LPARAM(&lvc as *const _ as isize));

            // Level column
            lvc.pszText = "Level\0".as_ptr() as *mut i8;
            lvc.cx = 60;
            SendMessageA(self.hwnd, LVM_INSERTCOLUMNA, WPARAM(1), LPARAM(&lvc as *const _ as isize));

            // Message column
            lvc.pszText = "Message\0".as_ptr() as *mut i8;
            lvc.cx = 400;
            SendMessageA(self.hwnd, LVM_INSERTCOLUMNA, WPARAM(2), LPARAM(&lvc as *const _ as isize));

            // Source column
            lvc.pszText = "Source\0".as_ptr() as *mut i8;
            lvc.cx = 100;
            SendMessageA(self.hwnd, LVM_INSERTCOLUMNA, WPARAM(3), LPARAM(&lvc as *const _ as isize));
        }

        Ok(())
    }

    pub fn add_entry(&mut self, entry: LogEntry) -> Result<(), Error> {
        todo!()
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub fn set_max_entries(&mut self, max: usize) {
        todo!()
    }

    pub fn get_entry(&self, index: usize) -> Option<LogEntry> {
        todo!()
    }

    pub fn get_entries(&self) -> Vec<LogEntry> {
        todo!()
    }

    pub fn set_font(&mut self, font_name: &str, size: i32) -> Result<(), Error> {
        todo!()
    }
}

impl UIComponent for LogCtrl {
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