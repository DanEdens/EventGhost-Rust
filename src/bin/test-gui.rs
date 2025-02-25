use gtk::prelude::*;
use gtk::{self, Application};
use gio::Resource;
extern crate eventghost;
use eventghost::eg::classes::MainFrame;

fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK");

    // Load and register resources
    let resource_bytes = include_bytes!("../resources.gresource");
    let resource = Resource::from_data(&glib::Bytes::from_static(resource_bytes))
        .expect("Failed to load resources");
    gio::resources_register(&resource);

    // Create application
    let app = Application::builder()
        .application_id("org.eventghost.test")
        .build();

    app.connect_activate(move |app| {
        let mut main_frame = MainFrame::new(app).expect("Failed to create main window");
        main_frame.update_button_tooltips();
        main_frame.show();
    });

    // Run application
    app.run();
} 