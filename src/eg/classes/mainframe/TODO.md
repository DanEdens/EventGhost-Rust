# MainFrame TODO List

## Current Progress
- [x] Basic GTK4 window structure
- [x] Header bar implementation
- [x] Toolbar basic setup with icons and tooltips
- [x] Menu bar implementation with full menu structure
- [x] Status bar implementation
- [x] Basic document handling
- [x] Keyboard shortcut hints in tooltips
- [x] Implement event logging view (based on original's log window)
- [x] Add log view menu controls (time, date, indent)

## Immediate Tasks
- [ ] Add tree view for configuration items
- [ ] Implement full action handlers for toolbar buttons
- [ ] Add keyboard shortcut bindings
- [ ] Implement plugin panel (right side)
- [ ] Add configuration panel

## UI Improvements
- [x] Match original EventGhost's layout for log panel
- [ ] Match original's layout for other panels
- [ ] Implement collapsible panels
- [x] Add splitter controls between panels
- [ ] Add icon support for tree items
- [ ] Match original's toolbar button ordering and grouping

## Log View Improvements
- [ ] Add search functionality in log
- [ ] Implement log filtering options
- [ ] Add log export functionality
- [ ] Improve log entry formatting
- [ ] Add log level filtering
- [ ] Implement context menu functionality for log entries
- [ ] Implement clipboard functionality for copying selected text
- [ ] Implement log saving and loading from a file
- [ ] Implement dynamic log entry formatting based on log level
- [ ] Add event handling for context menu items (e.g., "Copy", "Select All", "Clear Log")
- [ ] Implement detailed log entry parsing when loading from a file

## StatusBar Feature Enhancements
- [ ] Implement event handling for checkbox state changes in Rust version.
- [ ] Manage status bar context ID similarly to the Python version.
- [ ] Add icon handling to the Rust status bar to reflect processing state.
- [ ] Implement repositioning logic for the Rust status bar based on size events.
- [ ] Add logging/debugging functionality to the Rust version similar to the Python version.

## TreeCtrl Improvements
- [ ] Implement drag and drop functionality similar to Python version.
- [ ] Add event handlers for tree actions (e.g., OnBeginDragEvent, OnEndLabelEditEvent).
- [ ] Implement `EditControlProxy` functionality for label editing in Rust.
- [ ] Integrate `UndoHandler` for node operations in Rust version.
- [ ] Enhance selection change handling and notifications.
- [ ] Add context menu handling similar to Python version.
- [ ] Implement comprehensive tests for `TreeCtrl` functionalities.

## Next Steps (Priority Order)
1. Complete event logging view implementation
2. Add tree view for configuration
3. Implement plugin panel
4. Add keyboard shortcut support
5. Implement configuration dialogs
6. Add event monitoring system

