use crate::prelude::*;
use crate::prelude::{self, Box, Button, Image, Orientation, Separator};
use gio::{Icon, File as GFile, Resource};
use super::UIComponent;
use std::path::Path;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ToolbarButton {
    pub widget: Button,
    pub id: String,
    pub enabled: bool,
    pub tooltip: String,
}

/// A toolbar component for EventGhost.
#[derive(Clone)]
pub struct Toolbar {
    /// The toolbar widget
    pub widget: Box,
    /// The buttons in the toolbar
    pub buttons: Vec<ToolbarButton>,
}

impl Toolbar {
    pub fn new() -> Self {
        let widget = Box::new(Orientation::Horizontal, 2);
        widget.add_css_class("toolbar");
        
        Toolbar {
            widget,
            buttons: Vec::new(),
        }
    }
    
    pub fn add_button(&mut self, id: &str, icon_path: &str, tooltip: &str) -> Button {
        let button = Button::new();
        
        // Set up icon
        if icon_path.starts_with("/org/eventghost/") {
            // Load from GResource
            let icon = Icon::for_string(&format!("resource://{}", icon_path))
                .expect("Failed to create icon from resource path");
            let image = Image::from_gicon(&icon);
            image.set_icon_size(gtk::IconSize::Large);
            button.set_child(Some(&image));
        } else if let Some(icon) = Icon::for_string(icon_path).ok() {
            // Load from stock icon name
            let image = Image::from_gicon(&icon);
            image.set_icon_size(gtk::IconSize::Large);
            button.set_child(Some(&image));
        }
        
        // Set up tooltip
        button.set_tooltip_text(Some(tooltip));
        button.add_css_class("flat"); // Make button flat
        
        // Add to container
        self.widget.append(&button);
        
        let toolbar_button = ToolbarButton {
            widget: button.clone(),
            id: id.to_string(),
            enabled: true,
            tooltip: tooltip.to_string(),
        };
        
        self.buttons.push(toolbar_button);
        button
    }
    
    pub fn add_separator(&mut self) {
        let separator = Separator::new(Orientation::Vertical);
        self.widget.append(&separator);
    }
    
    pub fn enable_button(&mut self, id: &str, enabled: bool) {
        if let Some(button) = self.buttons.iter_mut().find(|b| b.id == id) {
            button.widget.set_sensitive(enabled);
            button.enabled = enabled;
        }
    }
    
    pub fn set_button_tooltip(&mut self, id: &str, tooltip: &str) {
        if let Some(button) = self.buttons.iter_mut().find(|b| b.id == id) {
            button.widget.set_tooltip_text(Some(tooltip));
            button.tooltip = tooltip.to_string();
        }
    }
    
    pub fn get_button(&self, id: &str) -> Option<&Button> {
        self.buttons.iter()
            .find(|b| b.id == id)
            .map(|b| &b.widget)
    }
}

// Implement UIComponent trait
impl UIComponent for Toolbar {
    fn get_widget(&self) -> &gtk::Widget {
        self.widget.upcast_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolbar_initialization() {
        gtk::init().expect("Failed to initialize GTK");
        
        let toolbar = Toolbar::new();
        assert!(toolbar.buttons.is_empty());
    }

    #[test]
    fn test_toolbar_buttons() {
        gtk::init().expect("Failed to initialize GTK");
        
        let mut toolbar = Toolbar::new();
        
        toolbar.add_button("test", "document-new", "Test Button");
        assert_eq!(toolbar.buttons.len(), 1);
        
        toolbar.enable_button("test", false);
        assert!(!toolbar.buttons[0].enabled);
        
        toolbar.set_button_tooltip("test", "New tooltip");
        assert_eq!(toolbar.buttons[0].tooltip, "New tooltip");
    }
} 
