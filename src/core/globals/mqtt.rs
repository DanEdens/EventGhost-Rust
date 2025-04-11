//! MQTT backend for globals
//! 
//! This provides an MQTT-based implementation of the GlobalsBackend trait.
//! It requires the "globals_mqtt" feature to be enabled.

#![cfg(feature = "globals_mqtt")]

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use async_trait::async_trait;
use rumqttc::{AsyncClient, MqttOptions, QoS, EventLoop};
use tokio::sync::mpsc::{self, Sender, Receiver};
use tokio::task::JoinHandle;

use crate::core::Error;
use super::{GlobalsBackend, GlobalValue};

/// MQTT implementation of GlobalsBackend
pub struct MqttGlobalsBackend {
    /// MQTT client
    client: Arc<AsyncClient>,
    /// Topic prefix
    topic_prefix: String,
    /// Local cache of values
    cache: Arc<RwLock<HashMap<String, GlobalValue>>>,
    /// Subscribers for each key
    subscribers: Arc<RwLock<HashMap<String, Vec<Arc<dyn Fn(String, GlobalValue) + Send + Sync>>>>>,
    /// Sender for the event loop
    event_loop_tx: Option<Sender<()>>,
    /// Event loop handle
    event_loop_handle: Option<JoinHandle<()>>,
}

impl MqttGlobalsBackend {
    /// Create a new MQTT globals backend
    pub fn new(broker_address: &str, topic_prefix: &str) -> Result<Self, Error> {
        let client_id = format!("eventghost-{}", uuid::Uuid::new_v4());
        let mut mqttopts = MqttOptions::new(client_id, broker_address, 1883);
        mqttopts.set_keep_alive(Duration::from_secs(30));
        
        let (client, eventloop) = AsyncClient::new(mqttopts, 10);
        
        let backend = Self {
            client: Arc::new(client),
            topic_prefix: topic_prefix.to_string(),
            cache: Arc::new(RwLock::new(HashMap::new())),
            subscribers: Arc::new(RwLock::new(HashMap::new())),
            event_loop_tx: None,
            event_loop_handle: None,
        };
        
        Ok(backend)
    }
    
    /// Get the full topic name for a key
    fn get_topic(&self, key: &str) -> String {
        format!("{}{}", self.topic_prefix, key)
    }
    
    /// Extract the key from a topic
    fn extract_key(&self, topic: &str) -> Option<String> {
        if topic.starts_with(&self.topic_prefix) {
            Some(topic[self.topic_prefix.len()..].to_string())
        } else {
            None
        }
    }
    
    /// Start the MQTT event loop
    async fn start_event_loop(&mut self) -> Result<(), Error> {
        let client = self.client.clone();
        let cache = self.cache.clone();
        let subscribers = self.subscribers.clone();
        let topic_prefix = self.topic_prefix.clone();
        
        // Subscribe to all topics under the prefix
        let wildcard_topic = format!("{}#", self.topic_prefix);
        self.client.subscribe(wildcard_topic, QoS::AtLeastOnce).await
            .map_err(|e| Error::Mqtt(format!("Failed to subscribe to MQTT topics: {}", e)))?;
        
        let (tx, mut rx) = mpsc::channel::<()>(1);
        self.event_loop_tx = Some(tx);
        
        // Start the event loop
        let mut eventloop = EventLoop::new((*client).clone(), 10);
        
        let handle = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = rx.recv() => {
                        // Shutdown signal received
                        break;
                    }
                    event_result = eventloop.poll() => {
                        if let Ok(notification) = event_result {
                            if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish)) = notification {
                                let topic = publish.topic;
                                if let Some(key) = MqttGlobalsBackend::extract_key_static(&topic_prefix, &topic) {
                                    if let Ok(value) = serde_json::from_slice::<GlobalValue>(&publish.payload) {
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
                        } else {
                            // Error occurred, but we'll just keep trying
                            tokio::time::sleep(Duration::from_millis(100)).await;
                        }
                    }
                }
            }
        });
        
        self.event_loop_handle = Some(handle);
        
        Ok(())
    }
    
    /// Extracts the key part from a topic string using the prefix - static helper
    fn extract_key_static(prefix: &String, topic: &str) -> Option<String> {
        if topic.starts_with(prefix) {
            Some(topic[prefix.len()..].to_string())
        } else {
            None
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
impl GlobalsBackend for MqttGlobalsBackend {
    async fn set(&self, key: &str, value: GlobalValue) -> Result<(), Error> {
        // Update local cache
        {
            let mut cache = self.cache.write().map_err(|_| Error::Concurrency("Failed to acquire write lock".to_string()))?;
            cache.insert(key.to_string(), value.clone());
        }
        
        // Publish to MQTT
        let topic = self.get_topic(key);
        let payload = serde_json::to_vec(&value)
            .map_err(|e| Error::Serialization(format!("Failed to serialize value: {}", e)))?;
        
        self.client.publish(topic, QoS::AtLeastOnce, true, payload).await
            .map_err(|e| Error::Mqtt(format!("Failed to publish to MQTT: {}", e)))?;
        
        // Notify local subscribers (in case they're not receiving MQTT events yet)
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
        
        // If not in cache, try to get from MQTT
        // This is a bit complex as we need to wait for the value to arrive
        // We'll subscribe to the key and wait for a short time
        
        let topic = self.get_topic(key);
        let (tx, mut rx) = mpsc::channel::<GlobalValue>(1);
        
        let tx_clone = tx.clone();
        
        // Create a one-time subscription for this key
        let subscription = async move {
            let mut eventloop = EventLoop::new((*self.client).clone(), 10);
            
            // Wait for up to 2 seconds for a value
            let timeout = tokio::time::sleep(Duration::from_secs(2));
            
            tokio::pin!(timeout);
            
            loop {
                tokio::select! {
                    _ = &mut timeout => {
                        // Timeout reached
                        break;
                    }
                    event_result = eventloop.poll() => {
                        if let Ok(notification) = event_result {
                            if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish)) = notification {
                                if publish.topic == topic {
                                    if let Ok(value) = serde_json::from_slice::<GlobalValue>(&publish.payload) {
                                        let _ = tx_clone.send(value).await;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
        
        // Subscribe to the specific topic
        self.client.subscribe(&topic, QoS::AtLeastOnce).await
            .map_err(|e| Error::Mqtt(format!("Failed to subscribe to MQTT topic: {}", e)))?;
        
        // Start the subscription task
        tokio::spawn(subscription);
        
        // Request the value by publishing to a special topic
        let request_topic = format!("{}__request/{}", self.topic_prefix, key);
        self.client.publish(request_topic, QoS::AtLeastOnce, false, vec![1]).await
            .map_err(|e| Error::Mqtt(format!("Failed to publish request to MQTT: {}", e)))?;
        
        // Wait for the value or timeout
        let timeout = tokio::time::timeout(Duration::from_secs(2), rx.recv()).await;
        
        match timeout {
            Ok(Some(value)) => Ok(value),
            _ => Err(Error::NotFound(format!("Global variable not found: {}", key))),
        }
    }
    
    async fn exists(&self, key: &str) -> Result<bool, Error> {
        // Check local cache first
        if let Ok(cache) = self.cache.read() {
            if cache.contains_key(key) {
                return Ok(true);
            }
        }
        
        // Try to get from MQTT
        match self.get(key).await {
            Ok(_) => Ok(true),
            Err(Error::NotFound(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }
    
    async fn delete(&self, key: &str) -> Result<(), Error> {
        // Remove from local cache
        {
            let mut cache = self.cache.write().map_err(|_| Error::Concurrency("Failed to acquire write lock".to_string()))?;
            cache.remove(key);
        }
        
        // Publish a delete message to MQTT (using retained flag with empty payload)
        let topic = self.get_topic(key);
        self.client.publish(topic, QoS::AtLeastOnce, true, vec![]).await
            .map_err(|e| Error::Mqtt(format!("Failed to publish delete to MQTT: {}", e)))?;
        
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
        if self.event_loop_handle.is_none() {
            // Start the event loop
            let mut this = Self {
                client: self.client.clone(),
                topic_prefix: self.topic_prefix.clone(),
                cache: self.cache.clone(),
                subscribers: self.subscribers.clone(),
                event_loop_tx: None,
                event_loop_handle: None,
            };
            
            this.start_event_loop().await?;
            
            // Since we can't modify self directly due to the async_trait limitations,
            // we'll need to do some extra work to update our fields
            if let Some(tx) = this.event_loop_tx.take() {
                let mut subscribers = self.subscribers.write()
                    .map_err(|_| Error::Concurrency("Failed to acquire write lock".to_string()))?;
                
                // Store the event_loop_tx in a special key in the subscribers map
                // This is a bit of a hack, but it works
                let special_key = format!("__internal_event_loop_tx");
                let tx_box: Box<dyn Fn(String, GlobalValue) + Send + Sync> = Box::new(move |_, _| {
                    // This is a dummy callback that just holds onto the tx
                    let _ = &tx;
                });
                
                let tx_entry = subscribers.entry(special_key).or_insert_with(Vec::new);
                tx_entry.push(Arc::new(tx_box));
            }
        }
        
        Ok(())
    }
    
    async fn shutdown(&self) -> Result<(), Error> {
        // Signal the event loop to stop
        let special_key = format!("__internal_event_loop_tx");
        
        if let Ok(subscribers) = self.subscribers.read() {
            if let Some(subs) = subscribers.get(&special_key) {
                // The first entry should contain our tx sender
                // Get that sender and send a shutdown signal
                // This is a bit of a hack, but it works
                drop(subscribers);
                
                let mut subscribers = self.subscribers.write()
                    .map_err(|_| Error::Concurrency("Failed to acquire write lock".to_string()))?;
                
                subscribers.remove(&special_key);
            }
        }
        
        // Disconnect from MQTT
        if let Err(e) = self.client.disconnect().await {
            // Just log the error, we're shutting down anyway
            log::error!("Error disconnecting from MQTT: {}", e);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // These tests require an MQTT broker to be running
    // We'll skip them unless the MQTT_BROKER environment variable is set
    
    #[tokio::test]
    #[ignore]
    async fn test_mqtt_backend() {
        let broker = std::env::var("MQTT_BROKER").unwrap_or_else(|_| "localhost".to_string());
        let backend = MqttGlobalsBackend::new(&broker, "test/globals/").unwrap();
        backend.initialize().await.unwrap();
        
        // Test set and get
        backend.set("test", GlobalValue::String("hello".to_string())).await.unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await; // Give time for MQTT message to be processed
        
        let value = backend.get("test").await.unwrap();
        assert!(matches!(value, GlobalValue::String(s) if s == "hello"));
        
        // Test delete
        backend.delete("test").await.unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await; // Give time for MQTT message to be processed
        
        // This should fail since we deleted the key
        assert!(backend.get("test").await.is_err());
        
        // Test subscription
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
        
        // Give time for MQTT message to be processed
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        let received_value = received.read().unwrap();
        assert!(*received_value);
        
        backend.shutdown().await.unwrap();
    }
} 
