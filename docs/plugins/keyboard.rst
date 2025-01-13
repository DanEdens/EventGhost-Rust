Keyboard Plugin
==============

Overview
--------
A remote plugin that generates and manages keyboard events (hotkeys). The plugin provides key blocking functionality when events are assigned to macros, preventing Windows or other applications from receiving the blocked keystrokes. This blocking only occurs when a macro would actually execute - if the macro or its parents are disabled, the keypress passes through normally.

Core Components
-------------
1. Event Generation System
   - Keyboard event capture via Windows hooks
   - Key blocking for macro-assigned keys
   - Universal modifier support (optional)
   - Event enduring state management

2. Configuration System
   - Universal modifiers toggle
   - Event blocking configuration
   - Callback management
   - Plugin state handling

3. Callback System
   - SetKeyboardCallback integration
   - Event triggering logic
   - Key code processing
   - Active handler checking

Key Features
-----------
1. Event Management
   - Hotkey event generation
   - Enduring event support
   - Event termination control
   - Active handler detection

2. Key Blocking
   - Selective key blocking
   - Macro-aware blocking
   - Parent state awareness
   - Pass-through handling

3. Universal Modifiers
   - Optional modifier support
   - Configuration persistence
   - Runtime configuration
   - State management

Migration Considerations
---------------------
1. Core Functionality Migration
   - Replace SetKeyboardCallback with Rust keyboard hook
   - Implement safe key code handling
   - Port event generation system
   - Maintain blocking behavior

2. Event System
   - Design Rust event types
   - Implement enduring events
   - Handle event termination
   - Manage event state

3. Configuration
   - Create type-safe settings
   - Implement configuration UI
   - Handle persistence
   - Manage defaults

Implementation Strategy
--------------------
1. Keyboard Hook System
   .. code-block:: rust

   pub struct KeyboardHook {
       callback: Box<dyn Fn(KeyEvent) -> bool>,
       universal_mods: bool,
       hook_handle: HHOOK,
   }

   impl KeyboardHook {
       pub fn new(universal_mods: bool) -> Result<Self, Error> {
           // Initialize keyboard hook
           // Set up callback system
           // Configure modifiers
           // Handle errors
       }

       fn process_key_event(&self, event: KeyEvent) -> bool {
           // Process key codes
           // Check active handlers
           // Handle blocking
           // Trigger events
       }
   }

2. Event Generation
   .. code-block:: rust

   pub struct KeyboardPlugin {
       hook: Option<KeyboardHook>,
       state: Arc<Mutex<PluginState>>,
       event_tx: mpsc::Sender<KeyboardEvent>,
   }

   impl KeyboardPlugin {
       fn trigger_enduring_event(&self, codes: &str) -> Result<(), Error> {
           // Validate codes
           // Create event
           // Send to handler
           // Track state
       }

       fn end_last_event(&self) -> Result<(), Error> {
           // Clean up state
           // Notify handlers
           // Update tracking
       }
   }

Testing Strategy
-------------
1. Unit Tests
   - Key code processing
   - Event generation
   - Blocking logic
   - Configuration handling

2. Integration Tests
   - Hook system
   - Event flow
   - Blocking behavior
   - Configuration persistence

3. Performance Tests
   - Hook latency
   - Event processing speed
   - Memory usage
   - Resource cleanup

Error Handling
------------
1. Hook Management
   - Safe hook installation
   - Proper cleanup
   - Error recovery
   - State consistency

2. Event Processing
   - Invalid key codes
   - Hook failures
   - Event errors
   - State corruption

Platform Considerations
--------------------
1. Windows Integration
   - Safe Windows hook API usage
   - Key code mapping
   - Event synchronization
   - Resource management

2. Cross-Platform Support
   - Abstract hook system
   - Platform-specific implementations
   - Common event interface
   - Unified configuration