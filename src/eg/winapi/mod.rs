use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow};
use gio;
// use glib;

pub fn init_application() -> gtk::Application {
    let application = Application::new(
        Some("org.eventghost.app"),
        gio::ApplicationFlags::FLAGS_NONE,
    );
    
    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title(Some("EventGhost"));
        window.set_default_size(800, 600);
        window.show();
    });
    
    application
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_application_initialization() {
        let app = init_application();
        assert_eq!(app.application_id(), Some(glib::GString::from("org.eventghost.app")));
    }
} 