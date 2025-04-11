use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// EventGhost command line interface
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to configuration file
    #[arg(short, long)]
    pub config_file: Option<PathBuf>,

    /// Path to configuration directory
    #[arg(long)]
    pub config_dir: Option<PathBuf>,

    /// Path to plugin directory
    #[arg(long)]
    pub plugin_dir: Option<PathBuf>,

    /// Debug level (0-3)
    #[arg(short, long, default_value_t = 0)]
    pub debug_level: u8,

    /// Start hidden in system tray
    #[arg(short = 'H', long)]
    pub hide: bool,

    /// Allow multiple instances
    #[arg(short, long)]
    pub multiload: bool,

    /// Subcommands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Trigger an event
    Event {
        /// Event name
        name: String,
        
        /// Event payload
        #[arg(trailing_var_arg = true)]
        payload: Vec<String>,
    },
    
    /// Install a plugin
    Plugin {
        /// Path to plugin file
        path: PathBuf,
    },
}

impl Cli {
    /// Parse command line arguments
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
    
    /// Get the configuration directory
    pub fn get_config_dir(&self) -> PathBuf {
        if let Some(dir) = &self.config_dir {
            dir.clone()
        } else {
            dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("EventGhost")
                .join("config")
        }
    }
    
    /// Get the configuration file path
    pub fn get_config_file(&self) -> PathBuf {
        if let Some(file) = &self.config_file {
            file.clone()
        } else {
            self.get_config_dir().join("default.json")
        }
    }
    
    /// Get the plugin directory
    pub fn get_plugin_dir(&self) -> PathBuf {
        if let Some(dir) = &self.plugin_dir {
            dir.clone()
        } else {
            dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("EventGhost")
                .join("plugins")
        }
    }
} 
