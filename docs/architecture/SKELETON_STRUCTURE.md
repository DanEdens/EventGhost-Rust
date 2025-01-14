# EventGhost Rust Skeleton Structure

## Directory Layout [X]
```
src/
├── core/ [X]
│   ├── mod.rs [X]
│   ├── event.rs        # Event system with async support [X]
│   ├── plugin.rs       # Plugin system with property support [X]
│   ├── gui.rs          # GUI abstractions [X]
│   ├── init.rs         # Initialization [X]
│   ├── named_pipe.rs   # IPC [ ]
│   ├── utils.rs        # Utilities [X]
│   └── error.rs        # Error types [X]
├── eg/ [X]
│   ├── mod.rs          # Main eg module [X]
│   ├── bunch.rs        # Thread-safe property storage [X]
│   ├── globals.rs      # Global state [X]
│   ├── document.rs     # Tree document management [X]
│   ├── action/ [ ]
│   │   ├── mod.rs
│   │   ├── base.rs     # ActionBase trait
│   │   ├── group.rs    # ActionGroup management
│   │   └── item.rs     # ActionItem implementation
│   ├── tree/ [X]
│   │   ├── mod.rs [X]
│   │   ├── item.rs     # Base TreeItem trait [X]
│   │   ├── link.rs     # TreeLink for references [X]
│   │   ├── folder.rs   # FolderItem implementation [X]
│   │   ├── macro.rs    # MacroItem implementation [X]
│   │   └── root.rs     # RootItem implementation [X]
│   ├── winapi/ [X]
│   │   ├── mod.rs [X]
│   │   └── utils.rs [X]
│   └── classes/ [X]
│       ├── mod.rs [X]
│       ├── main_frame.rs [X]
│       ├── tree_ctrl.rs [X]
│       ├── log_ctrl.rs [X]
│       ├── guid.rs [X]
│       ├── property_grid.rs [X]
│       ├── plugin_config.rs [X]
│       └── drag_drop.rs [X]
└── main.rs [X]
```

## Core Traits

### Plugin System [X]
```rust
pub trait Plugin: PropertySource + Send + Sync {
    fn get_info(&self) -> PluginInfo;
    fn initialize(&mut self) -> Result<(), Error>;
    fn start(&mut self) -> Result<(), Error>;
    fn stop(&mut self) -> Result<(), Error>;
    fn configure(&mut self) -> Option<ConfigDialog>;
    fn handle_event(&mut self, event: &Event) -> Result<(), Error>;
    fn add_action(&mut self, action: Box<dyn ActionBase>);
    fn get_actions(&self) -> &[Box<dyn ActionBase>];
}

pub trait PropertySource {
    fn get_properties(&self) -> Vec<Property>;
    fn set_property(&mut self, name: &str, value: PropertyValue) -> Result<(), Error>;
    fn validate_property(&self, name: &str, value: &PropertyValue) -> Result<(), String>;
}
```

### Action System [ ]
```rust
pub trait ActionBase: Send + Sync {
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
    fn get_plugin(&self) -> &dyn Plugin;
    fn execute(&mut self, args: &[Value]) -> Result<Value, Error>;
    fn configure(&mut self) -> Option<ConfigDialog>;
    fn clone_action(&self) -> Box<dyn ActionBase>;
}

pub trait ActionGroup {
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
    fn get_icon(&self) -> Option<Icon>;
    fn get_items(&self) -> &[Box<dyn ActionBase>];
    fn add_item(&mut self, item: Box<dyn ActionBase>);
}
```

### Tree System [X]
```rust
pub trait TreeItem: Send + Sync {
    fn get_id(&self) -> Uuid;
    fn get_parent(&self) -> Option<&dyn TreeItem>;
    fn get_children(&self) -> &[Box<dyn TreeItem>];
    fn get_name(&self) -> &str;
    fn is_enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool) -> Result<(), Error>;
    fn execute(&mut self) -> Result<Value, Error>;
    fn clone_item(&self) -> Box<dyn TreeItem>;
}

pub trait TreeLink {
    fn get_target(&self) -> Option<&dyn TreeItem>;
    fn set_target(&mut self, target: Option<Box<dyn TreeItem>>);
    fn clone_link(&self) -> Box<dyn TreeLink>;
}
```

### Document System [X]
```rust
pub struct Document {
    root: Box<dyn TreeItem>,
    selection: Option<Box<dyn TreeItem>>,
    undo_stack: Vec<UndoAction>,
    redo_stack: Vec<UndoAction>,
    is_dirty: bool,
    file_path: Option<PathBuf>,
}

impl Document {
    pub fn new() -> Self;
    pub fn load_file(&mut self, path: &Path) -> Result<(), Error>;
    pub fn save_file(&self, path: &Path) -> Result<(), Error>;
    pub fn undo(&mut self) -> Result<(), Error>;
    pub fn redo(&mut self) -> Result<(), Error>;
    pub fn is_dirty(&self) -> bool;
    pub fn get_selection(&self) -> Option<&dyn TreeItem>;
    pub fn set_selection(&mut self, item: Option<Box<dyn TreeItem>>);
}
```

### Event System [X]
```rust
pub trait Event: Send + Sync {
    fn get_id(&self) -> &str;
    fn get_type(&self) -> EventType;
    fn get_payload(&self) -> &EventPayload;
    fn get_timestamp(&self) -> DateTime<Local>;
    fn get_source(&self) -> Option<&str>;
}

pub trait EventHandler: Send + Sync {
    fn handle_event(&mut self, event: &Event) -> Result<(), Error>;
    fn can_handle(&self, event_type: EventType) -> bool;
}

pub struct EventManager {
    handlers: Vec<Box<dyn EventHandler>>,
    event_queue: VecDeque<Box<dyn Event>>,
}

impl EventManager {
    pub fn new() -> Self;
    pub fn register_handler(&mut self, handler: Box<dyn EventHandler>);
    pub fn unregister_handler(&mut self, id: &str);
    pub fn process_event(&mut self, event: Box<dyn Event>) -> Result<(), Error>;
}
```

### State Management [X]

#### Bunch Implementation [X]
```rust
pub struct Bunch {
    data: Arc<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>>,
}

impl Bunch {
    pub fn new() -> Self;
    pub fn set<T: 'static + Send + Sync>(&self, key: &str, value: T) -> Result<(), Error>;
    pub fn get<T: 'static + Clone>(&self, key: &str) -> Result<Option<T>, Error>;
    pub fn remove(&self, key: &str) -> Result<(), Error>;
}
```

#### Global State [X]
```rust
pub struct Globals {
    pub bunch: Bunch,
    pub plugins: PluginRegistry,
    pub event_handlers: Vec<Box<dyn EventHandler>>,
    pub config: Configuration,
    pub document: Document,
}

impl Globals {
    pub fn new() -> Self;
    pub fn initialize(&mut self) -> Result<(), Error>;
    pub fn register_event_handler(&mut self, handler: Box<dyn EventHandler>);
    pub fn dispatch_event(&mut self, event: &Event) -> Result<(), Error>;
}
```

## Error Handling [X]
```rust
#[derive(Debug, Error)]
pub enum Error {
    #[error("Plugin error: {0}")]
    Plugin(String),
    
    #[error("Event error: {0}")]
    Event(String),
    
    #[error("GUI error: {0}")]
    Gui(String),
    
    #[error("Property error: {0}")]
    Property(String),
    
    #[error("Config error: {0}")]
    Config(String),
    
    #[error("Tree error: {0}")]
    Tree(String),
    
    #[error("Document error: {0}")]
    Document(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

## Main Entry Point [X]

### src/main.rs [X]
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

## Next Steps [ ]

1. [X] Create the basic directory structure
2. [X] Implement core error handling
3. [X] Setup event system
4. [X] Implement state management (Bunch and Globals)
5. [X] Create UI component framework
6. [X] Implement tree system
7. [ ] Complete action system implementation
8. [ ] Implement IPC with named pipes
9. [ ] Add plugin hot-reloading support
10. [ ] Implement configuration persistence
11. [ ] Add comprehensive testing suite 