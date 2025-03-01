use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Box, Orientation, PopoverMenuBar, Paned, Notebook, TreeView, TreeStore, AboutDialog, License, Window};
use gio::{Menu, MenuItem};
use super::{Toolbar, StatusBar};
use crate::eg::classes::log_ctrl::LogCtrl;
use super::UIComponent;
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;
use crate::core::Error;
use std::cell::Cell;
use std::thread::LocalKey;
use super::config_view::ConfigView;
use log::{error, info, debug};
use crate::eg::classes::dialog::{FileDialogOptions, CommonDialogs};

// use glib::Error;

const DEFAULT_WINDOW_WIDTH: i32 = 800;
const DEFAULT_WINDOW_HEIGHT: i32 = 600;

/// The main application window
pub struct MainFrame {
    /// The application window
    pub window: ApplicationWindow,
    /// The menu bar
    pub menu_bar: PopoverMenuBar,
    /// The toolbar
    pub toolbar: Toolbar,
    /// The status bar
    pub status_bar: StatusBar,
    /// The log control
    pub log_ctrl: LogCtrl,
    /// The configuration view
    pub config_view: ConfigView,
    /// The main container
    pub container: Box,
    /// The paned container for log and tree
    pub paned: Paned,
    /// The notebook for tabs
    pub notebook: Notebook,
}

impl MainFrame {
    /// Thread-local storage for ConfigView
    thread_local! {
        static CONFIG_VIEW: RefCell<Option<ConfigView>> = RefCell::new(None);
    }

    /// Creates a new MainFrame instance.
    ///
    /// # Arguments
    /// * `app` - The GTK Application instance
    ///
    /// # Returns
    /// A new MainFrame with a configured GTK window
    pub fn new(app: &Application) -> Result<Self, Error> {
        // Create the application window
        let window = gtk::ApplicationWindow::new(app);
        window.set_title(Some("EventGhost"));
        window.set_default_size(800, 600);

        // Initialize log control with an empty widget for now
        let log_ctrl = crate::eg::classes::log_ctrl::LogCtrl::new();
        
        // Create config view first
        let config_view = crate::eg::classes::config_view::ConfigView::new();
        
        // Set up container boxes
        let container = Box::new(Orientation::Vertical, 0);
        window.set_child(Some(&container));
        
        // Set up paned container for tree view and tabs
        let paned = Paned::new(Orientation::Horizontal);
        
        // Create notebook for tabs
        let notebook = Notebook::new();

        // Initialize UI components
        let (menu_bar, toolbar, status_bar) = Self::init_ui_components();
        
        // Initialize the main frame
        let mut main_frame = MainFrame {
            window,
            menu_bar,
            toolbar,
            status_bar,
            log_ctrl,
            config_view,
            container,
            paned,
            notebook,
        };
        
        // Create menu model and set it on the menu bar
        let menu_model = main_frame.create_menu_model();
        main_frame.menu_bar.set_menu_model(Some(&menu_model));
        
        // Set up UI layout
        main_frame.setup_ui();
        
        // Connect menu actions
        main_frame.connect_menu_actions(app);
        
        // Connect toolbar buttons
        main_frame.init_toolbar_buttons(app);
        
        Ok(main_frame)
    }

    /// Initialize UI components (menu bar, toolbar, status bar)
    fn init_ui_components() -> (PopoverMenuBar, Toolbar, StatusBar) {
        // Initialize menu bar (with an empty model for now)
        let menu_bar = PopoverMenuBar::from_model(None::<&gio::MenuModel>);
        
        // Initialize toolbar with all buttons
        let mut toolbar = Toolbar::new();
        
        // Add file operations buttons
        toolbar.add_button("new", "/org/eventghost/images/new.png", "New");
        toolbar.add_button("open", "/org/eventghost/images/open.png", "Open");
        toolbar.add_button("save", "/org/eventghost/images/save.png", "Save");
        toolbar.add_separator();
        
        // Add edit operation buttons
        toolbar.add_button("cut", "/org/eventghost/images/cut.png", "Cut");
        toolbar.add_button("copy", "/org/eventghost/images/copy.png", "Copy");
        toolbar.add_button("paste", "/org/eventghost/images/paste.png", "Paste");
        toolbar.add_separator();
        
        // Add items buttons
        toolbar.add_button("add-plugin", "/org/eventghost/images/plugin.png", "Add Plugin");
        toolbar.add_button("add-folder", "/org/eventghost/images/folder.png", "Add Folder");
        toolbar.add_button("add-macro", "/org/eventghost/images/macro.png", "Add Macro");
        toolbar.add_button("add-event", "/org/eventghost/images/event.png", "Add Event");
        toolbar.add_button("add-action", "/org/eventghost/images/action.png", "Add Action");
        toolbar.add_separator();
        
        // Add execution button
        toolbar.add_button("execute", "/org/eventghost/images/Execute.png", "Execute");
        toolbar.add_separator();
        
        // Add expand/collapse buttons
        toolbar.add_button("expand", "/org/eventghost/images/expand.png", "Expand");
        toolbar.add_button("collapse", "/org/eventghost/images/collapse.png", "Collapse");
        toolbar.add_button("expand-children", "/org/eventghost/images/expand_children.png", "Expand Children");
        toolbar.add_button("collapse-children", "/org/eventghost/images/collapse_children.png", "Collapse Children");
        toolbar.add_button("expand-all", "/org/eventghost/images/expand_all.png", "Expand All");
        toolbar.add_button("collapse-all", "/org/eventghost/images/collapse_all.png", "Collapse All");
        
        // Initialize status bar
        let status_bar = StatusBar::new();
        
        (menu_bar, toolbar, status_bar)
    }

    /// Initialize toolbar tooltips
    fn init_toolbar_tooltips(toolbar: &mut Toolbar) {
        toolbar.set_button_tooltip("new", "New (Ctrl+N)");
        toolbar.set_button_tooltip("open", "Open (Ctrl+O)");
        toolbar.set_button_tooltip("save", "Save (Ctrl+S)");
        toolbar.set_button_tooltip("cut", "Cut (Ctrl+X)");
        toolbar.set_button_tooltip("copy", "Copy (Ctrl+C)");
        toolbar.set_button_tooltip("paste", "Paste (Ctrl+V)");
        toolbar.set_button_tooltip("undo", "Undo (Ctrl+Z)");
        toolbar.set_button_tooltip("redo", "Redo (Ctrl+Y)");
        toolbar.set_button_tooltip("add-plugin", "Add Plugin (Shift+Ctrl+P)");
        toolbar.set_button_tooltip("add-folder", "Add Folder (Shift+Ctrl+N)");
        toolbar.set_button_tooltip("add-macro", "Add Macro (Shift+Ctrl+M)");
        toolbar.set_button_tooltip("add-event", "Add Event (Shift+Ctrl+E)");
        toolbar.set_button_tooltip("add-action", "Add Action (Shift+Ctrl+A)");
        toolbar.set_button_tooltip("execute", "Execute (F5)");
        toolbar.set_button_tooltip("expand", "Expand");
        toolbar.set_button_tooltip("collapse", "Collapse");
        toolbar.set_button_tooltip("expand-children", "Expand Children");
        toolbar.set_button_tooltip("collapse-children", "Collapse Children");
        toolbar.set_button_tooltip("expand-all", "Expand All");
        toolbar.set_button_tooltip("collapse-all", "Collapse All");
    }

    /// Creates the menu model for the application.
    fn create_menu_model(&self) -> gio::Menu {
        let menu_model = gio::Menu::new();
        
        // File menu
        let file_menu = gio::Menu::new();
        file_menu.append(Some("New"), Some("app.new"));
        file_menu.append(Some("Open"), Some("app.open"));
        
        // Add separator
        let separator = gio::Menu::new();
        file_menu.append_section(None, &separator);
        
        file_menu.append(Some("Save"), Some("app.save"));
        file_menu.append(Some("Save As"), Some("app.save-as"));
        
        // Add separator
        let separator = gio::Menu::new();
        file_menu.append_section(None, &separator);
        
        file_menu.append(Some("Options"), Some("app.options"));
        
        // Add separator
        let separator = gio::Menu::new();
        file_menu.append_section(None, &separator);
        
        file_menu.append(Some("Restart"), Some("app.restart"));
        file_menu.append(Some("Restart as Admin"), Some("app.restart-admin"));
        file_menu.append(Some("Exit"), Some("app.quit"));
        
        menu_model.append_submenu(Some("File"), &file_menu);
        
        // Edit menu
        let edit_menu = gio::Menu::new();
        edit_menu.append(Some("Undo"), Some("app.undo"));
        edit_menu.append(Some("Redo"), Some("app.redo"));
        
        // Add separator
        let separator = gio::Menu::new();
        edit_menu.append_section(None, &separator);
        
        edit_menu.append(Some("Cut"), Some("app.cut"));
        edit_menu.append(Some("Copy"), Some("app.copy"));
        edit_menu.append(Some("Paste"), Some("app.paste"));
        
        // Add separator
        let separator = gio::Menu::new();
        edit_menu.append_section(None, &separator);
        
        edit_menu.append(Some("Find"), Some("app.find"));
        edit_menu.append(Some("Replace"), Some("app.replace"));
        
        menu_model.append_submenu(Some("Edit"), &edit_menu);
        
        // Configuration menu
        let config_menu = gio::Menu::new();
        config_menu.append(Some("Add Plugin"), Some("app.add-plugin"));
        config_menu.append(Some("Add Folder"), Some("app.add-folder"));
        config_menu.append(Some("Add Macro"), Some("app.add-macro"));
        config_menu.append(Some("Add Event"), Some("app.add-event"));
        config_menu.append(Some("Add Action"), Some("app.add-action"));
        
        // Add separator
        let separator = gio::Menu::new();
        config_menu.append_section(None, &separator);
        
        config_menu.append(Some("Expand All"), Some("app.expand-all"));
        config_menu.append(Some("Collapse All"), Some("app.collapse-all"));
        
        menu_model.append_submenu(Some("Configuration"), &config_menu);
        
        // Help menu
        let help_menu = gio::Menu::new();
        help_menu.append(Some("Documentation"), Some("app.documentation"));
        help_menu.append(Some("Website"), Some("app.website"));
        
        // Add separator
        let separator = gio::Menu::new();
        help_menu.append_section(None, &separator);
        
        help_menu.append(Some("About"), Some("app.about"));
        
        menu_model.append_submenu(Some("Help"), &help_menu);
        
        menu_model
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
        Self::init_toolbar_tooltips(&mut self.toolbar);
    }

    /// Connect menu item actions
    pub fn connect_menu_actions(&self, application: &gtk::Application) {
        // File menu actions
        let new_action = gio::SimpleAction::new("new", None);
        let config_view = self.config_view.clone();
        new_action.connect_activate(move |_, _| {
            config_view.new_config();
        });
        application.add_action(&new_action);
        
        let open_action = gio::SimpleAction::new("open", None);
        let config_view = self.config_view.clone();
        let window = self.window.clone();
        open_action.connect_activate(move |_, _| {
            let options = FileDialogOptions {
                title: "Open Configuration".to_string(),
                filter: "EventGhost Configuration (*.json;*.xml)|*.json;*.xml".to_string(),
                default_ext: "json".to_string(),
                initial_dir: "".to_string(),
                file_name: "".to_string(),
                multi_select: false,
            };
            
            let window_ref: &Window = window.upcast_ref();
            if let Ok(files) = CommonDialogs::open_file(Some(window_ref), options) {
                if let Some(file) = files.first() {
                    if let Err(e) = config_view.load_config(file) {
                        eprintln!("Error loading configuration: {}", e);
                    } else {
                        config_view.set_config_path(file);
                    }
                }
            }
        });
        application.add_action(&open_action);
        
        let save_action = gio::SimpleAction::new("save", None);
        let config_view = self.config_view.clone();
        save_action.connect_activate(move |_, _| {
            config_view.save_config();
        });
        application.add_action(&save_action);
        
        let save_as_action = gio::SimpleAction::new("saveas", None);
        let config_view = self.config_view.clone();
        let window = self.window.clone();
        save_as_action.connect_activate(move |_, _| {
            let options = FileDialogOptions {
                title: "Save Configuration As".to_string(),
                filter: "EventGhost Configuration (*.json)|*.json".to_string(),
                default_ext: "json".to_string(),
                initial_dir: "".to_string(),
                file_name: "".to_string(),
                multi_select: false,
            };
            
            let window_ref: &Window = window.upcast_ref();
            if let Ok(Some(file)) = CommonDialogs::save_file(Some(window_ref), options) {
                config_view.set_config_path(&file);
                config_view.save_config();
            }
        });
        application.add_action(&save_as_action);
        
        // Edit menu actions
        let undo_action = gio::SimpleAction::new("undo", None);
        undo_action.connect_activate(move |_, _| {
            debug!("Undo action triggered");
            // TODO: Implement undo
        });
        application.add_action(&undo_action);
        
        let redo_action = gio::SimpleAction::new("redo", None);
        redo_action.connect_activate(move |_, _| {
            debug!("Redo action triggered");
            // TODO: Implement redo
        });
        application.add_action(&redo_action);
        
        let cut_action = gio::SimpleAction::new("cut", None);
        let config_view_clone = self.config_view.clone();
        cut_action.connect_activate(move |_, _| {
            debug!("Cut action triggered");
            // TODO: Implement cut
        });
        application.add_action(&cut_action);
        
        let copy_action = gio::SimpleAction::new("copy", None);
        let config_view_clone = self.config_view.clone();
        copy_action.connect_activate(move |_, _| {
            debug!("Copy action triggered");
        });
        application.add_action(&copy_action);
        
        let paste_action = gio::SimpleAction::new("paste", None);
        paste_action.connect_activate(move |_, _| {
            debug!("Paste action triggered");
            // TODO: Implement paste
        });
        application.add_action(&paste_action);
        
        // Configuration menu actions
        let add_plugin_action = gio::SimpleAction::new("add_plugin", None);
        let main_frame = self.clone();
        add_plugin_action.connect_activate(move |_, _| {
            main_frame.add_plugin();
        });
        application.add_action(&add_plugin_action);
        
        let add_folder_action = gio::SimpleAction::new("add_folder", None);
        let main_frame = self.clone();
        add_folder_action.connect_activate(move |_, _| {
            main_frame.add_folder();
        });
        application.add_action(&add_folder_action);
        
        let add_macro_action = gio::SimpleAction::new("add_macro", None);
        let main_frame = self.clone();
        add_macro_action.connect_activate(move |_, _| {
            main_frame.add_macro();
        });
        application.add_action(&add_macro_action);
        
        let add_event_action = gio::SimpleAction::new("add_event", None);
        let main_frame = self.clone();
        add_event_action.connect_activate(move |_, _| {
            main_frame.add_event();
        });
        application.add_action(&add_event_action);
        
        let add_action_action = gio::SimpleAction::new("add_action", None);
        let main_frame = self.clone();
        add_action_action.connect_activate(move |_, _| {
            main_frame.add_action();
        });
        application.add_action(&add_action_action);
        
        let expand_all_action = gio::SimpleAction::new("expand_all", None);
        let config_view = self.config_view.clone();
        expand_all_action.connect_activate(move |_, _| {
            config_view.expand_all();
        });
        application.add_action(&expand_all_action);
        
        let collapse_all_action = gio::SimpleAction::new("collapse_all", None);
        let config_view = self.config_view.clone();
        collapse_all_action.connect_activate(move |_, _| {
            config_view.collapse_all();
        });
        application.add_action(&collapse_all_action);
        
        // Help menu actions
        let about_action = gio::SimpleAction::new("about", None);
        let window = self.window.clone();
        about_action.connect_activate(move |_, _| {
            let dialog = AboutDialog::new();
            dialog.set_transient_for(Some(&window));
            dialog.set_modal(true);
            dialog.set_program_name(Some("EventGhost"));
            dialog.set_version(Some("0.1.0"));
            dialog.set_comments(Some("EventGhost reimplementation in Rust"));
            dialog.set_authors(&["EventGhost Team"]);
            dialog.set_website(Some("https://eventghost.net"));
            dialog.set_website_label("EventGhost Website");
            dialog.set_license_type(License::Gpl30);
            dialog.show();
        });
        application.add_action(&about_action);
    }

    /// Set up the UI layout
    fn setup_ui(&mut self) {
        // Create main box
        let main_box = self.container.clone();
        self.window.set_child(Some(&main_box));

        // Add components to container in correct order
        main_box.append(&self.menu_bar);
        main_box.append(&self.toolbar.widget);

        // Configure paned container
        self.paned.set_wide_handle(true);
        self.paned.set_position(250); // Set initial position
        self.paned.set_hexpand(true);  // Allow the paned to expand horizontally
        self.paned.set_vexpand(true);  // Allow the paned to expand vertically
        main_box.append(&self.paned);
        
        // Ensure the config view container expands properly
        self.config_view.container.set_hexpand(true);
        self.config_view.container.set_vexpand(true);
        self.config_view.container.set_size_request(200, -1); // Set minimum width, default height

        // Configure notebook
        self.notebook.set_scrollable(true);
        self.notebook.set_show_border(true);
        self.notebook.set_hexpand(true);  // Allow the notebook to expand horizontally
        self.notebook.set_vexpand(true);  // Allow the notebook to expand vertically
        
        // Add configuration view to the start pane
        self.paned.set_start_child(Some(&self.config_view.container));
        
        // Add notebook to the end pane
        self.paned.set_end_child(Some(&self.notebook));

        // Configure log window
        self.log_ctrl.container.set_size_request(400, 300); // Set minimum size
        
        // Add log tab to the notebook
        let log_label = gtk::Label::new(Some("Log"));
        self.notebook.append_page(&self.log_ctrl.container, Some(&log_label));

        // Add status bar at the bottom
        main_box.append(&self.status_bar.widget);
    }

    /// Initialize toolbar buttons
    fn init_toolbar_buttons(&self, app: &Application) {
        // Connect file operations buttons
        if let Some(button) = self.toolbar.get_button("new") {
            let config_view = self.config_view.clone();
            button.connect_clicked(move |_| {
                config_view.new_config();
            });
        }
        
        if let Some(button) = self.toolbar.get_button("open") {
            let config_view = self.config_view.clone();
            button.connect_clicked(move |_| {
                if let Some(window) = config_view.container.root().and_downcast::<gtk::Window>() {
                    let dialog = gtk::FileChooserDialog::new(
                        Some("Open Configuration"),
                        Some(&window),
                        gtk::FileChooserAction::Open,
                        &[
                            ("Cancel", gtk::ResponseType::Cancel),
                            ("Open", gtk::ResponseType::Accept),
                        ],
                    );
                    
                    // Add file filters
                    let filter = gtk::FileFilter::new();
                    filter.set_name(Some("EventGhost Configuration Files"));
                    filter.add_pattern("*.json");
                    filter.add_pattern("*.egtree");
                    filter.add_pattern("*.xml");
                    dialog.add_filter(&filter);
                    
                    // Add an "All Files" filter
                    let all_files_filter = gtk::FileFilter::new();
                    all_files_filter.set_name(Some("All Files"));
                    all_files_filter.add_pattern("*");
                    dialog.add_filter(&all_files_filter);
                    
                    // Set current folder to config directory
                    if let Ok(config_dir) = crate::core::utils::get_config_dir() {
                        dialog.set_current_folder(Some(&gio::File::for_path(config_dir)));
                    }
                    
                    let config_view_clone = config_view.clone();
                    dialog.connect_response(move |dialog, response| {
                        if response == gtk::ResponseType::Accept {
                            if let Some(file) = dialog.file() {
                                if let Some(path) = file.path() {
                                    info!("Attempting to load configuration from {}", path.display());
                                    
                                    // Try to load the configuration
                                    match config_view_clone.load_config(&path) {
                                        Ok(_) => {
                                            // Configuration loaded successfully
                                            info!("Configuration loaded successfully from {}", path.display());
                                            
                                            // Update window title if needed
                                            if let Some(window) = config_view_clone.container.root().and_downcast::<gtk::Window>() {
                                                if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                                                    window.set_title(Some(&format!("EventGhost - {}", filename)));
                                                }
                                            }
                                        },
                                        Err(err) => {
                                            // Show error to user
                                            error!("Failed to load configuration: {}", err);
                                            
                                            // The error message dialog is shown in the load_config method
                                        }
                                    }
                                }
                            }
                        }
                        dialog.close();
                    });
                    
                    dialog.show();
                }
            });
        }
        
        if let Some(button) = self.toolbar.get_button("save") {
            let config_view = self.config_view.clone();
            button.connect_clicked(move |_| {
                config_view.save_config();
            });
        }
        
        // Add config item buttons
        if let Some(button) = self.toolbar.get_button("add-plugin") {
            let main_frame = self.clone();
            button.connect_clicked(move |_| {
                main_frame.add_plugin();
            });
        }
        
        if let Some(button) = self.toolbar.get_button("add-folder") {
            let main_frame = self.clone();
            button.connect_clicked(move |_| {
                main_frame.add_folder();
            });
        }
        
        if let Some(button) = self.toolbar.get_button("add-macro") {
            let main_frame = self.clone();
            button.connect_clicked(move |_| {
                main_frame.add_macro();
            });
        }
        
        if let Some(button) = self.toolbar.get_button("add-event") {
            let main_frame = self.clone();
            button.connect_clicked(move |_| {
                main_frame.add_event();
            });
        }
        
        if let Some(button) = self.toolbar.get_button("add-action") {
            let main_frame = self.clone();
            button.connect_clicked(move |_| {
                main_frame.add_action();
            });
        }
        
        // Expand/collapse buttons
        if let Some(button) = self.toolbar.get_button("expand") {
            let config_view = &self.config_view;
            let config_view_clone = config_view.clone();
            button.connect_clicked(move |_| {
                if let Some((model, iter)) = config_view_clone.get_selected() {
                    let path = model.path(&iter);
                    config_view_clone.get_tree_view().expand_row(&path, false);
                }
            });
        }
        
        if let Some(button) = self.toolbar.get_button("collapse") {
            let config_view = &self.config_view;
            let config_view_clone = config_view.clone();
            button.connect_clicked(move |_| {
                if let Some((model, iter)) = config_view_clone.get_selected() {
                    let path = model.path(&iter);
                    config_view_clone.get_tree_view().collapse_row(&path);
                }
            });
        }
        
        if let Some(button) = self.toolbar.get_button("expand-children") {
            let config_view = &self.config_view;
            let config_view_clone = config_view.clone();
            button.connect_clicked(move |_| {
                if let Some((model, iter)) = config_view_clone.get_selected() {
                    let path = model.path(&iter);
                    config_view_clone.get_tree_view().expand_row(&path, true);
                }
            });
        }
        
        if let Some(button) = self.toolbar.get_button("collapse-children") {
            let config_view = &self.config_view;
            let config_view_clone = config_view.clone();
            button.connect_clicked(move |_| {
                if let Some((model, iter)) = config_view_clone.get_selected() {
                    let path = model.path(&iter);
                    config_view_clone.get_tree_view().collapse_row(&path);
                }
            });
        }
        
        if let Some(button) = self.toolbar.get_button("expand-all") {
            let config_view = &self.config_view;
            let config_view_clone = config_view.clone();
            button.connect_clicked(move |_| {
                config_view_clone.expand_all();
            });
        }
        
        if let Some(button) = self.toolbar.get_button("collapse-all") {
            let config_view = &self.config_view;
            let config_view_clone = config_view.clone();
            button.connect_clicked(move |_| {
                config_view_clone.collapse_all();
            });
        }
    }

    fn add_folder(&self) {
        info!("Adding folder");
        // Access ConfigView directly
        let config_view = &self.config_view;
        config_view.add_folder();
    }

    fn add_plugin(&self) {
        info!("Adding plugin");
        // Access ConfigView directly
        let config_view = &self.config_view;
        config_view.add_plugin();
    }

    fn add_macro(&self) {
        info!("Adding macro");
        // Access ConfigView directly
        let config_view = &self.config_view;
        config_view.add_macro();
    }

    fn add_event(&self) {
        info!("Adding event");
        // Access ConfigView directly
        let config_view = &self.config_view;
        config_view.add_event();
    }

    fn add_action(&self) {
        info!("Adding action");
        // Access ConfigView directly
        let config_view = &self.config_view;
        config_view.add_action();
    }
}

impl Clone for MainFrame {
    fn clone(&self) -> Self {
        // Only clone the window reference, which is sufficient for most operations
        MainFrame {
            window: self.window.clone(),
            menu_bar: self.menu_bar.clone(),
            toolbar: self.toolbar.clone(),
            status_bar: self.status_bar.clone(),
            log_ctrl: self.log_ctrl.clone(),
            config_view: self.config_view.clone(),
            container: self.container.clone(),
            paned: self.paned.clone(),
            notebook: self.notebook.clone(),
        }
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