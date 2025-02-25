use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Box, Orientation, PopoverMenuBar, Paned, Notebook, TreeView, TreeStore};
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
use log::error;

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
    /// The log control
    pub log_ctrl: LogCtrl,
    /// The configuration view
    pub config_view: Rc<RefCell<Option<ConfigView>>>,
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
        static CONFIG_VIEW: RefCell<Rc<RefCell<Option<ConfigView>>>> = RefCell::new(Rc::new(RefCell::new(None)));
    }

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
            .default_width(800)
            .default_height(600)
            .build();

        // Create main box
        let main_box = Box::new(Orientation::Vertical, 0);
        window.set_child(Some(&main_box));

        // Initialize UI components
        let (menu_bar, toolbar, status_bar) = Self::init_ui_components();

        // Add components to container in correct order
        main_box.append(&menu_bar);
        main_box.append(&toolbar.widget);

        // Create horizontal paned container
        let paned = Paned::new(Orientation::Horizontal);
        paned.set_wide_handle(true);
        paned.set_position(400); // Set initial position
        main_box.append(&paned);

        // Create notebook for tabs
        let notebook = Notebook::new();
        notebook.set_scrollable(true);
        notebook.set_show_border(true);
        paned.set_start_child(Some(&notebook));

        // Create log window
        let log_ctrl = LogCtrl::new();
        log_ctrl.container.set_size_request(400, 300); // Set minimum size
        
        // Add log tab
        let log_label = gtk::Label::new(Some("Log"));
        notebook.append_page(&log_ctrl.container, Some(&log_label));

        // Create configuration view and initialize in the Rc<RefCell<Option<ConfigView>>>
        let config_view = ConfigView::new();
        let container = config_view.container.clone();
        let config_view_rc = Rc::new(RefCell::new(Some(config_view)));

        // Add configuration tab
        let config_label = gtk::Label::new(Some("Configuration"));
        notebook.append_page(&container, Some(&config_label));

        // Add status bar at the bottom
        main_box.append(&status_bar.widget);

        // Create MainFrame instance
        let main_frame = MainFrame {
            window,
            menu_bar,
            toolbar,
            status_bar,
            log_ctrl,
            config_view: config_view_rc,
            container: main_box,
            paned,
            notebook,
        };

        // Set up the menu model
        let menu_model = main_frame.create_menu_model();
        main_frame.menu_bar.set_menu_model(Some(&menu_model));

        Ok(main_frame)
    }

    /// Initialize UI components (menu bar, toolbar, status bar)
    fn init_ui_components() -> (PopoverMenuBar, Toolbar, StatusBar) {
        // Initialize menu bar
        let menu_model = Menu::new();
        let menu_bar = PopoverMenuBar::from_model(Some(&menu_model));

        // Initialize toolbar with all buttons
        let mut toolbar = Toolbar::new();
        Self::init_toolbar_buttons(&mut toolbar);

        // Initialize status bar
        let status_bar = StatusBar::new();

        (menu_bar, toolbar, status_bar)
    }

    /// Initialize toolbar buttons with their handlers
    fn init_toolbar_buttons(toolbar: &mut Toolbar) {
        // File operations
        let new_button = toolbar.add_button("new", "/org/eventghost/images/new.png", "New");
        let config_view = Rc::new(RefCell::new(None::<ConfigView>));
        let config_view_clone = config_view.clone();
        new_button.connect_clicked(move |_| {
            if let Some(config_view) = config_view_clone.borrow_mut().as_mut() {
                config_view.new_config();
            }
        });

        let open_button = toolbar.add_button("open", "/org/eventghost/images/open.png", "Open");
        let config_view_clone = config_view.clone();
        open_button.connect_clicked(move |_| {
            if let Some(config_view) = config_view_clone.borrow_mut().as_mut() {
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
                    let json_filter = gtk::FileFilter::new();
                    json_filter.set_name(Some("JSON Configuration Files"));
                    json_filter.add_pattern("*.json");
                    dialog.add_filter(&json_filter);
                    
                    let xml_filter = gtk::FileFilter::new();
                    xml_filter.set_name(Some("XML Configuration Files"));
                    xml_filter.add_pattern("*.xml");
                    xml_filter.add_pattern("*.egtree");
                    dialog.add_filter(&xml_filter);
                    
                    let all_filter = gtk::FileFilter::new();
                    all_filter.set_name(Some("All Configuration Files"));
                    all_filter.add_pattern("*.json");
                    all_filter.add_pattern("*.xml");
                    all_filter.add_pattern("*.egtree");
                    dialog.add_filter(&all_filter);
                    
                    // Set current folder to config directory
                    if let Ok(config_dir) = crate::core::utils::get_config_dir() {
                        dialog.set_current_folder(Some(&gio::File::for_path(config_dir)));
                    }
                    
                    let config_view_clone2 = config_view_clone.clone();
                    dialog.connect_response(move |dialog, response| {
                        if response == gtk::ResponseType::Accept {
                            if let Some(file) = dialog.file() {
                                if let Some(path) = file.path() {
                                    if let Some(config_view) = config_view_clone2.borrow_mut().as_mut() {
                                        // Try to load the configuration
                                        if let Err(err) = config_view.load_config(&path) {
                                            error!("Failed to load configuration: {}", err);
                                        } else {
                                            // Set the configuration path
                                            config_view.set_config_path(&path);
                                        }
                                    }
                                }
                            }
                        }
                        dialog.close();
                    });
                    
                    dialog.show();
                }
            }
        });

        let save_button = toolbar.add_button("save", "/org/eventghost/images/save.png", "Save");
        let config_view_clone = config_view.clone();
        save_button.connect_clicked(move |_| {
            if let Some(config_view) = config_view_clone.borrow_mut().as_mut() {
                config_view.save_config();
            }
        });

        toolbar.add_separator();

        // Store the config_view for later use
        MainFrame::CONFIG_VIEW.with(|cell| {
            *cell.borrow_mut() = config_view;
        });

        // Edit operations
        let cut_button = toolbar.add_button("cut", "/org/eventghost/images/cut.png", "Cut");
        cut_button.connect_clicked(|_| println!("Cut button clicked"));

        let copy_button = toolbar.add_button("copy", "/org/eventghost/images/copy.png", "Copy");
        copy_button.connect_clicked(|_| println!("Copy button clicked"));

        let paste_button = toolbar.add_button("paste", "/org/eventghost/images/paste.png", "Paste");
        paste_button.connect_clicked(|_| println!("Paste button clicked"));

        toolbar.add_separator();

        // Undo/Redo
        let undo_button = toolbar.add_button("undo", "/org/eventghost/images/undo.png", "Undo");
        undo_button.connect_clicked(|_| println!("Undo button clicked"));
        undo_button.set_sensitive(false);

        let redo_button = toolbar.add_button("redo", "/org/eventghost/images/redo.png", "Redo");
        redo_button.connect_clicked(|_| println!("Redo button clicked"));
        redo_button.set_sensitive(false);

        toolbar.add_separator();

        // Add items
        let add_plugin_button = toolbar.add_button("add-plugin", "/org/eventghost/images/plugin.png", "Add Plugin");
        add_plugin_button.connect_clicked(|_| println!("Add plugin button clicked"));

        let add_folder_button = toolbar.add_button("add-folder", "/org/eventghost/images/folder.png", "Add Folder");
        add_folder_button.connect_clicked(|_| println!("Add folder button clicked"));

        let add_macro_button = toolbar.add_button("add-macro", "/org/eventghost/images/macro.png", "Add Macro");
        add_macro_button.connect_clicked(|_| println!("Add macro button clicked"));

        let add_event_button = toolbar.add_button("add-event", "/org/eventghost/images/event.png", "Add Event");
        add_event_button.connect_clicked(|_| println!("Add event button clicked"));

        let add_action_button = toolbar.add_button("add-action", "/org/eventghost/images/action.png", "Add Action");
        add_action_button.connect_clicked(|_| println!("Add action button clicked"));

        toolbar.add_separator();

        // Execute and tree operations
        let execute_button = toolbar.add_button("execute", "/org/eventghost/images/Execute.png", "Execute");
        execute_button.connect_clicked(|_| println!("Execute button clicked"));

        toolbar.add_separator();

        let expand_button = toolbar.add_button("expand", "/org/eventghost/images/expand.png", "Expand");
        expand_button.connect_clicked(|_| println!("Expand button clicked"));

        let collapse_button = toolbar.add_button("collapse", "/org/eventghost/images/collapse.png", "Collapse");
        collapse_button.connect_clicked(|_| println!("Collapse button clicked"));

        let expand_children_button = toolbar.add_button("expand-children", "/org/eventghost/images/expand_children.png", "Expand Children");
        expand_children_button.connect_clicked(|_| println!("Expand children button clicked"));

        let collapse_children_button = toolbar.add_button("collapse-children", "/org/eventghost/images/collapse_children.png", "Collapse Children");
        collapse_children_button.connect_clicked(|_| println!("Collapse children button clicked"));

        let expand_all_button = toolbar.add_button("expand-all", "/org/eventghost/images/expand_all.png", "Expand All");
        expand_all_button.connect_clicked(|_| println!("Expand all button clicked"));

        let collapse_all_button = toolbar.add_button("collapse-all", "/org/eventghost/images/collapse_all.png", "Collapse All");
        collapse_all_button.connect_clicked(|_| println!("Collapse all button clicked"));

        // Set tooltips
        Self::init_toolbar_tooltips(toolbar);
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
            if let Some(config_view) = config_view.borrow_mut().as_mut() {
                config_view.new_config();
            }
        });
        application.add_action(&new_action);

        let open_action = gio::SimpleAction::new("open", None);
        let config_view = self.config_view.clone();
        open_action.connect_activate(move |_, _| {
            if let Some(window) = config_view.borrow_mut().as_mut().and_then(|config_view| {
                config_view.container.root().and_downcast::<gtk::Window>()
            }) {
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
                
                // Set current folder to config directory
                if let Ok(config_dir) = crate::core::utils::get_config_dir() {
                    dialog.set_current_folder(Some(&gio::File::for_path(config_dir)));
                }
                
                let config_view_clone = config_view.clone();
                let config_view_clone2 = config_view_clone.clone();
                dialog.connect_response(move |dialog, response| {
                    if response == gtk::ResponseType::Accept {
                        if let Some(file) = dialog.file() {
                            if let Some(path) = file.path() {
                                if let Some(config_view) = config_view_clone2.borrow_mut().as_mut() {
                                    // Try to load the configuration
                                    if let Err(err) = config_view.load_config(&path) {
                                        error!("Failed to load configuration: {}", err);
                                    } else {
                                        // Set the configuration path
                                        config_view.set_config_path(&path);
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
        application.add_action(&open_action);

        let save_action = gio::SimpleAction::new("save", None);
        let config_view = self.config_view.clone();
        save_action.connect_activate(move |_, _| {
            if let Some(config_view) = config_view.borrow_mut().as_mut() {
                config_view.save_config();
            }
        });
        application.add_action(&save_action);

        let save_as_action = gio::SimpleAction::new("save_as", None);
        let config_view = self.config_view.clone();
        save_as_action.connect_activate(move |_, _| {
            if let Some(window) = config_view.borrow_mut().as_mut().and_then(|config_view| {
                config_view.container.root().and_downcast::<gtk::Window>()
            }) {
                let dialog = gtk::FileChooserDialog::new(
                    Some("Save Configuration As"),
                    Some(&window),
                    gtk::FileChooserAction::Save,
                    &[
                        ("Cancel", gtk::ResponseType::Cancel),
                        ("Save", gtk::ResponseType::Accept),
                    ],
                );
                
                // Add file filters
                let filter = gtk::FileFilter::new();
                filter.set_name(Some("EventGhost Configuration Files"));
                filter.add_pattern("*.json");
                filter.add_pattern("*.egtree");
                filter.add_pattern("*.xml");
                dialog.add_filter(&filter);
                
                // Set current folder to config directory
                if let Ok(config_dir) = crate::core::utils::get_config_dir() {
                    dialog.set_current_folder(Some(&gio::File::for_path(config_dir)));
                }
                
                let config_view_clone = config_view.clone();
                dialog.connect_response(move |dialog, response| {
                    if response == gtk::ResponseType::Accept {
                        if let Some(file) = dialog.file() {
                            if let Some(path) = file.path() {
                                // Set the configuration path and save
                                config_view_clone.borrow_mut().as_mut().map(|config_view| {
                                    config_view.set_config_path(&path);
                                    config_view.save_config();
                                });
                            }
                        }
                    }
                    dialog.close();
                });
                
                dialog.show();
            }
        });
        application.add_action(&save_as_action);

        // Other file menu actions (placeholder implementations)
        let options_action = gio::SimpleAction::new("options", None);
        options_action.connect_activate(|_, _| {
            println!("Options menu item clicked");
        });
        application.add_action(&options_action);

        let restart_action = gio::SimpleAction::new("restart", None);
        restart_action.connect_activate(|_, _| {
            println!("Restart menu item clicked");
        });
        application.add_action(&restart_action);

        let restart_admin_action = gio::SimpleAction::new("restart_admin", None);
        restart_admin_action.connect_activate(|_, _| {
            println!("Restart as Admin menu item clicked");
        });
        application.add_action(&restart_admin_action);

        let quit_action = gio::SimpleAction::new("quit", None);
        let window = self.window.clone();
        quit_action.connect_activate(move |_, _| {
            window.close();
        });
        application.add_action(&quit_action);

        // Edit menu actions (placeholder implementations)
        let undo_action = gio::SimpleAction::new("undo", None);
        undo_action.connect_activate(|_, _| {
            println!("Undo menu item clicked");
        });
        application.add_action(&undo_action);

        let redo_action = gio::SimpleAction::new("redo", None);
        redo_action.connect_activate(|_, _| {
            println!("Redo menu item clicked");
        });
        application.add_action(&redo_action);

        let cut_action = gio::SimpleAction::new("cut", None);
        cut_action.connect_activate(|_, _| {
            println!("Cut menu item clicked");
        });
        application.add_action(&cut_action);

        let copy_action = gio::SimpleAction::new("copy", None);
        copy_action.connect_activate(|_, _| {
            println!("Copy menu item clicked");
        });
        application.add_action(&copy_action);

        let paste_action = gio::SimpleAction::new("paste", None);
        paste_action.connect_activate(|_, _| {
            println!("Paste menu item clicked");
        });
        application.add_action(&paste_action);

        let find_action = gio::SimpleAction::new("find", None);
        find_action.connect_activate(|_, _| {
            println!("Find menu item clicked");
        });
        application.add_action(&find_action);

        // Configuration menu actions (placeholder implementations)
        let add_plugin_action = gio::SimpleAction::new("add_plugin", None);
        add_plugin_action.connect_activate(|_, _| {
            println!("Add Plugin menu item clicked");
        });
        application.add_action(&add_plugin_action);

        let add_folder_action = gio::SimpleAction::new("add_folder", None);
        add_folder_action.connect_activate(|_, _| {
            println!("Add Folder menu item clicked");
        });
        application.add_action(&add_folder_action);

        let add_macro_action = gio::SimpleAction::new("add_macro", None);
        add_macro_action.connect_activate(|_, _| {
            println!("Add Macro menu item clicked");
        });
        application.add_action(&add_macro_action);

        let add_event_action = gio::SimpleAction::new("add_event", None);
        add_event_action.connect_activate(|_, _| {
            println!("Add Event menu item clicked");
        });
        application.add_action(&add_event_action);

        let add_action_action = gio::SimpleAction::new("add_action", None);
        add_action_action.connect_activate(|_, _| {
            println!("Add Action menu item clicked");
        });
        application.add_action(&add_action_action);

        let rename_item_action = gio::SimpleAction::new("rename_item", None);
        rename_item_action.connect_activate(|_, _| {
            println!("Rename Item menu item clicked");
        });
        application.add_action(&rename_item_action);

        let delete_item_action = gio::SimpleAction::new("delete_item", None);
        delete_item_action.connect_activate(|_, _| {
            println!("Delete Item menu item clicked");
        });
        application.add_action(&delete_item_action);

        // Help menu actions (placeholder implementations)
        let help_action = gio::SimpleAction::new("help", None);
        help_action.connect_activate(|_, _| {
            println!("Help Contents menu item clicked");
        });
        application.add_action(&help_action);

        let online_docs_action = gio::SimpleAction::new("online_docs", None);
        online_docs_action.connect_activate(|_, _| {
            println!("Online Documentation menu item clicked");
        });
        application.add_action(&online_docs_action);

        let check_updates_action = gio::SimpleAction::new("check_updates", None);
        check_updates_action.connect_activate(|_, _| {
            println!("Check for Updates menu item clicked");
        });
        application.add_action(&check_updates_action);

        let about_action = gio::SimpleAction::new("about", None);
        about_action.connect_activate(|_, _| {
            println!("About EventGhost menu item clicked");
        });
        application.add_action(&about_action);
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