# EventGhost Rust Skeleton Structure

## Directory Layout
```
src/
├── core/
│   ├── mod.rs
│   ├── event.rs
│   ├── plugin.rs
│   ├── gui.rs
│   ├── init.rs
│   ├── named_pipe.rs
│   ├── utils.rs
│   └── cli.rs
├── eg/
│   ├── mod.rs           # Main eg module
│   ├── bunch.rs         # Bunch implementation
│   ├── globals.rs       # Global state management
│   ├── winapi/
│   │   ├── mod.rs
│   │   └── utils.rs
│   └── classes/
│       ├── mod.rs
│       ├── main_frame.rs
│       ├── tree_ctrl.rs
│       ├── log_ctrl.rs
│       └── guid.rs
└── main.rs
```

## Core Module Skeletons

### src/core/mod.rs
```rust
pub mod event;
pub mod plugin;
pub mod gui;
pub mod init;
pub mod named_pipe;
pub mod utils;
pub mod cli;

pub use event::*;
pub use plugin::*;
pub use gui::*;
```

### src/core/event.rs
```rust
use chrono::{DateTime, Local};
use uuid::Uuid;

pub trait Event {
    fn get_id(&self) -> &str;
    fn get_type(&self) -> EventType;
    fn get_payload(&self) -> &EventPayload;
    fn get_timestamp(&self) -> DateTime<Local>;
}

pub struct EventGhostEvent {
    id: String,
    event_type: EventType,
    payload: EventPayload,
    timestamp: DateTime<Local>,
}

impl Event for EventGhostEvent {
    fn get_id(&self) -> &str { todo!() }
    fn get_type(&self) -> EventType { todo!() }
    fn get_payload(&self) -> &EventPayload { todo!() }
    fn get_timestamp(&self) -> DateTime<Local> { todo!() }
}
```

### src/core/plugin.rs
```rust
use std::path::Path;

pub trait Plugin {
    fn get_info(&self) -> PluginInfo;
    fn initialize(&mut self) -> Result<(), Error>;
    fn start(&mut self) -> Result<(), Error>;
    fn stop(&mut self) -> Result<(), Error>;
    fn configure(&mut self) -> Option<ConfigResult>;
}

pub struct PluginInfo {
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub guid: String,
}

pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self { todo!() }
    pub fn load_plugin(&mut self, path: &Path) -> Result<(), Error> { todo!() }
    pub fn start_plugin(&mut self, id: &str) -> Result<(), Error> { todo!() }
    pub fn stop_plugin(&mut self, id: &str) -> Result<(), Error> { todo!() }
}
```

### src/eg/mod.rs
```rust
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
    pub fn new() -> Self { todo!() }
    pub fn initialize(&mut self) -> Result<(), Error> { todo!() }
    pub fn start(&mut self) -> Result<(), Error> { todo!() }
    pub fn stop(&mut self) -> Result<(), Error> { todo!() }
}
```

### src/eg/bunch.rs
```rust
use std::collections::HashMap;
use std::any::Any;

pub struct Bunch {
    data: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl Bunch {
    pub fn new() -> Self { todo!() }
    pub fn set<T: 'static + Send + Sync>(&mut self, key: &str, value: T) { todo!() }
    pub fn get<T: 'static>(&self, key: &str) -> Option<&T> { todo!() }
    pub fn get_mut<T: 'static>(&mut self, key: &str) -> Option<&mut T> { todo!() }
}
```

### src/eg/globals.rs
```rust
use super::Bunch;

pub struct Globals {
    pub bunch: Bunch,
    pub debug_level: i32,
    pub system_encoding: String,
    pub program_counter: Option<usize>,
    pub stop_execution_flag: bool,
}

impl Globals {
    pub fn new() -> Self { todo!() }
    pub fn initialize(&mut self) -> Result<(), Error> { todo!() }
}
```

### src/eg/classes/main_frame.rs
```rust
use super::tree_ctrl::TreeCtrl;
use super::log_ctrl::LogCtrl;

pub struct MainFrame {
    pub tree_ctrl: TreeCtrl,
    pub log_ctrl: LogCtrl,
    pub status_bar: StatusBar,
}

impl MainFrame {
    pub fn new() -> Self { todo!() }
    pub fn show(&mut self) { todo!() }
    pub fn hide(&mut self) { todo!() }
    pub fn process_event(&mut self, event: WindowEvent) { todo!() }
}
```

## Error Types

### src/core/error.rs
```rust
#[derive(Debug, Error)]
pub enum Error {
    #[error("Plugin error: {0}")]
    Plugin(String),
    
    #[error("Event error: {0}")]
    Event(String),
    
    #[error("GUI error: {0}")]
    Gui(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("System error: {0}")]
    System(String),
}
```

## Constants

### src/core/constants.rs
```rust
pub const CORE_PLUGIN_GUIDS: &[&str] = &[
    "{9D499A2C-72B6-40B0-8C8C-995831B10BB4}",  // "EventGhost"
    "{A21F443B-221D-44E4-8596-E1ED7100E0A4}",  // "System"
    "{E974D074-B0A3-4D0C-BBD1-992475DDD69D}",  // "Window"
    "{6B1751BF-F94E-4260-AB7E-64C0693FD959}",  // "Mouse"
];

pub const DEFAULT_DEBUG_LEVEL: i32 = 0;
pub const DEFAULT_ENCODING: &str = "utf-8";
```

## Main Entry Point

### src/main.rs
```rust
use eg::EventGhost;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut eg = EventGhost::new();
    eg.initialize()?;
    eg.start()?;
    
    // Main event loop
    loop {
        if eg.globals.read().stop_execution_flag {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    
    eg.stop()?;
    Ok(())
}
```

## Next Steps

1. Create the basic directory structure
2. Add empty files with module declarations
3. Implement the skeleton structs and traits with `todo!()`
4. Setup basic error handling
5. Add initial test framework
6. Create basic build configuration 