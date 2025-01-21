use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Box, Orientation};
use super::{Menu, Toolbar, StatusBar, UIComponent};
use crate::core::Error;
// use glib::Error;

const DEFAULT_WINDOW_WIDTH: i32 = 800;
const DEFAULT_WINDOW_HEIGHT: i32 = 600;

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
    pub fn new(app: &Application) -> Result<Self, Error> {
        // Create main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("EventGhost")
            .default_width(DEFAULT_WINDOW_WIDTH)
            .default_height(DEFAULT_WINDOW_HEIGHT)
            .build();

        // Create main vertical container
        let container = Box::new(Orientation::Vertical, 0);
        
        // Create and initialize menu
        let menu = Menu::new();
        
        // Create and initialize toolbar with standard buttons
        let mut toolbar = Toolbar::new();
        
        // File operations
        let new_button = toolbar.add_button("new", "document-new", "New");
        let open_button = toolbar.add_button("open", "document-open", "Open");
        let save_button = toolbar.add_button("save", "document-save", "Save");
        toolbar.add_separator();
        
        // Edit operations
        let cut_button = toolbar.add_button("cut", "edit-cut", "Cut");
        let copy_button = toolbar.add_button("copy", "edit-copy", "Copy");
        let python_button = toolbar.add_button("python", "text-x-python", "Python");
        let paste_button = toolbar.add_button("paste", "edit-paste", "Paste");
        toolbar.add_separator();
        
        // Undo/Redo
        let undo_button = toolbar.add_button("undo", "edit-undo", "Undo");
        let redo_button = toolbar.add_button("redo", "edit-redo", "Redo");
        toolbar.add_separator();
        
        // Add items
        let add_plugin_button = toolbar.add_button("add-plugin", "list-add", "Add Plugin");
        let add_folder_button = toolbar.add_button("add-folder", "folder-new", "Add Folder");
        let add_macro_button = toolbar.add_button("add-macro", "insert-object", "Add Macro");
        let add_event_button = toolbar.add_button("add-event", "insert-text", "Add Event");
        let add_action_button = toolbar.add_button("add-action", "system-run", "Add Action");
        toolbar.add_separator();
        
        // Toggle and execute
        let disabled_button = toolbar.add_button("disabled", "dialog-error", "Disabled");
        toolbar.add_separator();
        let execute_button = toolbar.add_button("execute", "media-playback-start", "Execute");
        toolbar.add_separator();
        
        // Tree operations
        let expand_button = toolbar.add_button("expand", "go-down", "Expand");
        let collapse_button = toolbar.add_button("collapse", "go-up", "Collapse");
        let expand_children_button = toolbar.add_button("expand-children", "view-list-tree", "Expand Children");
        let collapse_children_button = toolbar.add_button("collapse-children", "view-list", "Collapse Children");
        let expand_all_button = toolbar.add_button("expand-all", "zoom-fit-best", "Expand All");
        let collapse_all_button = toolbar.add_button("collapse-all", "zoom-original", "Collapse All");
        
        // Initially disable some buttons
        toolbar.enable_button("save", false);
        toolbar.enable_button("undo", false);
        toolbar.enable_button("redo", false);
        
        // Create status bar
        let status_bar = StatusBar::new();
        
        // Add components to container
        container.append(&menu.widget);
        container.append(&toolbar.widget);
        
        // Add container to window
        window.set_child(Some(&container));
        
        // Create MainFrame instance
        let main_frame = MainFrame {
            window,
            menu,
            toolbar,
            status_bar,
            container,
        };
        
        // Connect button signals
        new_button.connect_clicked(move |_| {
            println!("New button clicked");
            // TODO: Implement new document functionality
        });
        
        open_button.connect_clicked(move |_| {
            println!("Open button clicked");
            // TODO: Implement open document functionality
        });
        
        save_button.connect_clicked(move |_| {
            println!("Save button clicked");
            // TODO: Implement save document functionality
        });
        
        cut_button.connect_clicked(move |_| {
            println!("Cut button clicked");
            // TODO: Implement cut functionality
        });
        
        copy_button.connect_clicked(move |_| {
            println!("Copy button clicked");
            // TODO: Implement copy functionality
        });
        
        python_button.connect_clicked(move |_| {
            println!("Python button clicked");
            // TODO: Implement Python shell functionality
        });
        
        paste_button.connect_clicked(move |_| {
            println!("Paste button clicked");
            // TODO: Implement paste functionality
        });
        
        undo_button.connect_clicked(move |_| {
            println!("Undo button clicked");
            // TODO: Implement undo functionality
        });
        
        redo_button.connect_clicked(move |_| {
            println!("Redo button clicked");
            // TODO: Implement redo functionality
        });
        
        add_plugin_button.connect_clicked(move |_| {
            println!("Add plugin button clicked");
            // TODO: Implement add plugin functionality
        });
        
        add_folder_button.connect_clicked(move |_| {
            println!("Add folder button clicked");
            // TODO: Implement add folder functionality
        });
        
        add_macro_button.connect_clicked(move |_| {
            println!("Add macro button clicked");
            // TODO: Implement add macro functionality
        });
        
        add_event_button.connect_clicked(move |_| {
            println!("Add event button clicked");
            // TODO: Implement add event functionality
        });
        
        add_action_button.connect_clicked(move |_| {
            println!("Add action button clicked");
            // TODO: Implement add action functionality
        });
        
        disabled_button.connect_clicked(move |_| {
            println!("Disabled button clicked");
            // TODO: Implement disable/enable functionality
        });
        
        execute_button.connect_clicked(move |_| {
            println!("Execute button clicked");
            // TODO: Implement execute functionality
        });
        
        expand_button.connect_clicked(move |_| {
            println!("Expand button clicked");
            // TODO: Implement expand functionality
        });
        
        collapse_button.connect_clicked(move |_| {
            println!("Collapse button clicked");
            // TODO: Implement collapse functionality
        });
        
        expand_children_button.connect_clicked(move |_| {
            println!("Expand children button clicked");
            // TODO: Implement expand children functionality
        });
        
        collapse_children_button.connect_clicked(move |_| {
            println!("Collapse children button clicked");
            // TODO: Implement collapse children functionality
        });
        
        expand_all_button.connect_clicked(move |_| {
            println!("Expand all button clicked");
            // TODO: Implement expand all functionality
        });
        
        collapse_all_button.connect_clicked(move |_| {
            println!("Collapse all button clicked");
            // TODO: Implement collapse all functionality
        });
        
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