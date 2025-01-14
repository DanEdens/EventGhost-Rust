use std::sync::Arc;
use parking_lot::RwLock;

pub mod bunch;
pub mod globals;
pub mod winapi;
pub mod classes;

pub use bunch::Bunch;
pub use globals::Globals;

pub struct EventGhost {
    pub globals: Arc<RwLock<Globals>>,
    pub plugins: Bunch,
    pub document: Option<Document>,
    pub main_frame: Option<MainFrame>,
    pub event: Option<EventGhostEvent>,
}

impl EventGhost {
    pub fn new() -> Self {
        todo!()
    }
    
    pub fn initialize(&mut self) -> Result<(), crate::core::Error> {
        todo!()
    }
    
    pub fn start(&mut self) -> Result<(), crate::core::Error> {
        todo!()
    }
    
    pub fn stop(&mut self) -> Result<(), crate::core::Error> {
        todo!()
    }
} 