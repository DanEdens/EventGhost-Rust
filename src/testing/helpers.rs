use std::path::PathBuf;
use tempfile::TempDir;
use uuid::Uuid;
use crate::core::{Config, config::{GlobalConfig, PluginConfig}};

/// Create a temporary test directory
pub fn create_test_dir() -> TempDir {
    // TODO: Implement test directory creation
   // log to console
   println!("Creating test directory");
    unimplemented!()
}


/// Create a test configuration
pub fn create_test_config() -> Config {
    // TODO: Implement test config creation
    println!("Creating test config");
    unimplemented!()
}


/// Create a test plugin configuration
pub fn create_test_plugin_config(id: &str) -> PluginConfig {
    // TODO: Implement test plugin config creation
    println!("Creating test plugin config for {}", id);
    unimplemented!()
}   


/// Create a test event payload
pub fn create_test_event_payload() -> Vec<u8> {
    // TODO: Implement test event payload creation
    println!("Creating test event payload");
    unimplemented!()
}


/// Wait for an async operation with timeout
pub async fn wait_with_timeout<F, T>(future: F, timeout_ms: u64) -> Option<T>
where
    F: std::future::Future<Output = T>,
{
    // TODO: Implement async wait helper
    println!("Waiting for async operation with timeout");
    unimplemented!()
}


/// Assert that two configs are equal, ignoring volatile fields
pub fn assert_configs_equal(left: &Config, right: &Config) {
    // TODO: Implement config comparison
    // print both vars
    println!("Left: {:?}", left);
    println!("Right: {:?}", right);
    unimplemented!()
}



/// Create a unique test identifier
pub fn create_test_id() -> String {
    // TODO: Implement test ID creation
    println!("Creating test ID");
    unimplemented!()
} 