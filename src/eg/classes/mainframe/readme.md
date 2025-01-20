# MainFrame Implementation

## Overview
The MainFrame is the primary user interface component of the EventGhost application, built using GTK4 in Rust. It provides a comprehensive workspace for managing events, logs, and application state.

## Key Components

### Window Structure
- **Header Bar**: Replaces traditional menu bar with a modern, compact design
- **Toolbar**: Provides quick access to common actions
- **Tree Control**: Displays hierarchical event and configuration data
- **Log Control**: Shows application logs and messages
- **Status Bar**: Displays current application status and additional information

## Architecture

### MainFrame Struct
The `MainFrame` struct manages the lifecycle and state of the main application window:

```rust
pub struct MainFrame {
    window: ApplicationWindow,
    tree_ctrl: Option<TreeCtrl>,
    log_ctrl: Option<LogCtrl>,
    status_bar: Option<StatusBar>,
    document: Option<Document>,
}
```

### Key Methods
- `new(app: &Application)`: Initializes the main window and all its components
- `show()`: Displays the main window
- `set_document(document: Document)`: Updates the current working document

## Actions and Interactions
The MainFrame supports various actions:
- File operations (New, Open, Save)
- Edit actions
- View management
- Plugin and macro interactions

## Testing
Includes a comprehensive test suite to validate:
- Window initialization
- Component integration
- Basic user interactions

## Performance Considerations
- Lazy initialization of components
- Optional components for flexibility
- Minimal overhead with GTK4's efficient rendering

## Future Improvements
- Enhanced drag-and-drop support
- More granular action management
- Improved document handling
- Customizable UI layouts

## Dependencies
- GTK4
- Gio
- Glib
- Custom EventGhost components

## Maintaining UI Consistency

1. **Adhere to EventGhost's UI Layout**:
    - Ensure that the positioning, sizing, and behavior of UI components mirror the Python version.
    - Reference the Python implementation for guidance on UI flows.

2. **Feedback from Users**:
    - Engage with EventGhost's community to gather feedback on the Rust UI.
    - Iterate based on suggestions to enhance user experience.

## Contribution
Please refer to the project's main contribution guidelines when making changes to the MainFrame implementation.

# MainFrame Implementation in Rust

This guide walks you through reproducing the `MainFrame` component of EventGhost from Python to Rust. The goal is to maintain a UI experience familiar to diehard EventGhost fans while leveraging Rust's performance and safety features.

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Project Setup](#project-setup)
4. [UI Components Identification](#ui-components-identification)
5. [Implementing the MainFrame Struct](#implementing-the-mainframe-struct)
6. [Creating UI Components in Rust](#creating-ui-components-in-rust)
    - [Toolbar](#toolbar)
    - [StatusBar](#statusbar)
    - [TreeCtrl](#treectrl)
    - [LogCtrl](#logctrl)
7. [Event Handling](#event-handling)
8. [Layout Management](#layout-management)
9. [Integration and Initialization](#integration-and-initialization)
10. [Memory and Resource Management](#memory-and-resource-management)
11. [Logging and Error Handling](#logging-and-error-handling)
12. [Testing](#testing)
13. [Maintaining UI Consistency](#maintaining-ui-consistency)
14. [Git Workflow](#git-workflow)
15. [References](#references)

---

## Overview

The `MainFrame` serves as the primary window in EventGhost, managing various UI components such as the toolbar, status bar, tree control, and log control. This guide outlines the steps to recreate this functionality in Rust, ensuring that the UI remains consistent with the original Python implementation.

## Prerequisites

Before starting, ensure you have the following installed:

- **Rust**: Install Rust and Cargo from [rustup.rs](https://rustup.rs/).
- **Windows SDK**: Required for interacting with Windows APIs.
- **Necessary Crates**: 
  - `windows` crate for Windows API interactions.
  - `wx-rs` or similar for GUI components (note: Rust GUI support is evolving; choose a library that best fits your needs).
- **Development Tools**: Visual Studio or similar for compiling Rust code on Windows.

## Project Setup

1. **Initialize a New Rust Project**:
    ```bash
    cargo new eventghost-rust
    cd eventghost-rust
    ```

2. **Configure Dependencies**:
    Update your `Cargo.toml` with necessary dependencies:
    ```toml
    [dependencies]
    windows = { version = "0.48", features = ["Win32_Foundation", "Win32_UI_WindowsAndMessaging"] }
    wx = "0.1" # Replace with the actual GUI crate you choose
    ```

3. **Project Structure**:
    Organize your project directories to mirror the Python structure for easier reference:
    ```
    src/
    ├── eg/
    │   ├── classes/
    │   │   ├── main_frame.rs
    │   │   ├── log_ctrl.rs
    │   │   ├── status_bar.rs
    │   │   └── tree_ctrl.rs
    ├── main.rs
    └── ...
    ```

## UI Components Identification

Identify and list all UI components present in the Python `MainFrame`:

- **Toolbar**: Contains buttons for various actions.
- **StatusBar**: Displays status information and includes interactive elements like checkboxes.
- **TreeCtrl**: Displays a hierarchical tree structure for plugins, folders, macros, events, and actions.
- **LogCtrl**: Displays log messages with support for different log levels and drag-and-drop functionality.

## Implementing the MainFrame Struct

Create the `MainFrame` struct in Rust to encapsulate all UI components and their interactions.
rust
// src/eg/classes/main_frame.rs
use windows::Win32::Foundation::;
use windows::Win32::UI::WindowsAndMessaging::;
use windows::core::PCSTR;
use crate::win32::{self, Error as Win32Error};
use super::tree_ctrl::TreeCtrl;
use super::log_ctrl::LogCtrl;
use super::status_bar::StatusBar;
use super::toolbar::Toolbar;
pub struct MainFrame {
hwnd: HWND,
tree_ctrl: TreeCtrl,
log_ctrl: LogCtrl,
status_bar: StatusBar,
toolbar: Toolbar,
is_visible: bool,
}
impl MainFrame {
pub fn new(instance: HINSTANCE) -> Result<Self, Win32Error> {
// Implementation details...
}
fn initialize(&mut self) -> Result<(), Win32Error> {
// Implementation details...
}
fn layout_controls(&mut self) {
// Implementation details...
}
pub fn show(&mut self) {
// Implementation details...
}
pub fn hide(&mut self) {
// Implementation details...
}
pub fn set_title(&self, title: &str) -> Result<(), Win32Error> {
// Implementation details...
}
unsafe extern "system" fn window_proc(
hwnd: HWND,
msg: u32,
wparam: WPARAM,
lparam: LPARAM,
) -> LRESULT {
// Implementation details...
}
}
impl Drop for MainFrame {
fn drop(&mut self) {
// Cleanup resources...
}
}


## Creating UI Components in Rust

### Toolbar

1. **Create the Toolbar Struct**:
    ```rust
    // src/eg/classes/toolbar.rs

    use windows::Win32::UI::WindowsAndMessaging::*;
    use crate::win32::{self, Error as Win32Error};

    pub struct Toolbar {
        hwnd: HWND,
        // Additional fields...
    }

    impl Toolbar {
        pub fn new(parent_hwnd: HWND, instance: HINSTANCE) -> Result<Self, Win32Error> {
            // Implementation details...
        }

        pub fn initialize(&mut self) -> Result<(), Win32Error> {
            // Implementation details...
        }

        pub fn get_hwnd(&self) -> HWND {
            self.hwnd
        }
    }
    ```

2. **Initialize Toolbar Buttons**:
    - Define toolbar buttons with appropriate icons and actions.
    - Handle button click events and link them to corresponding methods in `MainFrame`.

### StatusBar

1. **Create the StatusBar Struct**:
    ```rust
    // src/eg/classes/status_bar.rs

    use windows::Win32::UI::WindowsAndMessaging::*;
    use crate::win32::{self, Error as Win32Error};

    pub struct StatusBar {
        hwnd: HWND,
        // Additional fields...
    }

    impl StatusBar {
        pub fn new(parent_hwnd: HWND, instance: HINSTANCE) -> Result<Self, Win32Error> {
            // Implementation details...
        }

        pub fn initialize(&mut self) -> Result<(), Win32Error> {
            // Implementation details...
        }

        pub fn get_hwnd(&self) -> HWND {
            self.hwnd
        }
    }
    ```

2. **Handle StatusBar Interactions**:
    - Implement methods to update status messages.
    - Handle interactive elements like checkboxes.

### TreeCtrl

1. **Create the TreeCtrl Struct**:
    ```rust
    // src/eg/classes/tree_ctrl.rs

    use windows::Win32::UI::WindowsAndMessaging::*;
    use crate::win32::{self, Error as Win32Error};

    pub struct TreeCtrl {
        hwnd: HWND,
        // Additional fields...
    }

    impl TreeCtrl {
        pub fn new(parent_hwnd: HWND, instance: HINSTANCE) -> Result<Self, Win32Error> {
            // Implementation details...
        }

        pub fn initialize(&mut self) -> Result<(), Win32Error> {
            // Implementation details...
        }

        pub fn get_hwnd(&self) -> HWND {
            self.hwnd
        }
    }
    ```

2. **Populate the Tree Structure**:
    - Load plugins, folders, macros, events, and actions.
    - Implement drag-and-drop functionality.

### LogCtrl

1. **Create the LogCtrl Struct**:
    ```rust
    // src/eg/classes/log_ctrl.rs

    use windows::Win32::UI::WindowsAndMessaging::*;
    use crate::win32::{self, Error as Win32Error};

    pub struct LogCtrl {
        hwnd: HWND,
        // Additional fields...
    }

    impl LogCtrl {
        pub fn new(parent_hwnd: HWND, instance: HINSTANCE) -> Result<Self, Win32Error> {
            // Implementation details...
        }

        pub fn initialize(&mut self) -> Result<(), Win32Error> {
            // Implementation details...
        }

        pub fn get_hwnd(&self) -> HWND {
            self.hwnd
        }

        pub fn write_line(&self, message: &str, icon: u32) {
            // Implementation details...
        }
    }
    ```

2. **Implement Logging Features**:
    - Handle different log levels (Info, Warning, Error).
    - Support filtering and searching within logs.

## Event Handling

1. **Define Event Procedures**:
    - Implement `window_proc` to handle Windows messages such as `WM_DESTROY`, `WM_SIZE`, and `WM_CREATE`.
    - Link messages to corresponding methods in `MainFrame`.

2. **Bind Events to Handlers**:
    - Ensure that UI interactions trigger appropriate responses.
    - Example: Clicking a toolbar button executes a specific action.

## Layout Management

1. **Calculate Client Area Dimensions**:
    - Retrieve client area using `GetClientRect`.
    - Determine the size and position of each UI component based on the window size.

2. **Adjust Layout on Window Resize**:
    - Handle `WM_SIZE` messages to adjust the layout dynamically.
    - Ensure that components resize proportionally.

## Integration and Initialization

1. **Register Window Class**:
    ```rust
    // Within MainFrame::new
    win32::register_window_class(
        PCSTR::from_raw(MAIN_WINDOW_CLASS.as_ptr()),
        Some(Self::window_proc),
        instance,
    )?;
    ```

2. **Create Main Window**:
    ```rust
    let hwnd = win32::create_window(
        PCSTR::from_raw(MAIN_WINDOW_CLASS.as_ptr()),
        PCSTR::from_raw(b"EventGhost\0".as_ptr()),
        WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        800,
        600,
        None,
        instance,
    )?;
    ```

3. **Initialize UI Components**:
    - Instantiate `TreeCtrl`, `LogCtrl`, `StatusBar`, and `Toolbar`.
    - Call their `initialize` methods to set up UI elements.

## Memory and Resource Management

1. **Implement the `Drop` Trait**:
    - Ensure that resources like windows and controls are properly released.
    - Example:
        ```rust
        impl Drop for MainFrame {
            fn drop(&mut self) {
                unsafe {
                    DestroyWindow(self.hwnd);
                }
            }
        }
        ```

2. **Handle Unsafe Code Carefully**:
    - Minimize the use of `unsafe` blocks.
    - Ensure all pointers are valid before dereferencing.

## Logging and Error Handling

1. **Integrate Logging Mechanisms**:
    - Use Rust's `log` crate or similar for logging.
    - Redirect logs to `LogCtrl` for display within the UI.

2. **Handle Errors Gracefully**:
    - Use Rust's `Result` and `Option` types to manage potential errors.
    - Display error messages in the `StatusBar` or via dialog boxes.

## Testing

1. **Implement Unit Tests**:
    - Write tests for individual components like `Toolbar`, `StatusBar`, etc.
    - Ensure that each component behaves as expected.

2. **Integration Testing**:
    - Test the interaction between different UI components.
    - Simulate user interactions and verify responses.

3. **Use `cargo check` and `cargo test`**:
    - Regularly run these commands to ensure code correctness and test coverage.
    - Example:
        ```bash
        cargo check --message-format=json > output.json
        cargo test
        ```

## Maintaining UI Consistency

1. **Adhere to EventGhost's UI Layout**:
    - Ensure that the positioning, sizing, and behavior of UI components mirror the Python version.
    - Reference the Python implementation for guidance on UI flows.

2. **Handle Theme and Style**:
    - Implement consistent color schemes, fonts, and iconography.
    - Use Rust-compatible libraries or custom implementations to match the original aesthetic.

3. **Feedback from Users**:
    - Engage with EventGhost's community to gather feedback on the Rust UI.
    - Iterate based on suggestions to enhance user experience.


4. **Avoid Pagers on Windows**:
    - Use `--no-pager` for commands like `git diff` and `git log`.
    - Example:
        ```bash
        git --no-pager log
        ```

5. **Post-Feature Integration**:
    - Merge feature branches back into `main` upon completion.
    - Ensure all tests pass before merging.

## References

- **EventGhost Python Implementation**: [EventGhost GitHub Repository](https://github.com/eventghost/eventghost)
- **Rust Windows API Documentation**: [Windows Crate Docs](https://docs.rs/windows/)
- **Rust GUI Libraries**:
    - [wxRust](https://github.com/hecrj/wxRust) (if maintained)
    - [druid](https://druid.rs/)
    - [egui](https://github.com/emilk/egui)
- **Rust Logging**: [log Crate](https://crates.io/crates/log)
- **Windows API Tutorials in Rust**:
    - [Using the Windows API in Rust](https://learn.microsoft.com/en-us/windows/win32/learnwin32/learning-win32-in-rust)

---

By following this guide, you will recreate the `MainFrame` component of EventGhost in Rust, ensuring a familiar and consistent user experience while leveraging the benefits of Rust's system programming capabilities.

