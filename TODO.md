# TODO List

## Current Focus: Basic GUI Implementation

### Error Handling
- [x] Implement `From<core::error::Error>` for `win32::Error`
- [x] Add `Win32` variant to `core::Error`
- [x] Consolidate error types between core and win32 modules

### Window API Fixes
- [x] Fix imports for Windows API types (WNDCLASSA, RegisterClassA)
- [x] Implement proper string handling for window text (PCSTR conversion)
- [x] Fix result handling for Windows API calls
- [x] Remove duplicate window management code between eg/winapi and win32 modules

### Control Implementation
- [ ] Update control constructors to handle HINSTANCE properly:
  - [ ] TreeCtrl
  - [ ] LogCtrl
  - [ ] StatusBar
  - [ ] Toolbar
- [ ] Implement control layout in MainFrame
- [ ] Add window resizing handling

### Code Cleanup
- [ ] Remove unused imports
- [ ] Fix unused variable warnings in plugin registry
- [ ] Fix unused variable warnings in logging code
- [ ] Clean up and document window creation code

### Testing
- [ ] Add basic window creation tests
- [ ] Test window message handling
- [ ] Test control creation and layout
- [ ] Test window cleanup on drop
