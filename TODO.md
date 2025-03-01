# EventGhost Rust TODO

## UI
- [x] Main window layout
- [x] Menu bar
- [x] Tool bar
- [x] Status bar
- [x] Log window
- [x] Configuration view
  - [x] UI for adding/editing plugins, folders, macros, events, actions
  - [x] Integrate with main window 
- [x] Tree view
  - [x] Hierarchical view of plugins, folders, macros, events, actions
  - [x] Context menu
  - [x] Drag and drop support
  - [x] Parent-child validation
  - [x] Auto-save functionality
- [x] Basic UI layout
- [x] Toolbar with icons
  - [x] Fix resource loading for icons in main application
  - [x] Ensure consistent resource handling between test and main binaries
- [x] Menu bar
- [x] Status bar
- [x] Log view
- [x] Configuration tree view
- [x] Save/load configuration
  - [x] Command line argument handling for configuration paths
  - [x] Connect file menu actions to save/load functions
  - [x] Support for .egtree XML format
  - [x] Fix path handling for configuration files
  - [x] Enhance .egtree file format support with base64 encoding/decoding

## Core
- [x] Plugin System
  - [x] Plugin traits and registry
  - [x] Plugin discovery and loading infrastructure
  - [x] Dynamic library loading implementation
  - [x] Plugin hot-reloading
  - [x] Route events to/from plugins
  - [x] Error handling
  - [x] Plugin reload process
- [ ] Event Handling  
  - [x] Event bus / message passing
  - [x] Event dispatch and routing
- [x] Macro Execution
  - [x] Macro execution engine
  - [x] Map events to macros
  - [x] Flow control (if/else, loops, waits)
- [ ] Action Execution  
  - [ ] Execute action sequences
  - [ ] Builtin actions
  - [ ] Plugin-defined actions
- [x] Persistence
  - [ ] Save/load configuration (partially implemented but not working)
  - [x] Auto-save
  - [ ] Import/export
  
## Build and Deployment
- [x] Fix "export ordinal too large" errors in DLL builds
- [x] Configure build for different scenarios (testing, production)
- [x] Separate library (rlib) from dynamically linked library (cdylib)
- [ ] Windows installer
- [ ] Linux packages
- [ ] Mac bundle

## Plugins
- [ ] Python plugin loader
- [x] Example logger plugin
  - [x] Update to latest plugin traits
  - [x] Proper error handling
  - [x] State management
- [ ] Common plugins (to be defined)

## Documentation
- [x] User guide
- [ ] API docs for plugins
- [x] Contributor guide

## Action System Implementation

### Core Action Framework
- [x] Basic action trait and manager implementation
- [x] Action result handling
- [x] Action configuration system
- [x] Action grouping support
- [ ] Action compilation system
  - [ ] Pre-execution optimization
  - [ ] Caching compiled actions
  - [ ] Invalidation mechanism
- [ ] Action thread management
  - [ ] Dedicated action execution thread
  - [ ] Thread pool for parallel actions
  - [ ] Action execution queue
- [ ] Action state management
  - [ ] Persistent state storage
  - [ ] State restoration on reload
  - [ ] State sharing between actions

### Action UI Components
- [x] Action configuration dialog
  - [x] Dynamic form generation based on action parameters
  - [x] Property grid for parameter display and editing
  - [ ] Validation feedback
  - [ ] Help text integration
- [ ] Action group tree view
  - [ ] Hierarchical display of action groups
  - [ ] Drag and drop support
  - [ ] Context menu integration
- [ ] Action execution feedback
  - [ ] Progress indicators
  - [ ] Success/failure notifications
  - [ ] Execution log integration

### Standard Actions
- [x] Flow Control Actions
  - [x] Conditional execution (If/Else)
  - [x] Loops (While/For)
  - [x] Delay/Wait
  - [ ] Jump to macro
- [ ] System Actions
  - [ ] Execute program
  - [ ] System commands
  - [ ] Registry operations
  - [ ] File operations
- [ ] Window Actions
  - [ ] Window manipulation
  - [ ] Send keys
  - [ ] Mouse control
- [ ] Variable Actions
  - [ ] Set/Get variables
  - [ ] Mathematical operations
  - [ ] String operations

### Plugin Integration
- [x] Action registration API
  - [x] Plugin action discovery
  - [ ] Dynamic action loading
  - [ ] Version compatibility checking
- [ ] Action documentation
  - [ ] Help page generation
  - [ ] Example code
  - [ ] Parameter documentation
- [x] Action testing framework
  - [x] Unit test helpers
  - [ ] Integration test support
  - [ ] Mock event generation

### Persistence
- [ ] Action configuration storage
  - [ ] Save/load action parameters
  - [ ] Export/import actions
  - [ ] Configuration versioning
- [ ] Action state persistence
  - [ ] Runtime state serialization
  - [ ] State recovery mechanisms
  - [ ] Clean shutdown handling

### Documentation
- [ ] Action development guide
  - [ ] Creating new actions
  - [ ] Best practices
  - [ ] Example implementations
- [ ] User documentation
  - [ ] Action usage guide
  - [ ] Configuration examples
  - [ ] Troubleshooting guide

## Next Steps (Priority Order)
1. ~~Add action configuration UI~~
2. ~~Fix Result unwrapping in main.rs~~
3. ~~Fix GTK4 compatibility issues~~
4. ~~Enhance .egtree file format support~~
5. ~~Fix resource loading for icons~~
6. Create system action implementations 
7. Implement action thread management
8. Add action persistence
9. Develop plugin action integration
10. Write documentation

## Future Enhancements
- [ ] Action templates
- [ ] Action recording
- [ ] Action debugging tools
- [ ] Action performance profiling
- [ ] Action security sandbox
- [ ] Remote action execution

## Python Compatibility
- [x] Implement eg.globals equivalent
  - [x] Support for local, MQTT, and Redis backends
  - [x] Serialization of different data types
  - [x] Publish/subscribe functionality
  - [ ] Additional compatibility with legacy Python code
- [ ] Support legacy Python plugins
  - [ ] Python binding layer
  - [ ] Python plugin bridge
  - [ ] Support for Python callables in actions
- [ ] Core API compatibility
  - [ ] Match Python API function signatures
  - [ ] Support for Python events in Rust
- [ ] UI compatibility
  - [ ] Support for Python UI elements
  - [ ] Python-defined configuration panels
  - [ ] Legacy dialog support

## Plugin Parity
- [ ] IR Receiver Plugins
  - [ ] USB-UIRT
  - [ ] LIRC
  - [ ] Streamzap
  - [ ] Microsoft MCE Remote
- [ ] System Control Plugins
  - [ ] Window control
  - [ ] Keyboard emulation
  - [ ] Mouse control
  - [ ] File operations
- [ ] Network Plugins
  - [ ] NetworkSender/Receiver
  - [ ] Webserver
  - [ ] MQTT Client
- [ ] Media Control Plugins
  - [ ] VLC
  - [ ] Media Player Classic
  - [ ] WinAmp
- [ ] Hardware Support
  - [ ] Serial port communication
  - [ ] USB device support
  - [ ] HID device support
- [ ] Utility Plugins
  - [ ] Speech (TTS)
  - [ ] Sound mixer
  - [ ] Timer/Scheduler

## Core Functionality Gaps
- [ ] WinAPI integration for hardware/system access
- [ ] Plugin installation/update system
- [ ] Language translation system
- [ ] Remote control functionality
- [ ] Auto-start and tray icon functionality
- [ ] Registry handling

## Lessons Learned
- Rust's ownership model requires careful design of event and plugin systems
- GTK4 migration takes significant effort but provides a more modern UI framework
- Mock implementations are crucial for testing in a UI-heavy application
- Need to balance Python compatibility with modern Rust idioms
- Consider approach for plugins: native Rust vs Python bridge vs both
- Resource loading in GTK applications requires proper initialization and path handling
- Borrow checker issues in GTK applications often require restructuring UI initialization


