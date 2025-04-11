//! Redis backend for globals
//! 
//! This provides a Redis-based implementation of the GlobalsBackend trait.
//! It requires the "globals_redis" feature to be enabled.

#![cfg(feature = "globals_redis")]

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use async_trait::async_trait;
use redis::{Client, AsyncCommands, Connection, aio::ConnectionManager};
use tokio::sync::mpsc::{self, Sender, Receiver};
use tokio::task::JoinHandle;

use crate::core::Error;
use super::{GlobalsBackend, GlobalValue};

/// Redis implementation of GlobalsBackend
pub struct RedisGlobalsBackend {
    /// Redis client
    client: Client,
    /// Connection manager
    connection: Arc<RwLock<Option<ConnectionManager>>>,
    /// Pub/sub connection for subscribing to events
    pubsub_conn: Arc<RwLock<Option<Connection>>>,
    /// Key prefix
    key_prefix: String,
    /// Local cache of values
    cache: Arc<RwLock<HashMap<String, GlobalValue>>>,
    /// Subscribers for each key
    subscribers: Arc<RwLock<HashMap<String, Vec<Arc<dyn Fn(String, GlobalValue) + Send + Sync>>>>>,
    /// Sender for the pub/sub loop
    pubsub_tx: Arc<RwLock<Option<Sender<()>>>>,
    /// Pub/sub loop handle
    pubsub_handle: Arc<RwLock<Option<JoinHandle<()>>>>,
}

impl RedisGlobalsBackend {
    /// Create a new Redis globals backend
    pub fn new(redis_url: &str, key_prefix: &str) -> Result<Self, Error> {
        let client = Client::open(redis_url)
            .map_err(|e| Error::Redis(format!("Failed to connect to Redis: {}", e)))?;
        
        Ok(Self {
            client,
            connection: Arc::new(RwLock::new(None)),
            pubsub_conn: Arc::new(RwLock::new(None)),
            key_prefix: key_prefix.to_string(),
            cache: Arc::new(RwLock::new(HashMap::new())),
            subscribers: Arc::new(RwLock::new(HashMap::new())),
            pubsub_tx: Arc::new(RwLock::new(None)),
            pubsub_handle: Arc::new(RwLock::new(None)),
        })
    }
    
    /// Get the full key for a Redis key
    fn get_key(&self, key: &str) -> String {
        format!("{}{}", self.key_prefix, key)
    }
    
    /// Get the channel name for key changes
    fn get_channel(&self, key: &str) -> String {
        format!("{}__events:{}", self.key_prefix, key)
    }
    
    /// Extract the key from a channel name
    fn extract_key(&self, channel: &str) -> Option<String> {
        let prefix = format!("{}__events:", self.key_prefix);
        if channel.starts_with(&prefix) {
            Some(channel[prefix.len()..].to_string())
        } else {
            None
        }
    }
    
    /// Start the Redis pub/sub loop
    async fn start_pubsub_loop(&self) -> Result<(), Error> {
        let mut pubsub_conn = self.client.get_connection()
            .map_err(|e| Error::Redis(format!("Failed to get Redis pubsub connection: {}", e)))?;
        
        // Store the connection
        {
            let mut conn_guard = self.pubsub_conn.write()
                .map_err(|_| Error::Concurrency("Failed to acquire write lock".to_string()))?;
            *conn_guard = Some(pubsub_conn);
        }
        
        // We need to get the connection again for the pub/sub loop
        let mut pubsub_conn = self.client.get_connection()
            .map_err(|e| Error::Redis(format!("Failed to get Redis pubsub connection: {}", e)))?;
        
        let cache = self.cache.clone();
        let subscribers = self.subscribers.clone();
        let key_prefix = self.key_prefix.clone();
        
        // Subscribe to the pattern for all keys
        let pattern = format!("{}__events:*", self.key_prefix);
        let mut pubsub = pubsub_conn.as_pubsub();
        
        pubsub.psubscribe(&pattern)
            .map_err(|e| Error::Redis(format!("Failed to subscribe to Redis pattern: {}", e)))?;
        
        let (tx, mut rx) = mpsc::channel::<()>(1);
        
        // Store the sender
        {
            let mut tx_guard = self.pubsub_tx.write()
                .map_err(|_| Error::Concurrency("Failed to acquire write lock".to_string()))?;
            *tx_guard = Some(tx);
        }
        
        // Start the pub/sub loop
        let handle = tokio::spawn(async move {
            loop {
                // Check for shutdown signal
                if rx.try_recv().is_ok() {
                    break;
                }
                
                // Poll for messages
                let msg = pubsub.get_message();
                match msg {
                    Ok(msg) => {
                        let channel: String = msg.get_channel_name().unwrap_or_default();
                        if channel.starts_with(&pattern) {
                            if let Ok(payload) = msg.get_payload::<String>() {
                                // Extract the key from the channel
                                if let Some(key) = Self::extract_key_static(&key_prefix, &channel) {
                                    // Try to deserialize the value
                                    if let Ok(value) = serde_json::from_str::<GlobalValue>(&payload) {
                                        // Update cache
                                        if let Ok(mut cache_guard) = cache.write() {
                                            cache_guard.insert(key.clone(), value.clone());
                                        }
                                        
                                        // Notify subscribers
                                        if let Ok(subscribers_guard) = subscribers.read() {
                                            if let Some(subs) = subscribers_guard.get(&key) {
                                                for callback in subs {
                                                    callback(key.clone(), value.clone());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // Just a timeout, keep looping
                        std::thread::sleep(Duration::from_millis(10));
                    }
                }
            }
        });
        
        // Store the handle
        {
            let mut handle_guard = self.pubsub_handle.write()
                .map_err(|_| Error::Concurrency("Failed to acquire write lock".to_string()))?;
            *handle_guard = Some(handle);
        }
        
        Ok(())
    }
    
    /// Extracts the key part from a channel string using the prefix - static helper
    fn extract_key_static(prefix: &String, channel: &str) -> Option<String> {
        let events_prefix = format!("{}__events:", prefix);
        if channel.starts_with(&events_prefix) {
            Some(channel[events_prefix.len()..].to_string())
        } else {
            None
        }
    }
    
    /// Get a connection to Redis
    async fn get_connection(&self) -> Result<ConnectionManager, Error> {
        // Check if we already have a connection
        if let Ok(conn_guard) = self.connection.read() {
            if let Some(conn) = &*conn_guard {
                return Ok(conn.clone());
            }
        }
        
        // Create a new connection
        let conn = ConnectionManager::new(self.client.clone())
            .await
            .map_err(|e| Error::Redis(format!("Failed to get Redis connection: {}", e)))?;
        
        // Store it
        {
            let mut conn_guard = self.connection.write()
                .map_err(|_| Error::Concurrency("Failed to acquire write lock".to_string()))?;
            *conn_guard = Some(conn.clone());
        }
        
        Ok(conn)
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
impl GlobalsBackend for RedisGlobalsBackend {
    async fn set(&self, key: &str, value: GlobalValue) -> Result<(), Error> {
        // Update local cache
        {
            let mut cache = self.cache.write()
                .map_err(|_| Error::Concurrency("Failed to acquire write lock".to_string()))?;
            cache.insert(key.to_string(), value.clone());
        }
        
        // Convert value to JSON
        let json = serde_json::to_string(&value)
            .map_err(|e| Error::Serialization(format!("Failed to serialize value: {}", e)))?;
        
        // Get a Redis connection
        let mut conn = self.get_connection().await?;
        
        // Set the value in Redis
        let redis_key = self.get_key(key);
        conn.set::<_, _, ()>(&redis_key, &json)
            .await
            .map_err(|e| Error::Redis(format!("Failed to set value in Redis: {}", e)))?;
        
        // Publish an event
        let channel = self.get_channel(key);
        conn.publish::<_, _, ()>(&channel, &json)
            .await
            .map_err(|e| Error::Redis(format!("Failed to publish event to Redis: {}", e)))?;
        
        // Notify local subscribers
        self.notify_subscribers(key, value);
        
        Ok(())
    }
    
    async fn get(&self, key: &str) -> Result<GlobalValue, Error> {
        // Try to get from local cache first
        if let Ok(cache) = self.cache.read() {
            if let Some(value) = cache.get(key) {
                return Ok(value.clone());
            }
        }
        
        // Get from Redis
        let mut conn = self.get_connection().await?;
        
        let redis_key = self.get_key(key);
        let json: Option<String> = conn.get(&redis_key)
            .await
            .map_err(|e| Error::Redis(format!("Failed to get value from Redis: {}", e)))?;
        
        if let Some(json) = json {
            let value = serde_json::from_str(&json)
                .map_err(|e| Error::Deserialization(format!("Failed to deserialize value: {}", e)))?;
            
            // Update cache
            {
                let mut cache = self.cache.write()
                    .map_err(|_| Error::Concurrency("Failed to acquire write lock".to_string()))?;
                cache.insert(key.to_string(), value.clone());
            }
            
            Ok(value)
        } else {
            Err(Error::NotFound(format!("Global variable not found: {}", key)))
        }
    }
    
    async fn exists(&self, key: &str) -> Result<bool, Error> {
        // Check local cache first
        if let Ok(cache) = self.cache.read() {
            if cache.contains_key(key) {
                return Ok(true);
            }
        }
        
        // Check Redis
        let mut conn = self.get_connection().await?;
        
        let redis_key = self.get_key(key);
        let exists: bool = conn.exists(&redis_key)
            .await
            .map_err(|e| Error::Redis(format!("Failed to check if key exists in Redis: {}", e)))?;
        
        Ok(exists)
    }
    
    async fn delete(&self, key: &str) -> Result<(), Error> {
        // Remove from local cache
        {
            let mut cache = self.cache.write()
                .map_err(|_| Error::Concurrency("Failed to acquire write lock".to_string()))?;
            cache.remove(key);
        }
        
        // Delete from Redis
        let mut conn = self.get_connection().await?;
        
        let redis_key = self.get_key(key);
        conn.del::<_, ()>(&redis_key)
            .await
            .map_err(|e| Error::Redis(format!("Failed to delete key from Redis: {}", e)))?;
        
        // Publish a delete event
        let channel = self.get_channel(key);
        conn.publish::<_, _, ()>(&channel, "null")
            .await
            .map_err(|e| Error::Redis(format!("Failed to publish delete event to Redis: {}", e)))?;
        
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
        // Start the pub/sub loop if it's not already running
        if let Ok(handle_guard) = self.pubsub_handle.read() {
            if handle_guard.is_none() {
                drop(handle_guard);
                self.start_pubsub_loop().await?;
            }
        }
        
        Ok(())
    }
    
    async fn shutdown(&self) -> Result<(), Error> {
        // Signal the pub/sub loop to stop
        if let Ok(tx_guard) = self.pubsub_tx.read() {
            if let Some(tx) = &*tx_guard {
                let _ = tx.send(()).await;
            }
        }
        
        // Wait for the pub/sub loop to finish
        if let Ok(mut handle_guard) = self.pubsub_handle.write() {
            if let Some(handle) = handle_guard.take() {
                if !handle.is_finished() {
                    let _ = tokio::time::timeout(Duration::from_secs(1), handle).await;
                }
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // These tests require a Redis server to be running
    // We'll skip them unless the REDIS_URL environment variable is set
    
    #[tokio::test]
    #[ignore]
    async fn test_redis_backend() {
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost".to_string());
        let backend = RedisGlobalsBackend::new(&redis_url, "test:globals:").unwrap();
        backend.initialize().await.unwrap();
        
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
        
        // Test subscription (this requires the pubsub loop to be working)
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
        
        // Give time for Redis message to be processed
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let received_value = received.read().unwrap();
        assert!(*received_value);
        
        // Clean up
        backend.delete("subscribed").await.unwrap();
        backend.shutdown().await.unwrap();
    }
} 
