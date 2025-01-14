use windows::Win32::Foundation::HWND;
use crate::core::Error;
use super::UIComponent;

/// Base dialog trait
pub trait Dialog: UIComponent {
    fn show_modal(&mut self) -> Result<DialogResult, Error>;
    fn end_dialog(&mut self, result: DialogResult);
    fn on_init(&mut self) -> Result<(), Error>;
    fn on_command(&mut self, command: u32) -> Result<(), Error>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DialogResult {
    Ok,
    Cancel,
    Yes,
    No,
    Custom(i32),
}

/// File dialog options
#[derive(Debug, Clone)]
pub struct FileDialogOptions {
    pub title: String,
    pub filter: String,
    pub default_ext: String,
    pub initial_dir: String,
    pub file_name: String,
    pub multi_select: bool,
}

/// Common dialogs implementation
pub struct CommonDialogs;

impl CommonDialogs {
    /// Show an open file dialog
    pub fn open_file(parent: HWND, options: FileDialogOptions) -> Result<Vec<String>, Error> {
        todo!()
    }

    /// Show a save file dialog
    pub fn save_file(parent: HWND, options: FileDialogOptions) -> Result<Option<String>, Error> {
        todo!()
    }

    /// Show a message box
    pub fn message_box(
        parent: HWND,
        text: &str,
        caption: &str,
        style: MessageBoxStyle,
    ) -> Result<DialogResult, Error> {
        todo!()
    }

    /// Show a color picker dialog
    pub fn color_picker(parent: HWND, initial_color: Option<u32>) -> Result<Option<u32>, Error> {
        todo!()
    }

    /// Show a folder browser dialog
    pub fn browse_folder(
        parent: HWND,
        title: &str,
        initial_dir: Option<&str>,
    ) -> Result<Option<String>, Error> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MessageBoxStyle {
    Ok,
    OkCancel,
    YesNo,
    YesNoCancel,
    Info,
    Warning,
    Error,
    Question,
} 