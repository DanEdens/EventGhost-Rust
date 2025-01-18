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
- [ ] Test window message handling
- [ ] Test control creation and layout
- [ ] Test window cleanup on drop
