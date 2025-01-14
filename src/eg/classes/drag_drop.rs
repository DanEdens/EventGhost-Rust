use windows::Win32::Foundation::HWND;
use crate::core::Error;
use std::path::PathBuf;

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

pub trait DropTarget {
    fn can_drop(&self, data: &DragData, effects: DragEffects) -> bool;
    fn on_drop(&mut self, data: DragData, effect: DragEffects) -> Result<(), Error>;
    fn on_drag_enter(&mut self, data: &DragData, effects: DragEffects) -> DragEffects;
    fn on_drag_over(&mut self, data: &DragData, effects: DragEffects, x: i32, y: i32) -> DragEffects;
    fn on_drag_leave(&mut self);
}

pub trait DragSource {
    fn begin_drag(&mut self, data: DragData, allowed_effects: DragEffects) -> Result<DragEffects, Error>;
    fn on_give_data(&mut self) -> Result<DragData, Error>;
    fn on_drag_end(&mut self, effect: DragEffects);
}

pub struct DragDropManager {
    hwnd: HWND,
    current_data: Option<DragData>,
    current_effect: DragEffects,
}

impl DragDropManager {
    pub fn new(hwnd: HWND) -> Self {
        todo!()
    }

    pub fn register_drop_target(&mut self, target: Box<dyn DropTarget>) -> Result<(), Error> {
        todo!()
    }

    pub fn unregister_drop_target(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub fn start_drag(&mut self, source: Box<dyn DragSource>, data: DragData, allowed_effects: DragEffects) -> Result<DragEffects, Error> {
        todo!()
    }

    pub fn is_dragging(&self) -> bool {
        todo!()
    }

    pub fn get_drag_data(&self) -> Option<&DragData> {
        todo!()
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