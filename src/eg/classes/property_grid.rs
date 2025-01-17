use windows::Win32::Foundation::HWND;
use crate::core::Error;
use super::UIComponent;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

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
            .field("validator", &self.validator.as_ref().map(|_| "<validator_fn>"))
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

pub trait PropertySource {
    fn get_properties(&self) -> Vec<Property>;
    fn set_property(&mut self, name: &str, value: PropertyValue) -> Result<(), Error>;
}

pub struct PropertyGrid {
    hwnd: HWND,
    properties: HashMap<String, Property>,
    categories: Vec<String>,
    source: Option<Box<dyn PropertySource>>,
    is_visible: bool,
}

impl PropertyGrid {
    pub fn new(parent: HWND) -> Result<Self, Error> {
        todo!()
    }

    pub fn set_source(&mut self, source: Box<dyn PropertySource>) -> Result<(), Error> {
        todo!()
    }

    pub fn clear_source(&mut self) {
        todo!()
    }

    pub fn refresh(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub fn expand_all(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub fn collapse_all(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub fn get_selected_property(&self) -> Option<&Property> {
        todo!()
    }

    pub fn set_category_sort(&mut self, sort_alphabetically: bool) -> Result<(), Error> {
        todo!()
    }
}

impl UIComponent for PropertyGrid {
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

    struct TestObject {
        name: String,
        count: i32,
        enabled: bool,
    }

    impl PropertySource for TestObject {
        fn get_properties(&self) -> Vec<Property> {
            vec![
                Property::new("name", "General", PropertyValue::String(self.name.clone()))
                    .with_description("The object name"),
                Property::new("count", "General", PropertyValue::Int(self.count))
                    .with_description("Count value"),
                Property::new("enabled", "State", PropertyValue::Bool(self.enabled))
                    .with_description("Enable/disable the object"),
            ]
        }

        fn set_property(&mut self, name: &str, value: PropertyValue) -> Result<(), Error> {
            match (name, value) {
                ("name", PropertyValue::String(s)) => self.name = s,
                ("count", PropertyValue::Int(i)) => self.count = i,
                ("enabled", PropertyValue::Bool(b)) => self.enabled = b,
                _ => return Err(Error::Config("Invalid property".into())),
            }
            Ok(())
        }
    }
} 