use gtk::prelude::*;
use gtk::{self, Widget, DragSource as GtkDragSource, DropTarget as GtkDropTarget};
use gtk::gdk;
use gtk::glib;
use std::path::PathBuf;
use crate::core::Error;
use std::sync::{Arc, Mutex};
use gdk::DragAction;
use super::UIComponent;

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
        let source = DragSource::new();
        widget.add_controller(source);
    }

    pub fn setup_drop_target(&self, widget: &impl IsA<Widget>) {
        let target = DropTarget::new(None::<glib::Type>, gdk::DragAction::COPY);
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
        widget.add_controller(&source);
        
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
            let files: Vec<gtk::gio::File> = prepare_files()
                .into_iter()
                .map(|p| gtk::gio::File::for_path(p))
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
        widget.add_controller(&target);
        
        DropTargetWrapper {
            widget,
            target,
        }
    }
    
    pub fn new_for_files(widget: Widget) -> Self {
        let target = GtkDropTarget::new(gtk::gio::File::static_type(), gdk::DragAction::COPY);
        widget.add_controller(&target);
        
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
            if let Ok(files) = value.get::<Vec<gtk::gio::File>>() {
                let paths: Vec<PathBuf> = files
                    .iter()
                    .filter_map(|f| f.path())
                    .collect();
                callback(paths);
                true
            } else {
                false
            }
        });
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
} 