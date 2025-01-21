use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Box, Orientation, PopoverMenuBar};
use gio::{Menu, MenuItem};
use super::{Toolbar, StatusBar, UIComponent};
use crate::core::Error;
// use glib::Error;

const DEFAULT_WINDOW_WIDTH: i32 = 800;
const DEFAULT_WINDOW_HEIGHT: i32 = 600;

/// Represents the main application window for EventGhost.
pub struct MainFrame {
    /// The main GTK application window
    pub(crate) window: ApplicationWindow,
    /// The main menu bar
    pub menu_bar: PopoverMenuBar,
    /// The toolbar
    pub toolbar: Toolbar,
    /// The status bar
    pub status_bar: StatusBar,
    /// The main container
    pub container: Box,
}

impl MainFrame {
    fn create_menu_bar(&self) -> Menu {
        let menu_bar = Menu::new();

        // File Menu
        let file_menu = Menu::new();
        file_menu.append(Some("New"), Some("app.new"));
        file_menu.append(Some("Open"), Some("app.open"));
        file_menu.append(Some("Save"), Some("app.save"));
        file_menu.append(Some("Save As"), Some("app.save-as"));
        
        let separator = Menu::new();
        file_menu.append_section(None, &separator);
        
        file_menu.append(Some("Options"), Some("app.options"));
        
        let separator = Menu::new();
        file_menu.append_section(None, &separator);
        
        file_menu.append(Some("Restart"), Some("app.restart"));
        file_menu.append(Some("Restart as Admin"), Some("app.restart-admin"));
        
        let separator = Menu::new();
        file_menu.append_section(None, &separator);
        
        file_menu.append(Some("Exit"), Some("app.quit"));
        
        let file_item = MenuItem::new(Some("File"), Some("file"));
        file_item.set_submenu(Some(&file_menu));
        menu_bar.append_item(&file_item);

        // Edit Menu
        let edit_menu = Menu::new();
        edit_menu.append(Some("Undo"), Some("app.undo"));
        edit_menu.append(Some("Redo"), Some("app.redo"));
        
        let separator = Menu::new();
        edit_menu.append_section(None, &separator);
        
        edit_menu.append(Some("Cut"), Some("app.cut"));
        edit_menu.append(Some("Copy"), Some("app.copy"));
        edit_menu.append(Some("Python"), Some("app.python"));
        edit_menu.append(Some("Paste"), Some("app.paste"));
        edit_menu.append(Some("Delete"), Some("app.delete"));
        
        let separator = Menu::new();
        edit_menu.append_section(None, &separator);
        
        edit_menu.append(Some("Find"), Some("app.find"));
        edit_menu.append(Some("Find Next"), Some("app.find-next"));
        
        let edit_item = MenuItem::new(Some("Edit"), Some("edit"));
        edit_item.set_submenu(Some(&edit_menu));
        menu_bar.append_item(&edit_item);

        // View Menu
        let view_menu = Menu::new();
        view_menu.append(Some("Show Toolbar"), Some("app.show-toolbar"));
        
        let separator = Menu::new();
        view_menu.append_section(None, &separator);
        
        view_menu.append(Some("Expand"), Some("app.expand"));
        view_menu.append(Some("Collapse"), Some("app.collapse"));
        view_menu.append(Some("Expand Children"), Some("app.expand-children"));
        view_menu.append(Some("Collapse Children"), Some("app.collapse-children"));
        view_menu.append(Some("Expand All"), Some("app.expand-all"));
        view_menu.append(Some("Collapse All"), Some("app.collapse-all"));
        
        let view_item = MenuItem::new(Some("View"), Some("view"));
        view_item.set_submenu(Some(&view_menu));
        menu_bar.append_item(&view_item);

        // Configuration Menu
        let config_menu = Menu::new();
        config_menu.append(Some("Add Plugin"), Some("app.add-plugin"));
        config_menu.append(Some("Add Folder"), Some("app.add-folder"));
        config_menu.append(Some("Add Macro"), Some("app.add-macro"));
        config_menu.append(Some("Add Event"), Some("app.add-event"));
        config_menu.append(Some("Add Action"), Some("app.add-action"));
        
        let separator = Menu::new();
        config_menu.append_section(None, &separator);
        
        config_menu.append(Some("Configure"), Some("app.configure"));
        config_menu.append(Some("Rename"), Some("app.rename"));
        config_menu.append(Some("Execute"), Some("app.execute"));
        
        let config_item = MenuItem::new(Some("Configuration"), Some("configuration"));
        config_item.set_submenu(Some(&config_menu));
        menu_bar.append_item(&config_item);

        // Help Menu
        let help_menu = Menu::new();
        help_menu.append(Some("Contents"), Some("app.help-contents"));
        
        let separator = Menu::new();
        help_menu.append_section(None, &separator);
        
        help_menu.append(Some("Web Homepage"), Some("app.web-homepage"));
        help_menu.append(Some("Web Forum"), Some("app.web-forum"));
        help_menu.append(Some("Web Wiki"), Some("app.web-wiki"));
        
        let separator = Menu::new();
        help_menu.append_section(None, &separator);
        
        help_menu.append(Some("Check for Updates"), Some("app.check-update"));
        
        let separator = Menu::new();
        help_menu.append_section(None, &separator);
        
        help_menu.append(Some("Python Shell"), Some("app.python-shell"));
        help_menu.append(Some("About"), Some("app.about"));
        
        let help_item = MenuItem::new(Some("Help"), Some("help"));
        help_item.set_submenu(Some(&help_menu));
        menu_bar.append_item(&help_item);

        menu_bar
    }

    /// Creates a new MainFrame instance.
    ///
    /// # Arguments
    /// * `app` - The GTK Application instance
    ///
    /// # Returns
    /// A new MainFrame with a configured GTK window
    pub fn new(app: &Application) -> Result<Self, Error> {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("EventGhost")
            .default_width(DEFAULT_WINDOW_WIDTH)
            .default_height(DEFAULT_WINDOW_HEIGHT)
            .build();

        let container = Box::new(Orientation::Vertical, 0);
        let menu_model = Menu::new();
        let menu_bar = PopoverMenuBar::from_model(Some(&menu_model));
        let mut toolbar = Toolbar::new();
        let status_bar = StatusBar::new();

        // Create main frame instance first
        let main_frame = MainFrame {
            window,
            menu_bar,
            toolbar,
            status_bar,
            container,
        };

        // Create and add menu bar
        let menu_model = main_frame.create_menu_bar();
        main_frame.menu_bar.set_menu_model(Some(&menu_model));
        main_frame.container.append(&main_frame.menu_bar);
        
        // Add toolbar and status bar
        main_frame.container.append(&main_frame.toolbar.widget);
        main_frame.container.append(&main_frame.status_bar.widget);
        
        // Add container to window
        main_frame.window.set_child(Some(&main_frame.container));

        Ok(main_frame)
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

    /// Updates the enabled state of toolbar buttons based on document state
    pub fn update_toolbar_state(&mut self, can_save: bool, can_undo: bool, can_redo: bool) {
        self.toolbar.enable_button("save", can_save);
        self.toolbar.enable_button("undo", can_undo);
        self.toolbar.enable_button("redo", can_redo);
    }
    
    /// Updates button tooltips with additional information (like keyboard shortcuts)
    pub fn update_button_tooltips(&mut self) {
        self.toolbar.set_button_tooltip("new", "New (Ctrl+N)");
        self.toolbar.set_button_tooltip("open", "Open (Ctrl+O)");
        self.toolbar.set_button_tooltip("save", "Save (Ctrl+S)");
        self.toolbar.set_button_tooltip("cut", "Cut (Ctrl+X)");
        self.toolbar.set_button_tooltip("copy", "Copy (Ctrl+C)");
        self.toolbar.set_button_tooltip("paste", "Paste (Ctrl+V)");
        self.toolbar.set_button_tooltip("undo", "Undo (Ctrl+Z)");
        self.toolbar.set_button_tooltip("redo", "Redo (Ctrl+Y)");
        self.toolbar.set_button_tooltip("add-plugin", "Add Plugin (Shift+Ctrl+P)");
        self.toolbar.set_button_tooltip("add-folder", "Add Folder (Shift+Ctrl+N)");
        self.toolbar.set_button_tooltip("add-macro", "Add Macro (Shift+Ctrl+M)");
        self.toolbar.set_button_tooltip("add-event", "Add Event (Shift+Ctrl+E)");
        self.toolbar.set_button_tooltip("add-action", "Add Action (Shift+Ctrl+A)");
        self.toolbar.set_button_tooltip("disabled", "Disabled (Ctrl+D)");
        self.toolbar.set_button_tooltip("execute", "Execute (F5)");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_main_frame_creation() {
        gtk::init().expect("Failed to initialize GTK");
        
        let app = Application::builder()
            .application_id("org.eventghost.test")
            .build();
            
        let main_frame = MainFrame::new(&app).expect("Failed to create MainFrame");
        assert!(main_frame.toolbar.buttons.len() > 0);
    }
} 