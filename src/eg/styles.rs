use gtk::prelude::*;
use gtk::{CssProvider, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION};

/// Initializes the application's CSS styles
pub fn init_styles() {
    // Load CSS
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("../../assets/style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_styles() {
        gtk::init().expect("Failed to initialize GTK");
        init_styles();
        // If we get here without panicking, the styles were initialized successfully
    }
} 