use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Box};
use super::{Menu, Toolbar, StatusBar, UIComponent};
use glib::Error;

/// Represents the main application window for EventGhost.
pub struct MainFrame {
    /// The main GTK application window
    pub(crate) window: ApplicationWindow,
    /// The main menu
    pub menu: Menu,
    /// The toolbar
    pub toolbar: Toolbar,
    /// The status bar
    pub status_bar: StatusBar,
    /// The main container
    pub container: Box,
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

        // Create main vertical container
        let container = Box::new(gtk::Orientation::Vertical, 0);
        window.set_child(Some(&container));

        // Create UI components
        let menu = Menu::new();
        let mut toolbar = Toolbar::new();
        let status_bar = StatusBar::new();

        // Add some default buttons to toolbar
        toolbar.add_button("New", Some("Create new configuration"));
        toolbar.add_button("Open", Some("Open configuration"));
        toolbar.add_button("Save", Some("Save configuration"));

        // Add components to container
        container.append(menu.get_widget());
        container.append(toolbar.get_widget());
        container.append(&status_bar.widget);
        
        MainFrame { 
            window,
            menu,
            toolbar,
            status_bar,
            container,
        }
    }
    
    /// Shows the main application window.
    pub fn show(&self) {
        self.window.show();
    }

    /// Gets the window title
    pub fn get_title(&self) -> Option<String> {
        self.window.title().map(|s| s.to_string())
    }

    /// Gets the default width
    pub fn get_default_width(&self) -> i32 {
        self.window.default_width()
    }

    /// Gets the default height
    pub fn get_default_height(&self) -> i32 {
        self.window.default_height()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mainframe_initialization() {
        // Initialize GTK before running the test
        gtk::init().expect("Failed to initialize GTK");
        
        // Create a test application
        let app = Application::builder()
            .application_id("org.eventghost.app")
            .build();
            
        // Ensure app was created successfully
        assert!(app.is_ok(), "Failed to create application: {:?}", app.err());
        let app = app.unwrap();
            
        // Create the main frame
        let main_frame = MainFrame::new(&app);
        
        // Test the window properties
        assert_eq!(main_frame.get_title().as_deref(), Some("EventGhost"));
        assert_eq!(main_frame.get_default_width(), 800);
        assert_eq!(main_frame.get_default_height(), 600);
        
        // Test that UI components are initialized
        assert!(main_frame.menu.get_widget().is_visible());
        assert!(main_frame.toolbar.get_widget().is_visible());
        assert!(main_frame.status_bar.widget.is_visible());
    }
} 