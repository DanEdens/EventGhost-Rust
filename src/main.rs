mod core;
mod eg;
mod cli;

use gtk::prelude::*;
use gtk::{self, Application};
use gio::Resource;
use log::{debug, error, info, LevelFilter};
use std::path::Path;
use std::fs;

// Import from local modules
use crate::eg::classes::main_frame::MainFrame;
use crate::cli::Cli;
use crate::core::Error;

fn main() -> glib::ExitCode {
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Initialize logging based on debug level
    let log_level = match cli.debug_level {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    
    env_logger::Builder::new()
        .filter_level(log_level)
        .format_timestamp_secs()
        .init();
    
    info!("EventGhost Rust starting up");
    debug!("Debug logging enabled");
    
    // Ensure config directory exists
    let config_dir = cli.get_config_dir();
    if !config_dir.exists() {
        debug!("Creating config directory: {:?}", config_dir);
        if let Err(e) = fs::create_dir_all(&config_dir) {
            error!("Failed to create config directory: {}", e);
        }
    }
    
    // Initialize GTK application
    let application = gtk::Application::new(
        Some("org.eventghost.rust"),
        gio::ApplicationFlags::FLAGS_NONE,
    );
    
    // Load resources - comment out for now until resources are available
    // let resource_bytes = glib::Bytes::from_static(include_bytes!("resources.gresource"));
    // let resource = gio::Resource::from_data(&resource_bytes).expect("Failed to load resources");
    // gio::resources_register(&resource);
    
    // Connect startup signal
    application.connect_startup(|_| {
        info!("Application startup");
    });
    
    // Connect activate signal
    application.connect_activate(move |app| {
        info!("Application activated");
        
        // Create main frame
        let main_frame = MainFrame::new(app);
        let main_frame = match main_frame {
            Ok(frame) => frame,
            Err(e) => {
                error!("Failed to create main frame: {}", e);
                return;
            }
        };
        
        // Get config file path from CLI
        let config_file = cli.get_config_file();
        debug!("Config file path: {:?}", config_file);
        
        // Try to load configuration if file exists
        if config_file.exists() {
            debug!("Loading configuration from {:?}", config_file);
            if let Err(e) = main_frame.config_view.borrow().as_ref().unwrap().load_config(&config_file) {
                error!("Failed to load configuration: {}", e);
            } else {
                info!("Configuration loaded successfully");
                // Set the config path
                main_frame.config_view.borrow_mut().as_mut().unwrap().set_config_path(&config_file);
            }
        } else {
            debug!("Config file does not exist, using default configuration");
            // Create a new configuration
            main_frame.config_view.borrow_mut().as_mut().unwrap().new_config();
            // Set the config path
            main_frame.config_view.borrow_mut().as_mut().unwrap().set_config_path(&config_file);
        }
        
        // Show the main window
        main_frame.window.show();
    });
    
    // Run the application
    application.run()
} 