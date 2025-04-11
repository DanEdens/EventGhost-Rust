use crate::prelude::*;
use crate::prelude::{self, Box, CheckButton, Statusbar};
// use glib;
use super::UIComponent;

#[derive(Debug, Clone)]
pub struct StatusPart {
    pub text: String,
    pub width: i32,
    pub style: StatusPartStyle,
}

#[derive(Debug, Clone, Copy)]
pub enum StatusPartStyle {
    Normal,
    Progress,
    Owner,
}

pub struct StatusBar {
    pub widget: Box,
    status_bar: Statusbar,
    check_box: CheckButton,
    context_id: u32,
}

impl StatusBar {
    pub fn new() -> Self {
        // Create horizontal box for status bar items
        let widget = Box::new(gtk::Orientation::Horizontal, 5);
        widget.set_margin_start(5);
        widget.set_margin_end(5);
        
        // Create status bar
        let status_bar = Statusbar::new();
        let context_id = status_bar.context_id("main");
        
        // Create checkbox for "Log only assigned events"
        let check_box = CheckButton::with_label("Log only assigned and activated events");
        check_box.set_active(false);
        
        // Pack widgets
        widget.append(&status_bar);
        widget.append(&check_box);
        
        StatusBar {
            widget,
            status_bar,
            check_box,
            context_id,
        }
    }
    
    pub fn set_status_text(&self, text: &str) {
        self.status_bar.remove_all(self.context_id);
        self.status_bar.push(self.context_id, text);
    }
    
    pub fn set_check_box_state(&self, checked: bool) {
        self.check_box.set_active(checked);
    }
    
    pub fn get_check_box_state(&self) -> bool {
        self.check_box.is_active()
    }
    
    pub fn set_check_box_color(&self, enabled: bool) {
        if enabled {
            self.check_box.add_css_class("success");
        } else {
            self.check_box.remove_css_class("success");
        }
    }
}

impl UIComponent for StatusBar {
    fn get_widget(&self) -> &gtk::Widget {
        self.widget.upcast_ref()
    }
}

impl Clone for StatusBar {
    fn clone(&self) -> Self {
        // Create a new StatusBar
        let mut new_bar = StatusBar::new();
        
        // Copy the checkbox state
        new_bar.set_check_box_state(self.get_check_box_state());
        
        new_bar
    }
} 
