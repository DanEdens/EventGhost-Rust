use gtk::prelude::*;
use gtk::{self, Box, Button};
use gtk::Orientation;

pub struct Toolbar {
    pub widget: Box,
    pub buttons: Vec<Button>,
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
    
    pub fn add_button(&mut self, label: &str, tooltip: Option<&str>) -> Button {
        let button = Button::new();
        button.set_label(label);
        if let Some(tip) = tooltip {
            button.set_tooltip_text(Some(tip));
        }
        self.widget.append(&button);
        self.buttons.push(button.clone());
        button
    }
    
    pub fn remove_button(&mut self, button: &Button) {
        if let Some(pos) = self.buttons.iter().position(|b| b == button) {
            self.widget.remove(button);
            self.buttons.remove(pos);
        }
    }
    
    pub fn enable_button(&mut self, button: &Button, enabled: bool) {
        button.set_sensitive(enabled);
    }
    
    pub fn show(&self) {
        self.widget.show();
    }
    
    pub fn hide(&self) {
        self.widget.hide();
    }
    
    pub fn is_visible(&self) -> bool {
        self.widget.is_visible()
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
        
        let button = toolbar.add_button("Test", Some("Test Button"));
        assert_eq!(toolbar.buttons.len(), 1);
        
        toolbar.remove_button(&button);
        assert!(toolbar.buttons.is_empty());
    }
} 