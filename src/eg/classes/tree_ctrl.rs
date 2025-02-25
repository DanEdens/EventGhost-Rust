use gtk::prelude::*;
use gtk::{self, TreeView, TreeStore, TreeSelection, TreePath, TreeIter, SelectionMode};
// use gdk4::{self, DragAction}; // Comment this out as it has version conflicts
use gtk::{DragSource as GtkDragSource};
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
    pub view: TreeView,
    drag_source: GtkDragSource,
}

impl TreeCtrl {
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let store = TreeStore::new(&[glib::Type::STRING]);
        let tree_view = TreeView::new();
        tree_view.set_model(Some(&store));
        
        let selection = tree_view.selection();
        selection.set_mode(SelectionMode::Single);
        
        let drag_source = GtkDragSource::new();
        drag_source.set_actions(gtk::gdk::DragAction::COPY | gtk::gdk::DragAction::MOVE);
        tree_view.add_controller(drag_source.clone());
        
        container.append(&tree_view);
        
        TreeCtrl {
            container,
            tree_view: tree_view.clone(),
            store,
            selection,
            drag_source,
            view: tree_view,
        }
    }
    
    pub fn expand_item(&self, iter: &gtk::TreeIter) {
        let path = self.store.path(iter);
        self.tree_view.expand_row(&path, false);
    }
    
    pub fn collapse_item(&self, iter: &gtk::TreeIter) {
        let path = self.store.path(iter);
        self.tree_view.collapse_row(&path);
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
    
    pub fn get_path(&self, iter: &TreeIter) -> TreePath {
        self.store.path(iter)
    }

    pub fn get_iter(&self, path: &TreePath) -> Option<TreeIter> {
        self.store.iter(path)
    }

    pub fn get_path_string(&self, iter: &TreeIter) -> Option<String> {
        let path = self.store.path(iter);
        // Convert the TreePath to a string representation
        let indices = path.indices();
        let mut index_strs = Vec::new();
        for i in 0..indices.len() {
            index_strs.push(indices[i].to_string());
        }
        Some(index_strs.join(":"))
    }
    
    pub fn expand_row(&self, iter: &TreeIter, recursive: bool) -> bool {
        let path = self.store.path(iter);
        self.view.expand_row(&path, recursive)
    }
    
    pub fn collapse_row(&self, iter: &TreeIter) -> bool {
        let path = self.store.path(iter);
        self.view.collapse_row(&path)
    }

    // Get the selected path as a string
    pub fn get_selected_path_string(&self) -> Option<String> {
        self.get_selected_path()
            .map(|path| {
                // Convert the TreePath to a string representation
                let indices = path.indices();
                let mut index_strs = Vec::new();
                for i in 0..indices.len() {
                    index_strs.push(indices[i].to_string());
                }
                index_strs.join(":")
            })
    }

    pub fn get_selected_path(&self) -> Option<TreePath> {
        if let Some((model, iter)) = self.selection.selected() {
            Some(model.path(&iter))
        } else {
            None
        }
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