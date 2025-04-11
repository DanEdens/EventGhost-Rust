use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use log::{debug, error};
use crate::core::Error;

/// Ensures that a directory exists, creating it if necessary.
pub fn ensure_directory<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        debug!("Creating directory: {:?}", path);
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Gets the application data directory.
pub fn get_app_data_dir() -> io::Result<PathBuf> {
    let app_data_dir = if cfg!(windows) {
        // On Windows, use %APPDATA%\EventGhost
        let app_data = std::env::var("APPDATA").map_err(|_| {
            io::Error::new(io::ErrorKind::NotFound, "APPDATA environment variable not found")
        })?;
        PathBuf::from(app_data).join("EventGhost")
    } else if cfg!(target_os = "macos") {
        // On macOS, use ~/Library/Application Support/EventGhost
        let home = std::env::var("HOME").map_err(|_| {
            io::Error::new(io::ErrorKind::NotFound, "HOME environment variable not found")
        })?;
        PathBuf::from(home).join("Library/Application Support/EventGhost")
    } else {
        // On Linux, use ~/.config/eventghost
        let home = std::env::var("HOME").map_err(|_| {
            io::Error::new(io::ErrorKind::NotFound, "HOME environment variable not found")
        })?;
        PathBuf::from(home).join(".config/eventghost")
    };
    
    ensure_directory(&app_data_dir)?;
    Ok(app_data_dir)
}

/// Gets the configuration directory.
pub fn get_config_dir() -> io::Result<PathBuf> {
    let config_dir = get_app_data_dir()?.join("configs");
    ensure_directory(&config_dir)?;
    Ok(config_dir)
}

/// Gets the plugin directory.
pub fn get_plugin_dirs() -> io::Result<Vec<PathBuf>> {
    let app_data_dir = get_app_data_dir()?;
    let plugin_dir = app_data_dir.join("plugins");
    ensure_directory(&plugin_dir)?;
    
    // Return a list of plugin directories
    Ok(vec![plugin_dir])
}

/// Formats an error message for display.
pub fn format_error<E: std::error::Error>(err: E) -> String {
    format!("Error: {}", err)
} 
