# EventGhost-Rust

A Rust implementation of EventGhost, a powerful automation tool for Windows. This project aims to modernize EventGhost while maintaining compatibility with existing plugins and configurations.

## Features

### Core Features
- [x] Modern GTK4-based user interface
- [x] Configuration persistence with JSON serialization
- [x] Drag-and-drop support for configuration items
- [x] Plugin system architecture
- [x] Event handling system
- [x] Logging system with filtering and search

### UI Components
- [x] Main window with menu bar and toolbar
- [x] Log window with real-time updates
- [x] Configuration tree view
- [x] Configuration dialogs for items:
  - [x] Plugins
  - [x] Folders
  - [x] Macros
  - [x] Events
  - [x] Actions

### Configuration Management
- [x] JSON-based configuration storage
- [x] Auto-save functionality
- [x] Hierarchical configuration structure
- [x] CRUD operations for all item types
- [x] Drag-and-drop reordering

## Getting Started

### Prerequisites
- Rust 1.70 or later
- GTK4 development libraries
- Windows 10 or later

### Building from Source
1. Install Rust using [rustup](https://rustup.rs/)
2. Install GTK4 development libraries:
   ```bash
   # Windows (using MSYS2)
   pacman -S mingw-w64-x86_64-gtk4

   # Linux
   sudo apt install libgtk-4-dev
   ```
3. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/EventGhost-Rust.git
   cd EventGhost-Rust
   ```
4. Build the project:
   ```bash
   # Build the binary application
   cargo build --bin test-gui --release
   
   # Running tests requires the testing feature flag
   cargo test --features testing
   ```

### Build Options
The project uses feature flags to control the build:

- `testing`: Enables testing utilities and mocks for unit tests
- `production`: Used for production builds
- `plugin_hot_reload`: Enables hot reloading of plugins during development

### Known Issues
- When building with `cdylib` crate type, you may encounter an "export ordinal too large" error due to the size of the project. To work around this, the main build uses only `rlib` crate type. The `cdylib` type will only be enabled for releases in the future.

### Running
```bash
cargo run --bin test-gui --release
```

## Development

### Project Structure
```
src/
├── core/           # Core functionality
├── eg/            # EventGhost-specific code
│   ├── classes/   # UI components
│   ├── config/    # Configuration handling
│   └── plugins/   # Plugin system
├── bin/          # Binary entry points
│   └── test-gui.rs # Main application entry point
└── resources/     # Application resources
```

### Coding Standards
- Follow Rust standard naming conventions
- Use meaningful commit messages following conventional commits
- Write tests for new functionality
- Document public APIs
- Keep code modular and maintainable

### Testing
Run the test suite:
```bash
cargo test --features testing
```

## Contributing
1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to your branch
5. Create a Pull Request

## License
This project is licensed under the GPL-2.0 License - see the LICENSE file for details.

## Acknowledgments
- Original EventGhost project and contributors
- GTK4 team for the excellent UI toolkit
- Rust community for the amazing ecosystem

## Current Status
This project is under active development. Current focus areas:
- Implementing remaining core functionality
- Improving UI/UX
- Adding plugin compatibility layer
- Enhancing configuration management

## Roadmap
See [TODO.md](TODO.md) for planned features and enhancements.

## Features

- **Modern GTK4-based UI**
- **Plugin System**: Load and unload plugins dynamically
- **Event System**: Create and manage events
- **Macro System**: Trigger actions based on events
- **Action System**: Configure and execute actions in response to events
- **Configuration UI**: Create and manage your configuration
- **Action Configuration UI**: Configure actions with dynamic parameters

## Current Status

EventGhost-Rust is currently in active development. The core architecture is in place, and we're actively implementing features to reach feature parity with the original EventGhost.

## GTK4 Compatibility

We've updated several files to ensure compatibility with GTK4:

1. Updated import paths in `src/eg/classes/tree_ctrl.rs` to use GTK4 components
2. Fixed ModifierType imports in `src/eg/classes/config_view.rs` to use `gdk4::ModifierType`
3. Updated Rectangle imports in `src/eg/classes/log_ctrl.rs` to use `gdk4::Rectangle`
4. Fixed RGBA imports in `src/eg/classes/dialog.rs` to use `gdk4::RGBA`
5. Implemented MockPlugin and MockEvent directly in core modules for testing

These changes ensure that the application works correctly with GTK4 and avoids deprecation warnings.

### Known Issues

There is an access violation occurring in some tests when run together, but individual tests run fine. This will require further investigation.

## Comparison with Python EventGhost

The original EventGhost is written in Python and has a rich feature set developed over many years. This Rust implementation aims to:

1. **Preserve core functionality**: Event handling, plugin system, and macro execution
2. **Modernize the codebase**: Use Rust's strong type system and modern UI technologies
3. **Improve performance**: Take advantage of Rust's efficiency and safety

### Current Gaps

While we're making steady progress, there are still substantial gaps compared to the Python version:

1. **Plugin Ecosystem**: The original has 100+ plugins for various devices and applications
2. **Python Compatibility**: Legacy plugins require Python interoperability
3. **UI Features**: Some advanced UI features are still being implemented
4. **Hardware Support**: Many specialized device drivers need to be ported

### Future Plans

Our roadmap includes:

1. Complete the GTK4 migration for a modern UI experience
2. Develop a Python compatibility layer for legacy plugins
3. Implement the most commonly used plugins natively in Rust
4. Add comprehensive testing and error handling
5. Create better documentation for both users and developers

See [TODO.md](TODO.md) for detailed implementation plans.

## Lessons Learned During Development

Throughout this project, we've learned:

1. **GTK4 Transition**: Moving from older GTK versions requires careful management of imports and APIs
2. **Rust for UI**: Rust provides excellent safety guarantees but requires different patterns than traditional UI frameworks
3. **Testing Complexity**: UI-heavy applications need specialized testing approaches
4. **Balancing New vs Legacy**: Finding the right balance between modern design and compatibility is challenging

We continue to document our learning in each major feature implementation.