use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow};

/// Represents the main application window for EventGhost.
pub struct MainFrame {
    /// The main GTK application window
    window: ApplicationWindow,
}

impl MainFrame {
    /// Creates a new MainFrame instance.
    ///
    /// # Arguments
    /// * `app` - The GTK Application instance
    ///
    /// # Returns
    /// A new MainFrame with a configured GTK window
    pub fn new(app: &Application) -> Self {
        // Create main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("EventGhost")
            .default_width(800)
            .default_height(600)
            .build();
        
        MainFrame { window }
    }
    
    /// Shows the main application window.
    pub fn show(&self) {
        self.window.show();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// Tests the initialization of the MainFrame.
    ///
    /// This test does the following:
    /// 1. Initializes GTK
    /// 2. Creates an application
    /// 3. Creates a MainFrame
    /// 4. Shows the window
    /// 5. Automatically closes after 2 seconds
    #[test]
    fn test_mainframe_initialization() {
        // Initialize GTK
        gtk::init().expect("Failed to initialize GTK");

        // Create application
        let app = Application::builder()
            .application_id("org.eventghost.test")
            .build();

        app.connect_activate(move |app| {
            // Create main frame
            let frame = MainFrame::new(app);
            frame.show();
            
            // Schedule window close after 2 seconds for automated testing
            let main_context = glib::MainContext::default();
            let window = frame.window.clone();
            main_context.spawn_local(async move {
                glib::timeout_future_seconds(2).await;
                window.close();
            });
        });

        // Run the application
        app.run();
    }
} 