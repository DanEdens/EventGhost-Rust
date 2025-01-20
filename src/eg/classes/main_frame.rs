use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Box, HeaderBar, MenuButton, Button, Toolbar};
use gtk::Orientation;
use gio::SimpleAction;
use glib;

use crate::win32::{self, Error as Win32Error};
use super::tree_ctrl::TreeCtrl;
use super::log_ctrl::LogCtrl;
use super::status_bar::StatusBar;
use super::toolbar::Toolbar;
use super::UIComponent;

const MAIN_WINDOW_CLASS: &[u8] = b"EventGhostMainFrame\0";

pub struct MainFrame {
    window: ApplicationWindow,
    tree_ctrl: Option<TreeCtrl>,
    log_ctrl: Option<LogCtrl>,
    status_bar: Option<StatusBar>,
    document: Option<Document>,
}

impl MainFrame {
    pub fn new(app: &Application) -> Self {
        // Create main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("EventGhost")
            .default_width(800)
            .default_height(600)
            .build();

        // Create main vertical box
        let main_box = Box::new(Orientation::Vertical, 0);
        
        // Create header bar (replaces traditional menu bar)
        let header = HeaderBar::new();
        
        // Create toolbar
        let toolbar = Self::create_toolbar();
        main_box.append(&toolbar);
        
        // Create main menu button
        let menu_button = MenuButton::new();
        menu_button.set_icon_name("open-menu-symbolic");
        header.pack_end(&menu_button);
        
        window.set_titlebar(Some(&header));
        
        // Create horizontal paned for tree and log views
        let h_paned = gtk::Paned::new(Orientation::Horizontal);
        
        // Create tree view
        let tree_scroll = gtk::ScrolledWindow::new();
        let tree_ctrl = TreeCtrl::new();
        tree_scroll.set_child(Some(&tree_ctrl.widget));
        h_paned.set_start_child(Some(&tree_scroll));
        
        // Create log view
        let log_scroll = gtk::ScrolledWindow::new();
        let log_ctrl = LogCtrl::new();
        log_scroll.set_child(Some(&log_ctrl.widget));
        h_paned.set_end_child(Some(&log_scroll));
        
        // Add paned to main box
        main_box.append(&h_paned);
        
        // Create status bar
        let status_bar = StatusBar::new();
        main_box.append(&status_bar.widget);
        
        window.set_child(Some(&main_box));
        
        // Create actions
        Self::setup_actions(&window);
        
        // Create menus
        Self::create_menus(&menu_button);
        
        MainFrame {
            window,
            tree_ctrl: Some(tree_ctrl),
            log_ctrl: Some(log_ctrl), 
            status_bar: Some(status_bar),
            document: None,
        }
    }
    
    fn setup_actions(window: &ApplicationWindow) {
        // File actions
        let new_action = SimpleAction::new("new", None);
        new_action.connect_activate(glib::clone!(@weak window => move |_, _| {
            println!("New action activated");
        }));
        window.add_action(&new_action);
        
        let open_action = SimpleAction::new("open", None);
        open_action.connect_activate(glib::clone!(@weak window => move |_, _| {
            println!("Open action activated");
        }));
        window.add_action(&open_action);
        
        let save_action = SimpleAction::new("save", None);
        save_action.connect_activate(glib::clone!(@weak window => move |_, _| {
            println!("Save action activated");
        }));
        window.add_action(&save_action);
        
        // Edit actions
        let cut_action = SimpleAction::new("cut", None);
        cut_action.connect_activate(glib::clone!(@weak window => move |_, _| {
            println!("Cut action activated");
        }));
        window.add_action(&cut_action);
        
        let copy_action = SimpleAction::new("copy", None);
        copy_action.connect_activate(glib::clone!(@weak window => move |_, _| {
            println!("Copy action activated");
        }));
        window.add_action(&copy_action);
        
        let paste_action = SimpleAction::new("paste", None);
        paste_action.connect_activate(glib::clone!(@weak window => move |_, _| {
            println!("Paste action activated");
        }));
        window.add_action(&paste_action);
        
        // Configuration actions
        let add_plugin_action = SimpleAction::new("add-plugin", None);
        add_plugin_action.connect_activate(glib::clone!(@weak window => move |_, _| {
            println!("Add plugin action activated");
        }));
        window.add_action(&add_plugin_action);
        
        let add_folder_action = SimpleAction::new("add-folder", None);
        add_folder_action.connect_activate(glib::clone!(@weak window => move |_, _| {
            println!("Add folder action activated");
        }));
        window.add_action(&add_folder_action);
        
        let add_macro_action = SimpleAction::new("add-macro", None);
        add_macro_action.connect_activate(glib::clone!(@weak window => move |_, _| {
            println!("Add macro action activated");
        }));
        window.add_action(&add_macro_action);
    }
    
    fn create_menus(menu_button: &MenuButton) {
        let menu = gio::Menu::new();
        
        // File menu
        let file_menu = gio::Menu::new();
        file_menu.append(Some("New"), Some("win.new"));
        file_menu.append(Some("Open"), Some("win.open")); 
        file_menu.append(Some("Save"), Some("win.save"));
        menu.append_submenu(Some("File"), &file_menu);
        
        // Edit menu
        let edit_menu = gio::Menu::new();
        edit_menu.append(Some("Cut"), Some("win.cut"));
        edit_menu.append(Some("Copy"), Some("win.copy"));
        edit_menu.append(Some("Paste"), Some("win.paste"));
        menu.append_submenu(Some("Edit"), &edit_menu);
        
        menu_button.set_menu_model(Some(&menu));
    }
    
    fn create_toolbar() -> gtk::Box {
        let toolbar = gtk::Box::new(Orientation::Horizontal, 2);
        toolbar.add_css_class("toolbar");
        
        // New button
        let new_button = Button::from_icon_name("document-new-symbolic");
        new_button.set_tooltip_text(Some("New"));
        new_button.set_action_name(Some("win.new"));
        toolbar.append(&new_button);
        
        // Open button
        let open_button = Button::from_icon_name("document-open-symbolic");
        open_button.set_tooltip_text(Some("Open"));
        open_button.set_action_name(Some("win.open"));
        toolbar.append(&open_button);
        
        // Save button
        let save_button = Button::from_icon_name("document-save-symbolic");
        save_button.set_tooltip_text(Some("Save"));
        save_button.set_action_name(Some("win.save"));
        toolbar.append(&save_button);
        
        // Separator
        let separator1 = gtk::Separator::new(Orientation::Vertical);
        toolbar.append(&separator1);
        
        // Cut button
        let cut_button = Button::from_icon_name("edit-cut-symbolic");
        cut_button.set_tooltip_text(Some("Cut"));
        cut_button.set_action_name(Some("win.cut"));
        toolbar.append(&cut_button);
        
        // Copy button
        let copy_button = Button::from_icon_name("edit-copy-symbolic");
        copy_button.set_tooltip_text(Some("Copy"));
        copy_button.set_action_name(Some("win.copy"));
        toolbar.append(&copy_button);
        
        // Paste button
        let paste_button = Button::from_icon_name("edit-paste-symbolic");
        paste_button.set_tooltip_text(Some("Paste"));
        paste_button.set_action_name(Some("win.paste"));
        toolbar.append(&paste_button);
        
        // Separator
        let separator2 = gtk::Separator::new(Orientation::Vertical);
        toolbar.append(&separator2);
        
        // Add Plugin button
        let add_plugin_button = Button::from_icon_name("list-add-symbolic");
        add_plugin_button.set_tooltip_text(Some("Add Plugin"));
        add_plugin_button.set_action_name(Some("win.add-plugin"));
        toolbar.append(&add_plugin_button);
        
        // Add Folder button
        let add_folder_button = Button::from_icon_name("folder-new-symbolic");
        add_folder_button.set_tooltip_text(Some("Add Folder"));
        add_folder_button.set_action_name(Some("win.add-folder"));
        toolbar.append(&add_folder_button);
        
        // Add Macro button
        let add_macro_button = Button::from_icon_name("insert-object-symbolic");
        add_macro_button.set_tooltip_text(Some("Add Macro"));
        add_macro_button.set_action_name(Some("win.add-macro"));
        toolbar.append(&add_macro_button);
        
        toolbar
    }
    
    pub fn show(&self) {
        self.window.show();
    }
    
    pub fn set_document(&mut self, document: Document) {
        self.document = Some(document);
        // Update tree and other views with document
    }
}

impl Drop for MainFrame {
    fn drop(&mut self) {
        // Child controls will be destroyed automatically when parent window is destroyed
        unsafe {
            DestroyWindow(self.hwnd);
        }
    }
}

#[cfg(feature = "mainframe-test")]
mod tests {
    use super::*;
    use gtk::prelude::*;
    use std::time::Duration;

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
            
            // Add some test content
            if let Some(log_ctrl) = &frame.log_ctrl {
                log_ctrl.write("Test log message", super::log_ctrl::LogLevel::Info);
                log_ctrl.write("Test warning", super::log_ctrl::LogLevel::Warning);
                log_ctrl.write("Test error", super::log_ctrl::LogLevel::Error);
            }

            if let Some(status_bar) = &frame.status_bar {
                status_bar.set_status_text("Ready");
                status_bar.set_check_box_state(true);
            }

            // Schedule window close after 5 seconds for automated testing
            let main_context = glib::MainContext::default();
            let window = frame.window.clone();
            main_context.spawn_local(async move {
                glib::timeout_future_seconds(5).await;
                window.close();
            });
        });

        // Run the application
        app.run();
    }
} 