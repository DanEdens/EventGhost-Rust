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


