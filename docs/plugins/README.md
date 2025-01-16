# EventGhost-Rust Plugins Documentation

This directory contains detailed documentation for all plugins in the EventGhost-Rust project. Each plugin is documented in RST format, providing comprehensive information about its functionality, implementation details, and migration status from Python to Rust.

## Documentation Structure

The plugins are organized into several categories:

### Complete Analysis with Rust Implementation
- [`global_monitor.rst`](global_monitor.rst) - Global system monitoring capabilities
- [`mouse.rst`](mouse.rst) - Mouse event handling and control

### Core Components and Migration Analysis
- [`eventghost_core.rst`](eventghost_core.rst) - Core EventGhost functionality
- [`keyboard.rst`](keyboard.rst) - Keyboard event handling
- [`system.rst`](system.rst) - System-level operations
- [`radiosure.rst`](radiosure.rst) - Radio control functionality

### Initial Analysis
- [`file_operations.rst`](file_operations.rst) - File system operations
- [`directory_watcher.rst`](directory_watcher.rst) - Directory monitoring

### Remaining Core Plugins
- [`window.rst`](window.rst) - Window management
- [`task.rst`](task.rst) - Task scheduling and management
- [`network.rst`](network.rst) - Network operations

### Additional Plugins
- [`process_watcher.rst`](process_watcher.rst) - Process monitoring
- [`speech.rst`](speech.rst) - Text-to-speech capabilities
- [`scheduleghost.rst`](scheduleghost.rst) - Advanced scheduling
- [`timer.rst`](timer.rst) - Timer functionality
- [`serial.rst`](serial.rst) - Serial port communication
- [`ping.rst`](ping.rst) - Network ping utilities
- [`mqtt.rst`](mqtt.rst) - MQTT protocol support
- [`google_chrome.rst`](google_chrome.rst) - Chrome browser integration

## Documentation Format

Each plugin documentation file follows a standard RST format including:
1. Overview and purpose
2. Configuration options
3. Events generated/handled
4. Actions provided
5. Implementation details
6. Migration notes from Python to Rust

## Contributing

When adding new plugin documentation:
1. Use the [`template.rst`](template.rst) as a starting point
2. Follow the established RST formatting
3. Include all necessary sections
4. Update the [`index.rst`](index.rst) to include your new plugin
5. Ensure proper categorization in the documentation structure

## Plugin Development Status

The documentation reflects different stages of plugin development:
- ✅ Complete Analysis - Full documentation with Rust implementation
- 🔄 Migration Analysis - Detailed analysis for Python to Rust migration
- 📝 Initial Analysis - Basic documentation and planning
- ⏳ Pending - Documentation to be expanded

## See Also

- [Plugin Architecture](../../PLUGIN_ARCHITECTURE.md)
- [Architecture Overview](../../ARCHITECTURE.md)
- [GUI Architecture](../../GUI_ARCHITECTURE.md)
