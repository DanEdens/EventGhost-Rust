use gtk::prelude::*;
use gtk::{self, Dialog as GtkDialog, Box as GtkBox, Label, Entry, Button, ResponseType};
use crate::core::Error;
use super::UIComponent;
use super::dialog::{Dialog, DialogResult};
use super::property_grid::{PropertyGrid, PropertySource, Property};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ConfigPage {
    title: String,
    description: String,
    property_grid: PropertyGrid,
}

pub struct ConfigDialog {
    widget: GtkDialog,
    container: GtkBox,
    pages: Vec<ConfigPage>,
    current_page: usize,
    is_visible: bool,
    result: DialogResult,
    changes: HashMap<String, Property>,
}

// Phase 4: Configuration System - Not yet implemented
#[allow(dead_code)]
impl ConfigDialog {
    const DEFAULT_DESCRIPTION: &'static str = "";

    pub fn new(title: &str) -> Self {
        let widget = GtkDialog::new();
        widget.set_title(Some(title));
        
        let container = GtkBox::new(gtk::Orientation::Vertical, 6);
        widget.content_area().append(&container);
        
        let property_grid = PropertyGrid::new();
        container.append(&property_grid.get_widget());
        
        ConfigDialog {
            widget,
            container,
            pages: Vec::new(),
            current_page: 0,
            is_visible: false,
            result: DialogResult::Cancel,
            changes: HashMap::new(),
        }
    }
    
    pub fn add_field(&self, label: &str, value: &str) -> Entry {
        let hbox = GtkBox::new(gtk::Orientation::Horizontal, 12);
        let label = Label::new(Some(label));
        let entry = Entry::new();
        entry.set_text(value);
        
        hbox.append(&label);
        hbox.append(&entry);
        self.container.append(&hbox);
        
        entry
    }
    
    pub fn add_button(&self, label: &str) -> Button {
        let button = Button::with_label(label);
        self.widget.add_action_widget(&button, ResponseType::Ok);
        button
    }

    pub fn add_page(&mut self, title: &str, description: &str) -> Result<(), Error> {
        todo!("Phase 4: Configuration System")
    }

    pub fn set_plugin(&mut self, plugin: Box<dyn PropertySource + Send + Sync>) -> Result<(), Error> {
        todo!()
    }

    pub fn set_current_page(&mut self, index: usize) -> Result<(), Error> {
        if index >= self.pages.len() {
            return Err(Error::Config("Invalid page index".into()));
        }
        self.current_page = index;
        Ok(())
    }

    pub fn get_current_page(&self) -> usize {
        self.current_page
    }

    pub fn get_page_count(&self) -> usize {
        self.pages.len()
    }

    pub fn set_description(&mut self, text: &str) {
        if let Some(page) = self.pages.get_mut(self.current_page) {
            page.description = text.to_string();
        }
    }

    pub fn get_description(&self) -> String {
        self.pages.get(self.current_page)
            .map(|page| page.description.clone())
            .unwrap_or_else(String::new)
    }

    pub fn validate_changes(&self) -> Result<(), String> {
        for property in self.changes.values() {
            property.validate()?;
        }
        Ok(())
    }

    pub fn apply_changes(&mut self) -> Result<(), Error> {
        self.validate_changes()
            .map_err(|e| Error::Config(e.into()))?;
        todo!()
    }

    pub fn reset_changes(&mut self) -> Result<(), Error> {
        self.changes.clear();
        for page in &mut self.pages {
            page.property_grid.refresh()?;
        }
        Ok(())
    }

    fn on_property_changed(&mut self, name: String, value: Property) {
        self.changes.insert(name, value);
    }

    pub fn run(&self) -> ResponseType {
        self.widget.run()
    }
    
    pub fn close(&self) {
        self.widget.close();
    }
}

impl Dialog for ConfigDialog {
    fn show_modal(&mut self) -> Result<DialogResult, Error> {
        let response = self.widget.run();
        self.widget.close();
        Ok(response.into())
    }

    fn end_dialog(&mut self, result: DialogResult) {
        self.result = result;
    }

    fn on_init(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn on_command(&mut self, _command: u32) -> Result<(), Error> {
        Ok(())
    }
}

impl UIComponent for ConfigDialog {
    fn get_widget(&self) -> &gtk::Widget {
        self.widget.upcast_ref()
    }

    fn show(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn hide(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::property_grid::{PropertyValue};

    struct TestPlugin {
        name: String,
        enabled: bool,
    }

    impl PropertySource for TestPlugin {
        fn get_properties(&self) -> Vec<Property> {
            vec![
                Property::new("name", "General", PropertyValue::String(self.name.clone()))
                    .with_description("Plugin name")
                    .with_validator(|value| {
                        if let PropertyValue::String(s) = value {
                            if s.is_empty() {
                                return Err("Name cannot be empty".into());
                            }
                        }
                        Ok(())
                    }),
                Property::new("enabled", "State", PropertyValue::Bool(self.enabled))
                    .with_description("Enable/disable plugin"),
            ]
        }

        fn set_property(&mut self, name: &str, value: PropertyValue) -> Result<(), Error> {
            match (name, value) {
                ("name", PropertyValue::String(s)) => self.name = s,
                ("enabled", PropertyValue::Bool(b)) => self.enabled = b,
                _ => return Err(Error::Config("Invalid property".into())),
            }
            Ok(())
        }
    }

    #[test]
    fn test_config_dialog_initialization() {
        gtk::init().expect("Failed to initialize GTK");
        
        let dialog = ConfigDialog::new("Test Config");
        assert!(!dialog.widget.is_visible());
    }
} 