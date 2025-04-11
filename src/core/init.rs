use crate::core::Error;
use crate::eg::Globals;
use std::path::PathBuf;

pub struct InitOptions {
    pub config_dir: PathBuf,
    pub plugin_dirs: Vec<PathBuf>,
    pub log_level: log::LevelFilter,
    pub debug_mode: bool,
}

impl Default for InitOptions {
    fn default() -> Self {
        Self {
            config_dir: PathBuf::from("config"),
            plugin_dirs: vec![PathBuf::from("plugins")],
            log_level: log::LevelFilter::Info,
            debug_mode: false,
        }
    }
}

pub fn initialize(options: InitOptions) -> Result<Globals, Error> {
    // TODO: Implement initialization
    Ok(Globals::new())
}

pub fn cleanup() -> Result<(), Error> {
    // TODO: Implement cleanup
    Ok(())
} 
