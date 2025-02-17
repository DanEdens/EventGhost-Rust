use gtk::prelude::*;
use gtk::{self, Box, TreeView, TreeStore, TreeViewColumn, CellRendererPixbuf, CellRendererText, TreeIter};
use gio::{Menu, SimpleAction, SimpleActionGroup};
use glib;
use uuid::Uuid;
use std::cell::RefCell;
use std::rc::Rc;

use crate::eg::config::{Config, ConfigItem, Plugin, Folder, Macro, Event, Action};

/// Column indices for the tree store
const COL_NAME: i32 = 0;
const COL_TYPE: i32 = 1;
const COL_ICON: i32 = 2;
const COL_ID: i32 = 3;

/// Represents the configuration view for EventGhost.
pub struct ConfigView {
    /// The main container for the configuration view
    pub container: Box,
    /// The tree view displaying the configuration hierarchy
    tree_view: TreeView,
    /// The tree store holding the configuration data
    tree_store: TreeStore,
    /// The configuration data
    config: Rc<RefCell<Config>>,
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
        };

        // Set up context menu
        config_view.setup_context_menu();
        
        // Add tree view to container
        config_view.container.append(&config_view.tree_view);

        // Add some initial data
        config_view.add_root_folders();

        config_view
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
        let tree_store = self.tree_store.clone();
        let config = self.config.clone();

        let add_plugin_action = SimpleAction::new("add-plugin", None);
        add_plugin_action.connect_activate(glib::clone!(@weak tree_store, @weak config => move |_, _| {
            println!("Add plugin");
            // TODO: Show add plugin dialog
        }));
        action_group.add_action(&add_plugin_action);

        let add_folder_action = SimpleAction::new("add-folder", None);
        add_folder_action.connect_activate(glib::clone!(@weak tree_store, @weak config => move |_, _| {
            println!("Add folder");
            // TODO: Show add folder dialog
        }));
        action_group.add_action(&add_folder_action);

        let add_macro_action = SimpleAction::new("add-macro", None);
        add_macro_action.connect_activate(glib::clone!(@weak tree_store, @weak config => move |_, _| {
            println!("Add macro");
            // TODO: Show add macro dialog
        }));
        action_group.add_action(&add_macro_action);

        let add_event_action = SimpleAction::new("add-event", None);
        add_event_action.connect_activate(glib::clone!(@weak tree_store, @weak config => move |_, _| {
            println!("Add event");
            // TODO: Show add event dialog
        }));
        action_group.add_action(&add_event_action);

        let add_action_action = SimpleAction::new("add-action", None);
        add_action_action.connect_activate(glib::clone!(@weak tree_store, @weak config => move |_, _| {
            println!("Add action");
            // TODO: Show add action dialog
        }));
        action_group.add_action(&add_action_action);

        let edit_action = SimpleAction::new("edit", None);
        edit_action.connect_activate(glib::clone!(@weak tree_store, @weak config => move |_, _| {
            println!("Edit item");
            // TODO: Show edit dialog
        }));
        action_group.add_action(&edit_action);

        let delete_action = SimpleAction::new("delete", None);
        delete_action.connect_activate(glib::clone!(@weak tree_store, @weak config => move |_, _| {
            println!("Delete item");
            // TODO: Show delete confirmation
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

    /// Adds a configuration item to the tree store
    fn add_item_to_tree(&self, item: ConfigItem, parent: Option<&TreeIter>) -> TreeIter {
        let iter = self.tree_store.append(parent);
        
        self.tree_store.set_value(&iter, COL_NAME as u32, &item.name().to_value());
        self.tree_store.set_value(&iter, COL_TYPE as u32, &self.get_item_type(&item).to_value());
        self.tree_store.set_value(&iter, COL_ICON as u32, &self.get_item_icon(&item).to_value());
        self.tree_store.set_value(&iter, COL_ID as u32, &item.id().to_string().to_value());

        // Add to configuration
        self.config.borrow_mut().add_item(item);

        iter
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
                self.config.borrow_mut().remove_item(id);
                self.tree_store.remove(iter);
            }
        }
    }

    /// Updates a configuration item in the tree store
    fn update_item_in_tree(&self, iter: &TreeIter, item: &ConfigItem) {
        self.tree_store.set_value(iter, COL_NAME as u32, &item.name().to_value());
        self.tree_store.set_value(iter, COL_TYPE as u32, &self.get_item_type(item).to_value());
        self.tree_store.set_value(iter, COL_ICON as u32, &self.get_item_icon(item).to_value());
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