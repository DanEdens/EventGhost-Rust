# EventGhost Architecture Overview

## Files Analyzed

### Core Classes
- eg/Classes/ActionBase.py - Base class for all plugin actions
- eg/Classes/ActionGroup.py - Handles grouping of related actions
- eg/Classes/ActionItem.py - Represents individual action instances
- eg/Classes/ActionSelectButton.py - UI component for action selection
- eg/Classes/ActionThread.py - Main execution thread for actions
- eg/Classes/ActionWithStringParameter.py - Action subclass with string config
- eg/Classes/AddActionDialog.py - Dialog for adding new actions
- eg/Classes/AddActionGroupDialog.py - Dialog for adding action groups
- eg/Classes/AddEventDialog.py - Dialog for adding new events
- eg/Classes/AddPluginDialog.py - Dialog for adding plugins
- eg/Classes/AnimatedWindow.py - Animated UI component
- eg/Classes/App.py - Main application class
- eg/Classes/AutostartItem.py - Handles autostart functionality
- eg/Classes/BoxedGroup.py - UI layout component
- eg/Classes/ButtonRow.py - UI button container
- eg/Classes/CheckBoxGrid.py - UI grid of checkboxes
- eg/Classes/CheckUpdate.py - Update checker functionality
- eg/Classes/Choice.py - UI choice component
- eg/Classes/Colour.py - Color management
- eg/Classes/ColourSelectButton.py - UI color picker
- eg/Classes/Config.py - Configuration management
- eg/Classes/ConfigDialog.py - Base configuration dialog
- eg/Classes/ConfigPanel.py - Configuration panel component
- eg/Classes/ContainerItem.py - Base container class

### Core Infrastructure
#### Windows Integration
- eg/NamedPipe.py - IPC via Windows named pipes
  - Implements command processing via named pipes
  - Handles multi-instance pipe connections
  - Provides secure IPC between admin/user contexts
  - Supports async command execution
  - Uses daemon threads for pipe management

#### Event System
- eg/EventThread.py
  - Manages event filtering and execution
  - Handles memory quotas and limitations
  - Controls event session lifecycle
  - Provides synchronous/async event triggers
  - Implements event filtering system

#### Threading and Task Management
- eg/Classes/ThreadWorker.py - Core message pumping and task execution system
- eg/Classes/Tasklet.py - Stackless Python tasklet wrapper implementation
- eg/Classes/TaskletDialog.py - Dialog management using tasklets
- eg/Classes/Scheduler.py - Task scheduling and timing system
- eg/__init__.py - Core initialization and stackless integration
- extensions/cFunctions/hooks.c - Native hooks and idle detection system

#### Plugin System
- eg/Classes/PluginBase.py - Base plugin class and lifecycle management
- eg/Classes/PluginManager.py - Plugin loading and instance management
- eg/Classes/PluginModuleInfo.py - Plugin metadata and registration
- eg/Classes/PluginInstanceInfo.py - Plugin instance state tracking
- eg/Classes/PluginItem.py - Plugin tree item representation
- eg/Classes/PluginInstall.py - Plugin installation and updates

### Analyzed Plugins
#### Complete Analysis with Rust Implementation
- plugins/GlobalMonitor/ - System performance monitoring and metrics
  - Performance counter system
  - Resource monitoring
  - Event generation pipeline
  - Data collection services

- plugins/Mouse/ - Mouse input and control
  - Event generation (button, movement, wheel)
  - Mouse actions and state management
  - Movement control system
  - Multi-monitor support
  - Complete Rust implementation with async/await

#### Core Components and Migration Analysis
- plugins/RadioSure/ - Media player control plugin
  - Window observation system
  - Event generation pipeline
  - Thread management
  - Windows API integration

- plugins/EventGhost/ - Core plugin for events and macros
  - Action management (Python scripts, macros)
  - Flow control (jumps, conditionals)
  - UI integration (messages, displays)
  - System integration

- plugins/Keyboard/ - Keyboard event handling
  - Hotkey detection and blocking
  - Key code translation
  - Windows hook integration
  - Event system binding

- plugins/System/ - System control and hardware
  - Power management
  - Display control
  - Audio control
  - System integration

#### Initial Analysis
- plugins/FileOperations/ - File system operations and monitoring
  - File system watchers
  - Operation interceptors
  - Path management
  - Event triggers

- plugins/DirectoryWatcher/ - Directory monitoring and change detection
  - Directory monitor service
  - Change detection system
  - Filter management
  - Event dispatcher

## Core Systems

### 1. Event System
- Event creation and propagation
- Event queue management
- Event filtering and routing
- Event binding and callbacks
- Custom event types (eg.EventGhostEvent)
- Event Thread Implementation:
  - ThreadWorker-based event processing
  - Memory management and quotas
  - Event filtering system
  - Event execution modes:
    - Normal events
    - Enduring events
    - Wait-for-completion events
- Event Execution:
  - Action thread delegation
  - Event source filtering
  - Payload handling
  - Timeout management
  - Session control
- Event Processing Flow:
  - Event source detection
  - Event object creation
  - Queue management
  - Event dispatching
  - Action triggering
- Event Types:
  - System events
  - Plugin events
  - User events
  - Remote events
  - Timer events
- Event Properties:
  - Event prefix
  - Event suffix
  - Payload data
  - Source information
  - Timestamp

### 2. Threading Model
- Main thread (UI)
- Action thread (Plugin execution)
- Worker threads (Background tasks)
- Stackless Python tasklets
- Thread synchronization mechanisms
- Named pipe communication
- Inter-process messaging
- Windows-specific thread handling

### 3. Plugin System
- Plugin loading/unloading
- Plugin lifecycle management
- Plugin configuration
- Action registration
- Event handling registration
- Dynamic module imports
- Plugin dependency resolution
- Version compatibility checking

#### Core Components:

##### PluginBase Class
- Base class for all EventGhost plugins
- Provides core plugin lifecycle methods:
  - `__init__`: Plugin initialization and action registration
  - `__start__`: Called when plugin is enabled
  - `__stop__`: Called when plugin is disabled
  - `__close__`: Called when plugin is unloaded
- Handles event triggering and management
- Provides configuration interface
- Manages plugin metadata (name, description, version)
- Supports internationalization through text attribute

##### Plugin Manager
- Manages plugin lifecycle and instances
- Maintains plugin database and registration
- Handles plugin loading and instantiation
- Provides plugin info lookup and management
- Supports multi-loading of plugins
- Manages plugin dependencies and versions

##### Plugin Instance Info
- Tracks individual plugin instance state
- Manages plugin actions and events
- Handles plugin evaluation names
- Maintains instance configuration
- Tracks plugin execution state
- Manages plugin exceptions

##### Plugin Module Info
- Handles plugin module registration
- Manages plugin metadata and properties
- Supports plugin localization
- Handles plugin icons and resources
- Validates plugin requirements
- Tracks plugin paths and GUIDs

##### Plugin Installation
- Handles plugin package installation
- Manages plugin dependencies
- Validates plugin compatibility
- Handles plugin updates
- Manages plugin resources

##### Action System
- Base class for all plugin actions
- Supports action configuration
- Handles action execution
- Manages action metadata
- Provides action grouping
- Supports action localization

#### Key Features:

##### Plugin Lifecycle
1. Plugin Discovery
   - Scans plugin directories
   - Loads plugin metadata
   - Validates plugin requirements

2. Plugin Loading
   - Imports plugin module
   - Creates plugin instance
   - Initializes plugin state
   - Registers plugin actions

3. Plugin Execution
   - Handles plugin start/stop
   - Manages plugin state
   - Processes plugin events
   - Executes plugin actions

4. Plugin Cleanup
   - Handles plugin shutdown
   - Cleans up resources
   - Unregisters actions
   - Removes plugin instance

##### Plugin Architecture
1. Module Structure
   - Plugin base class
   - Action definitions
   - Event handlers
   - Configuration interface

2. Resource Management
   - Plugin icons
   - Localization files
   - Configuration data
   - Plugin dependencies

3. Security Model
   - Plugin isolation
   - Resource access control
   - Event filtering
   - Configuration validation

4. Extension Points
   - Action registration
   - Event handling
   - Configuration interface
   - Resource management

##### Plugin Development
1. Base Classes
   - PluginBase for core functionality
   - ActionBase for plugin actions
   - Custom exceptions for error handling

2. Registration System
   - Plugin metadata declaration
   - Action registration
   - Event binding
   - Resource declaration

3. Configuration System
   - Plugin settings
   - Action configuration
   - Persistent storage
   - UI integration

4. Event System
   - Event generation
   - Event handling
   - Event filtering
   - Event persistence

### 4. Configuration Management
- XML-based configuration storage
- Runtime configuration
- User preferences
- Plugin settings persistence
- Tree structure serialization
- Registry interaction
- Environment variables
- System state detection

### 5. UI Framework
- Main window
- Tree view
- Configuration dialogs
- Log window
- System tray integration

#### Core Components:

##### Main Frame (MainFrame)
- Central UI container using wxPython
- Manages layout with wxAUI (Advanced User Interface)
- Handles window events and user interactions
- Implements menu and toolbar systems
- Manages dialog lifecycle
- Provides configuration persistence
- Supports window state management

##### Dialog System
- Base Dialog class with common functionality
- TaskletDialog for async operations
- ConfigDialog for plugin/action configuration
- MessageDialog for user notifications
- TransferDialog for progress indication
- Custom dialog controls and layouts
- Modal and non-modal support

##### Tree Control
- Hierarchical view of configuration
- Drag and drop support
- Context menu integration
- Item editing capabilities
- Selection management
- Visual feedback system
- Custom item rendering

##### Log Control
- Event logging display
- Column management
- Filtering capabilities
- Auto-scrolling
- Text formatting
- Timestamp handling

##### UI Components
- Custom controls and widgets
- Header boxes
- Animated windows
- Hyperlink controls
- Color pickers
- Button arrays
- Grid layouts

#### Key Features:

##### Layout Management
1. Window Layout
   - Dockable panels
   - Persistent layouts
   - Size management
   - Position tracking
   - Split views

2. Dialog Management
   - Dialog stacking
   - Modal handling
   - Window positioning
   - Focus management
   - Dialog persistence

3. Control Integration
   - Custom control creation
   - Event routing
   - State management
   - Visual feedback
   - Accessibility support

4. Theme Support
   - Custom drawing
   - Icon management
   - Color schemes
   - Font handling
   - Visual styles

##### User Interaction
1. Input Handling
   - Keyboard shortcuts
   - Mouse events
   - Context menus
   - Drag and drop
   - Focus tracking

2. Visual Feedback
   - Status messages
   - Progress indicators
   - Error displays
   - Tooltips
   - Highlighting

3. Configuration Interface
   - Property editors
   - Value validation
   - Live preview
   - Default handling
   - State persistence

4. Window Management
   - Minimize/Maximize
   - System tray
   - Multiple monitors
   - Window states
   - Focus handling

#### Migration Considerations:

##### Current wxPython Dependencies
1. Core Dependencies
   - wxPython for UI framework
   - wxAUI for docking
   - wx.TreeCtrl for tree view
   - wx.ListCtrl for logging
   - Custom wx controls

2. Platform Integration
   - Native widgets
   - System dialogs
   - Window management
   - Clipboard handling
   - Drag and drop

##### Rust UI Options
1. Potential Frameworks
   - Iced for native Rust UI
   - Druid for performance
   - egui for immediate mode
   - gtk-rs for GTK binding
   - Qt bindings via rust-qt

2. Framework Requirements
   - Tree view support
   - Docking capability
   - Rich text display
   - Custom controls
   - Native look and feel

3. Migration Challenges
   - Complex layouts
   - Custom controls
   - Event handling
   - Dialog system
   - Plugin UI integration

4. Implementation Strategy
   - Gradual component migration
   - Parallel UI systems
   - Compatibility layer
   - State synchronization
   - Plugin adaptation

##### Plugin Considerations
1. UI Integration
   - Plugin dialog system
   - Custom controls
   - Resource management
   - Event handling
   - State persistence

2. Compatibility Layer
   - wxPython wrapper
   - Control mapping
   - Event translation
   - Resource handling
   - State management

3. Migration Path
   - Plugin UI guidelines
   - Transition helpers
   - UI component library
   - Testing tools
   - Documentation

### 6. Windows Integration
- Named Pipe Communication
  - Inter-process messaging
  - Remote control interface
  - Plugin communication
  - Command processing
  - Security descriptor handling
  - Multiple pipe instance support
  - Message queuing system
- Windows API Wrappers
  - Dynamic API loading
  - Error handling
  - Resource management
  - Win32 security attributes
  - File handle management
  - Pipe state management
- System Hooks
  - Keyboard monitoring
  - Mouse tracking
  - Window events
  - Message filtering
  - Event propagation
- Registry Access
  - Configuration storage
  - System settings
  - Application detection
  - Permission handling
- Process Management
  - Application launching
  - Window manipulation
  - Process monitoring
  - Thread synchronization
  - Resource cleanup

## Core Features

### 1. Event Processing
- Event creation from various sources
- Event binding to actions
- Event filtering and conditions
- Event logging
- Event replay
- Event Binding System:
  - Dynamic binding registration
  - Binding priority handling
  - Conditional binding
  - Binding groups
  - Temporary bindings
- Event Sources:
  - Hardware input devices
  - System state changes
  - Plugin-generated events
  - Network events
  - Timer events
  - User-triggered events
- Event Processing Pipeline:
  - Source validation
  - Event normalization
  - Queue prioritization
  - Action triggering
  - Result handling

### 2. Action Management
- Action execution
- Action groups
- Action configuration
- Action compilation
- Action state management

### 3. Macro System
- Macro creation
- Macro execution
- Macro nesting
- Conditional execution
- Variables and scope

### 4. Plugin Interface
- Base plugin class
- Action definition
- Event generation
- Configuration interface
- Resource management

### 5. Tree Structure
- Item hierarchy
- Item types:
  - Folders
  - Macros
  - Actions
  - Events
  - Plugins
- Drag and drop
- Copy/paste
- Import/export

### 6. Logging System
- Debug logging
- Action logging
- Event logging
- Error handling
- Log filtering

### 7. Remote Management
- Network interface
- Remote execution
- API endpoints
- Security

## Plugin Categories

### 1. Program Control
- Application launching
- Window management
- Process control
- System commands

### 2. Hardware Control
- Input devices
- Output devices
- System hardware
- Custom hardware

### 3. Network
- TCP/IP communication
- HTTP requests
- Network protocols
- Remote control

### 4. Multimedia
- Media players
- Sound control
- Display control
- Media keys

### 5. System Functions
- Registry manipulation
- File operations
- System settings
- Power management

## Development Tools

### 1. Plugin Development
- Plugin templates
- Action templates
- Testing framework
- Documentation tools

### 2. Debugging
- Debug output
- Event monitor
- Action tester
- Plugin logger

### 3. Configuration
- Tree editor
- Action editor
- Event binding
- Plugin configuration

## Migration Considerations

### 1. Python 2 to 3
- String handling
- Threading model
- Exception handling
- Print statements
- Division operations
- Library compatibility

### 2. Rust Implementation
- FFI interface
- Memory management
- Thread safety
- Error handling
- Plugin system
- UI framework selection

#### Stackless Python Migration
1. Current Stackless Usage:
   - Core Threading Model:
     - Stackless tasklets for lightweight concurrency
     - Custom ThreadWorker bridging tasklets with Win32 messages
     - Event processing and UI interactions
   - Dialog Management:
     - Modal and non-modal dialog handling via tasklets
     - Asynchronous UI updates through channels
     - Dialog lifecycle management
   - Event Processing:
     - Event filtering and routing via tasklets
     - Event queuing through stackless channels
     - Cross-thread event synchronization

2. Migration Challenges:
   - Tasklet Replacement:
     - Implementing Rust async/await equivalents
     - Tokio runtime for async event loop
     - Custom task scheduling requirements
   - Channel Communication:
     - Replacing stackless channels with Rust channels
     - Cross-thread communication patterns
     - Event routing through async channels
   - Threading Model:
     - ThreadWorker reimplementation in Rust
     - Async tasks vs stackless tasklets
     - Thread safety considerations
   - Win32 Integration:
     - Windows-rs API bindings usage
     - Message pump reimplementation
     - COM initialization handling

3. Rust Advantages:
   - Threading Model:
     - Ownership system prevents data races
     - Modern async/await concurrency
     - No Global Interpreter Lock
   - Performance:
     - Native code execution
     - Zero-cost abstractions
     - Improved memory management
   - Safety:
     - Memory safety guarantees
     - Thread safety by design
     - Enhanced error handling
   - Modern Async:
     - Built-in async/await support
     - Rich async ecosystem
     - Superior async performance

4. Implementation Patterns:
   - Event System:
     ```rust
     // Event processing with async
     struct EventSystem {
         event_tx: mpsc::Sender<Event>,
         event_rx: mpsc::Receiver<Event>,
     }

     impl EventSystem {
         async fn process_events(&mut self) {
             while let Some(event) = self.event_rx.recv().await {
                 self.handle_event(event).await;
             }
         }
     }
     ```
   - UI Integration:
     ```rust
     // UI message handling
     use windows_rs::Win32::UI::WindowsAndMessaging::*;

     struct MessagePump {
         msg_tx: mpsc::Sender<Message>,
     }

     impl MessagePump {
         async fn run(&mut self) {
             let mut msg = MSG::default();
             while GetMessage(&mut msg, None, 0, 0).as_bool() {
                 TranslateMessage(&msg);
                 DispatchMessage(&msg);
             }
         }
     }
     ```
   - Task Management:
     ```rust
     // Instead of stackless tasklets
     async fn handle_event(event: Event) {
         // Event processing
     }

     // Instead of stackless channels
     use tokio::sync::mpsc;
     let (tx, rx) = mpsc::channel(32);
     ```

5. Migration Benefits:
   - Eliminates Stackless Python dependency
   - Provides modern async/await system
   - Improves performance and safety
   - Better Windows integration
   - Enhanced error handling
   - Simplified concurrency model
   - Reduced memory overhead
   - Improved debugging capabilities

6. Key Considerations:
   - Careful handling of existing plugin interfaces
   - Gradual migration of core components
   - Maintaining Windows API compatibility
   - Testing strategy for async behavior
   - Performance monitoring during transition
   - Documentation of new patterns
   - Training for Rust async concepts

### 3. Testing Requirements
- Unit tests
- Integration tests
- Plugin tests
- UI tests
- Migration validation

### 4. Documentation Needs
- Architecture documentation
- API documentation
- Plugin development guide
- User guide
- Migration guide


## Configuration Management System

### Core Components

1. **Config Class** (`Config.py`)
   - Central configuration store with default values
   - Handles persistent storage of application settings
   - Manages runtime configuration state
   - Supports section-based organization of settings

2. **PersistentData** (`PersistentData.py`)
   - Metaclass-based configuration persistence
   - Automatic configuration hierarchy building
   - Supports nested configuration sections

3. **Document Management** (`Document.py`)
   - XML-based configuration file handling
   - Undo/Redo support for configuration changes
   - Save/Load functionality with change tracking
   - Configuration tree state persistence

4. **Configuration UI** (`ConfigDialog.py`, `OptionsDialog.py`)
   - Dialog-based configuration interface
   - Settings categorization and organization
   - Real-time configuration updates
   - User preference management

### Storage and Persistence

1. **File Format**
   - Primary storage in Python-based config files
   - XML format for tree configuration
   - Base64 encoding for sensitive data
   - Hierarchical structure matching UI tree

2. **Configuration Paths**
   - User-specific configuration directory
   - Application-wide default settings
   - Plugin-specific configuration storage
   - Temporary configuration handling

3. **State Management**
   - Tree expansion state tracking
   - Window positions and sizes
   - User preferences persistence
   - Plugin state management

### Migration Considerations

1. **Current Implementation**
   - Python-based configuration files
   - Direct filesystem access
   - wxPython UI integration
   - In-memory configuration caching

2. **Rust Migration Path**
   - Consider using Serde for serialization
   - Implement configuration trait system
   - Maintain backward compatibility
   - Add migration tooling support

3. **Key Challenges**
   - Configuration format versioning
   - Plugin configuration compatibility
   - UI state persistence
   - Cross-platform paths handling

4. **Plugin Considerations**
   - Plugin-specific configuration storage
   - Configuration validation system
   - Default value handling
   - Configuration upgrade paths

### Security Considerations

1. **Sensitive Data**
   - Password storage encryption
   - Secure configuration paths
   - Permission management
   - Configuration backup

2. **Access Control**
   - User-specific settings
   - Plugin sandboxing
   - Configuration isolation
   - Validation and sanitization

## Logging System

### Core Components

1. **Log Class** (`Log.py`)
   - Central logging manager
   - Handles multiple log types (info, error, debug, warning)
   - Supports log listeners and event listeners
   - Manages stdout/stderr redirection
   - Debug level control
   - Stack trace formatting

2. **LogCtrl** (`MainFrame/LogCtrl.py`)
   - Virtual list control for log display
   - Circular buffer implementation
   - Configurable display options
   - Copy/paste support
   - Event replay functionality
   - Custom formatting

3. **Debug System**
   - Debug level configuration
   - Debug notice printing
   - Warning notice handling
   - Performance logging
   - System information logging

4. **UI Integration**
   - Log window with filtering
   - Color-coded messages
   - Timestamp formatting
   - Indentation support
   - Context menu actions
   - Drag and drop support

### Key Features

1. **Log Types**
   - Standard output logging
   - Error logging
   - Debug logging
   - Event logging
   - Warning messages
   - Stack traces
   - System information

2. **Display Features**
   - Configurable timestamps
   - Message indentation
   - Color coding by type
   - Icon indicators
   - Virtual scrolling
   - Auto-scrolling

3. **Interaction**
   - Copy to clipboard
   - Event replay
   - Message filtering
   - Clear log
   - Select all
   - Drag and drop

4. **Performance**
   - Circular buffer management
   - Virtual list implementation
   - Batch updates
   - Memory management
   - Buffer size control

### Migration Considerations

1. **Current Implementation**
   - Python logging integration
   - wxPython UI components
   - File-based logging
   - Memory-based buffering
   - Thread-safe logging

2. **Rust Migration Path**
   - Consider using `tracing` crate
   - Implement structured logging
   - Support async logging
   - Maintain performance
   - Enable log rotation

3. **Key Challenges**
   - Log format compatibility
   - Performance optimization
   - UI integration
   - Thread safety
   - Buffer management

4. **Plugin Considerations**
   - Plugin-specific logging
   - Log level control
   - Format standardization
   - Performance impact
   - Debug support

### Security Considerations

1. **Log Content**
   - Sensitive data filtering
   - Log file permissions
   - User data protection
   - Error message safety
   - Debug info control

2. **Access Control**
   - Log file access
   - Clear log permissions
   - Export restrictions
   - Remote logging security
   - Debug level control




## STACKLESS PYTHON MIGRATION
Rust's channels (mpsc, crossbeam) could replace stackless channels
Would need to handle cross-thread communication differently
Could use async channels for event routing
Threading Model:
Would need to reimplement the ThreadWorker using Rust's threading model
Could use async tasks instead of stackless tasklets

Would need careful consideration of thread safety
Win32 Integration:
Rust has good Win32 bindings through windows-rs
Message pump implementation would need to be rewritten
Would need to handle COM initialization differently
Advantages of moving to Rust:
Better Threading Model:
Rust's ownership system prevents data races
Async/await provides modern concurrency
No GIL limitations
Performance:
Native code execution
Zero-cost abstractions
Better memory management
Safety:
Memory safety guarantees
Thread safety by design
Better error handling
Modern Async:
Built-in async/await
Rich ecosystem of async tools
Better performance than Python's async

Core Design Principles
-------------------

EventGhost's architecture is built on several key principles that should be maintained in the Rust implementation:

1. **Event-Driven Simplicity**
   - Direct event handling without unnecessary complexity
   - Synchronous processing where possible
   - Clear and predictable event flow

2. **Plugin Architecture**
   - Simple plugin lifecycle (start/stop)
   - Direct event handling methods
   - Minimal interface requirements
   - Easy plugin development

3. **Resource Management**
   - Direct resource acquisition and release
   - Clear ownership and cleanup patterns
   - Predictable resource lifecycles

Async Considerations
------------------

While Rust provides powerful async capabilities, they should be used judiciously:

1. **When to Use Async**:
   - Network I/O operations
   - Long-running file operations
   - External service communication
   
2. **When to Avoid Async**:
   - Simple event handling
   - Direct system calls
   - UI interactions
   - Plugin lifecycle methods

3. **Implementation Guidelines**:
   - Keep async at the edges (e.g., network boundaries)
   - Use synchronous core for event processing
   - Maintain simple plugin interfaces
   - Avoid unnecessary complexity

Example Plugin Interface:
```rust
pub trait Plugin {
    // Core lifecycle methods
    fn start(&mut self) -> Result<(), Error>;
    fn stop(&mut self) -> Result<(), Error>;
    
    // Direct event handling
    fn handle_event(&mut self, event: &Event) -> Result<(), Error>;
    
    // Optional async operations where necessary
    #[cfg(feature = "async")]
    async fn handle_network_io(&mut self) -> Result<(), Error>;
}
```