use gtk4 as gtk;
use crate::prelude::*;
use crate::prelude::{CssProvider, StyleContext};
use gtk::gdk;
use std::str::from_utf8;

/// Initializes the application's CSS styles
pub fn init_styles() {
    // Load CSS
    let provider = CssProvider::new();
    let css_data = include_bytes!("../../assets/style.css");
    let css_str = from_utf8(css_data).expect("CSS file is not valid UTF-8");
    provider.load_from_data(css_str);

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn apply_css(css: &str) {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(css);
    
    if let Some(display) = gtk4::Display::default() {
        gtk4::StyleContext::add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
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
