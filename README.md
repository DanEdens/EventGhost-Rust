# EventGhost-Rust

A modern, fast, and extensible automation tool written in Rust. This is a reimplementation of the original [EventGhost](http://www.eventghost.net/) project.

## Project Status

We are currently in the early development phase, focusing on building a robust plugin system. Here's our current progress:

### Completed
- ✅ Project foundation and architecture
- ✅ Core plugin system design
- ✅ Documentation framework
- ✅ Basic test infrastructure
- ✅ Initial Win32 API integration

### Project Structure
```
src/
├── core/      # Core functionality and shared components
├── win32/     # Windows-specific implementations
├── testing/   # Test utilities and helpers
├── eg/        # EventGhost-specific functionality
├── lib.rs     # Library root
└── main.rs    # Application entry point
```

### In Progress
We are implementing the plugin system in phases:

1. **Current Phase: Win32 Integration & Plugin Foundation**
   - Win32 API bindings
   - Core event system
   - Plugin manifest format
   - Dynamic library loading

2. **Upcoming: Plugin System Core**
   - Plugin registry system
   - Metadata management
   - File system monitoring
   - Hot-reload support

3. **Future: Plugin Features**
   - Dependency resolution
   - Inter-plugin messaging
   - Resource management
   - Plugin isolation

4. **Final Phase: Configuration & UI**
   - Configuration system
   - Plugin UI integration
   - Management interface
   - Settings persistence

## Getting Started

### Prerequisites
- Rust 1.75+ (we use the latest stable features)
- Cargo
- Windows 10/11 
- Visual Studio Build Tools (for Win32 development)

### Building
```bash
# Clone the repository
pacman -S mingw-w64-x86_64-pkg-config mingw-w64-x86_64-gtk3

git clone https://github.com/DanEdens/EventGhost-Rust
cd EventGhost-Rust

# Build the project
cargo build

# Run tests
cargo test
```
   

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details. Here's how you can help:

1. **Code Contributions**
   - Review open issues
   - Submit pull requests
   - Discuss design in Issues

2. **Documentation**
   - Improve existing docs
   - Add examples
   - Write tutorials
   - Report unclear sections

3. **Testing**
   - Report bugs
   - Suggest features
   - Test on different platforms
   - Performance testing

## Documentation

Our documentation is organized into several sections:

- [Documentation Overview](docs/README.md)
- Architecture Documentation:
  - [Core Architecture](docs/architecture/ARCHITECTURE.md)
  - [Plugin Architecture](docs/architecture/PLUGIN_ARCHITECTURE.md)
  - [Inital supported plugins](docs/plugins/README.md)
  - [GUI Architecture](docs/architecture/GUI_ARCHITECTURE.md)
  - [Lessons Learned](docs/architecture/LESSONS_LEARNED.md)
- API Documentation:

  - [Core API](docs/api/core/README.md)
  - [Plugin API](docs/api/plugins/README.md)
  - [GUI API](docs/api/gui/README.md)
- Guides:
  - [User Guide](docs/guides/user/README.md)
  - [Developer Guide](docs/guides/developer/README.md)
  - [Migration Guide](docs/guides/migration/README.md)

## Project Structure
```
.
├── src/               # Source code
│   ├── core/         # Core functionality
│   ├── win32/        # Windows API integration
│   ├── testing/      # Test utilities
│   ├── eg/          # EventGhost-specific code
│   ├── lib.rs       # Library root
│   └── main.rs      # Application entry
├── docs/             # Documentation
│   ├── api/         # API documentation
│   ├── guides/      # User and developer guides
│   ├── architecture/# Design documents
│   └── examples/    # Code examples
└── target/           # Build outputs
```

## License

This project is licensed under the GNU General Public License v2.0 - see the [gpl-2.0.md](gpl-2.0.md) file for details.

## Acknowledgments

- Original EventGhost team for their pioneering work
- Rust community for excellent tools and libraries


## Roadmap

See our [Implementation Plan](docs/architecture/IMPLEMENTATION_PLAN.md) for detailed development phases and milestones.

## Contact

- GitHub Issues: For bug reports and feature requests
- Discussions: For questions and community interaction
