use crate::prelude::*;
use crate::prelude::{self, Box, TreeView, TreeStore, TreeViewColumn, CellRendererText};
use gtk::glib;
use super::UIComponent;
use crate::core::Error;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum PropertyValueType {
    String,
    Int,
    Float,
    Bool,
    Color,
    Enum,
    Custom
}

#[derive(Debug, Clone)]
pub enum PropertyValue {
    String(String),
    Int(i32),
    Float(f64),
    Bool(bool),
    Color(u32),
    Enum(String, Vec<String>),
    Custom(Arc<dyn Any + Send + Sync>),
}

impl PropertyValue {
    pub fn get_type(&self) -> PropertyValueType {
        match self {
            PropertyValue::String(_) => PropertyValueType::String,
            PropertyValue::Int(_) => PropertyValueType::Int,
            PropertyValue::Float(_) => PropertyValueType::Float,
            PropertyValue::Bool(_) => PropertyValueType::Bool,
            PropertyValue::Color(_) => PropertyValueType::Color,
            PropertyValue::Enum(_, _) => PropertyValueType::Enum,
            PropertyValue::Custom(_) => PropertyValueType::Custom,
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        if let PropertyValue::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_int(&self) -> Option<i32> {
        if let PropertyValue::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let PropertyValue::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            PropertyValue::String(s) => s.clone(),
            PropertyValue::Int(i) => i.to_string(),
            PropertyValue::Float(f) => f.to_string(),
            PropertyValue::Bool(b) => b.to_string(),
            PropertyValue::Enum(s, _) => s.clone(),
            PropertyValue::Color(c) => format!("#{:08x}", c),
            PropertyValue::Custom(_) => "<custom>".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Property {
    name: String,
    category: String,
    description: String,
    value: PropertyValue,
    readonly: bool,
    validator: Option<Arc<dyn Fn(&PropertyValue) -> Result<(), String> + Send + Sync>>,
}

impl std::fmt::Debug for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Property")
            .field("name", &self.name)
            .field("category", &self.category)
            .field("description", &self.description)
            .field("value", &self.value)
            .field("readonly", &self.readonly)
            .field("validator", &"<validator_fn>")
            .finish()
    }
}


impl Property {
    pub fn new(name: &str, category: &str, value: PropertyValue) -> Self {
        Self {
            name: name.to_string(),
            category: category.to_string(),
            description: String::new(),
            value,
            readonly: false,
            validator: None,
        }
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    pub fn with_validator<F>(mut self, validator: F) -> Self 
    where
        F: Fn(&PropertyValue) -> Result<(), String> + Send + Sync + 'static
    {
        self.validator = Some(Arc::new(validator));
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if let Some(validator) = &self.validator {
            validator(&self.value)
        } else {
            Ok(())
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_category(&self) -> &str {
        &self.category
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn is_readonly(&self) -> bool {
        self.readonly
    }

    pub fn get_value(&self) -> &PropertyValue {
        &self.value
    }
}

pub trait PropertySource: Send + Sync + Debug {
    fn get_property(&self, name: &str) -> Option<Property>;
    fn set_property(&mut self, name: &str, value: Property) -> Result<(), Error>;
    fn get_properties(&self) -> Vec<Property>;
}

#[derive(Debug, Clone)]
pub struct PropertyGrid {
    pub widget: Box,
    pub tree_view: TreeView,
    pub store: TreeStore,
}

impl PropertyGrid {
    pub fn new() -> Self {
        let widget = Box::new(gtk::Orientation::Vertical, 0);
        let store = TreeStore::new(&[
            glib::Type::STRING, // Name
            glib::Type::STRING, // Value
            glib::Type::STRING, // Type
        ]);
        
        let tree_view = TreeView::new();
        tree_view.set_model(Some(&store));
        
        // Add columns
        let name_renderer = CellRendererText::new();
        let name_column = TreeViewColumn::new();
        name_column.pack_start(&name_renderer, true);
        name_column.add_attribute(&name_renderer, "text", 0);
        name_column.set_title("Property");
        tree_view.append_column(&name_column);
        
        let value_renderer = CellRendererText::new();
        let value_column = TreeViewColumn::new();
        value_column.pack_start(&value_renderer, true);
        value_column.add_attribute(&value_renderer, "text", 1);
        value_column.set_title("Value");
        tree_view.append_column(&value_column);
        
        widget.append(&tree_view);
        
        PropertyGrid {
            widget,
            tree_view,
            store,
        }
    }
    
    pub fn set_property(&self, name: &str, value: &str, prop_type: &str) {
        let iter = self.store.append(None);
        self.store.set(&iter, &[
            (0, &name),
            (1, &value),
            (2, &prop_type),
        ]);
    }
    
    pub fn clear(&self) {
        self.store.clear();
    }
    
    /// Get the value of a property by its name
    pub fn get_property_value(&self, name: &str) -> Option<String> {
        if let Some(model) = self.tree_view.model() {
            // Iterate through all rows to find the property
            if let Some(iter) = model.iter_first() {
                let mut current_iter = iter;
                loop {
                    let property_name: String = model.get(&current_iter, 0);
                    if property_name == name {
                        return Some(model.get(&current_iter, 1));
                    }
                    
                    if !model.iter_next(&current_iter) {
                        break;
                    }
                }
            }
        }
        
        None
    }
    
    /// Update the value of a property by name
    pub fn update_property_value(&self, name: &str, value: &str) -> bool {
        if let Some(model) = self.tree_view.model() {
            // Iterate through all rows to find the property
            if let Some(iter) = model.iter_first() {
                let mut current_iter = iter;
                loop {
                    let property_name: String = model.get(&current_iter, 0);
                    if property_name == name {
                        self.store.set(&current_iter, &[(1, &value)]);
                        return true;
                    }
                    
                    if !model.iter_next(&current_iter) {
                        break;
                    }
                }
            }
        }
        
        false
    }
}

impl UIComponent for PropertyGrid {
    fn get_widget(&self) -> &gtk::Widget {
        self.widget.upcast_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_property_grid_initialization() {
        gtk::init().expect("Failed to initialize GTK");
        
        let grid = PropertyGrid::new();
        assert!(grid.widget.is_visible());
    }
} 
