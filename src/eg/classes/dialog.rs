use gtk::prelude::*;
use gtk::{self, Dialog as GtkDialog, FileChooserDialog, MessageDialog, ColorChooserDialog, ResponseType, FileChooserAction};
use gtk::gdk;
use crate::core::Error;
use super::UIComponent;
use gtk::{PrintOperation, Application};
use std::path::PathBuf;

/// Base dialog trait
pub trait Dialog {
    fn show(&self);
    fn hide(&self);
    fn show_modal(&mut self) -> Result<DialogResult, Error>; 
    fn end_dialog(&mut self, result: DialogResult); 
    fn on_init(&mut self) -> Result<(), Error>; 
    fn on_command(&mut self, _command: u32) -> Result<(), Error>; 
}

pub struct DialogImpl {
    widget: gtk::Dialog
}

impl DialogImpl {
    pub fn new() -> Self {
        Self {
            widget: gtk::Dialog::new()
        }
    }
}

impl Dialog for DialogImpl {
    fn show(&self) {
        self.widget.present();
    }

    fn hide(&self) {
        self.widget.hide();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DialogResult {
    Ok,
    Cancel,
    Yes,
    No,
    None,
}

impl From<ResponseType> for DialogResult {
    fn from(response: ResponseType) -> Self {
        match response {
            ResponseType::Ok => DialogResult::Ok,
            ResponseType::Cancel => DialogResult::Cancel,
            ResponseType::Yes => DialogResult::Yes,
            ResponseType::No => DialogResult::No,
            _ => DialogResult::None,
        }
    }
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
    pub fn open_file(parent: Option<&gtk::Window>, options: FileDialogOptions) -> Result<Vec<String>, Error> {
        let dialog = FileChooserDialog::new(
            Some(&options.title),
            parent,
            FileChooserAction::Open,
            &[("Cancel", ResponseType::Cancel), ("Open", ResponseType::Accept)],
        );
        
        dialog.set_select_multiple(options.multi_select);
        if !options.initial_dir.is_empty() {
            dialog.set_current_folder(Some(&gtk::gio::File::for_path(&options.initial_dir)));
        }
        
        let response = dialog.run();
        let result = if response == ResponseType::Accept {
            dialog.files()
                .iter()
                .filter_map(|f| f.path())
                .filter_map(|p| p.to_str().map(String::from))
                .collect()
        } else {
            Vec::new()
        };
        
        dialog.close();
        Ok(result)
    }

    /// Show a save file dialog
    pub fn save_file(parent: Option<&gtk::Window>, options: FileDialogOptions) -> Result<Option<String>, Error> {
        let dialog = FileChooserDialog::new(
            Some(&options.title),
            parent,
            FileChooserAction::Save,
            &[("Cancel", ResponseType::Cancel), ("Save", ResponseType::Accept)],
        );
        
        if !options.initial_dir.is_empty() {
            dialog.set_current_folder(Some(&gtk::gio::File::for_path(&options.initial_dir)));
        }
        if !options.file_name.is_empty() {
            dialog.set_current_name(&options.file_name);
        }
        
        let response = dialog.run();
        let result = if response == ResponseType::Accept {
            dialog.file()
                .and_then(|f| f.path())
                .and_then(|p| p.to_str().map(String::from))
        } else {
            None
        };
        
        dialog.close();
        Ok(result)
    }

    /// Show a message dialog
    pub fn message_dialog(
        parent: Option<&gtk::Window>,
        text: &str,
        caption: &str,
        style: MessageBoxStyle,
    ) -> Result<DialogResult, Error> {
        let dialog_flags = match style {
            MessageBoxStyle::Info => gtk::MessageType::Info,
            MessageBoxStyle::Warning => gtk::MessageType::Warning,
            MessageBoxStyle::Error => gtk::MessageType::Error,
            MessageBoxStyle::Question => gtk::MessageType::Question,
            _ => gtk::MessageType::Other,
        };
        
        let buttons = match style {
            MessageBoxStyle::Ok => gtk::ButtonsType::Ok,
            MessageBoxStyle::OkCancel => gtk::ButtonsType::OkCancel,
            MessageBoxStyle::YesNo => gtk::ButtonsType::YesNo,
            MessageBoxStyle::YesNoCancel => gtk::ButtonsType::None,
            _ => gtk::ButtonsType::Ok,
        };
        
        let dialog = gtk::MessageDialog::new(
            parent,
            gtk::DialogFlags::MODAL,
            dialog_flags,
            buttons,
            text,
        );
        dialog.set_title(Some(caption));
        
        let response = dialog.run();
        dialog.close();
        
        Ok(response.into())
    }

    /// Show a color picker dialog
    pub fn color_picker(parent: Option<&gtk::Window>, initial_color: Option<gtk::gdk::RGBA>) -> Result<Option<gtk::gdk::RGBA>, Error> {
        let dialog = ColorChooserDialog::new(Some("Choose a color"), parent);
        
        if let Some(color) = initial_color {
            dialog.set_rgba(&color);
        }
        
        let response = dialog.run();
        let result = if response == ResponseType::Ok {
            Some(dialog.rgba())
        } else {
            None
        };
        
        dialog.close();
        Ok(result)
    }

    /// Show a folder browser dialog
    pub fn browse_folder(
        parent: Option<&gtk::Window>,
        title: &str,
        initial_dir: Option<&str>,
    ) -> Result<Option<String>, Error> {
        let dialog = FileChooserDialog::new(
            Some(title),
            parent,
            FileChooserAction::SelectFolder,
            &[("Cancel", ResponseType::Cancel), ("Select", ResponseType::Accept)],
        );
        
        if let Some(dir) = initial_dir {
            dialog.set_current_folder(Some(&gtk::gio::File::for_path(dir)));
        }
        
        let response = dialog.run();
        let result = if response == ResponseType::Accept {
            dialog.file()
                .and_then(|f| f.path())
                .and_then(|p| p.to_str().map(String::from))
        } else {
            None
        };
        
        dialog.close();
        Ok(result)
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

pub struct CustomDialog {
    pub widget: GtkDialog,
    result: DialogResult,
}

impl CustomDialog {
    pub fn new(app: &Application) -> Self {
        let dialog = GtkDialog::new();
        dialog.set_application(Some(app));
        dialog.set_modal(true);
        
        CustomDialog {
            widget: dialog,
            result: DialogResult::None,
        }
    }
    
    pub fn run(&mut self) -> ResponseType {
        let response = self.widget.run();
        self.result = response.into();
        self.widget.close();
        response
    }
    
    pub fn close(&self) {
        self.widget.close();
    }

    pub fn end_dialog(&mut self, result: DialogResult) {
        self.result = result;
        self.widget.close();
    }

    pub fn on_init(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub fn on_command(&mut self, _command: u32) -> Result<(), Error> {
        Ok(())
    }
}

impl UIComponent for CustomDialog {
    fn get_widget(&self) -> &gtk::Widget {
        self.widget.upcast_ref()
    }
}

pub struct FileDialog {
    widget: FileChooserDialog,
}

impl FileDialog {
    pub fn new_open() -> Self {
        let dialog = FileChooserDialog::new(
            Some("Open File"),
            None::<&gtk::Window>,
            FileChooserAction::Open,
            &[("Cancel", ResponseType::Cancel), ("Open", ResponseType::Accept)],
        );
        Self { widget: dialog }
    }

    pub fn show(&self) -> Option<PathBuf> {
        self.widget.present();
        if self.widget.response_type() == ResponseType::Accept {
            self.widget.file().and_then(|f| f.path())
        } else {
            None
        }
    }
}

pub struct MessageDialog {
    widget: gtk::MessageDialog,
}

impl MessageDialog {
    pub fn show(&self) -> DialogResult {
        self.widget.present();
        self.widget.response_type().into()
    }
}

pub struct ColorDialog {
    widget: ColorChooserDialog,
}

impl ColorDialog {
    pub fn show(&self) -> Option<gdk::RGBA> {
        self.widget.present();
        if self.widget.response_type() == ResponseType::Accept {
            Some(self.widget.rgba())
        } else {
            None
        }
    }
}

impl Dialog for FileDialog {
    fn show(&self) {
        self.widget.present();
    }

    fn hide(&self) {
        self.widget.hide();
    }
}

impl Dialog for MessageDialog {
    fn show(&self) {
        self.widget.present();
    }

    fn hide(&self) {
        self.widget.hide();
    }
}

impl Dialog for ColorDialog {
    fn show(&self) {
        self.widget.present();
    }

    fn hide(&self) {
        self.widget.hide();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dialog_initialization() {
        gtk::init().expect("Failed to initialize GTK");
        
        let dialog = CustomDialog::new(None);
        assert!(!dialog.widget.is_visible());
    }
} 