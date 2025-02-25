use gtk::prelude::*;
use gtk::{self, Box, TreeView, TreeStore, TreeViewColumn, CellRendererPixbuf, CellRendererText, TreeIter, SelectionMode, TreePath};
use gio::{Menu, SimpleAction, SimpleActionGroup};
use gtk::gdk::ModifierType;
use gtk::glib::{self, clone};
use uuid::Uuid;
use std::cell::RefCell;
use std::rc::Rc;
use std::path::{Path, PathBuf};
use std::io;
use log::{debug, error, info};

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
        tree_view.append_column(&column);
        
        // Add name column
        let column = TreeViewColumn::new();
        column.set_title("Name");
        let cell = CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", COL_NAME);
        tree_view.append_column(&column);
        
        // Add type column
        let column = TreeViewColumn::new();
        column.set_title("Type");
        let cell = CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", COL_TYPE);
        tree_view.append_column(&column);

        // Create configuration data
        let config = Rc::new(RefCell::new(Config::new()));
        
        // Create instance
        let config_view = ConfigView {
            container,
            tree_view,
            tree_store,
            config,
            config_path: Rc::new(RefCell::new(None)),
        };

        // Set up context menu
        config_view.setup_context_menu();
        
        // Add tree view to container
        config_view.container.append(&config_view.tree_view);

        // Add some initial data
        config_view.add_root_folders();

        config_view
    }

    /// Sets the configuration path.
    pub fn set_config_path<P: AsRef<Path>>(&self, path: P) {
        *self.config_path.borrow_mut() = Some(path.as_ref().to_path_buf());
    }

    /// Creates a new empty configuration
    pub fn new_config(&self) {
        // Create a new empty configuration
        let config = Config::new();
        self.config.replace(config);
        
        // Clear the tree store
        self.tree_store.clear();
        
        // Add root folders
        self.add_root_folders();
        
        // Reset the config path
        *self.config_path.borrow_mut() = None;
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

    /// Loads a configuration from a file
    pub fn load_config<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
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
            Ok(config) => {
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
        
        // Find the root folders
        let mut plugins_iter = None;
        let mut macros_iter = None;
        
        // Get the root iterators
        let mut iter = self.tree_store.iter_first();
        while let Some(it) = iter {
            let name = self.tree_store.get::<String>(&it, 0);
            if name == "Plugins" {
                plugins_iter = Some(it.clone());
            } else if name == "Macros" {
                macros_iter = Some(it.clone());
            }
            if !self.tree_store.iter_next(&it) {
                break;
            } else {
                iter = Some(it);
            }
        }
        
        // Add items to the tree
        for item in &config.items {
            match item {
                ConfigItem::Plugin(plugin) => {
                    if let Some(ref parent) = plugins_iter {
                        self.add_item_to_tree(item.clone(), Some(parent));
                    }
                },
                ConfigItem::Folder(folder) => {
                    // Add folders to the appropriate parent
                    // For simplicity, we'll add all folders to the root for now
                    if let Some(ref parent) = macros_iter {
                        self.add_item_to_tree(item.clone(), Some(parent));
                    }
                },
                ConfigItem::Macro(macro_) => {
                    if let Some(ref parent) = macros_iter {
                        self.add_item_to_tree(item.clone(), Some(parent));
                    }
                },
                ConfigItem::Event(event) => {
                    // Events should be added to their parent macros
                    // For simplicity, we'll add all events to the macros folder for now
                    if let Some(ref parent) = macros_iter {
                        self.add_item_to_tree(item.clone(), Some(parent));
                    }
                },
                ConfigItem::Action(action) => {
                    // Actions should be added to their parent macros
                    // For simplicity, we'll add all actions to the macros folder for now
                    if let Some(ref parent) = macros_iter {
                        self.add_item_to_tree(item.clone(), Some(parent));
                    }
                },
            }
        }
        
        // Expand the root folders
        if let Some(plugins_iter) = plugins_iter {
            let path = self.tree_store.path(&plugins_iter);
            self.tree_view.expand_row(&path, false);
        }
        
        if let Some(macros_iter) = macros_iter {
            let path = self.tree_store.path(&macros_iter);
            self.tree_view.expand_row(&path, false);
        }
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
        let popover = gtk::PopoverMenu::from_model(Some(&menu));
        
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

    /// Adds the root folders (Plugins and Macros) to the tree
    fn add_root_folders(&self) {
        // Add Plugins folder
        let plugins_folder = Folder {
            id: Uuid::new_v4(),
            name: "Plugins".to_string(),
        };
        self.add_item_to_tree(ConfigItem::Folder(plugins_folder), None);

        // Add Macros folder
        let macros_folder = Folder {
            id: Uuid::new_v4(),
            name: "Macros".to_string(),
        };
        self.add_item_to_tree(ConfigItem::Folder(macros_folder), None);
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

    /// Adds a configuration item to the tree store with validation
    fn add_item_to_tree(&self, item: ConfigItem, parent: Option<&TreeIter>) -> Option<TreeIter> {
        // Get parent item if it exists
        let parent_item = parent.and_then(|p| self.get_item_for_iter(p));
        
        // Validate parent-child relationship
        if !self.validate_parent_child(parent_item.as_ref(), &item) {
            let error_msg = format!(
                "Cannot add {} to {}",
                self.get_item_type(&item),
                parent_item.map_or("root".to_string(), |p| self.get_item_type(&p).to_string())
            );
            let dialog = gtk::MessageDialog::new(
                None::<&gtk::Window>,
                gtk::DialogFlags::MODAL,
                gtk::MessageType::Error,
                gtk::ButtonsType::Ok,
                &error_msg
            );
            dialog.connect_response(move |dialog, _| {
                dialog.close();
            });
            dialog.show();
            return None;
        }

        let iter = self.tree_store.append(parent);
        
        self.tree_store.set_value(&iter, COL_NAME as u32, &item.name().to_value());
        self.tree_store.set_value(&iter, COL_TYPE as u32, &self.get_item_type(&item).to_value());
        self.tree_store.set_value(&iter, COL_ICON as u32, &self.get_item_icon(&item).to_value());
        self.tree_store.set_value(&iter, COL_ID as u32, &item.id().to_string().to_value());

        // Add to configuration
        self.config.borrow_mut().add_item(item);

        // Save changes
        self.save_config();

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
                        self.config.borrow_mut().remove_item(id);
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
        assert_eq!(n_children, 2); // Plugins and Macros folders
    }
} 