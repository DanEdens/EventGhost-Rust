//! Globals module for EventGhost
//! 
//! This module provides a global variable store that can be accessed from anywhere
//! in the application. It supports local in-memory storage as well as optional
//! distributed storage using MQTT or Redis.

mod mqtt;
mod redis;
mod local;

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::core::Error;
use self::local::LocalGlobalsBackend;

#[cfg(feature = "globals_mqtt")]
use self::mqtt::MqttGlobalsBackend;

#[cfg(feature = "globals_redis")]
use self::redis::RedisGlobalsBackend;

/// Value types that can be stored in the globals system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GlobalValue {
    /// String value
    String(String),
    /// Integer value
    Integer(i64),
    /// Floating point value
    Float(f64),
    /// Boolean value
    Boolean(bool),
    /// Binary data
    Binary(Vec<u8>),
    /// JSON object (serialized)
    Json(String),
}

impl GlobalValue {
    /// Get the value as a string if possible
    pub fn as_string(&self) -> Option<String> {
        match self {
            GlobalValue::String(s) => Some(s.clone()),
            GlobalValue::Integer(i) => Some(i.to_string()),
            GlobalValue::Float(f) => Some(f.to_string()),
            GlobalValue::Boolean(b) => Some(b.to_string()),
            GlobalValue::Json(j) => Some(j.clone()),
            _ => None,
        }
    }

    /// Get the value as an integer if possible
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            GlobalValue::Integer(i) => Some(*i),
            GlobalValue::Float(f) => Some(*f as i64),
            GlobalValue::String(s) => s.parse::<i64>().ok(),
            GlobalValue::Boolean(b) => Some(if *b { 1 } else { 0 }),
            _ => None,
        }
    }

    /// Get the value as a float if possible
    pub fn as_float(&self) -> Option<f64> {
        match self {
            GlobalValue::Float(f) => Some(*f),
            GlobalValue::Integer(i) => Some(*i as f64),
            GlobalValue::String(s) => s.parse::<f64>().ok(),
            GlobalValue::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
            _ => None,
        }
    }

    /// Get the value as a boolean if possible
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            GlobalValue::Boolean(b) => Some(*b),
            GlobalValue::Integer(i) => Some(*i != 0),
            GlobalValue::Float(f) => Some(*f != 0.0),
            GlobalValue::String(s) => {
                match s.to_lowercase().as_str() {
                    "true" | "yes" | "1" | "on" => Some(true),
                    "false" | "no" | "0" | "off" => Some(false),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Get the value as binary data if possible
    pub fn as_binary(&self) -> Option<Vec<u8>> {
        match self {
            GlobalValue::Binary(b) => Some(b.clone()),
            GlobalValue::String(s) => Some(s.as_bytes().to_vec()),
            _ => None,
        }
    }
}

/// Backend trait for globals storage
#[async_trait]
pub trait GlobalsBackend: Send + Sync {
    /// Set a value
    async fn set(&self, key: &str, value: GlobalValue) -> Result<(), Error>;
    
    /// Get a value
    async fn get(&self, key: &str) -> Result<GlobalValue, Error>;
    
    /// Check if a key exists
    async fn exists(&self, key: &str) -> Result<bool, Error>;
    
    /// Delete a key
    async fn delete(&self, key: &str) -> Result<(), Error>;
    
    /// Subscribe to key changes
    async fn subscribe(&self, key: &str, callback: Arc<dyn Fn(String, GlobalValue) + Send + Sync>) -> Result<(), Error>;
    
    /// Unsubscribe from key changes
    async fn unsubscribe(&self, key: &str) -> Result<(), Error>;
    
    /// Initialize the backend
    async fn initialize(&self) -> Result<(), Error>;
    
    /// Shutdown the backend
    async fn shutdown(&self) -> Result<(), Error>;
}

/// GlobalsStore configuration
#[derive(Debug, Clone)]
pub struct GlobalsConfig {
    /// Backend type to use
    pub backend_type: GlobalsBackendType,
    
    /// MQTT broker address (if using MQTT)
    pub mqtt_broker: Option<String>,
    
    /// MQTT topic prefix (if using MQTT)
    pub mqtt_topic_prefix: Option<String>,
    
    /// Redis URL (if using Redis)
    pub redis_url: Option<String>,
    
    /// Redis key prefix (if using Redis)
    pub redis_key_prefix: Option<String>,
}

impl Default for GlobalsConfig {
    fn default() -> Self {
        Self {
            backend_type: GlobalsBackendType::Local,
            mqtt_broker: None,
            mqtt_topic_prefix: Some("eventghost/globals/".to_string()),
            redis_url: None,
            redis_key_prefix: Some("eventghost:globals:".to_string()),
        }
    }
}

/// Backend types for the globals system
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlobalsBackendType {
    /// Local in-memory storage
    Local,
    /// MQTT-based distributed storage
    #[cfg(feature = "globals_mqtt")]
    Mqtt,
    /// Redis-based distributed storage
    #[cfg(feature = "globals_redis")]
    Redis,
}

/// Global variable store
pub struct GlobalsStore {
    /// Backend for storage
    backend: Arc<dyn GlobalsBackend>,
}

impl GlobalsStore {
    /// Create a new globals store with the specified configuration
    pub async fn new(config: GlobalsConfig) -> Result<Self, Error> {
        let backend: Arc<dyn GlobalsBackend> = match config.backend_type {
            GlobalsBackendType::Local => {
                Arc::new(LocalGlobalsBackend::new())
            },
            #[cfg(feature = "globals_mqtt")]
            GlobalsBackendType::Mqtt => {
                let broker = config.mqtt_broker
                    .ok_or_else(|| Error::Configuration("MQTT broker address not provided".to_string()))?;
                let prefix = config.mqtt_topic_prefix
                    .unwrap_or_else(|| "eventghost/globals/".to_string());
                Arc::new(MqttGlobalsBackend::new(&broker, &prefix)?)
            },
            #[cfg(feature = "globals_redis")]
            GlobalsBackendType::Redis => {
                let url = config.redis_url
                    .ok_or_else(|| Error::Configuration("Redis URL not provided".to_string()))?;
                let prefix = config.redis_key_prefix
                    .unwrap_or_else(|| "eventghost:globals:".to_string());
                Arc::new(RedisGlobalsBackend::new(&url, &prefix)?)
            },
        };
        
        backend.initialize().await?;
        
        Ok(Self {
            backend,
        })
    }
    
    /// Set a string value
    pub async fn set_string(&self, key: &str, value: String) -> Result<(), Error> {
        self.backend.set(key, GlobalValue::String(value)).await
    }
    
    /// Set an integer value
    pub async fn set_integer(&self, key: &str, value: i64) -> Result<(), Error> {
        self.backend.set(key, GlobalValue::Integer(value)).await
    }
    
    /// Set a float value
    pub async fn set_float(&self, key: &str, value: f64) -> Result<(), Error> {
        self.backend.set(key, GlobalValue::Float(value)).await
    }
    
    /// Set a boolean value
    pub async fn set_boolean(&self, key: &str, value: bool) -> Result<(), Error> {
        self.backend.set(key, GlobalValue::Boolean(value)).await
    }
    
    /// Set binary data
    pub async fn set_binary(&self, key: &str, value: Vec<u8>) -> Result<(), Error> {
        self.backend.set(key, GlobalValue::Binary(value)).await
    }
    
    /// Set a JSON value
    pub async fn set_json<T: Serialize>(&self, key: &str, value: &T) -> Result<(), Error> {
        let json = serde_json::to_string(value)
            .map_err(|e| Error::Serialization(e.to_string()))?;
        self.backend.set(key, GlobalValue::Json(json)).await
    }
    
    /// Get a value
    pub async fn get(&self, key: &str) -> Result<GlobalValue, Error> {
        self.backend.get(key).await
    }
    
    /// Get a string value
    pub async fn get_string(&self, key: &str) -> Result<String, Error> {
        let value = self.backend.get(key).await?;
        value.as_string().ok_or_else(|| Error::Type("Value is not a string".to_string()))
    }
    
    /// Get an integer value
    pub async fn get_integer(&self, key: &str) -> Result<i64, Error> {
        let value = self.backend.get(key).await?;
        value.as_integer().ok_or_else(|| Error::Type("Value is not an integer".to_string()))
    }
    
    /// Get a float value
    pub async fn get_float(&self, key: &str) -> Result<f64, Error> {
        let value = self.backend.get(key).await?;
        value.as_float().ok_or_else(|| Error::Type("Value is not a float".to_string()))
    }
    
    /// Get a boolean value
    pub async fn get_boolean(&self, key: &str) -> Result<bool, Error> {
        let value = self.backend.get(key).await?;
        value.as_boolean().ok_or_else(|| Error::Type("Value is not a boolean".to_string()))
    }
    
    /// Get binary data
    pub async fn get_binary(&self, key: &str) -> Result<Vec<u8>, Error> {
        let value = self.backend.get(key).await?;
        value.as_binary().ok_or_else(|| Error::Type("Value is not binary data".to_string()))
    }
    
    /// Get a JSON value
    pub async fn get_json<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<T, Error> {
        let value = self.backend.get(key).await?;
        let json = value.as_string()
            .ok_or_else(|| Error::Type("Value is not a JSON string".to_string()))?;
        serde_json::from_str(&json)
            .map_err(|e| Error::Deserialization(e.to_string()))
    }
    
    /// Check if a key exists
    pub async fn exists(&self, key: &str) -> Result<bool, Error> {
        self.backend.exists(key).await
    }
    
    /// Delete a key
    pub async fn delete(&self, key: &str) -> Result<(), Error> {
        self.backend.delete(key).await
    }
    
    /// Subscribe to changes for a key
    pub async fn subscribe<F>(&self, key: &str, callback: F) -> Result<(), Error>
    where
        F: Fn(String, GlobalValue) + Send + Sync + 'static,
    {
        self.backend.subscribe(key, Arc::new(callback)).await
    }
    
    /// Unsubscribe from changes for a key
    pub async fn unsubscribe(&self, key: &str) -> Result<(), Error> {
        self.backend.unsubscribe(key).await
    }
    
    /// Shutdown the globals store
    pub async fn shutdown(&self) -> Result<(), Error> {
        self.backend.shutdown().await
    }
}

// Add to tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_local_globals() {
        let config = GlobalsConfig::default();
        let globals = GlobalsStore::new(config).await.unwrap();
        
        // Test string values
        globals.set_string("test_string", "hello".to_string()).await.unwrap();
        let value = globals.get_string("test_string").await.unwrap();
        assert_eq!(value, "hello");
        
        // Test integer values
        globals.set_integer("test_int", 42).await.unwrap();
        let value = globals.get_integer("test_int").await.unwrap();
        assert_eq!(value, 42);
        
        // Test float values
        globals.set_float("test_float", 3.14).await.unwrap();
        let value = globals.get_float("test_float").await.unwrap();
        assert_eq!(value, 3.14);
        
        // Test boolean values
        globals.set_boolean("test_bool", true).await.unwrap();
        let value = globals.get_boolean("test_bool").await.unwrap();
        assert_eq!(value, true);
        
        // Test JSON values
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct TestStruct {
            name: String,
            age: i32,
        }
        
        let test_struct = TestStruct {
            name: "Test".to_string(),
            age: 30,
        };
        
        globals.set_json("test_json", &test_struct).await.unwrap();
        let value: TestStruct = globals.get_json("test_json").await.unwrap();
        assert_eq!(value.name, "Test");
        assert_eq!(value.age, 30);
        
        // Test exists and delete
        assert!(globals.exists("test_string").await.unwrap());
        globals.delete("test_string").await.unwrap();
        assert!(!globals.exists("test_string").await.unwrap());
    }
} 
