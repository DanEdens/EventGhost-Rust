use std::collections::HashMap;
use std::any::Any;
use std::sync::Arc;
use parking_lot::RwLock;

/// A dynamic property container that can store any type that is Send + Sync
#[derive(Default)]
pub struct Bunch {
    data: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl Bunch {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Set a value in the bunch
    pub fn set<T: 'static + Send + Sync>(&mut self, key: &str, value: T) {
        self.data.insert(key.to_string(), Box::new(value));
    }

    /// Get a reference to a value if it exists and is of the correct type
    pub fn get<T: 'static>(&self, key: &str) -> Option<&T> {
        self.data.get(key)
            .and_then(|value| value.downcast_ref::<T>())
    }

    /// Get a mutable reference to a value if it exists and is of the correct type
    pub fn get_mut<T: 'static>(&mut self, key: &str) -> Option<&mut T> {
        self.data.get_mut(key)
            .and_then(|value| value.downcast_mut::<T>())
    }

    /// Remove a value from the bunch
    pub fn remove(&mut self, key: &str) -> Option<Box<dyn Any + Send + Sync>> {
        self.data.remove(key)
    }

    /// Check if a key exists
    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Clear all values
    pub fn clear(&mut self) {
        self.data.clear();
    }
} 