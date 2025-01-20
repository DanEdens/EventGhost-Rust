use gtk::prelude::*;
use gtk::{self, TreeView, TreeStore, TreeSelection, TreePath, TreeIter};
use glib;
use super::UIComponent;

pub struct TreeItem {
    pub data: Option<Box<dyn std::any::Any + Send + Sync>>,
    pub iter: TreeIter,
}

pub struct TreeCtrl {
    pub container: gtk::Box,
    pub tree_view: TreeView,
    pub store: TreeStore,
    selection: TreeSelection,
}

impl TreeCtrl {
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let store = TreeStore::new(&[glib::Type::STRING]);
        let tree_view = TreeView::new();
        tree_view.set_model(Some(&store));
        
        let selection = tree_view.selection();
        
        container.append(&tree_view);
        
        TreeCtrl {
            container,
            tree_view,
            store,
            selection,
        }
    }
    
    pub fn expand_item(&self, iter: &gtk::TreeIter) {
        if let Some(path) = self.store.path(iter) {
            self.tree_view.expand_row(&path, false);
        }
    }
    
    pub fn collapse_item(&self, iter: &gtk::TreeIter) {
        if let Some(path) = self.store.path(iter) {
            self.tree_view.collapse_row(&path);
        }
    }
    
    pub fn expand_all(&self) {
        self.tree_view.expand_all();
    }
    
    pub fn collapse_all(&self) {
        self.tree_view.collapse_all();
    }
    
    pub fn get_selection(&self) -> &TreeSelection {
        &self.selection
    }
    
    pub fn get_path(&self, iter: &TreeIter) -> Option<TreePath> {
        self.store.path(iter)
    }
}

impl UIComponent for TreeCtrl {
    fn get_widget(&self) -> &gtk::Widget {
        self.container.upcast_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tree_ctrl_initialization() {
        gtk::init().expect("Failed to initialize GTK");
        
        let tree_ctrl = TreeCtrl::new();
        assert!(tree_ctrl.container.is_visible());
    }
} 