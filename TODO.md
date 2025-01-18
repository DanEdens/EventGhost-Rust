# EventGhost-Rust TODO List

## Core Implementation Progress

### Completed
- [x] Project setup and initial structure
- [x] Basic documentation framework
- [x] System architecture and wireframe
  - [x] Core plugin traits and interfaces
  - [x] Plugin discovery system skeleton
  - [x] Dependency management design
  - [x] Plugin lifecycle management

### In Progress - Phase 1: Plugin Loading & Metadata
- [ ] Plugin File Format
  - [ ] Define plugin manifest format
  - [ ] Create plugin file structure
  - [ ] Implement metadata extraction
  - [ ] Add version parsing
- [ ] Dynamic Loading
  - [ ] Safe library loading
  - [ ] Symbol resolution
  - [ ] Plugin initialization
  - [ ] Error handling
- [ ] Plugin Registry Enhancement
  - [ ] Metadata caching
  - [ ] State tracking
  - [ ] Health monitoring
  - [ ] Registry persistence

### Pending - Phase 2: Discovery & Hot-Reloading
- [ ] File System Integration
  - [ ] Directory watching
  - [ ] Change detection
  - [ ] File validation
  - [ ] Recursive scanning
- [ ] Hot-Reloading
  - [ ] Safe unloading
  - [ ] State transfer
  - [ ] Reload coordination
  - [ ] Rollback mechanism
- [ ] Version Management
  - [ ] Semantic version parsing
  - [ ] Requirement checking
  - [ ] Conflict resolution
  - [ ] Upgrade handling

### Future - Phase 3: Dependencies & Communication
- [ ] Dependency Resolution
  - [ ] Graph building
  - [ ] Circular detection
  - [ ] Optional dependencies
  - [ ] Version constraints
- [ ] Inter-Plugin Communication
  - [ ] Message system
  - [ ] Event routing
  - [ ] Capability negotiation
  - [ ] Resource management
- [ ] Plugin Isolation
  - [ ] Sandboxing
  - [ ] Resource limits
  - [ ] Error isolation
  - [ ] Cleanup mechanisms

### Future - Phase 4: Configuration & UI
- [ ] Configuration System
  - [ ] Schema system
  - [ ] Validation
  - [ ] Persistence
  - [ ] Migration tools
- [ ] Plugin UI Integration
  - [ ] Framework integration
  - [ ] Settings panels
  - [ ] Dynamic updates
  - [ ] State persistence
- [ ] Plugin Management UI
  - [ ] Plugin browser
  - [ ] Installation UI
  - [ ] Update management
  - [ ] Dependency viewer

## Testing Infrastructure
- [x] Test utilities and helpers
- [x] Mock implementations
- [x] Integration test framework
- [ ] Plugin test framework
- [ ] Unit test coverage
- [ ] Performance benchmarks
- [ ] Platform-specific tests

## Documentation Tasks
- [x] Documentation structure
- [x] API documentation skeleton
- [ ] Plugin development guide
- [ ] User guide
- [ ] Architecture documentation
- [ ] Migration guide

## Next Steps
1. Begin with plugin manifest format design
2. Implement basic metadata extraction
3. Create initial loading mechanism
4. Add basic version validation

## Current Error Resolution Plan

## Completed
- [X] Add proper PIPE_ACCESS_DUPLEX and related constants in named_pipe.rs
- [X] Fix Windows API type conversions for FILE_FLAGS_AND_ATTRIBUTES and NAMED_PIPE_MODE
- [X] Fix RwLock usage in PluginRegistry
- [X] Add missing plugin_dir field to PluginRegistry
- [X] Fix Event trait bounds mismatch in macro_.rs (get_trigger_event return type)

## Immediate Code Fixes (Miswritten Code)
- [ ] Fix ConfigDialog imports in action modules:
  - src/eg/action/base.rs
  - src/eg/action/group.rs
  - src/eg/action/item.rs
  - src/eg/action/common.rs
- [ ] Implement From<RegistryError> for core::error::Error
- [ ] Implement From<LoaderError> for core::error::Error
- [ ] Fix Bunch import in globals.rs

## Next Implementation Priority
- [ ] Implement ConfigDialog in plugin_config module
- [ ] Add proper error conversion traits between different error types
- [ ] Complete plugin loading system implementation

## Planned for Future Phases
### Phase 1: Plugin System
- [ ] Complete plugin loading from directory
- [ ] Implement plugin lifecycle management
- [ ] Add plugin configuration handling

### Phase 2: Event System
- [ ] Complete event handling system
- [ ] Add event routing and dispatch
- [ ] Implement event filtering

### Phase 3: Action System
- [ ] Complete action execution framework
- [ ] Add action configuration support
- [ ] Implement action chaining

### Phase 4: Configuration System
- [ ] Implement ConfigDialog
- [ ] Add configuration persistence
- [ ] Support plugin-specific configurations

## Code Cleanup
- [ ] Fix unused variable warnings in plugin registry
- [ ] Fix unused import warnings
- [ ] Update naming conventions for constants in types.rs
