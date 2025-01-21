use gtk::prelude::*;
use gtk::Application;

use eventghost::eg::classes::main_frame::MainFrame;

fn main() {
    // Create GTK application
    let app = Application::builder()
        .application_id("org.eventghost.app")
        .build();

    app.connect_activate(|app| {
        // Create main window
        let main_frame = MainFrame::new(app).expect("Failed to create main window");
        
        // Show window
        main_frame.show();
    });

    // Run the application
    app.run();
} 