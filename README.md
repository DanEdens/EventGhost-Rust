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