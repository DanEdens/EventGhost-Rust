use gtk::prelude::*;
use gtk::{self, Box, Button, Image, Orientation};
use glib;
use crate::core::Error;
use crate::win32;
use super::UIComponent;

#[derive(Debug, Clone)]
pub struct ToolbarButton {
    pub id: i32,
    pub text: String,
    pub tooltip: String,
    pub icon_name: String,
    pub style: ButtonStyle,
    pub state: ButtonState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonStyle {
    Normal,
    Check,
    Group,
    Separator,
    DropDown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonState {
    Normal,
    Pressed,
    Checked,
    Disabled,
}

pub struct Toolbar {
    pub widget: Box,
    buttons: Vec<(ToolbarButton, Button)>,
}

impl Toolbar {
    pub fn new() -> Self {
        let widget = Box::new(Orientation::Horizontal, 5);
        widget.add_css_class("toolbar");
        
        Toolbar {
            widget,
            buttons: Vec::new(),
        }
    }

    pub fn add_button(&mut self, button: ToolbarButton) {
        let btn = Button::new();
        
        // Set icon if specified
        if !button.icon_name.is_empty() {
            let image = Image::from_icon_name(&button.icon_name);
            btn.set_child(Some(&image));
        }
        
        // Set tooltip
        if !button.tooltip.is_empty() {
            btn.set_tooltip_text(Some(&button.tooltip));
        }
        
        // Set style
        match button.style {
            ButtonStyle::Check => {
                btn.add_css_class("toggle");
            }
            ButtonStyle::Group => {
                btn.add_css_class("group");
            }
            ButtonStyle::Separator => {
                btn.add_css_class("separator");
            }
            ButtonStyle::DropDown => {
                btn.add_css_class("dropdown");
            }
            _ => {}
        }
        
        // Set initial state
        match button.state {
            ButtonState::Disabled => {
                btn.set_sensitive(false);
            }
            ButtonState::Checked => {
                if button.style == ButtonStyle::Check {
                    btn.set_css_classes(&["toggle", "active"]);
                }
            }
            _ => {}
        }
        
        self.widget.append(&btn);
        self.buttons.push((button, btn));
    }

    pub fn remove_button(&mut self, button_id: i32) {
        if let Some(index) = self.buttons.iter().position(|(btn, _)| btn.id == button_id) {
            let (_, button) = &self.buttons[index];
            self.widget.remove(button);
            self.buttons.remove(index);
        }
    }

    pub fn enable_button(&mut self, button_id: i32, enabled: bool) {
        if let Some((_, button)) = self.buttons.iter().find(|(btn, _)| btn.id == button_id) {
            button.set_sensitive(enabled);
        }
    }

    pub fn set_button_state(&mut self, button_id: i32, state: ButtonState) {
        if let Some((btn, button)) = self.buttons.iter_mut().find(|(b, _)| b.id == button_id) {
            match state {
                ButtonState::Disabled => {
                    button.set_sensitive(false);
                }
                ButtonState::Checked => {
                    if btn.style == ButtonStyle::Check {
                        button.set_css_classes(&["toggle", "active"]);
                    }
                }
                _ => {
                    button.set_sensitive(true);
                    if btn.style == ButtonStyle::Check {
                        button.set_css_classes(&["toggle"]);
                    }
                }
            }
            btn.state = state;
        }
    }

    pub fn get_button_state(&self, button_id: i32) -> Option<ButtonState> {
        self.buttons.iter()
            .find(|(btn, _)| btn.id == button_id)
            .map(|(btn, _)| btn.state)
    }

    pub fn set_button_text(&mut self, button_id: i32, text: &str) {
        if let Some((btn, button)) = self.buttons.iter_mut().find(|(b, _)| b.id == button_id) {
            button.set_label(text);
            btn.text = text.to_string();
        }
    }

    pub fn set_button_tooltip(&mut self, button_id: i32, tooltip: &str) {
        if let Some((btn, button)) = self.buttons.iter_mut().find(|(b, _)| b.id == button_id) {
            button.set_tooltip_text(Some(tooltip));
            btn.tooltip = tooltip.to_string();
        }
    }
}

impl UIComponent for Toolbar {
    fn get_hwnd(&self) -> HWND {
        self.hwnd
    }

    fn show(&mut self) -> Result<(), Error> {
        unsafe {
            ShowWindow(self.hwnd, SW_SHOW);
        }
        self.is_visible = true;
        Ok(())
    }

    fn hide(&mut self) -> Result<(), Error> {
        unsafe {
            ShowWindow(self.hwnd, SW_HIDE);
        }
        self.is_visible = false;
        Ok(())
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }
}

#[cfg(feature = "toolbar-test")]
mod tests {
    use super::*;
    use windows::Win32::Foundation::HWND;

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
        
        let button = ToolbarButton {
            id: 1,
            text: "Test".to_string(),
            tooltip: "Test Button".to_string(),
            icon_name: "document-new-symbolic".to_string(),
            style: ButtonStyle::Normal,
            state: ButtonState::Normal,
        };
        
        toolbar.add_button(button);
        assert_eq!(toolbar.buttons.len(), 1);
        
        toolbar.remove_button(1);
        assert!(toolbar.buttons.is_empty());
    }
} 