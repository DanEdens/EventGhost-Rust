use gtk::prelude::*;
use gtk::{self, Dialog, Box, Label, Entry, Grid, Button, Frame, CheckButton, 
          ResponseType, ScrolledWindow, Orientation, Align, Widget};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;

use crate::core::action::{ActionConfig, Action};
use crate::eg::config::Action as ConfigAction;
use super::property_grid::{PropertyGrid, Property, PropertyValue};
use super::UIComponent;
use crate::core::Error;

/// Represents a parameter that can be configured in the action configuration dialog
pub struct ActionParameter {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub value_type: String,
    pub default_value: String,
    pub options: Option<Vec<String>>, // For enum/dropdown parameters
    pub constraints: Option<HashMap<String, String>>, // For validation
}

impl ActionParameter {
    pub fn new(name: &str, display_name: &str, value_type: &str, default_value: &str) -> Self {
        Self {
            name: name.to_string(),
            display_name: display_name.to_string(),
            description: String::new(),
            value_type: value_type.to_string(),
            default_value: default_value.to_string(),
            options: None,
            constraints: None,
        }
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn with_options(mut self, options: Vec<String>) -> Self {
        self.options = Some(options);
        self
    }

    pub fn with_constraints(mut self, constraints: HashMap<String, String>) -> Self {
        self.constraints = Some(constraints);
        self
    }

    pub fn to_property(&self, current_value: Option<&str>) -> Property {
        let value = current_value.unwrap_or(&self.default_value);

        // Create property based on type
        match self.value_type.as_str() {
            "string" => {
                Property::new(&self.display_name, "Parameters", PropertyValue::String(value.to_string()))
                    .with_description(&self.description)
            },
            "int" => {
                let int_value = value.parse::<i32>().unwrap_or_default();
                Property::new(&self.display_name, "Parameters", PropertyValue::Int(int_value))
                    .with_description(&self.description)
            },
            "float" => {
                let float_value = value.parse::<f64>().unwrap_or_default();
                Property::new(&self.display_name, "Parameters", PropertyValue::Float(float_value))
                    .with_description(&self.description)
            },
            "bool" => {
                let bool_value = value.parse::<bool>().unwrap_or_default();
                Property::new(&self.display_name, "Parameters", PropertyValue::Bool(bool_value))
                    .with_description(&self.description)
            },
            "enum" => {
                if let Some(options) = &self.options {
                    Property::new(&self.display_name, "Parameters", 
                                PropertyValue::Enum(value.to_string(), options.clone()))
                        .with_description(&self.description)
                } else {
                    // Fallback to string if no options provided
                    Property::new(&self.display_name, "Parameters", PropertyValue::String(value.to_string()))
                        .with_description(&self.description)
                }
            },
            _ => {
                // Default to string for unknown types
                Property::new(&self.display_name, "Parameters", PropertyValue::String(value.to_string()))
                    .with_description(&self.description)
            }
        }
    }
}

/// Dialog for configuring actions with dynamic parameters
pub struct ActionConfigDialog {
    dialog: Dialog,
    name_entry: Entry,
    enabled_check: CheckButton,
    select_on_execute_check: CheckButton,
    property_grid: PropertyGrid,
    action: Option<Arc<dyn Action>>,
    config: ActionConfig,
    // Map to track parameter name -> property display name
    param_name_map: HashMap<String, String>,
}

impl ActionConfigDialog {
    pub fn new() -> Self {
        let dialog = Dialog::builder()
            .title("Configure Action")
            .modal(true)
            .default_width(500)
            .default_height(400)
            .build();

        dialog.add_button("Cancel", ResponseType::Cancel);
        dialog.add_button("OK", ResponseType::Ok);

        let content_area = dialog.content_area();
        content_area.set_orientation(Orientation::Vertical);
        content_area.set_spacing(12);
        content_area.set_margin_top(12);
        content_area.set_margin_bottom(12);
        content_area.set_margin_start(12);
        content_area.set_margin_end(12);
        
        // Basic settings section
        let basic_frame = Frame::new(Some("Basic Settings"));
        let basic_grid = Grid::new();
        basic_grid.set_row_spacing(6);
        basic_grid.set_column_spacing(12);
        basic_grid.set_margin_top(6);
        basic_grid.set_margin_bottom(6);
        basic_grid.set_margin_start(12);
        basic_grid.set_margin_end(12);
        
        // Name field
        let name_label = Label::new(Some("Name:"));
        name_label.set_halign(Align::Start);
        basic_grid.attach(&name_label, 0, 0, 1, 1);
        
        let name_entry = Entry::new();
        basic_grid.attach(&name_entry, 1, 0, 1, 1);
        
        // Enabled checkbox
        let enabled_check = CheckButton::with_label("Enabled");
        enabled_check.set_active(true);
        basic_grid.attach(&enabled_check, 0, 1, 2, 1);
        
        // Select on execute checkbox
        let select_on_execute_check = CheckButton::with_label("Select on Execute");
        select_on_execute_check.set_active(false);
        basic_grid.attach(&select_on_execute_check, 0, 2, 2, 1);
        
        basic_frame.set_child(Some(&basic_grid));
        content_area.append(&basic_frame);
        
        // Parameters section
        let params_frame = Frame::new(Some("Parameters"));
        let params_box = Box::new(Orientation::Vertical, 6);
        params_box.set_margin_top(6);
        params_box.set_margin_bottom(6);
        params_box.set_margin_start(12);
        params_box.set_margin_end(12);
        
        let property_grid = PropertyGrid::new();
        
        // Wrap the property grid in a scrolled window
        let scrolled_window = ScrolledWindow::new();
        scrolled_window.set_hexpand(true);
        scrolled_window.set_vexpand(true);
        scrolled_window.set_child(Some(property_grid.get_widget()));
        
        params_box.append(&scrolled_window);
        params_frame.set_child(Some(&params_box));
        content_area.append(&params_frame);

        let mut dialog = ActionConfigDialog {
            dialog,
            name_entry,
            enabled_check,
            select_on_execute_check,
            property_grid,
            action: None,
            config: ActionConfig::default(),
            param_name_map: HashMap::new(),
        };
        
        // Initialize the sample parameters
        dialog.add_sample_parameters();
        
        dialog
    }
    
    /// Set the action to be configured
    pub fn set_action(&mut self, action: Arc<dyn Action>) {
        self.action = Some(action);
        self.name_entry.set_text(action.get_name());
        
        // Clear existing properties
        self.property_grid.clear();
        self.param_name_map.clear();
        
        // TODO: In the future, we should get parameters from the action when that API is ready
        // For now, we'll add some sample parameters
        self.add_sample_parameters();
    }
    
    /// For testing/demo, adds sample parameters to the property grid
    fn add_sample_parameters(&mut self) {
        let sample_params = vec![
            ActionParameter::new("delay", "Delay (ms)", "int", "1000")
                .with_description("Delay in milliseconds before executing the action"),
            ActionParameter::new("repeat", "Repeat Count", "int", "1")
                .with_description("Number of times to repeat the action"),
            ActionParameter::new("condition", "Condition", "string", "")
                .with_description("Condition that must be true for the action to execute"),
            ActionParameter::new("log_level", "Log Level", "enum", "Info")
                .with_options(vec![
                    "Debug".to_string(), 
                    "Info".to_string(), 
                    "Warning".to_string(), 
                    "Error".to_string()
                ])
                .with_description("Severity level for logging"),
            ActionParameter::new("enabled_logging", "Enable Logging", "bool", "true")
                .with_description("Whether to log the action execution"),
        ];
        
        // Add each parameter to the property grid
        for param in sample_params {
            let current_value = self.config.args.iter()
                .find(|arg| arg.starts_with(&format!("{}=", param.name)))
                .map(|arg| arg.splitn(2, '=').nth(1).unwrap_or(""));
                
            // Add to property grid
            let prop = param.to_property(current_value);
            self.property_grid.set_property(
                &prop.get_name(), 
                &self.format_property_value(&prop.get_value()),
                prop.get_value().get_type().to_string().as_str()
            );
            
            // Track parameter name mapping
            self.param_name_map.insert(param.name.clone(), prop.get_name().to_string());
        }
    }
    
    /// Format property value for display
    fn format_property_value(&self, value: &PropertyValue) -> String {
        match value {
            PropertyValue::String(s) => s.clone(),
            PropertyValue::Int(i) => i.to_string(),
            PropertyValue::Float(f) => f.to_string(),
            PropertyValue::Bool(b) => b.to_string(),
            PropertyValue::Enum(s, _) => s.clone(),
            PropertyValue::Color(c) => format!("#{:08x}", c),
            PropertyValue::Custom(_) => "<custom>".to_string(),
        }
    }
    
    /// Configure with existing config values
    pub fn set_config(&mut self, config: ActionConfig) {
        self.config = config.clone();
        self.enabled_check.set_active(config.enabled);
        self.select_on_execute_check.set_active(config.should_select_on_execute);
        
        // Parse args and populate property grid when available
        // Format is typically "name=value"
        for arg in &config.args {
            if let Some((name, value)) = arg.split_once('=') {
                if let Some(display_name) = self.param_name_map.get(name) {
                    // Update the property grid with this value
                    // For a real implementation, we would need to update the actual Property
                    // object and then refresh the grid. For now, we'll assume string properties.
                    self.property_grid.set_property(display_name, value, "string");
                }
            }
        }
    }
    
    /// Run the dialog and return the updated config if OK was pressed
    pub fn run(&self) -> Option<ActionConfig> {
        if self.dialog.run() == ResponseType::Ok {
            let mut config = ActionConfig {
                args: Vec::new(),
                enabled: self.enabled_check.is_active(),
                should_select_on_execute: self.select_on_execute_check.is_active(),
            };
            
            // TODO: Extract values from property grid into config.args
            // In a real implementation, we would iterate through the property grid
            // and convert each property back to a string in "name=value" format
            
            Some(config)
        } else {
            None
        }
    }
    
    /// Run the dialog for a ConfigAction and return the updated action if OK was pressed
    pub fn run_for_config_action(&self, action: &ConfigAction) -> Option<ConfigAction> {
        self.name_entry.set_text(&action.name);
        
        // Set enabled/select flags if they exist in parameters
        if let Some(enabled) = action.parameters.get("enabled") {
            self.enabled_check.set_active(enabled == "true");
        }
        
        if let Some(select) = action.parameters.get("select_on_execute") {
            self.select_on_execute_check.set_active(select == "true");
        }
        
        // Add sample parameters to the property grid for demonstration
        for (name, value) in &action.parameters {
            if name != "enabled" && name != "select_on_execute" {
                if let Some(display_name) = self.param_name_map.get(name) {
                    self.property_grid.set_property(display_name, value, "string");
                }
            }
        }
        
        if self.dialog.run() == ResponseType::Ok {
            // Create a new action with updated values
            let mut parameters = HashMap::new();
            
            // Update basic parameters
            parameters.insert("enabled".to_string(), 
                              self.enabled_check.is_active().to_string());
            parameters.insert("select_on_execute".to_string(), 
                              self.select_on_execute_check.is_active().to_string());
            
            // TODO: In a real implementation, we would extract all values from the property grid
            // For now, we'll just copy over the existing parameters
            for (k, v) in &action.parameters {
                if k != "enabled" && k != "select_on_execute" {
                    parameters.insert(k.clone(), v.clone());
                }
            }
            
            Some(ConfigAction {
                id: action.id,
                name: self.name_entry.text().to_string(),
                parameters,
            })
        } else {
            None
        }
    }
    
    /// Run dialog for a new ConfigAction
    pub fn run_for_new_config_action(&self) -> Option<ConfigAction> {
        // Add sample parameters to the property grid for demonstration
        let mut param_map = HashMap::new();
        param_map.insert("delay".to_string(), "1000".to_string());
        param_map.insert("repeat".to_string(), "1".to_string());
        param_map.insert("condition".to_string(), "".to_string());
        param_map.insert("log_level".to_string(), "Info".to_string());
        param_map.insert("enabled_logging".to_string(), "true".to_string());
        
        for (name, value) in &param_map {
            if let Some(display_name) = self.param_name_map.get(name) {
                self.property_grid.set_property(display_name, value, "string");
            }
        }
        
        if self.dialog.run() == ResponseType::Ok {
            let mut parameters = HashMap::new();
            
            // Add basic parameters
            parameters.insert("enabled".to_string(), 
                              self.enabled_check.is_active().to_string());
            parameters.insert("select_on_execute".to_string(), 
                              self.select_on_execute_check.is_active().to_string());
            
            // Add sample parameters
            parameters.extend(param_map);
            
            Some(ConfigAction {
                id: Uuid::new_v4(),
                name: self.name_entry.text().to_string(),
                parameters,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_action_config_dialog() {
        // Initialize GTK for testing
        gtk::init().expect("Failed to initialize GTK");
        
        // Create a dialog
        let dialog = ActionConfigDialog::new();
        
        // Test that we can create the dialog without errors
        assert!(dialog.dialog.is_visible());
    }
} 