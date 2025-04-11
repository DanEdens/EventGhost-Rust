# EventGhost-Rust UML Diagrams

This directory contains UML diagrams for the EventGhost-Rust project in PlantUML format. These diagrams provide a visual representation of the project's architecture and design patterns.

## Diagrams

1. **architecture.puml** - Overall architecture of the EventGhost-Rust project
2. **ui_components.puml** - UI component hierarchy and relationships
3. **plugin_system.puml** - Plugin system architecture including events and actions
4. **refcell_patterns.puml** - Borrowing patterns for Rc<RefCell<>> in GTK applications

## Viewing the Diagrams

These diagrams are in PlantUML format. To view them, you have several options:

### Online PlantUML Viewer

1. Go to [PlantUML Online Server](https://www.plantuml.com/plantuml/uml/)
2. Copy and paste the content of any `.puml` file into the editor
3. The diagram will render automatically

### VS Code Extension

1. Install the "PlantUML" extension for VS Code
2. Open any `.puml` file in VS Code
3. Use Alt+D to preview the diagram

### Command Line

If you have Java installed:

1. Download the PlantUML JAR file from [PlantUML website](https://plantuml.com/download)
2. Run: `java -jar plantuml.jar architecture.puml`
3. This will generate `architecture.png` in the same directory

## Updating Diagrams

When making significant changes to the codebase structure, consider updating these diagrams to keep them in sync with the actual implementation. This will help maintain up-to-date documentation for developers. 