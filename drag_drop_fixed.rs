use gtk::prelude::*;
use gtk::{self, Widget, DragSource as GtkDragSource, DropTarget as GtkDropTarget};
use gtk::gdk;
use gtk::glib;
use std::path::PathBuf;
use crate::core::Error;
use std::sync::{Arc, Mutex};
use gdk::DragAction;
use super::UIComponent;
use gtk::TreeView;
use gtk::TreeStore;
use gtk::TreeIter;
use gtk::TreePath;
use gdk4::ModifierType;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum DragData {
    Text(String),
    Files(Vec<PathBuf>),
    Custom(String, Vec<u8>),
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct DragEffects: u32 {
        const NONE = 0;
        const COPY = 1;
        const MOVE = 2;
        const LINK = 4;
        const SCROLL = 0x80000000;
    }
}

pub trait DropTarget: Send + Sync {
    fn can_drop(&self, data: &DragData, effects: DragEffects) -> bool;
    fn on_drop(&mut self, data: DragData, effect: DragEffects) -> Result<(), Error>;
    fn on_drag_enter(&mut self, data: &DragData, effects: DragEffects) -> DragEffects;
    fn on_drag_over(&mut self, data: &DragData, effects: DragEffects, x: i32, y: i32) -> DragEffects;
    fn on_drag_leave(&mut self);
}

pub trait DragSource: Send + Sync {
    fn begin_drag(&mut self, data: DragData, allowed_effects: DragEffects) -> Result<DragEffects, Error>;
    fn on_give_data(&mut self) -> Result<DragData, Error>;
    fn on_drag_end(&mut self, effect: DragEffects);
}

#[derive(Debug)]
pub enum DragDropError {
    AlreadyDragging,
    NoTarget,
    InvalidData,
    OperationFailed(String),
}

impl std::fmt::Display for DragDropError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyDragging => write!(f, "Drag operation already in progress"),
            Self::NoTarget => write!(f, "No drop target registered"),
            Self::InvalidData => write!(f, "Invalid drag data"),
            Self::OperationFailed(msg) => write!(f, "Operation failed: {}", msg),
        }
    }
}

impl std::error::Error for DragDropError {}

pub struct DragDropManager {
    widget: gtk::Widget,
    current_data: Arc<Mutex<Option<DragData>>>,
    current_effect: Arc<Mutex<DragEffects>>,
    target: Arc<Mutex<Option<Box<dyn DropTarget>>>>,
}

impl DragDropManager {
    pub fn new(widget: gtk::Widget) -> Self {
        Self {
            widget,
            current_data: Arc::new(Mutex::new(None)),
            current_effect: Arc::new(Mutex::new(DragEffects::NONE)),
            target: Arc::new(Mutex::new(None)),
        }
    }

    pub fn register_drop_target(&mut self, target: Box<dyn DropTarget>) -> Result<(), Error> {
        let mut target_lock = self.target.lock().map_err(|_| {
            Error::Config("Failed to lock target mutex".into())
        })?;
        *target_lock = Some(target);
        Ok(())
    }

    pub fn unregister_drop_target(&mut self) -> Result<(), Error> {
        let mut target_lock = self.target.lock().map_err(|_| {
            Error::Config("Failed to lock target mutex".into())
        })?;
        *target_lock = None;
        Ok(())
    }

    pub fn start_drag(&mut self, mut source: Box<dyn DragSource>, data: DragData, allowed_effects: DragEffects) -> Result<DragEffects, Error> {
        let mut current_data = self.current_data.lock().map_err(|_| {
            Error::Config("Failed to lock data mutex".into())
        })?;

        if current_data.is_some() {
            return Err(Error::Config(DragDropError::AlreadyDragging.to_string().into()));
        }

        *current_data = Some(data);
        let effect = source.begin_drag(current_data.clone().unwrap(), allowed_effects)?;
        
        let mut effect_lock = self.current_effect.lock().map_err(|_| {
            Error::Config("Failed to lock effect mutex".into())
        })?;
        *effect_lock = effect;
        
        Ok(effect)
    }

    pub fn is_dragging(&self) -> bool {
        self.current_data.lock().ok()
            .map(|data| data.is_some())
            .unwrap_or(false)
    }

    pub fn get_drag_data(&self) -> Option<DragData> {
        self.current_data.lock().ok()
            .and_then(|data| data.clone())
    }

    pub fn setup_drag_source(&self, widget: &impl IsA<Widget>) {
        let source = GtkDragSource::new();
        widget.add_controller(source);
    }

    pub fn setup_drop_target(&self, widget: &impl IsA<Widget>) {
        let target = GtkDropTarget::new(glib::Type::STRING, gdk::DragAction::COPY);
        widget.add_controller(target);
    }
}

pub struct DragSourceWrapper {
    pub widget: Widget,
    pub source: GtkDragSource,
}

impl DragSourceWrapper {
    pub fn new(widget: Widget) -> Self {
        let source = GtkDragSource::new();
        source.set_actions(gdk::DragAction::COPY | gdk::DragAction::MOVE);
        widget.add_controller(source.clone());
        
        DragSourceWrapper {
            widget,
            source,
        }
    }
    
    pub fn set_data<F>(&self, prepare_data: F)
    where
        F: Fn() -> String + 'static,
    {
        self.source.connect_prepare(move |_, _, _| {
            Some(gdk::ContentProvider::for_value(&prepare_data().to_value()))
        });
    }
    
    pub fn set_files<F>(&self, prepare_files: F)
    where
        F: Fn() -> Vec<PathBuf> + 'static,
    {
        self.source.connect_prepare(move |_, _, _| {
            let files: Vec<String> = prepare_files()
                .into_iter()
                .filter_map(|p| p.to_str().map(|s| s.to_string()))
                .collect();
            Some(gdk::ContentProvider::for_value(&files.to_value()))
        });
    }
}

pub struct DropTargetWrapper {
    pub widget: Widget,
    pub target: GtkDropTarget,
}

impl DropTargetWrapper {
    pub fn new(widget: Widget) -> Self {
        let target = GtkDropTarget::new(glib::Type::STRING, gdk::DragAction::COPY);
        widget.add_controller(target.clone());
        
        DropTargetWrapper {
            widget,
            target,
        }
    }
    
    pub fn new_for_files(widget: Widget) -> Self {
        let target = GtkDropTarget::new(gtk::gio::File::static_type(), gdk::DragAction::COPY);
        widget.add_controller(target.clone());
        
        DropTargetWrapper {
            widget,
            target,
        }
    }
    
    pub fn on_drop<F>(&self, callback: F)
    where
        F: Fn(String) + 'static,
    {
        self.target.connect_drop(move |_, value, _, _| {
            if let Ok(text) = value.get::<String>() {
                callback(text);
                true
            } else {
                false
            }
        });
    }
    
    pub fn on_drop_files<F>(&self, callback: F)
    where
        F: Fn(Vec<PathBuf>) + 'static,
    {
        self.target.connect_drop(move |_, value, _, _| {
            // GTK4 doesn't support File lists directly through Value, so we use strings
            if let Ok(paths_string) = value.get::<String>() {
                let paths: Vec<PathBuf> = paths_string.split('\n')
                    .filter(|s| !s.is_empty())
                    .map(|s| PathBuf::from(s))
                    .collect();
                if !paths.is_empty() {
                    callback(paths);
                    return true;
                }
            }
            false
        });
    }
}

/// Enables drag and drop for a tree view
pub fn enable_drag_drop(tree_view: &TreeView, tree_store: &TreeStore) {
    // Set up drag source
    let drag_source = GtkDragSource::new();
    drag_source.set_actions(gdk::DragAction::MOVE);
    
    // Set up content provider when drag begins
    let tree_view_for_drag = tree_view.clone();
    drag_source.connect_prepare(move |_, x, y| {
        let selection = tree_view_for_drag.selection();
        if let Some((model, iter)) = selection.selected() {
            // Convert TreeIter to path string
            let path = model.path(&iter);
            
            // Convert path to string
            let indices = path.indices();
            let mut index_strs = Vec::new();
            for i in 0..indices.len() {
                index_strs.push(indices[i].to_string());
            }
            let path_str = index_strs.join(":");
            
            return Some(gdk::ContentProvider::for_value(&path_str.to_value()));
        }
        None
    });
    
    tree_view.add_controller(drag_source);
    
    // Set up drop target
    let drop_target = GtkDropTarget::new(glib::Type::STRING, gdk::DragAction::MOVE);
    let tree_view_for_drop = tree_view.clone();
    let tree_store_for_drop = tree_store.clone();
    
    drop_target.connect_drop(move |_, value, x, y| {
        if let Ok(path_string) = value.get::<String>() {
            // Parse source path string back to TreePath
            if let Some(source_path_components) = path_string.split(':')
                .map(|s| s.parse::<i32>().ok())
                .collect::<Option<Vec<i32>>>() {
                let mut source_path = TreePath::new();
                for idx in source_path_components {
                    source_path.append_index(idx);
                }
                
                // Get the target path at the drop position
                if let Some((target_path, _, _, _)) = tree_view_for_drop.path_at_pos(x as i32, y as i32) {
                    // Handle the drop logic here
                    // Get source and target iterators
                    if let Some\(source_iter\) = tree_store_for_drop.iter\(&source_path\) {`n                        if let Some\(target_iter\) = tree_store_for_drop.iter\(&target_path\) {
                        
                        
                    ) {
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
        false
    });
    
    tree_view.add_controller(drop_target);
}

/// Handler that prepares data when starting a drag from an item
pub fn on_drag_prepare(view: &TreeView, x: f64, y: f64) -> glib::Value {
    // Create an empty value to return if no path is found
    let empty_value = glib::Value::from(&"");
    
    // Convert float coordinates to integers for GTK4
    if let Some((path, _, _, _)) = view.path_at_pos(x as i32, y as i32) {
        // Get path indices and convert to string
        let indices = path.indices();
        let mut index_strs = Vec::new();
        for i in 0..indices.len() {
            index_strs.push(indices[i].to_string());
        }
        let path_str = index_strs.join(":");
        
        // Return the path string as a Value
        glib::Value::from(&path_str)
    } else {
        empty_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gtk::Button;
    
    #[test]
    fn test_drag_drop() {
        gtk::init().expect("Failed to initialize GTK");
        
        let button = Button::new();
        let source = DragSourceWrapper::new(button.upcast());
        let target = DropTargetWrapper::new(button.upcast());
        
        source.set_data(|| "test data".to_string());
        target.on_drop(|text| {
            assert_eq!(text, "test data");
        });
    }
    
    #[test]
    fn test_drag_drop_files() {
        gtk::init().expect("Failed to initialize GTK");
        
        let button = Button::new();
        let source = DragSourceWrapper::new(button.upcast());
        let target = DropTargetWrapper::new_for_files(button.upcast());
        
        let test_path = PathBuf::from("/test/path");
        source.set_files(move || vec![test_path.clone()]);
        target.on_drop_files(|paths| {
            assert_eq!(paths.len(), 1);
            assert_eq!(paths[0], test_path);
        });
    }
    
    #[test]
    fn test_enable_drag_drop() {
        gtk::init().expect("Failed to initialize GTK");
        
        let tree_store = TreeStore::new(&[
            glib::Type::STRING, // name
            glib::Type::STRING, // type
            glib::Type::STRING, // icon
            glib::Type::STRING, // id
        ]);
        
        let tree_view = TreeView::with_model(&tree_store);
        
        // This should not panic
        enable_drag_drop(&tree_view, &tree_store);
    }
} 
