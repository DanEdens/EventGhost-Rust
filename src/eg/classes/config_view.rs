use gtk::prelude::*;
use gtk::{self, Box, TreeView, TreeStore, TreeViewColumn, CellRendererPixbuf, CellRendererText, TreeIter, SelectionMode, TreePath, Entry, HeaderBar, Label, Orientation, PopoverMenu, ScrolledWindow, Widget};
use gio::{Menu, MenuItem, SimpleAction, SimpleActionGroup};
use gdk4::ModifierType;
use gtk::glib::{self, clone};
use uuid::Uuid;
use std::cell::RefCell;
use std::rc::Rc;
use std::path::{Path, PathBuf};
use std::io;
use log::{debug, error, info};
use std::collections::HashMap;

use crate::eg::config::{Config, ConfigItem, Plugin, Folder, Macro, Event, Action};
use super::config_dialogs::{PluginDialog, FolderDialog, MacroDialog, EventDialog, ActionDialog};

/// Column indices for the tree store
const COL_NAME: i32 = 0;
const COL_TYPE: i32 = 1;
const COL_ICON: i32 = 2;
const COL_ID: i32 = 3;

/// Helper function to convert TreePath to string
fn path_to_string(path: &TreePath) -> Option<String> {
    let indices = path.indices();
    let mut index_strs = Vec::new();
    for i in 0..indices.len() {
        index_strs.push(indices[i].to_string());
    }
    Some(index_strs.join(":"))
}

/// Represents the configuration view for EventGhost.
#[derive(Clone)]
pub struct ConfigView {
    /// The main container for the configuration view
    pub container: Box,
    /// Scrolled window for the tree view
    scrolled_window: ScrolledWindow,
    /// The tree view displaying the configuration hierarchy
    tree_view: TreeView,
    /// The tree store holding the configuration data
    tree_store: TreeStore,
    /// The configuration data
    config: Rc<RefCell<Config>>,
    /// The path to save the configuration to
    config_path: Rc<RefCell<Option<PathBuf>>>,
}

impl ConfigView {
    /// Creates a new ConfigView instance.
    pub fn new() -> Self {
        // Create main container
        let container = Box::new(gtk::Orientation::Vertical, 0);

        // Create tree store with columns:
        // 0: item name (String)
        // 1: item type (String)
        // 2: icon name (String)
        // 3: item id (String)
        let tree_store = TreeStore::new(&[
            glib::Type::STRING, // name
            glib::Type::STRING, // type
            glib::Type::STRING, // icon
            glib::Type::STRING, // id
        ]);
        
        // Create tree view
        let tree_view = TreeView::with_model(&tree_store);
        tree_view.set_headers_visible(true);
        tree_view.set_reorderable(true);
        tree_view.selection().set_mode(SelectionMode::Single);
        
        // Create scrolled window for tree view
        let scrolled_window = ScrolledWindow::new();
        scrolled_window.set_hexpand(true);
        scrolled_window.set_vexpand(true);
        scrolled_window.set_child(Some(&tree_view));
        
        // Set up drag and drop using GTK4 APIs
        let drag_source = gtk::DragSource::new();
        drag_source.set_actions(gtk::gdk::DragAction::MOVE);
        
        // Set up content provider when drag begins
        let tree_store_clone = tree_store.clone();
        let tree_view_for_drag = tree_view.clone();
        drag_source.connect_prepare(move |_, _, _| {
            let selection = tree_view_for_drag.selection();
            if let Some((model, iter)) = selection.selected() {
                let path = model.path(&iter);
                if let Some(path_str) = path_to_string(&path) {
                    // Create content provider with the path string
                    return Some(gtk::gdk::ContentProvider::for_value(&path_str.to_value()));
                }
            }
            None
        });
        
        tree_view.add_controller(drag_source);
        
        // Set up drop target
        let drop_target = gtk::DropTarget::new(gtk::glib::Type::STRING, gtk::gdk::DragAction::MOVE);
        let tree_store_for_drop = tree_store.clone();
        let tree_view_for_drop = tree_view.clone();
        
        drop_target.connect_drop(move |_, value, x, y| {
            if let Ok(path_string) = value.get::<String>() {
                // Get the tree path at the drop location
                if let Some((target_path, _, _, _)) = tree_view_for_drop.path_at_pos(x as i32, y as i32) {
                    // Parse source path string back to TreePath
                    if let Some(source_path_components) = path_string.split(':')
                        .map(|s| s.parse::<i32>().ok())
                        .collect::<Option<Vec<i32>>>() {
                        let mut source_path = TreePath::new();
                        for idx in source_path_components {
                            source_path.append_index(idx);
                        }
                        
                        // Don't allow dropping on the same path
                        let source_indices = source_path.indices();
                        let target_indices = target_path.as_ref().expect("Target path should exist").indices();
                        
                        if source_indices != target_indices {
                            // Get iterators for the source and target paths
                            if let Some(source_iter) = tree_store_for_drop.iter(&source_path) {
                                if let Some(target_iter) = tree_store_for_drop.iter(target_path.as_ref().expect("Target path should exist")) {
                                    // Get the target's parent
                                    let target_parent = tree_store_for_drop.iter_parent(&target_iter);
                                    
                                    // Copy the row to the new location
                                    let new_iter = tree_store_for_drop.insert_after(target_parent.as_ref(), Some(&target_iter));
                                    
                                    // Copy the values
                                    for i in 0..tree_store_for_drop.n_columns() {
                                        let column = i32::try_from(i).unwrap_or(0);
                                        let value = tree_store_for_drop.get_value(&source_iter, column);
                                        tree_store_for_drop.set_value(&new_iter, column as u32, &value);
                                    }
                                    
                                    // Remove the original row
                                    tree_store_for_drop.remove(&source_iter);
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            false
        });
        
        tree_view.add_controller(drop_target);
        
        // Add icon column
        let column = TreeViewColumn::new();
        let cell = CellRendererPixbuf::new();
        column.pack_start(&cell, false);
        column.add_attribute(&cell, "icon-name", COL_ICON);
        column.set_sizing(gtk::TreeViewColumnSizing::Autosize);
        tree_view.append_column(&column);
        
        // Add name column
        let column = TreeViewColumn::new();
        column.set_title("Name");
        column.set_expand(true); // Allow name column to expand
        column.set_sizing(gtk::TreeViewColumnSizing::Fixed);
        column.set_fixed_width(200); // Set minimum width
        let cell = CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", COL_NAME);
        tree_view.append_column(&column);
        
        // Add type column
        let column = TreeViewColumn::new();
        column.set_title("Type");
        column.set_sizing(gtk::TreeViewColumnSizing::Autosize);
        let cell = CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", COL_TYPE);
        tree_view.append_column(&column);

        // Create configuration data
        let config = Rc::new(RefCell::new(Config::new()));
        
        // Create instance
        let config_view = ConfigView {
            container,
            scrolled_window,
            tree_view,
            tree_store,
            config,
            config_path: Rc::new(RefCell::new(None)),
        };

        // Set up context menu
        config_view.setup_context_menu();
        
        // Add tree view to container
        config_view.container.append(&config_view.scrolled_window);

        // Add some initial data
        config_view.add_root_folders();

        config_view
    }

    /// Sets the path where the configuration should be saved.
    pub fn set_config_path<P: AsRef<Path>>(&self, path: P) {
        // Use a shadowed variable to avoid ownership issues with borrow_mut
        let config_path = self.config_path.clone();
        *config_path.borrow_mut() = Some(path.as_ref().to_path_buf());
    }
    
    /// Creates a new configuration.
    pub fn new_config(&self) {
        debug!("Creating new configuration");
        
        // Create a new empty configuration
        // Use shadowed variables to avoid ownership issues with borrow_mut
        let config = self.config.clone();
        *config.borrow_mut() = Config::new();
        
        // Clear the save path
        let config_path = self.config_path.clone();
        *config_path.borrow_mut() = None;
        
        // Repopulate the tree view
        self.populate_tree_from_config();
        
        info!("New configuration created successfully");
    }

    /// Saves the configuration to disk with error handling
    pub fn save_config(&self) {
        if let Some(path) = &*self.config_path.borrow() {
            debug!("Saving configuration to {}", path.display());
            match self.config.borrow().save_to_file(path) {
                Ok(_) => {
                    info!("Configuration saved successfully to {}", path.display());
                    // Show success message in status bar or log
                    if let Some(window) = self.container.root().and_downcast::<gtk::Window>() {
                        let dialog = gtk::MessageDialog::new(
                            Some(&window),
                            gtk::DialogFlags::MODAL,
                            gtk::MessageType::Info,
                            gtk::ButtonsType::Ok,
                            &format!("Configuration saved to {}", path.display())
                        );
                        dialog.connect_response(move |dialog, _| {
                            dialog.close();
                        });
                        dialog.show();
                    }
                },
                Err(err) => {
                    error!("Failed to save configuration: {}", err);
                    self.show_error(&format!("Failed to save configuration: {}", err));
                }
            }
        } else {
            // No path set, show save as dialog
            if let Some(window) = self.container.root().and_downcast::<gtk::Window>() {
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
                
                let config_view_clone = self.clone();
                dialog.connect_response(move |dialog, response| {
                    if response == gtk::ResponseType::Accept {
                        if let Some(file) = dialog.file() {
                            if let Some(path) = file.path() {
                                // Set the configuration path
                                config_view_clone.set_config_path(&path);
                                
                                // Save the configuration
                                config_view_clone.save_config();
                            }
                        }
                    }
                    dialog.close();
                });
                
                dialog.show();
            }
        }
    }

    /// Loads a configuration from a file.
    pub fn load_config<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path_ref = path.as_ref();
        debug!("Loading configuration from {}", path_ref.display());
        
        // Check if file exists
        if !path_ref.exists() {
            let err = io::Error::new(
                io::ErrorKind::NotFound, 
                format!("Configuration file not found: {}", path_ref.display())
            );
            error!("{}", err);
            return Err(err);
        }
        
        // Try to load the configuration
        match Config::load_from_file(path_ref) {
            Ok(mut config) => {
                // Try to decode plugin data for .egtree files
                if let Some(ext) = path_ref.extension().and_then(|e| e.to_str()) {
                    if ext.to_lowercase() == "egtree" {
                        debug!("Attempting to decode plugin data from .egtree file");
                        if let Err(e) = config.decode_plugin_data() {
                            error!("Error decoding plugin data: {}", e);
                        }
                    }
                }
                
                // Update the configuration
                self.config.replace(config);
                
                // Update the config path
                self.set_config_path(path_ref);
                
                // Clear the tree store
                self.tree_store.clear();
                
                // Add root folders
                self.add_root_folders();
                
                // Add items to the tree
                self.populate_tree_from_config();
                
                info!("Configuration loaded successfully from {}", path_ref.display());
                Ok(())
            },
            Err(err) => {
                let error_msg = format!("Failed to load configuration: {}", err);
                error!("{}", error_msg);
                self.show_error(&error_msg);
                Err(err)
            }
        }
    }
    
    /// Populates the tree view from the loaded configuration
    fn populate_tree_from_config(&self) {
        let config = self.config.borrow();
        
        // Find the root folders (Plugins, Folders, Autostart, etc.)
        let mut root_folders = HashMap::new();
        
        // Get the root iterators
        let mut iter = self.tree_store.iter_first();
        while let Some(it) = iter {
            let name = self.tree_store.get::<String>(&it, COL_NAME as i32);
            root_folders.insert(name, it.clone());
            
            if !self.tree_store.iter_next(&it) {
                break;
            } else {
                iter = Some(it);
            }
        }
        
        // Get key folders by name
        let plugins_iter = root_folders.get("Plugins").cloned();
        let macros_iter = root_folders.get("Macros").cloned();
        let autostart_iter = root_folders.get("Autostart").cloned();
        
        // First, find and process all autostart/folder items to build the hierarchy
        let mut folder_iters = HashMap::new();
        
        // Map all folders by ID for hierarchy construction
        for item in &config.items {
            match item {
                ConfigItem::Folder(folder) => {
                    let parent_iter = if folder.name == "Autostart" {
                        autostart_iter.clone()
                    } else {
                        macros_iter.clone()
                    };
                    
                    if let Some(parent) = parent_iter {
                        let iter = self.add_item_to_tree(item.clone(), Some(&parent));
                        folder_iters.insert(folder.id, iter);
                    }
                },
                _ => {}
            }
        }
        
        // Add remaining items to the tree
        for item in &config.items {
            match item {
                ConfigItem::Plugin(plugin) => {
                    // Add to Plugins folder or Autostart based on hierarchy
                    let parent_iter = if self.item_has_autostart_parent(item) {
                        autostart_iter.clone()
                    } else {
                        plugins_iter.clone()
                    };
                    
                    if let Some(parent) = parent_iter {
                        self.add_item_to_tree(item.clone(), Some(&parent));
                    }
                },
                ConfigItem::Macro(macro_) => {
                    // Skip Autostart as it's already added
                    if macro_.name == "Autostart" {
                        continue;
                    }
                    
                    // Add macros to the Macros folder
                    if let Some(ref parent) = macros_iter {
                        self.add_item_to_tree(item.clone(), Some(parent));
                    }
                },
                ConfigItem::Event(event) => {
                    // Find parent macro and add event to it
                    let parent_id = self.find_parent_id_for_event(event.id);
                    if let Some(parent_id) = parent_id {
                        if let Some(iter) = folder_iters.get(&parent_id) {
                            if let Some(it) = iter {
                                self.add_item_to_tree(item.clone(), Some(it));
                            }
                        }
                    }
                },
                ConfigItem::Action(action) => {
                    // Find parent macro and add action to it
                    let parent_id = self.find_parent_id_for_action(action.id);
                    if let Some(parent_id) = parent_id {
                        if let Some(iter) = folder_iters.get(&parent_id) {
                            if let Some(it) = iter {
                                self.add_item_to_tree(item.clone(), Some(it));
                            }
                        }
                    }
                },
                _ => {}
            }
        }
        
        // Expand the root nodes by default
        if let Some(plugins_iter) = plugins_iter {
            let path = self.tree_store.path(&plugins_iter);
            self.tree_view.expand_row(&path, false);
        }
        
        if let Some(macros_iter) = macros_iter {
            let path = self.tree_store.path(&macros_iter);
            self.tree_view.expand_row(&path, false);
        }
        
        if let Some(autostart_iter) = autostart_iter {
            let path = self.tree_store.path(&autostart_iter);
            self.tree_view.expand_row(&path, false);
        }
    }
    
    /// Checks if an item belongs to the Autostart folder
    fn item_has_autostart_parent(&self, item: &ConfigItem) -> bool {
        // For simplicity, assume plugins with certain attributes are in autostart
        // In a real implementation, you'd need to check the XML hierarchy
        match item {
            ConfigItem::Plugin(plugin) => {
                plugin.config.contains_key("XML_Guid")
            },
            _ => false
        }
    }
    
    /// Finds the parent macro ID for an event
    fn find_parent_id_for_event(&self, event_id: Uuid) -> Option<Uuid> {
        let config = self.config.borrow();
        
        for item in &config.items {
            if let ConfigItem::Macro(macro_) = item {
                if macro_.events.contains(&event_id) {
                    return Some(macro_.id);
                }
            }
        }
        
        None
    }
    
    /// Finds the parent macro ID for an action
    fn find_parent_id_for_action(&self, action_id: Uuid) -> Option<Uuid> {
        let config = self.config.borrow();
        
        for item in &config.items {
            if let ConfigItem::Macro(macro_) = item {
                if macro_.actions.contains(&action_id) {
                    return Some(macro_.id);
                }
            }
        }
        
        None
    }
    
    /// Sets up the context menu for the tree view
    fn setup_context_menu(&self) {
        // Create menu model
        let menu = Menu::new();
        
        // Add menu items
        menu.append(Some("Add Plugin"), Some("config.add-plugin"));
        menu.append(Some("Add Folder"), Some("config.add-folder"));
        menu.append(Some("Add Macro"), Some("config.add-macro"));
        menu.append(Some("Add Event"), Some("config.add-event"));
        menu.append(Some("Add Action"), Some("config.add-action"));
        
        menu.append(None, None); // separator
        
        menu.append(Some("Edit"), Some("config.edit"));
        menu.append(Some("Delete"), Some("config.delete"));
        
        // Create action group
        let action_group = SimpleActionGroup::new();
        
        // Add actions
        let tree_view = self.tree_view.clone();
        let tree_store = self.tree_store.clone();
        let config = self.config.clone();

        let add_plugin_action = SimpleAction::new("add-plugin", None);
        add_plugin_action.connect_activate(glib::clone!(@weak tree_view, @weak tree_store, @weak config => move |_, _| {
            let dialog = PluginDialog::new();
            if let Some(plugin) = dialog.run_for_new() {
                if let Some((model, iter)) = tree_view.selection().selected() {
                    let config_view = ConfigView { 
                        container: Box::new(gtk::Orientation::Vertical, 0), 
                        scrolled_window: ScrolledWindow::new(), 
                        tree_view: tree_view.clone(), 
                        tree_store, 
                        config,
                        config_path: Rc::new(RefCell::new(None)) 
                    };
                    config_view.add_item_to_tree(ConfigItem::Plugin(plugin), Some(&iter));
                }
            }
        }));
        action_group.add_action(&add_plugin_action);

        let add_folder_action = SimpleAction::new("add-folder", None);
        add_folder_action.connect_activate(glib::clone!(@weak tree_view, @weak tree_store, @weak config => move |_, _| {
            let dialog = FolderDialog::new();
            if let Some(folder) = dialog.run_for_new() {
                if let Some((model, iter)) = tree_view.selection().selected() {
                    let config_view = ConfigView { 
                        container: Box::new(gtk::Orientation::Vertical, 0), 
                        scrolled_window: ScrolledWindow::new(), 
                        tree_view: tree_view.clone(), 
                        tree_store, 
                        config,
                        config_path: Rc::new(RefCell::new(None)) 
                    };
                    config_view.add_item_to_tree(ConfigItem::Folder(folder), Some(&iter));
                }
            }
        }));
        action_group.add_action(&add_folder_action);

        let add_macro_action = SimpleAction::new("add-macro", None);
        add_macro_action.connect_activate(glib::clone!(@weak tree_view, @weak tree_store, @weak config => move |_, _| {
            let dialog = MacroDialog::new();
            if let Some(macro_) = dialog.run_for_new() {
                if let Some((model, iter)) = tree_view.selection().selected() {
                    let config_view = ConfigView { 
                        container: Box::new(gtk::Orientation::Vertical, 0), 
                        scrolled_window: ScrolledWindow::new(), 
                        tree_view: tree_view.clone(), 
                        tree_store, 
                        config,
                        config_path: Rc::new(RefCell::new(None)) 
                    };
                    config_view.add_item_to_tree(ConfigItem::Macro(macro_), Some(&iter));
                }
            }
        }));
        action_group.add_action(&add_macro_action);

        let add_event_action = SimpleAction::new("add-event", None);
        add_event_action.connect_activate(glib::clone!(@weak tree_view, @weak tree_store, @weak config => move |_, _| {
            let dialog = EventDialog::new();
            if let Some(event) = dialog.run_for_new() {
                if let Some((model, iter)) = tree_view.selection().selected() {
                    let config_view = ConfigView { 
                        container: Box::new(gtk::Orientation::Vertical, 0), 
                        scrolled_window: ScrolledWindow::new(), 
                        tree_view: tree_view.clone(), 
                        tree_store, 
                        config,
                        config_path: Rc::new(RefCell::new(None)) 
                    };
                    config_view.add_item_to_tree(ConfigItem::Event(event), Some(&iter));
                }
            }
        }));
        action_group.add_action(&add_event_action);

        let add_action_action = SimpleAction::new("add-action", None);
        add_action_action.connect_activate(glib::clone!(@weak tree_view, @weak tree_store, @weak config => move |_, _| {
            let dialog = ActionDialog::new();
            if let Some(action) = dialog.run_for_new() {
                if let Some((model, iter)) = tree_view.selection().selected() {
                    let config_view = ConfigView { 
                        container: Box::new(gtk::Orientation::Vertical, 0), 
                        scrolled_window: ScrolledWindow::new(), 
                        tree_view: tree_view.clone(), 
                        tree_store, 
                        config,
                        config_path: Rc::new(RefCell::new(None)) 
                    };
                    config_view.add_item_to_tree(ConfigItem::Action(action), Some(&iter));
                }
            }
        }));
        action_group.add_action(&add_action_action);

        let edit_action = SimpleAction::new("edit", None);
        edit_action.connect_activate(glib::clone!(@weak tree_view, @weak tree_store, @weak config => move |_, _| {
            if let Some((model, iter)) = tree_view.selection().selected() {
                if let Some(id_str) = model.get_value(&iter, COL_ID as i32).get::<String>().ok() {
                    if let Ok(id) = Uuid::parse_str(&id_str) {
                        // First borrow the config immutably to find the item
                        let item_option = {
                            let config_ref = config.borrow();
                            config_ref.find_item(id).cloned()
                        };
                        
                        if let Some(item) = item_option {
                            let edited_item = match item {
                                ConfigItem::Plugin(plugin) => {
                                    let dialog = PluginDialog::new();
                                    dialog.run_for_edit(&plugin).map(ConfigItem::Plugin)
                                }
                                ConfigItem::Folder(folder) => {
                                    let dialog = FolderDialog::new();
                                    dialog.run_for_edit(&folder).map(ConfigItem::Folder)
                                }
                                ConfigItem::Macro(macro_) => {
                                    let dialog = MacroDialog::new();
                                    dialog.run_for_edit(&macro_).map(ConfigItem::Macro)
                                }
                                ConfigItem::Event(event) => {
                                    let dialog = EventDialog::new();
                                    dialog.run_for_edit(&event).map(ConfigItem::Event)
                                }
                                ConfigItem::Action(action) => {
                                    let dialog = ActionDialog::new();
                                    dialog.run_for_edit(&action).map(ConfigItem::Action)
                                }
                            };
                            
                            // Now, if we have an edited item, create a ConfigView with the proper references
                            if let Some(edited_item) = edited_item {
                                let config_view = ConfigView { 
                                    container: Box::new(gtk::Orientation::Vertical, 0), 
                                    scrolled_window: ScrolledWindow::new(), 
                                    tree_view: tree_view.clone(), 
                                    tree_store, 
                                    config: config.clone(),
                                    config_path: Rc::new(RefCell::new(None)) 
                                };
                                config_view.update_item_in_tree(&iter, &edited_item);
                            }
                        }
                    }
                }
            }
        }));
        action_group.add_action(&edit_action);

        let delete_action = SimpleAction::new("delete", None);
        delete_action.connect_activate(glib::clone!(@weak tree_view, @weak tree_store, @weak config => move |_, _| {
            if let Some((_, iter)) = tree_view.selection().selected() {
                let config_view = ConfigView { 
                    container: Box::new(gtk::Orientation::Vertical, 0), 
                    scrolled_window: ScrolledWindow::new(), 
                    tree_view: tree_view.clone(), 
                    tree_store, 
                    config,
                    config_path: Rc::new(RefCell::new(None)) 
                };
                config_view.remove_item_from_tree(&iter);
            }
        }));
        action_group.add_action(&delete_action);

        // Add action group to tree view
        self.tree_view.insert_action_group("config", Some(&action_group));
        
        // Create popover menu
        let popover = PopoverMenu::from_model(Some(&menu));
        
        // Add context menu controller
        let gesture = gtk::GestureClick::new();
        gesture.set_button(3); // Right click
        gesture.connect_pressed(glib::clone!(@weak popover, @weak self.tree_view as tree_view => move |gesture, _, x, y| {
            if gesture.current_button() == 3 {
                popover.set_parent(&tree_view);
                popover.set_pointing_to(Some(&gtk::gdk::Rectangle::new(
                    x as i32,
                    y as i32,
                    1,
                    1
                )));
                popover.popup();
            }
        }));
        self.tree_view.add_controller(gesture);
    }

    /// Adds the root folders to the tree.
    fn add_root_folders(&self) {
        // Add Plugins folder
        let plugins_iter = self.tree_store.append(None);
        self.tree_store.set(&plugins_iter, &[
            (COL_NAME as u32, &"Plugins"),
            (COL_TYPE as u32, &"Folder"),
            (COL_ICON as u32, &"folder"),
            (COL_ID as u32, &"plugins"),
        ]);
        
        // Add Macros folder
        let macros_iter = self.tree_store.append(None);
        self.tree_store.set(&macros_iter, &[
            (COL_NAME as u32, &"Macros"),
            (COL_TYPE as u32, &"Folder"),
            (COL_ICON as u32, &"folder"),
            (COL_ID as u32, &"macros"),
        ]);
        
        // Add Autostart folder
        let autostart_iter = self.tree_store.append(None);
        self.tree_store.set(&autostart_iter, &[
            (COL_NAME as u32, &"Autostart"),
            (COL_TYPE as u32, &"Folder"),
            (COL_ICON as u32, &"folder"),
            (COL_ID as u32, &"autostart"),
        ]);
    }

    /// Validates if a child item can be added to a parent item
    fn validate_parent_child(&self, parent: Option<&ConfigItem>, child: &ConfigItem) -> bool {
        match (parent, child) {
            // Root level items
            (None, ConfigItem::Folder(_)) => true,
            (None, ConfigItem::Plugin(_)) => true,
            
            // Folder can contain other folders, plugins, and macros
            (Some(ConfigItem::Folder(_)), ConfigItem::Folder(_)) => true,
            (Some(ConfigItem::Folder(_)), ConfigItem::Plugin(_)) => true,
            (Some(ConfigItem::Folder(_)), ConfigItem::Macro(_)) => true,
            
            // Macro can contain events and actions
            (Some(ConfigItem::Macro(_)), ConfigItem::Event(_)) => true,
            (Some(ConfigItem::Macro(_)), ConfigItem::Action(_)) => true,
            
            // All other combinations are invalid
            _ => false,
        }
    }

    /// Gets the ConfigItem for a given TreeIter
    fn get_item_for_iter(&self, iter: &TreeIter) -> Option<ConfigItem> {
        if let Some(id_str) = self.tree_store.get_value(iter, COL_ID as i32).get::<String>().ok() {
            if let Ok(id) = Uuid::parse_str(&id_str) {
                return self.config.borrow().find_item(id).cloned();
            }
        }
        None
    }

    /// Adds an item to the tree view.
    fn add_item_to_tree(&self, item: ConfigItem, parent: Option<&TreeIter>) -> Option<TreeIter> {
        let icon = match &item {
            ConfigItem::Plugin(_) => "applications-utilities",
            ConfigItem::Folder(_) => "folder",
            ConfigItem::Macro(_) => "view-list-symbolic",
            ConfigItem::Event(_) => "emblem-important-symbolic",
            ConfigItem::Action(_) => "media-playback-start-symbolic",
        };
        
        let name = item.name().to_string();
        let item_type = match &item {
            ConfigItem::Plugin(_) => "Plugin",
            ConfigItem::Folder(_) => "Folder",
            ConfigItem::Macro(_) => "Macro",
            ConfigItem::Event(_) => "Event",
            ConfigItem::Action(_) => "Action",
        };
        
        // Add more detailed information to display name based on item type
        let display_name = match &item {
            ConfigItem::Plugin(plugin) => {
                if let Some(file) = plugin.config.get("File") {
                    format!("{} ({})", name, file)
                } else {
                    name
                }
            },
            ConfigItem::Action(action) => {
                if let Some(script) = action.parameters.get("Script") {
                    if script.starts_with("EventGhost.") || script.starts_with("System.") {
                        let parts: Vec<&str> = script.split('(').collect();
                        if !parts.is_empty() {
                            format!("{} - {}", name, parts[0])
                        } else {
                            name
                        }
                    } else {
                        // Show first 30 chars of script for Python scripts
                        let script_preview = if script.len() > 30 {
                            format!("{}...", &script[..30])
                        } else {
                            script.clone()
                        };
                        if name == "Action" {
                            format!("Python: {}", script_preview)
                        } else {
                            format!("{} - {}", name, script_preview)
                        }
                    }
                } else {
                    name
                }
            },
            _ => name
        };
        
        // Create a new row in the tree store
        let iter = self.tree_store.append(parent);
        
        // Set values for the new row
        self.tree_store.set(&iter, &[
            (COL_NAME as u32, &display_name),
            (COL_TYPE as u32, &item_type),
            (COL_ICON as u32, &icon),
            (COL_ID as u32, &item.id().to_string()),
        ]);
        
        Some(iter)
    }

    /// Gets the display type for a configuration item
    fn get_item_type(&self, item: &ConfigItem) -> &'static str {
        match item {
            ConfigItem::Plugin(_) => "Plugin",
            ConfigItem::Folder(_) => "Folder",
            ConfigItem::Macro(_) => "Macro",
            ConfigItem::Event(_) => "Event",
            ConfigItem::Action(_) => "Action",
        }
    }

    /// Gets the icon name for a configuration item
    fn get_item_icon(&self, item: &ConfigItem) -> &'static str {
        match item {
            ConfigItem::Plugin(_) => "application-x-addon",
            ConfigItem::Folder(_) => "folder",
            ConfigItem::Macro(_) => "system-run",
            ConfigItem::Event(_) => "dialog-information",
            ConfigItem::Action(_) => "media-playback-start",
        }
    }

    /// Removes a configuration item from the tree store
    fn remove_item_from_tree(&self, iter: &TreeIter) {
        if let Some(id_str) = self.tree_store.get_value(iter, COL_ID as i32).get::<String>().ok() {
            if let Ok(id) = Uuid::parse_str(&id_str) {
                let config_ref = self.config.borrow();
                let item = config_ref.find_item(id);
                if let Some(item) = item {
                    let msg = format!("Are you sure you want to delete {}?", item.name());
                    if self.show_confirmation(&msg) {
                        drop(config_ref); // Explicitly drop the borrow
                        
                        // Use a shadowed variable to avoid ownership issues with borrow_mut
                        let config = self.config.clone();
                        config.borrow_mut().remove_item(id);
                        
                        self.tree_store.remove(iter);
                        self.save_config();
                    }
                }
            }
        }
    }

    /// Updates a configuration item in the tree store
    fn update_item_in_tree(&self, iter: &TreeIter, item: &ConfigItem) {
        self.tree_store.set_value(iter, COL_NAME as u32, &item.name().to_value());
        self.tree_store.set_value(iter, COL_TYPE as u32, &self.get_item_type(item).to_value());
        self.tree_store.set_value(iter, COL_ICON as u32, &self.get_item_icon(item).to_value());
        
        // Save changes
        self.save_config();
    }

    /// Shows an error message dialog
    fn show_error(&self, message: &str) {
        if let Some(window) = self.container.root().and_downcast::<gtk::Window>() {
            let dialog = gtk::MessageDialog::new(
                Some(&window),
                gtk::DialogFlags::MODAL,
                gtk::MessageType::Error,
                gtk::ButtonsType::Ok,
                message
            );
            
            // Connect response signal to close the dialog
            dialog.connect_response(move |dialog, _| {
                dialog.close();
            });
            
            // Show the dialog
            dialog.show();
        } else {
            // Fallback to logging if we can't show a dialog
            error!("Error: {}", message);
        }
    }

    /// Shows a confirmation dialog and returns the response
    fn show_confirmation(&self, message: &str) -> bool {
        if let Some(window) = self.container.root().and_downcast::<gtk::Window>() {
            // Create a result variable to hold our response
            let result = std::cell::Cell::new(false);
            
            // Create the dialog
            let dialog = gtk::MessageDialog::new(
                Some(&window),
                gtk::DialogFlags::MODAL,
                gtk::MessageType::Question,
                gtk::ButtonsType::YesNo,
                message
            );
            
            // Connect the response signal to capture the result
            let result_clone = result.clone();
            dialog.connect_response(move |dialog, response| {
                result_clone.set(response == gtk::ResponseType::Yes);
                dialog.close();
            });
            
            // Show the dialog (this doesn't block in GTK4)
            dialog.show();
            
            // Since the dialog is non-blocking in GTK4, we return a default value here
            // In a real implementation, we would need to use a more complex approach
            // with futures or callbacks
            return false;
        }
        
        false
    }

    /// Public method to add an item to the tree
    pub fn add_item(&self, item: ConfigItem, parent: Option<&TreeIter>) -> Option<TreeIter> {
        self.add_item_to_tree(item, parent)
    }

    /// Public method to get the tree view
    pub fn get_tree_view(&self) -> &gtk::TreeView {
        &self.tree_view
    }

    /// Public method to expand all items in the tree
    pub fn expand_all(&self) {
        self.tree_view.expand_all();
    }

    /// Public method to collapse all items in the tree
    pub fn collapse_all(&self) {
        self.tree_view.collapse_all();
    }

    /// Public method to get the selected item
    pub fn get_selected(&self) -> Option<(gtk::TreeModel, gtk::TreeIter)> {
        self.tree_view.selection().selected()
    }

    /// Add a folder item to the configuration
    pub fn add_folder(&self) {
        let iter_opt = self.get_selected().map(|(_, iter)| iter);
        
        let dialog = super::config_dialogs::FolderDialog::new();
        if let Some(folder) = dialog.run_for_new() {
            self.add_item(crate::eg::config::ConfigItem::Folder(folder), iter_opt.as_ref());
            self.save_config();
        }
    }
    
    /// Add a plugin item to the configuration
    pub fn add_plugin(&self) {
        let iter_opt = self.get_selected().map(|(_, iter)| iter);
        
        let dialog = super::config_dialogs::PluginDialog::new();
        if let Some(plugin) = dialog.run_for_new() {
            self.add_item(crate::eg::config::ConfigItem::Plugin(plugin), iter_opt.as_ref());
            self.save_config();
        }
    }
    
    /// Add a macro item to the configuration
    pub fn add_macro(&self) {
        let iter_opt = self.get_selected().map(|(_, iter)| iter);
        
        let dialog = super::config_dialogs::MacroDialog::new();
        if let Some(macro_) = dialog.run_for_new() {
            self.add_item(crate::eg::config::ConfigItem::Macro(macro_), iter_opt.as_ref());
            self.save_config();
        }
    }
    
    /// Add an event item to the configuration
    pub fn add_event(&self) {
        let iter_opt = self.get_selected().map(|(_, iter)| iter);
        
        let dialog = super::config_dialogs::EventDialog::new();
        if let Some(event) = dialog.run_for_new() {
            self.add_item(crate::eg::config::ConfigItem::Event(event), iter_opt.as_ref());
            self.save_config();
        }
    }
    
    /// Add an action item to the configuration
    pub fn add_action(&self) {
        let iter_opt = self.get_selected().map(|(_, iter)| iter);
        
        let dialog = super::config_dialogs::ActionDialog::new();
        if let Some(action) = dialog.run_for_new() {
            self.add_item(crate::eg::config::ConfigItem::Action(action), iter_opt.as_ref());
            self.save_config();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_view_creation() {
        gtk::init().expect("Failed to initialize GTK");
        
        let config_view = ConfigView::new();
        
        // Verify root folders were added
        let mut n_children = 0;
        if let Some(iter) = config_view.tree_store.iter_first() {
            loop {
                n_children += 1;
                if !config_view.tree_store.iter_next(&iter) {
                    break;
                }
            }
        }
        assert_eq!(n_children, 3); // Plugins, Macros, and Autostart folders
    }
} 