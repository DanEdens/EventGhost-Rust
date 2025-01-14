use std::collections::HashMap;
use std::any::Any;
use std::sync::{Arc, RwLock};
use crate::core::Error;

#[derive(Default)]
pub struct Bunch {
    values: HashMap<String, Arc<RwLock<Box<dyn Any + Send + Sync>>>>,
}

impl Bunch {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set<T: 'static + Send + Sync>(&mut self, key: &str, value: T) {
        self.values.insert(
            key.to_string(),
            Arc::new(RwLock::new(Box::new(value))),
        );
    }

    pub fn get<T: 'static + Send + Sync>(&self, key: &str) -> Result<Arc<RwLock<Box<dyn Any + Send + Sync>>>, Error> {
        self.values
            .get(key)
            .cloned()
            .ok_or_else(|| Error::Property(format!("Property not found: {}", key)))
    }

    pub fn remove(&mut self, key: &str) -> Option<Arc<RwLock<Box<dyn Any + Send + Sync>>>> {
        self.values.remove(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bunch_operations() {
        let mut bunch = Bunch::new();
        
        // Test setting and getting values
        bunch.set("number", 42);
        bunch.set("text", "Hello".to_string());
        
        assert!(bunch.contains_key("number"));
        assert!(bunch.contains_key("text"));
        
        // Test removing values
        bunch.remove("number");
        assert!(!bunch.contains_key("number"));
        
        // Test clearing
        bunch.clear();
        assert!(!bunch.contains_key("text"));
    }
} 