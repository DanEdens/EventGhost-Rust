use windows::Win32::Foundation::HWND;
use crate::core::Error;
use crate::eg::winapi;
use super::{UIComponent, TreeCtrl, LogCtrl, StatusBar, Toolbar};

pub struct MainFrame {
    hwnd: HWND,
    tree_ctrl: TreeCtrl,
    log_ctrl: LogCtrl,
    status_bar: StatusBar,
    toolbar: Toolbar,
    is_visible: bool,
}

impl MainFrame {
    pub fn new() -> Result<Self, Error> {
        todo!()
    }

    pub fn initialize(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub fn process_message(&mut self, msg: u32, wparam: usize, lparam: isize) -> Result<(), Error> {
        todo!()
    }

    pub fn create_window(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub fn set_title(&mut self, title: &str) -> Result<(), Error> {
        todo!()
    }

    pub fn get_tree_ctrl(&self) -> &TreeCtrl {
        &self.tree_ctrl
    }

    pub fn get_tree_ctrl_mut(&mut self) -> &mut TreeCtrl {
        &mut self.tree_ctrl
    }

    pub fn get_log_ctrl(&self) -> &LogCtrl {
        &self.log_ctrl
    }

    pub fn get_log_ctrl_mut(&mut self) -> &mut LogCtrl {
        &mut self.log_ctrl
    }
}

impl UIComponent for MainFrame {
    fn get_hwnd(&self) -> HWND {
        self.hwnd
    }

    fn show(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn hide(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }
} 