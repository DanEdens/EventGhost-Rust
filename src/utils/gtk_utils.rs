use crate::prelude::*;
use crate::core::Error;

/// Convert a GTK error to our application Error type
pub fn gtk_error_to_app_error(err: glib::Error) -> Error {
    Error::InvalidOperation(format!("GTK error: {}", err))
}

/// Create a simple error dialog
pub fn show_error_dialog(parent: Option<&Window>, title: &str, message: &str) {
    let dialog = MessageDialog::new(
        parent,
        gtk::DialogFlags::MODAL,
        MessageType::Error,
        gtk::ButtonsType::Ok,
        message
    );
    dialog.set_title(Some(title));
    
    // Use clone to avoid borrow checker issues
    let dialog_clone = dialog.clone();
    dialog.connect_response(move |_, _| {
        dialog_clone.destroy();
    });
    
    dialog.show();
}

/// Create a simple confirmation dialog
pub fn show_confirmation_dialog(
    parent: Option<&Window>, 
    title: &str, 
    message: &str
) -> bool {
    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    
    let dialog = MessageDialog::new(
        parent,
        gtk::DialogFlags::MODAL,
        MessageType::Question,
        gtk::ButtonsType::YesNo,
        message
    );
    dialog.set_title(Some(title));
    
    let dialog_clone = dialog.clone();
    dialog.connect_response(move |_, response| {
        let confirmed = response == ResponseType::Yes;
        sender.send(confirmed).expect("Failed to send response");
        dialog_clone.destroy();
    });
    
    dialog.show();
    
    // Block until response is received
    glib::MainContext::default().block_on(async {
        receiver.recv().await.unwrap_or(false)
    })
}

/// Show a file chooser dialog for opening files
pub fn show_file_chooser_open<F>(
    parent: Option<&Window>,
    title: &str,
    filters: Vec<(String, Vec<String>)>,
    callback: F
) where
    F: FnOnce(Option<PathBuf>) + 'static,
{
    let dialog = FileChooserDialog::new(
        Some(title),
        parent,
        gtk::FileChooserAction::Open,
        &[("Cancel", ResponseType::Cancel), ("Open", ResponseType::Accept)]
    );
    
    // Add filters
    for (name, patterns) in filters {
        let filter = gtk::FileFilter::new();
        filter.set_name(Some(&name));
        for pattern in patterns {
            filter.add_pattern(&pattern);
        }
        dialog.add_filter(&filter);
    }
    
    let dialog_clone = dialog.clone();
    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            let file = dialog.file().and_then(|f| f.path());
            callback(file);
        } else {
            callback(None);
        }
        dialog_clone.destroy();
    });
    
    dialog.show();
}

/// Show a file chooser dialog for saving files
pub fn show_file_chooser_save<F>(
    parent: Option<&Window>,
    title: &str,
    filters: Vec<(String, Vec<String>)>,
    default_name: Option<&str>,
    callback: F
) where
    F: FnOnce(Option<PathBuf>) + 'static,
{
    let dialog = FileChooserDialog::new(
        Some(title),
        parent,
        gtk::FileChooserAction::Save,
        &[("Cancel", ResponseType::Cancel), ("Save", ResponseType::Accept)]
    );
    
    // Set default name if provided
    if let Some(name) = default_name {
        dialog.set_current_name(name);
    }
    
    // Add filters
    for (name, patterns) in filters {
        let filter = gtk::FileFilter::new();
        filter.set_name(Some(&name));
        for pattern in patterns {
            filter.add_pattern(&pattern);
        }
        dialog.add_filter(&filter);
    }
    
    let dialog_clone = dialog.clone();
    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            let file = dialog.file().and_then(|f| f.path());
            callback(file);
        } else {
            callback(None);
        }
        dialog_clone.destroy();
    });
    
    dialog.show();
}

/// Create a simple input dialog
pub fn show_input_dialog<F>(
    parent: Option<&Window>,
    title: &str,
    message: &str,
    default_value: Option<&str>,
    callback: F
) where
    F: FnOnce(Option<String>) + 'static,
{
    let dialog = Dialog::new();
    dialog.set_title(Some(title));
    dialog.set_modal(true);
    if let Some(parent_window) = parent {
        dialog.set_transient_for(Some(parent_window));
    }
    
    // Add buttons
    dialog.add_button("Cancel", ResponseType::Cancel);
    dialog.add_button("OK", ResponseType::Accept);
    
    // Create layout
    let content_area = dialog.content_area();
    
    let vbox = Box::new(Orientation::Vertical, 8);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    
    let label = Label::new(Some(message));
    label.set_halign(gtk::Align::Start);
    vbox.append(&label);
    
    let entry = Entry::new();
    if let Some(value) = default_value {
        entry.set_text(value);
    }
    entry.set_activates_default(true);
    vbox.append(&entry);
    
    content_area.append(&vbox);
    
    // Set default response
    dialog.set_default_response(ResponseType::Accept);
    
    let entry_clone = entry.clone();
    let dialog_clone = dialog.clone();
    dialog.connect_response(move |_, response| {
        if response == ResponseType::Accept {
            let text = entry_clone.text().to_string();
            callback(Some(text));
        } else {
            callback(None);
        }
        dialog_clone.destroy();
    });
    
    dialog.show();
}

/// Convert a Result with glib::Error to our application Result
pub fn convert_gtk_result<T>(result: Result<T, glib::Error>) -> Result<T, Error> {
    result.map_err(gtk_error_to_app_error)
} 