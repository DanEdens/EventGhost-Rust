use gtk::prelude::*;
use gtk::Widget;

// Core modules needed for basic window functionality
pub mod main_frame;
pub mod menu;
pub mod toolbar;
pub mod status_bar;
pub mod dialog;
pub mod actions;

// Re-export the main components
pub use main_frame::MainFrame;
pub use menu::Menu;
pub use toolbar::Toolbar;
pub use status_bar::StatusBar;
pub use dialog::ConfigDialog;
pub use actions::add_actions;

// Comment out modules that need GTK conversion
// pub mod drag_drop;
// pub mod log_ctrl;
// pub mod plugin_config;
// pub mod property_grid;
// pub mod tree_ctrl;

/// Trait for UI components that can be shown/hidden
pub trait UIComponent {
    fn get_widget(&self) -> &Widget;
    
    fn show(&self) {
        self.get_widget().show();
    }
    
    fn hide(&self) {
        self.get_widget().hide();
    }
    
    fn is_visible(&self) -> bool {
        self.get_widget().is_visible()
    }
} 