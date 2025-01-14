use std::path::Path;
use crate::core::Error;

pub fn ensure_directory(path: &Path) -> Result<(), Error> {
    if !path.exists() {
        std::fs::create_dir_all(path).map_err(Error::Io)?;
    }
    Ok(())
}

pub fn get_app_data_dir() -> Result<std::path::PathBuf, Error> {
    let app_data = dirs::data_local_dir()
        .ok_or_else(|| Error::Config("Could not determine app data directory".into()))?;
    Ok(app_data.join("EventGhost"))
}

pub fn get_config_dir() -> Result<std::path::PathBuf, Error> {
    let config_dir = get_app_data_dir()?.join("config");
    ensure_directory(&config_dir)?;
    Ok(config_dir)
}

pub fn get_plugin_dirs() -> Result<Vec<std::path::PathBuf>, Error> {
    let mut dirs = vec![];
    
    // Add built-in plugins directory
    let app_dir = std::env::current_exe()?
        .parent()
        .ok_or_else(|| Error::Config("Could not determine application directory".into()))?
        .to_path_buf();
    dirs.push(app_dir.join("plugins"));

    // Add user plugins directory
    dirs.push(get_app_data_dir()?.join("plugins"));

    // Ensure all directories exist
    for dir in &dirs {
        ensure_directory(dir)?;
    }

    Ok(dirs)
}

pub fn format_error(error: &Error) -> String {
    format!("{:#}", error)
} 