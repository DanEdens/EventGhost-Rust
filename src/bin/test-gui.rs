use gtk4 as gtk;
use gtk::prelude::*;
use gtk::Application;
use std::rc::Rc;
use std::cell::RefCell;
use gtk::glib;
use gio::Resource;

use eventghost_rust::eg::classes::main_frame::MainFrame;

fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK");

    // Load and register resources
    let resource_bytes = include_bytes!("../resources.gresource");
    let resource = Resource::from_data(&glib::Bytes::from_static(resource_bytes))
        .expect("Failed to load resources");
    gio::resources_register(&resource);

    // Create application
    let application = Application::new(
        Some("org.eventghost.test"),
        gio::ApplicationFlags::NON_UNIQUE,
    );

    // Connect to the activate signal
    application.connect_activate(|app| {
        // Create main frame
        let main_frame = Rc::new(MainFrame::new(app.clone()));
        
        // Initialize UI
        main_frame.initialize();
        
        // Show window
        main_frame.window.borrow().present();
    });

    // Run the application
    application.run();
} 