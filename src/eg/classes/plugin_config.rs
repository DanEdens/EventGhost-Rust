use windows::Win32::Foundation::HWND;
use crate::core::Error;
use super::{Dialog, DialogResult, UIComponent, PropertyGrid, PropertySource, Property};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ConfigPage {
    title: String,
    description: String,
    property_grid: PropertyGrid,
}

#[derive(Debug)]
pub struct ConfigDialog {
    hwnd: HWND,
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

    pub fn new(parent: HWND) -> Result<Self, Error> {
        todo!("Phase 4: Configuration System")
    }

    pub fn add_page(&mut self, title: &str, description: &str) -> Result<(), Error> {
        todo!("Phase 4: Configuration System")
    }

    pub fn set_plugin(&mut self, plugin: Box<dyn PropertySource>) -> Result<(), Error> {
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
            .map_err(|e| Error::Config(e))?;
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
}

impl Dialog for ConfigDialog {
    fn show_modal(&mut self) -> Result<DialogResult, Error> {
        todo!()
    }

    fn end_dialog(&mut self, result: DialogResult) {
        self.result = result;
    }

    fn on_init(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn on_command(&mut self, command: u32) -> Result<(), Error> {
        todo!()
    }
}

impl UIComponent for ConfigDialog {
    fn get_hwnd(&self) -> HWND {
        self.hwnd
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
} 