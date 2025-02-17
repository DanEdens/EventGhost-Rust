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
  - [x] Plugin traits and registry
  - [x] Plugin discovery and loading infrastructure
  - [x] Dynamic library loading implementation
  - [x] Plugin hot-reloading
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
- [x] Example logger plugin
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
1. Set up event routing system
2. Create common plugins
3. Add import/export functionality
4. Set up deployment pipeline


