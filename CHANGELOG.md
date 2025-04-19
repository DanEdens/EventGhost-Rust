# Changelog
All notable changes to EventGhost-Rust will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Modern GTK4-based user interface
  - Main window with menu bar and toolbar
  - Log window with real-time updates
  - Configuration tree view with drag-and-drop support
  - Configuration dialogs for all item types
  - Context menus for tree view operations
- Configuration system
  - JSON-based configuration storage
  - Auto-save functionality
  - Hierarchical configuration structure
  - CRUD operations for configuration items
  - Drag-and-drop reordering support
- Logging system
  - Real-time log updates
  - Log filtering by type
  - Log search functionality
  - Log entry formatting
  - Timestamp and source tracking
- System Actions
  - Run Command action for executing system commands
  - File Operations action for file management (copy, move, delete, create)
  - Window Actions for window manipulation (minimize, maximize, activate, etc.)
  - Support for background command execution

### Changed
- Migrated to GTK4 from GTK3
  - Updated all widget implementations
  - Improved drag-and-drop system
  - Modern styling system
  - Better widget hierarchy

### Fixed
- GTK CSS node hierarchy issues
- Drag-and-drop signal handling
- Configuration persistence edge cases
- Tree view selection handling
- Menu item activation issues

### Development Changes
- Added comprehensive test suite
- Improved documentation
  - Added detailed README
  - Created TODO tracking
  - Added code documentation
- Implemented proper error handling
- Added logging and debugging support

## [0.1.0] - 2024-02-14

### Added
- Initial project setup
- Basic window management
- Core event system
- Plugin system architecture
- Configuration data model
- Logging framework

[Unreleased]: https://github.com/yourusername/EventGhost-Rust/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/EventGhost-Rust/releases/tag/v0.1.0
