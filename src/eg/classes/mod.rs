use crate::prelude::*;
use gtk::Widget;

// Module declarations
pub mod main_frame;
pub mod menu;
pub mod toolbar;
pub mod status_bar;
pub mod dialog;
pub mod actions;
pub mod log_ctrl;
pub mod config_view;
pub mod config_dialogs;
pub mod drag_drop;
pub mod property_grid;
pub mod plugin_config;
pub mod tree_ctrl;
pub mod action_config_dialog;

// Re-export the main components
pub use self::main_frame::MainFrame;
pub use self::menu::Menu;
pub use self::toolbar::Toolbar;
pub use self::status_bar::StatusBar;
pub use self::dialog::{ConfigDialog, Dialog};
pub use self::actions::add_actions;
pub use self::log_ctrl::LogCtrl;
pub use self::config_view::ConfigView;
pub use self::config_dialogs::{PluginDialog, FolderDialog, MacroDialog, EventDialog, ActionDialog};
pub use self::drag_drop::{DragDropManager, DragSourceWrapper, DropTargetWrapper, DragData};
pub use self::property_grid::PropertyGrid;
pub use self::plugin_config::PluginConfigDialog;
pub use self::tree_ctrl::TreeCtrl;
pub use self::action_config_dialog::ActionConfigDialog;

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
