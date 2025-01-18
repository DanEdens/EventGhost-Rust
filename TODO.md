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

### Immediate Code Fixes (Miswritten Code)
- [X] Fix RwLock usage in PluginRegistry
  - [X] Replace `HashMap::new()` with proper `Arc<RwLock<HashMap>>` initialization
  - [X] Fix `clear()` method usage on Arc<RwLock>
- [ ] Fix Event trait bounds consistency
  - [ ] Align `Event` trait bounds between `macro_.rs` and `event.rs`
  - [ ] Fix type mismatch in `get_trigger_event()` return type
- [X] Fix struct field errors
  - [X] Add missing `plugin_dir` field to `PluginRegistry`
  - [X] Update constructor to properly initialize fields

### Next Implementation Priority
- [ ] Fix Windows API imports
  - [ ] Add proper PIPE_ACCESS_DUPLEX and related constants in named_pipe.rs
  - [ ] Update imports to use correct paths from windows-rs
- [ ] Fix ConfigDialog imports
  - [ ] Export ConfigDialog trait from plugin_config module
  - [ ] Update imports in action modules (base.rs, group.rs, item.rs, common.rs)
- [ ] Implement error conversions
  - [ ] Add From<RegistryError> for core::Error
  - [ ] Add From<LoaderError> for core::Error

### Planned for Future Phases
- [ ] Plugin Loading System (Phase 1)
  - [X] Implement `load_all()` in PluginRegistry (commented out)
  - [X] Implement `unload_all()` with proper cleanup (commented out)
  - [ ] Add proper error conversion between RegistryError and core::Error
- [ ] Plugin Configuration (Phase 4)
  - [X] Complete ConfigDialog implementation (commented out)
  - [ ] Add proper property validation
  - [ ] Implement configuration persistence

### Code Cleanup
- [ ] Fix unused import warnings
- [ ] Fix unused variable warnings
- [ ] Fix non-camel-case enum variants
