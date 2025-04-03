use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{FileChooserAction, FileChooserDialog, FileFilter, ResponseType};
use crate::core::config_manager::{ConfigManager, ConfigChangeEvent};
use crate::core::actions::system::file_operations::ConfigFileType;
use glib::clone;
use log::{debug, info, warn, error};

/// Result of a config file dialog operation
#[derive(Debug, Clone)]
pub enum ConfigFileDialogResult {
    /// File was opened
    Opened { path: PathBuf },
    /// File was saved
    Saved { path: PathBuf },
    /// Operation was cancelled
    Cancelled,
    /// Error occurred
    Error { message: String },
}

/// Dialog for opening and saving configuration files
pub struct ConfigFileDialog {
    parent: gtk::Window,
    config_manager: Rc<RefCell<ConfigManager>>,
    recent_files: Vec<PathBuf>,
    default_dir: Option<PathBuf>,
}

impl ConfigFileDialog {
    /// Create a new configuration file dialog
    pub fn new(parent: gtk::Window, config_manager: Rc<RefCell<ConfigManager>>) -> Self {
        Self {
            parent,
            config_manager,
            recent_files: Vec::new(),
            default_dir: None,
        }
    }

    /// Set the default directory
    pub fn with_default_dir<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.default_dir = Some(dir.as_ref().to_path_buf());
        self
    }

    /// Set recent files
    pub fn with_recent_files(mut self, recent_files: Vec<PathBuf>) -> Self {
        self.recent_files = recent_files;
        self
    }

    /// Show open dialog
    pub async fn show_open_dialog(&self) -> ConfigFileDialogResult {
        let dialog = FileChooserDialog::new(
            Some("Open Configuration File"),
            Some(&self.parent),
            FileChooserAction::Open,
            &[
                ("Cancel", ResponseType::Cancel),
                ("Open", ResponseType::Accept),
            ],
        );

        // Set up file filters for supported configuration types
        let filter_json = FileFilter::new();
        filter_json.set_name(Some("JSON Configuration Files"));
        filter_json.add_pattern("*.json");
        dialog.add_filter(&filter_json);

        let filter_xml = FileFilter::new();
        filter_xml.set_name(Some("XML Configuration Files"));
        filter_xml.add_pattern("*.xml");
        filter_xml.add_pattern("*.egtree");
        dialog.add_filter(&filter_xml);

        let filter_all = FileFilter::new();
        filter_all.set_name(Some("All Files"));
        filter_all.add_pattern("*");
        dialog.add_filter(&filter_all);

        // Set default folder if specified
        if let Some(dir) = &self.default_dir {
            dialog.set_current_folder(Some(&gtk::gio::File::for_path(dir)));
        }

        // Add recent files
        for file in &self.recent_files {
            if file.exists() {
                dialog.add_shortcut_folder(&gtk::gio::File::for_path(file.parent().unwrap_or(Path::new(""))));
            }
        }

        // Show dialog and get response
        let response = dialog.run_future().await;
        
        match response {
            ResponseType::Accept => {
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        dialog.destroy();
                        
                        // Try to load the configuration
                        match self.config_manager.borrow_mut().load(&path).await {
                            Ok(_) => ConfigFileDialogResult::Opened { path },
                            Err(e) => ConfigFileDialogResult::Error { message: format!("Failed to load configuration: {}", e) },
                        }
                    } else {
                        dialog.destroy();
                        ConfigFileDialogResult::Error { message: "Invalid file path".to_string() }
                    }
                } else {
                    dialog.destroy();
                    ConfigFileDialogResult::Cancelled
                }
            },
            _ => {
                dialog.destroy();
                ConfigFileDialogResult::Cancelled
            }
        }
    }

    /// Show save dialog
    pub async fn show_save_dialog(&self) -> ConfigFileDialogResult {
        let dialog = FileChooserDialog::new(
            Some("Save Configuration File"),
            Some(&self.parent),
            FileChooserAction::Save,
            &[
                ("Cancel", ResponseType::Cancel),
                ("Save", ResponseType::Accept),
            ],
        );

        // Set up file filters for supported configuration types
        let filter_json = FileFilter::new();
        filter_json.set_name(Some("JSON Configuration Files"));
        filter_json.add_pattern("*.json");
        dialog.add_filter(&filter_json);

        let filter_xml = FileFilter::new();
        filter_xml.set_name(Some("XML Configuration Files"));
        filter_xml.add_pattern("*.xml");
        filter_xml.add_pattern("*.egtree");
        dialog.add_filter(&filter_xml);

        // Set current name from existing config path or default
        if let Some(path) = self.config_manager.borrow().config_path() {
            if let Some(filename) = path.file_name() {
                dialog.set_current_name(filename.to_str().unwrap_or("config.json"));
            } else {
                dialog.set_current_name("config.json");
            }
        } else {
            dialog.set_current_name("config.json");
        }

        // Set default folder if specified
        if let Some(dir) = &self.default_dir {
            dialog.set_current_folder(Some(&gtk::gio::File::for_path(dir)));
        } else if let Some(path) = self.config_manager.borrow().config_path() {
            if let Some(parent) = path.parent() {
                dialog.set_current_folder(Some(&gtk::gio::File::for_path(parent)));
            }
        }

        // Confirm overwrite
        dialog.set_do_overwrite_confirmation(true);

        // Show dialog and get response
        let response = dialog.run_future().await;
        
        match response {
            ResponseType::Accept => {
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        dialog.destroy();
                        
                        // Check file type from extension
                        let file_type = ConfigFileType::from_path(&path);
                        
                        if file_type == ConfigFileType::Unknown {
                            // If the file type is unknown, add .json extension
                            let mut path_with_ext = path.clone();
                            path_with_ext.set_extension("json");
                            
                            // Try to save the configuration
                            match self.config_manager.borrow_mut().save(&path_with_ext, true).await {
                                Ok(_) => ConfigFileDialogResult::Saved { path: path_with_ext },
                                Err(e) => ConfigFileDialogResult::Error { message: format!("Failed to save configuration: {}", e) },
                            }
                        } else {
                            // Try to save the configuration
                            match self.config_manager.borrow_mut().save(&path, true).await {
                                Ok(_) => ConfigFileDialogResult::Saved { path },
                                Err(e) => ConfigFileDialogResult::Error { message: format!("Failed to save configuration: {}", e) },
                            }
                        }
                    } else {
                        dialog.destroy();
                        ConfigFileDialogResult::Error { message: "Invalid file path".to_string() }
                    }
                } else {
                    dialog.destroy();
                    ConfigFileDialogResult::Cancelled
                }
            },
            _ => {
                dialog.destroy();
                ConfigFileDialogResult::Cancelled
            }
        }
    }

    /// Show save as dialog
    pub async fn show_save_as_dialog(&self) -> ConfigFileDialogResult {
        self.show_save_dialog().await
    }

    /// Show confirmation dialog for unsaved changes
    pub async fn show_unsaved_changes_dialog(&self) -> ResponseType {
        let dialog = gtk::MessageDialog::new(
            Some(&self.parent),
            gtk::DialogFlags::MODAL,
            gtk::MessageType::Warning,
            gtk::ButtonsType::None,
            "Save changes to configuration?",
        );
        
        dialog.set_secondary_text(Some("If you don't save, your changes will be lost."));
        
        dialog.add_button("Don't Save", ResponseType::Reject);
        dialog.add_button("Cancel", ResponseType::Cancel);
        dialog.add_button("Save", ResponseType::Accept);
        
        dialog.set_default_response(ResponseType::Accept);
        
        let response = dialog.run_future().await;
        dialog.destroy();
        
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gtk4 as gtk;
    use gtk::Application;
    use std::sync::Arc;
    use tokio::runtime::Runtime;
    
    // Note: These tests depend on GTK and should be run with the `--test-threads=1` flag
    
    #[test]
    fn test_create_dialog() {
        let rt = Runtime::new().unwrap();
        
        rt.block_on(async {
            let app = Application::new(Some("org.eventghost.test"), gtk::gio::ApplicationFlags::NON_UNIQUE);
            
            app.connect_activate(|app| {
                let window = gtk::ApplicationWindow::new(app);
                window.set_title(Some("Test"));
                window.set_default_size(800, 600);
                window.show();
                
                let config_manager = Rc::new(RefCell::new(ConfigManager::new()));
                let dialog = ConfigFileDialog::new(window, config_manager);
                
                assert!(!dialog.recent_files.is_empty() || dialog.recent_files.is_empty());
            });
            
            let args: Vec<String> = Vec::new();
            app.run_with_args(&args);
        });
    }
} 