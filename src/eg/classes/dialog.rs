use gtk::prelude::*;
use gtk::{self, Dialog as GtkDialog, FileChooserDialog, MessageDialog as GtkMessageDialog, ColorChooserDialog, ResponseType, FileChooserAction, Window};
use gtk::gdk;
use crate::core::Error;
use super::UIComponent;
use gtk::{PrintOperation, Application};
use std::path::PathBuf;
use tokio::runtime::Runtime;

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

    fn show_modal(&mut self) -> Result<DialogResult, Error> {
        self.widget.set_modal(true);
        let rt = Runtime::new().map_err(|e| Error::Other(e.to_string()))?;
        let response = rt.block_on(self.widget.run_future());
        Ok(response.into())
    }

    fn end_dialog(&mut self, result: DialogResult) {
        match result {
            DialogResult::Ok => {
                self.widget.close();
            }
            DialogResult::Cancel => {
                self.widget.close();
            }
            _ => {}
        }
    }

    fn on_init(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn on_command(&mut self, _command: u32) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogResult {
    Ok,
    Cancel,
    Yes,
    No,
    None,
    Custom(i32),
}

impl From<ResponseType> for DialogResult {
    fn from(response: ResponseType) -> Self {
        match response {
            ResponseType::Ok => DialogResult::Ok,
            ResponseType::Cancel => DialogResult::Cancel,
            ResponseType::Yes => DialogResult::Yes,
            ResponseType::No => DialogResult::No,
            ResponseType::None => DialogResult::None,
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
        
        let rt = Runtime::new().map_err(|e| Error::Other(e.to_string()))?;
        let response = rt.block_on(dialog.run_future());
        let result = if response == ResponseType::Accept {
            let files = dialog.files();
            let mut paths = Vec::new();
            for i in 0..files.n_items() {
                if let Some(file) = files.item(i) {
                    if let Some(file) = file.downcast_ref::<gtk::gio::File>() {
                        if let Some(path) = file.path() {
                            if let Some(path_str) = path.to_str() {
                                paths.push(path_str.to_string());
                            }
                        }
                    }
                }
            }
            paths
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
        
        let rt = Runtime::new().map_err(|e| Error::Other(e.to_string()))?;
        let response = rt.block_on(dialog.run_future());
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
        
        let rt = Runtime::new().map_err(|e| Error::Other(e.to_string()))?;
        let response = rt.block_on(dialog.run_future());
        dialog.close();
        
        Ok(response.into())
    }

    /// Show a color picker dialog
    pub fn color_picker(parent: Option<&gtk::Window>, initial_color: Option<gtk::gdk::RGBA>) -> Result<Option<gtk::gdk::RGBA>, Error> {
        let dialog = ColorChooserDialog::new(Some("Choose a color"), parent);
        
        if let Some(color) = initial_color {
            dialog.set_rgba(&color);
        }
        
        let rt = Runtime::new().map_err(|e| Error::Other(e.to_string()))?;
        let response = rt.block_on(dialog.run_future());
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
        
        let rt = Runtime::new().map_err(|e| Error::Other(e.to_string()))?;
        let response = rt.block_on(dialog.run_future());
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
    pub fn new(app: Option<&Application>) -> Self {
        let dialog = GtkDialog::new();
        if let Some(app) = app {
            dialog.set_application(Some(app));
        }
        dialog.set_modal(true);
        
        CustomDialog {
            widget: dialog,
            result: DialogResult::None,
        }
    }
    
    pub fn run(&mut self) -> ResponseType {
        let rt = Runtime::new().expect("Failed to create runtime");
        let response = rt.block_on(self.widget.run_future());
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
    dialog: FileChooserDialog,
}

impl FileDialog {
    pub fn new(title: &str, parent: Option<&Window>, action: FileChooserAction) -> Self {
        let dialog = FileChooserDialog::new(
            Some(title),
            parent,
            action,
            &[("_Cancel", ResponseType::Cancel), ("_Open", ResponseType::Accept)],
        );
        
        FileDialog { dialog }
    }
}

impl Dialog for FileDialog {
    fn show(&self) {
        self.dialog.present();
    }

    fn hide(&self) {
        self.dialog.hide();
    }

    fn show_modal(&mut self) -> Result<DialogResult, Error> {
        self.dialog.set_modal(true);
        let rt = Runtime::new().map_err(|e| Error::Other(e.to_string()))?;
        let response = rt.block_on(self.dialog.run_future());
        Ok(response.into())
    }
    
    fn end_dialog(&mut self, _result: DialogResult) {
        self.dialog.close();
    }
    
    fn on_init(&mut self) -> Result<(), Error> {
        Ok(())
    }
    
    fn on_command(&mut self, _command: u32) -> Result<(), Error> {
        Ok(())
    }
}

pub struct MessageDialog {
    dialog: GtkMessageDialog,
}

impl MessageDialog {
    pub fn new(title: &str, message: &str, parent: Option<&Window>) -> Self {
        let dialog = GtkMessageDialog::new(
            parent,
            gtk::DialogFlags::MODAL,
            gtk::MessageType::Info,
            gtk::ButtonsType::Ok,
            message,
        );
        dialog.set_title(Some(title));
        
        MessageDialog { dialog }
    }
}

impl Dialog for MessageDialog {
    fn show(&self) {
        self.dialog.present();
    }

    fn hide(&self) {
        self.dialog.hide();
    }

    fn show_modal(&mut self) -> Result<DialogResult, Error> {
        self.dialog.set_modal(true);
        let rt = Runtime::new().map_err(|e| Error::Other(e.to_string()))?;
        let response = rt.block_on(self.dialog.run_future());
        Ok(response.into())
    }
    
    fn end_dialog(&mut self, _result: DialogResult) {
        self.dialog.close();
    }
    
    fn on_init(&mut self) -> Result<(), Error> {
        Ok(())
    }
    
    fn on_command(&mut self, _command: u32) -> Result<(), Error> {
        Ok(())
    }
}

pub struct ColorDialog {
    dialog: ColorChooserDialog,
}

impl ColorDialog {
    pub fn new(title: &str, parent: Option<&Window>) -> Self {
        let dialog = ColorChooserDialog::new(Some(title), parent);
        ColorDialog { dialog }
    }
}

impl Dialog for ColorDialog {
    fn show(&self) {
        self.dialog.present();
    }

    fn hide(&self) {
        self.dialog.hide();
    }

    fn show_modal(&mut self) -> Result<DialogResult, Error> {
        self.dialog.set_modal(true);
        let rt = Runtime::new().map_err(|e| Error::Other(e.to_string()))?;
        let response = rt.block_on(self.dialog.run_future());
        Ok(response.into())
    }
    
    fn end_dialog(&mut self, _result: DialogResult) {
        self.dialog.close();
    }
    
    fn on_init(&mut self) -> Result<(), Error> {
        Ok(())
    }
    
    fn on_command(&mut self, _command: u32) -> Result<(), Error> {
        Ok(())
    }
}

/// A dialog for configuring plugin settings
pub struct ConfigDialog {
    dialog: GtkDialog,
}

impl ConfigDialog {
    pub fn new(parent: Option<&Window>) -> Self {
        let dialog = GtkDialog::builder()
            .title("Configure")
            .modal(true)
            .build();
            
        if let Some(parent) = parent {
            dialog.set_transient_for(Some(parent));
        }
        
        ConfigDialog { dialog }
    }
}

impl Dialog for ConfigDialog {
    fn show(&self) {
        self.dialog.present();
    }

    fn hide(&self) {
        self.dialog.hide();
    }

    fn show_modal(&mut self) -> Result<DialogResult, Error> {
        self.dialog.set_modal(true);
        let rt = Runtime::new().map_err(|e| Error::Other(e.to_string()))?;
        let response = rt.block_on(self.dialog.run_future());
        Ok(response.into())
    }
    
    fn end_dialog(&mut self, _result: DialogResult) {
        self.dialog.close();
    }
    
    fn on_init(&mut self) -> Result<(), Error> {
        Ok(())
    }
    
    fn on_command(&mut self, _command: u32) -> Result<(), Error> {
        Ok(())
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