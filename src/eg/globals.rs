use crate::eg::bunch::Bunch;
use crate::core::constants::{DEFAULT_DEBUG_LEVEL, DEFAULT_ENCODING};
use crate::core::Error;

/// Global state container for EventGhost
pub struct Globals {
    /// Dynamic property storage
    pub bunch: Bunch,
    /// Current debug level
    pub debug_level: i32,
    /// System encoding for text operations
    pub system_encoding: String,
    /// Current program counter for macro execution
    pub program_counter: Option<usize>,
    /// Flag to stop execution
    pub stop_execution_flag: bool,
    /// Configuration directory path
    pub config_dir: String,
    /// Plugin directory path
    pub plugin_dir: String,
}

impl Globals {
    pub fn new() -> Self {
        Self {
            bunch: Bunch::new(),
            debug_level: DEFAULT_DEBUG_LEVEL,
            system_encoding: DEFAULT_ENCODING.to_string(),
            program_counter: None,
            stop_execution_flag: false,
            config_dir: String::new(),
            plugin_dir: String::new(),
        }
    }

    pub fn initialize(&mut self) -> Result<(), Error> {
        // Initialize paths
        self.config_dir = std::env::current_dir()?.join("config").to_string_lossy().into_owned();
        self.plugin_dir = std::env::current_dir()?.join("plugins").to_string_lossy().into_owned();

        // Create directories if they don't exist
        std::fs::create_dir_all(&self.config_dir)?;
        std::fs::create_dir_all(&self.plugin_dir)?;

        Ok(())
    }

    /// Set a global variable
    pub fn set_var<T: 'static + Send + Sync>(&mut self, name: &str, value: T) {
        self.bunch.set(name, value);
    }

    /// Get a global variable
    pub fn get_var<T: 'static + Send + Sync>(&self, name: &str) -> Option<T> {
        self.bunch.get::<T>(name).ok().and_then(|val| {
            if let Ok(guard) = val.read() {
                guard.downcast_ref::<T>().cloned()
            } else {
                None
            }
        })
    }

    /// Remove a global variable
    pub fn remove_var(&mut self, name: &str) {
        self.bunch.remove(name);
    }
} 