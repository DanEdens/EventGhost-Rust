//! Local in-memory backend for globals
//! 
//! This provides a simple in-memory implementation of the GlobalsBackend trait.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::collections::HashSet;
use async_trait::async_trait;

use crate::core::Error;
use super::{GlobalsBackend, GlobalValue};

/// In-memory implementation of GlobalsBackend
pub struct LocalGlobalsBackend {
    /// Values store
    values: Arc<RwLock<HashMap<String, GlobalValue>>>,
    /// Subscribers for each key
    subscribers: Arc<RwLock<HashMap<String, Vec<Arc<dyn Fn(String, GlobalValue) + Send + Sync>>>>>,
}

impl LocalGlobalsBackend {
    /// Create a new local globals backend
    pub fn new() -> Self {
        Self {
            values: Arc::new(RwLock::new(HashMap::new())),
            subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Notify subscribers of a key change
    fn notify_subscribers(&self, key: &str, value: GlobalValue) {
        if let Ok(subscribers) = self.subscribers.read() {
            if let Some(subs) = subscribers.get(key) {
                let key_owned = key.to_string();
                let value_owned = value.clone();
                for callback in subs {
                    callback(key_owned.clone(), value_owned.clone());
                }
            }
        }
    }
}

#[async_trait]
impl GlobalsBackend for LocalGlobalsBackend {
    async fn set(&self, key: &str, value: GlobalValue) -> Result<(), Error> {
        let mut values = self.values.write().map_err(|_| Error::Concurrency("Failed to acquire write lock".to_string()))?;
        values.insert(key.to_string(), value.clone());
        drop(values); // Release the lock before notifying subscribers
        
        self.notify_subscribers(key, value);
        Ok(())
    }
    
    async fn get(&self, key: &str) -> Result<GlobalValue, Error> {
        let values = self.values.read().map_err(|_| Error::Concurrency("Failed to acquire read lock".to_string()))?;
        values.get(key)
            .cloned()
            .ok_or_else(|| Error::NotFound(format!("Global variable not found: {}", key)))
    }
    
    async fn exists(&self, key: &str) -> Result<bool, Error> {
        let values = self.values.read().map_err(|_| Error::Concurrency("Failed to acquire read lock".to_string()))?;
        Ok(values.contains_key(key))
    }
    
    async fn delete(&self, key: &str) -> Result<(), Error> {
        let mut values = self.values.write().map_err(|_| Error::Concurrency("Failed to acquire write lock".to_string()))?;
        values.remove(key);
        Ok(())
    }
    
    async fn subscribe(&self, key: &str, callback: Arc<dyn Fn(String, GlobalValue) + Send + Sync>) -> Result<(), Error> {
        let mut subscribers = self.subscribers.write()
            .map_err(|_| Error::Concurrency("Failed to acquire write lock".to_string()))?;
        
        let entry = subscribers.entry(key.to_string()).or_insert_with(Vec::new);
        entry.push(callback);
        
        Ok(())
    }
    
    async fn unsubscribe(&self, key: &str) -> Result<(), Error> {
        let mut subscribers = self.subscribers.write()
            .map_err(|_| Error::Concurrency("Failed to acquire write lock".to_string()))?;
        
        subscribers.remove(key);
        
        Ok(())
    }
    
    async fn initialize(&self) -> Result<(), Error> {
        // Nothing to initialize for local backend
        Ok(())
    }
    
    async fn shutdown(&self) -> Result<(), Error> {
        // Nothing to shut down for local backend
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_local_backend() {
        let backend = LocalGlobalsBackend::new();
        
        // Test set and get
        backend.set("test", GlobalValue::String("hello".to_string())).await.unwrap();
        let value = backend.get("test").await.unwrap();
        assert!(matches!(value, GlobalValue::String(s) if s == "hello"));
        
        // Test exists
        assert!(backend.exists("test").await.unwrap());
        assert!(!backend.exists("nonexistent").await.unwrap());
        
        // Test delete
        backend.delete("test").await.unwrap();
        assert!(!backend.exists("test").await.unwrap());
        
        // Test subscribe
        let received = Arc::new(RwLock::new(false));
        let received_clone = received.clone();
        let callback = move |k: String, v: GlobalValue| {
            assert_eq!(k, "subscribed");
            if let GlobalValue::Integer(i) = v {
                assert_eq!(i, 42);
                let mut received = received_clone.write().unwrap();
                *received = true;
            }
        };
        
        backend.subscribe("subscribed", Arc::new(callback)).await.unwrap();
        backend.set("subscribed", GlobalValue::Integer(42)).await.unwrap();
        
        // Give a little time for the callback to execute
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        
        let received_value = received.read().unwrap();
        assert!(*received_value);
    }
} 
