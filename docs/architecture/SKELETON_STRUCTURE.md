# EventGhost Rust Skeleton Structure

## Directory Layout
```
src/
├── core/
│   ├── mod.rs
│   ├── event.rs        # Event system with async support
│   ├── plugin.rs       # Plugin system with property support
│   ├── gui.rs          # GUI abstractions
│   ├── init.rs         # Initialization
│   ├── named_pipe.rs   # IPC
│   ├── utils.rs        # Utilities
│   └── error.rs        # Error types
├── eg/
│   ├── mod.rs          # Main eg module
│   ├── bunch.rs        # Thread-safe property storage
│   ├── globals.rs      # Global state
│   ├── document.rs     # Tree document management
│   ├── action/
│   │   ├── mod.rs
│   │   ├── base.rs     # ActionBase trait
│   │   ├── group.rs    # ActionGroup management
│   │   └── item.rs     # ActionItem implementation
│   ├── tree/
│   │   ├── mod.rs
│   │   ├── item.rs     # Base TreeItem trait
│   │   ├── link.rs     # TreeLink for references
│   │   ├── folder.rs   # FolderItem implementation
│   │   ├── macro.rs    # MacroItem implementation
│   │   └── root.rs     # RootItem implementation
│   ├── winapi/
│   │   ├── mod.rs
│   │   └── utils.rs
│   └── classes/
│       ├── mod.rs
│       ├── main_frame.rs
│       ├── tree_ctrl.rs
│       ├── log_ctrl.rs
│       ├── guid.rs
│       ├── property_grid.rs
│       ├── plugin_config.rs
│       └── drag_drop.rs
└── main.rs
```

## Core Traits

### Plugin System
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

### Action System
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

### Tree System
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

### Document System
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

### Event System
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

### State Management

#### Bunch Implementation
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

#### Global State
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

## Error Handling
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