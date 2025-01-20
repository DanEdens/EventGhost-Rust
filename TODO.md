# TODO List

## Current Focus: Fix Build Errors

### Import Fixes
- [ ] Fix CREATESTRUCTA import (move from Foundation to WindowsAndMessaging)
- [ ] Fix win32 module import path in winapi/mod.rs
- [ ] Add missing Windows API type imports (WNDCLASSA, BOOL)
- [ ] Add missing Windows API function imports (RegisterClassA, UpdateWindow, etc.)

### Trait Implementation Fixes
- [ ] Implement UIComponent trait properly for:
  - [ ] Toolbar
  - [ ] StatusBar
  - [ ] TreeCtrl
  - [ ] LogCtrl

### Error Handling Fixes
- [ ] **Fix casting error in `src/win32/utils.rs` (E0606)**
- [ ] Fix LRESULT error handling (map_err issue)
- [ ] Fix i32 error handling in Windows API calls
- [ ] Fix type casting issue in window enumeration callback

### Previous Tasks (In Progress)
- [x] Update control constructors to handle HINSTANCE properly:
  - [x] TreeCtrl
  - [x] LogCtrl
  - [x] StatusBar
  - [x] Toolbar
- [x] Implement control layout in MainFrame
- [x] Add window resizing handling

### Testing
- [ ] Add basic window creation tests
  - [ ] Implement test for creating a window
  - [ ] Implement test for window properties
- [ ] Test window message handling
  - [ ] Implement test for message processing
  - [ ] Implement test for message routing
- [ ] Test control creation and layout
  - [ ] Implement test for control initialization
  - [ ] Implement test for control layout
- [ ] Test window cleanup on drop
  - [ ] Implement test for cleanup process
  - [ ] Implement test for resource deallocation

### Code Cleanup
- [ ] Implement items seen as unused imports in the following files:
  - [ ] `src/core/config.rs`
  - [ ] `src/core/logging.rs`
  - [ ] `src/core/named_pipe.rs`
  - [ ] `src/eg/action/common.rs`
  - [ ] `src/win32/window.rs`
  - [ ] `src/win32/utils.rs`

## Completed
- Implemented basic GTK UI components
- Added dialog implementations with proper async handling
- Fixed action handlers and error handling
- Improved file path handling in dialogs
- Added Debug trait implementations
- Fixed ConditionalAction closure type to be cloneable

## In Progress
- Clean up unused imports and variables
- Implement proper file type handling in dialogs
- Complete event type system implementation
- Implement plugin loading functionality

## Implementation Plan
1. Code Cleanup
   - Remove unused imports across all files
   - Fix unused variable warnings by implementing or removing them
   - Add proper type annotations where missing

2. Dialog System
   - Complete file chooser dialog implementation
   - Add proper error handling for file operations
   - Implement dialog result handling

3. Event System
   - Complete event type implementation
   - Add proper event payload handling
   - Implement event routing system

4. Plugin System
   - Implement plugin loading mechanism
   - Add plugin state management
   - Implement plugin configuration handling

## Next Steps
1. Clean up warnings by removing unused imports
2. Implement the missing functionality in plugin registry
3. Complete the event system implementation
4. Add comprehensive tests for all components


