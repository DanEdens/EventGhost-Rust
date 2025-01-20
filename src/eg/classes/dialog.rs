use gtk::prelude::*;
use gtk::{self, Dialog as GtkDialog, ResponseType, FileChooserDialog, FileChooserAction, ColorChooserDialog};
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

impl From<ResponseType> for DialogResult {
    fn from(response: ResponseType) -> Self {
        match response {
            ResponseType::Ok => DialogResult::Ok,
            ResponseType::Cancel => DialogResult::Cancel,
            ResponseType::Yes => DialogResult::Yes,
            ResponseType::No => DialogResult::No,
            _ => DialogResult::Custom(response.into()),
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
}

impl CustomDialog {
    pub fn new(title: &str) -> Self {
        let widget = GtkDialog::new();
        widget.set_title(Some(title));
        widget.set_modal(true);
        
        CustomDialog {
            widget,
        }
    }
    
    pub fn run(&self) -> ResponseType {
        self.widget.run()
    }
    
    pub fn close(&self) {
        self.widget.close();
    }
}

impl UIComponent for CustomDialog {
    fn get_widget(&self) -> &gtk::Widget {
        self.widget.upcast_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dialog_initialization() {
        gtk::init().expect("Failed to initialize GTK");
        
        let dialog = CustomDialog::new("Test Dialog");
        assert!(!dialog.widget.is_visible());
    }
} 