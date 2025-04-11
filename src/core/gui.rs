use windows::Win32::Foundation::HWND;
use crate::core::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DialogResult {
    Ok,
    Cancel,
    Yes,
    No,
    Custom(i32),
}

pub trait UIComponent {
    fn get_hwnd(&self) -> HWND;
    fn show(&mut self) -> Result<(), Error>;
    fn hide(&mut self) -> Result<(), Error>;
    fn is_visible(&self) -> bool;
    fn handle_command(&mut self, cmd: u32) -> Result<(), Error>;
}

pub trait Dialog: UIComponent {
    fn show_modal(&mut self) -> Result<DialogResult, Error>;
    fn end_dialog(&mut self, result: DialogResult);
    fn on_init(&mut self) -> Result<(), Error>;
    fn on_command(&mut self, command: u32) -> Result<(), Error>;
}

pub trait Window: UIComponent {
    fn set_title(&mut self, title: &str) -> Result<(), Error>;
    fn get_title(&self) -> String;
    fn set_position(&mut self, x: i32, y: i32) -> Result<(), Error>;
    fn set_size(&mut self, width: i32, height: i32) -> Result<(), Error>;
    fn get_position(&self) -> (i32, i32);
    fn get_size(&self) -> (i32, i32);
    fn center(&mut self) -> Result<(), Error>;
}

pub trait Control: UIComponent {
    fn get_parent(&self) -> HWND;
    fn set_enabled(&mut self, enabled: bool) -> Result<(), Error>;
    fn is_enabled(&self) -> bool;
    fn set_focus(&mut self) -> Result<(), Error>;
    fn has_focus(&self) -> bool;
} 
