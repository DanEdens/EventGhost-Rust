# EventGhost-Rust

A modern, fast, and extensible automation tool written in Rust. This is a reimplementation of the original [EventGhost](http://www.eventghost.net/) project.

## Project Status

We are currently in the early development phase, focusing on building a robust plugin system. Here's our current progress:

### Completed
- ✅ Project foundation and architecture
- ✅ Core plugin system design
- ✅ Documentation framework
- ✅ Basic test infrastructure

### In Progress
We are implementing the plugin system in phases:

1. **Current Phase: Plugin Loading & Metadata**
   - Plugin manifest format
   - Dynamic library loading
   - Plugin registry system
   - Metadata management

2. **Upcoming: Plugin Discovery & Hot-Reloading**
   - File system monitoring
   - Hot-reload support
   - Version management
   - Plugin validation

3. **Future: Dependencies & Communication**
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

### Building
```bash
# Clone the repository
git clone https://github.com/DanEdens/EventGhost-Rust
cd EventGhost-Rust

# Build the project
cargo build

# Run tests
cargo test
```

## Project Goals

1. **Modern Architecture**
   - Async-first design
   - Strong type safety
   - Robust error handling
   - Comprehensive testing

2. **Enhanced Plugin System**
   - Hot-reloading support
   - Dependency management
   - Resource isolation
   - Version control

3. **Improved Performance**
   - Fast event processing
   - Low resource usage
   - Quick startup time
   - Efficient plugin loading

4. **Developer Experience**
   - Clear documentation
   - Type-safe plugin API
   - Development tools
   - Testing utilities

## Contributing

We welcome contributions! While we're in early development, here's how you can help:

1. **Code Contributions**
   - Check our [Implementation Plan](docs/architecture/IMPLEMENTATION_PLAN.md)
   - Review open issues
   - Submit pull requests
   - Discuss design


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

- [Architecture Overview](docs/architecture/OVERVIEW.md)
- [Implementation Plan](docs/architecture/IMPLEMENTATION_PLAN.md)
- [Plugin Development](docs/guides/developer/README.md) (OLD)


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Original EventGhost team for their pioneering work
- Rust community for excellent tools and libraries


## Roadmap

See our [Implementation Plan](docs/architecture/IMPLEMENTATION_PLAN.md) for detailed development phases and milestones.

## Contact

- GitHub Issues: For bug reports and feature requests
- Discussions: For questions and community interaction
