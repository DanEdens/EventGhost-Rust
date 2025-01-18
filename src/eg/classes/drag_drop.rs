use windows::Win32::Foundation::HWND;
use crate::core::Error;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

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
    hwnd: HWND,
    current_data: Arc<Mutex<Option<DragData>>>,
    current_effect: Arc<Mutex<DragEffects>>,
    target: Arc<Mutex<Option<Box<dyn DropTarget>>>>,
}

impl DragDropManager {
    pub fn new(hwnd: HWND) -> Self {
        Self {
            hwnd,
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

    pub fn start_drag(&mut self, source: Box<dyn DragSource>, data: DragData, allowed_effects: DragEffects) -> Result<DragEffects, Error> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestDropTarget;
    
    impl DropTarget for TestDropTarget {
        fn can_drop(&self, data: &DragData, effects: DragEffects) -> bool {
            matches!(data, DragData::Text(_))
        }

        fn on_drop(&mut self, data: DragData, effect: DragEffects) -> Result<(), Error> {
            Ok(())
        }

        fn on_drag_enter(&mut self, data: &DragData, effects: DragEffects) -> DragEffects {
            if self.can_drop(data, effects) {
                DragEffects::COPY
            } else {
                DragEffects::NONE
            }
        }

        fn on_drag_over(&mut self, data: &DragData, effects: DragEffects, _x: i32, _y: i32) -> DragEffects {
            if self.can_drop(data, effects) {
                DragEffects::COPY
            } else {
                DragEffects::NONE
            }
        }

        fn on_drag_leave(&mut self) {}
    }
} 