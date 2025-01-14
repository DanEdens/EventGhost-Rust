use windows::Win32::Foundation::HWND;
use crate::core::Error;
use super::{Dialog, DialogResult, UIComponent, PropertyGrid, PropertySource};

pub struct PluginConfigDialog {
    hwnd: HWND,
    property_grid: PropertyGrid,
    description_text: String,
    is_visible: bool,
    result: DialogResult,
}

impl PluginConfigDialog {
    pub fn new(parent: HWND) -> Result<Self, Error> {
        todo!()
    }

    pub fn set_plugin(&mut self, plugin: Box<dyn PropertySource>) -> Result<(), Error> {
        todo!()
    }

    pub fn set_description(&mut self, text: &str) {
        todo!()
    }

    pub fn get_description(&self) -> &str {
        todo!()
    }

    pub fn apply_changes(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub fn reset_changes(&mut self) -> Result<(), Error> {
        todo!()
    }
}

impl Dialog for PluginConfigDialog {
    fn show_modal(&mut self) -> Result<DialogResult, Error> {
        todo!()
    }

    fn end_dialog(&mut self, result: DialogResult) {
        todo!()
    }

    fn on_init(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn on_command(&mut self, command: u32) -> Result<(), Error> {
        todo!()
    }
}

impl UIComponent for PluginConfigDialog {
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
    use super::super::property_grid::{Property, PropertyValue};

    struct TestPlugin {
        name: String,
        enabled: bool,
    }

    impl PropertySource for TestPlugin {
        fn get_properties(&self) -> Vec<Property> {
            vec![
                Property::new("name", "General", PropertyValue::String(self.name.clone()))
                    .with_description("Plugin name"),
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