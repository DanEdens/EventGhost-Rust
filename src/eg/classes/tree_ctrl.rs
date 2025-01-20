use gtk::prelude::*;
use gtk::{self, TreeView, TreeStore, TreeViewColumn, CellRendererText};
use glib;

pub struct TreeItem {
    pub id: String,
    pub text: String,
    pub icon_index: i32,
    pub selected_icon_index: i32,
    pub data: Option<Box<dyn std::any::Any + Send + Sync>>,
}

pub struct TreeCtrl {
    pub widget: TreeView,
    store: TreeStore,
}

impl TreeCtrl {
    pub fn new() -> Self {
        // Create tree store with columns
        let store = TreeStore::new(&[
            glib::Type::STRING, // Name
            glib::Type::STRING, // Icon
            glib::Type::BOOL,   // Enabled
            glib::Type::STRING, // Type
        ]);

        // Create tree view
        let widget = TreeView::new();
        widget.set_model(Some(&store));
        
        // Add columns
        let renderer = CellRendererText::new();
        let column = TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.add_attribute(&renderer, "text", 0);
        column.set_title("Name");
        widget.append_column(&column);
        
        // Enable selection
        let selection = widget.selection();
        selection.set_mode(gtk::SelectionMode::Single);
        
        // Enable drag and drop
        widget.set_reorderable(true);
        
        TreeCtrl {
            widget,
            store,
        }
    }

    pub fn append_item(&self, parent: Option<&gtk::TreeIter>, name: &str, item_type: &str) -> gtk::TreeIter {
        self.store.append(parent)
    }
    
    pub fn set_item_values(&self, iter: &gtk::TreeIter, name: &str, icon: &str, enabled: bool, item_type: &str) {
        self.store.set(iter, &[
            (0, &name),
            (1, &icon),
            (2, &enabled),
            (3, &item_type),
        ]);
    }
    
    pub fn get_selection(&self) -> Option<gtk::TreeIter> {
        let selection = self.widget.selection();
        if let Some((_, iter)) = selection.selected() {
            Some(iter)
        } else {
            None
        }
    }
    
    pub fn expand_item(&self, iter: &gtk::TreeIter) {
        let path = self.store.path(iter);
        if let Some(path) = path {
            self.widget.expand_row(&path, false);
        }
    }
    
    pub fn collapse_item(&self, iter: &gtk::TreeIter) {
        let path = self.store.path(iter);
        if let Some(path) = path {
            self.widget.collapse_row(&path);
        }
    }
    
    pub fn expand_all(&self) {
        self.widget.expand_all();
    }
    
    pub fn collapse_all(&self) {
        self.widget.collapse_all();
    }
} 