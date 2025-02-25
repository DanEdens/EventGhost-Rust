use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{Level, Record};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::str::FromStr;

/// Wrapper for log::Level to implement Serialize and Deserialize
#[derive(Debug, Clone)]
pub struct SerializableLevel(Level);

impl From<Level> for SerializableLevel {
    fn from(level: Level) -> Self {
        SerializableLevel(level)
    }
}

impl From<SerializableLevel> for Level {
    fn from(level: SerializableLevel) -> Self {
        level.0
    }
}

impl Serialize for SerializableLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for SerializableLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Level::from_str(&s)
            .map(SerializableLevel)
            .map_err(serde::de::Error::custom)
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    /// Log file path
    pub file_path: PathBuf,
    /// Console logging level
    #[serde(with = "level_serde")]
    pub console_level: Level,
    /// File logging level
    #[serde(with = "level_serde")]
    pub file_level: Level,
    /// Maximum log file size in bytes
    pub max_file_size: u64,
    /// Maximum number of backup files
    pub max_backup_count: u32,
}

/// Serde module for log::Level serialization
mod level_serde {
    use super::*;
    use serde::{Serializer, Deserializer};

    pub fn serialize<S>(level: &Level, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&level.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Level, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Level::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// Custom log target for EventGhost
#[derive(Debug, Clone)]
pub enum LogTarget {
    /// Core system events
    Core,
    /// Plugin-related events
    Plugin(String),
    /// GUI-related events
    Gui,
    /// User actions
    User,
}

/// Log entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Timestamp of the log entry
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Log level
    #[serde(with = "level_serde")]
    pub level: Level,
    /// Log target
    pub target: String,
    /// Log message
    pub message: String,
    /// Additional structured data
    pub metadata: serde_json::Value,
}

/// Logger implementation for EventGhost
pub struct Logger {
    config: Arc<Mutex<LogConfig>>,
    entries: Arc<Mutex<Vec<LogEntry>>>,
}

impl Logger {
    /// Create a new logger instance
    pub fn new(config: LogConfig) -> Result<Self, std::io::Error> {
        // TODO: Implement logger creation
        println!("Creating logger with config: {:?}", config);
        unimplemented!()
    }

    /// Update logger configuration
    pub async fn update_config(&self, config: LogConfig) -> Result<(), std::io::Error> {
        // TODO: Implement config update
        println!("Updating logger configuration: {:?}", config);
        unimplemented!()
    }


    /// Rotate log files if needed
    async fn rotate_logs(&self) -> Result<(), std::io::Error> {
        // TODO: Implement log rotation
        unimplemented!()
    }
}

/// Log formatter for structured logging
pub struct LogFormatter {
    include_metadata: bool,
    time_format: String,
}

impl LogFormatter {
    /// Format a log record
    pub fn format(&self, record: &Record) -> String {
        // TODO: Implement log formatting
        println!("Formatting log record: {:?}", record);
        unimplemented!()
    }

}

/// Initialize the logging system
pub fn init(config: LogConfig) -> Result<(), log::SetLoggerError> {
    // TODO: Implement logging initialization
    println!("Initializing logging system with config: {:?}", config);
    unimplemented!()
}

// #[cfg(test)]

// mod tests {
//     use super::*;
//     use tempfile::tempdir;

//     #[tokio::test]
//     async fn test_log_rotation(tempdir: tempfile::TempDir) {
//         // TODO: Implement log rotation tests
//         unimplemented!()
//         // print the unused var tempdir
//         println!("Tempdir: {:?}", tempdir);
        

//     }



//     #[test]
//     fn test_log_formatting() {
//         // TODO: Implement formatting tests
//         unimplemented!()
//     }

//     #[tokio::test]
//     async fn test_concurrent_logging() {
//         // TODO: Implement concurrency tests
//         unimplemented!()
//     }
// } 