# EventGhost Architecture Overview

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

### Windows Integration
- eg/NamedPipe.py - IPC via Windows named pipes
  - Implements command processing via named pipes
  - Handles multi-instance pipe connections
  - Provides secure IPC between admin/user contexts
  - Supports async command execution
  - Uses daemon threads for pipe management 

### EventThread.py
- Manages event filtering and execution
- Handles memory quotas and limitations
- Controls event session lifecycle
- Provides synchronous/async event triggers
- Implements event filtering system 
