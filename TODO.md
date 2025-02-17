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

## Core
- [ ] Plugin System
  - [ ] Plugin traits and registry
  - [ ] Plugin discovery and loading
  - [ ] Route events to/from plugins
- [ ] Event Handling  
  - [ ] Event bus / message passing
  - [ ] Event dispatch and routing
- [ ] Macro Execution
  - [ ] Macro execution engine
  - [ ] Map events to macros
  - [ ] Flow control (if/else, loops, waits)
- [ ] Action Execution  
  - [ ] Execute action sequences
  - [ ] Builtin actions
  - [ ] Plugin-defined actions
- [x] Persistence
  - [x] Save/load configuration
  - [x] Auto-save
  - [ ] Import/export  

## Plugins
- [ ] Python plugin loader
- [ ] Common plugins (to be defined)

## Deployment
- [ ] Windows installer
- [ ] Linux packages
- [ ] Mac bundle

## Documentation
- [x] User guide
- [ ] API docs for plugins
- [x] Contributor guide

## Next Steps
1. Complete plugin system implementation
2. Implement event handling and macro execution
3. Add import/export functionality for configurations
4. Create common plugins
5. Set up deployment pipeline


