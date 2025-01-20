use gtk::prelude::*;
use gtk::Widget;

pub mod dialog;
pub mod drag_drop;
pub mod log_ctrl;
pub mod main_frame;
pub mod menu;
pub mod plugin_config;
pub mod property_grid;
pub mod status_bar;
pub mod toolbar;
pub mod tree_ctrl;

pub use dialog::CustomDialog;
pub use log_ctrl::LogCtrl;
pub use main_frame::MainFrame;
pub use menu::Menu;
pub use plugin_config::ConfigDialog;
pub use property_grid::PropertyGrid;
pub use status_bar::StatusBar;
pub use toolbar::Toolbar;
pub use tree_ctrl::TreeCtrl;

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