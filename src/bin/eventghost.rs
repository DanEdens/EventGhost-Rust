use gtk::prelude::*;
use gtk::Application;
use eventghost::eg::classes::{MainFrame, add_actions};

fn main() {
    // Create GTK application
    let app = Application::builder()
        .application_id("org.eventghost.app")
        .build();

    // Add actions before connecting activate signal
    app.connect_startup(|app| {
        add_actions(app);
    });

    app.connect_activate(move |app| {
        // Create main window
        let mut main_frame = MainFrame::new(app).expect("Failed to create main window");
        
        // Update tooltips with keyboard shortcuts
        main_frame.update_button_tooltips();
        
        // Show window
        main_frame.show();
    });

    // Run the application
    app.run();
} 