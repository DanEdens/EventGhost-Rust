//! Prelude module for EventGhost
//!
//! This module exports common traits and types used throughout the codebase
//! to simplify imports and ensure consistency.

// Re-export GTK traits
pub use gtk4 as gtk;
pub use glib;
pub use gdk4 as gdk;
pub use gio;

// Common GTK traits
pub use gtk::prelude::*;

// Common GTK types
pub use gtk::{
    Application, ApplicationWindow, Box, Button, CheckButton, Dialog, Entry,
    FileChooserDialog, Grid, Label, ListBox, Menu, MenuButton, MessageDialog, 
    MessageType, Notebook, Orientation, Paned, PopoverMenu, PopoverMenuBar, 
    ResponseType, ScrolledWindow, Separator, Statusbar, TreeIter, TreePath, 
    TreeSelection, TreeStore, TreeView, TreeViewColumn, Window
};

// Additional specific GTK traits that might not be included in prelude
pub use gtk::prelude::{
    ApplicationExt, ApplicationWindowExt, BoxExt, ButtonExt, 
    CellLayoutExt, CellRendererExt, DialogExt, EditableExt, 
    EntryExt, FileChooserExt, GridExt, GtkWindowExt, LabelExt, 
    ListBoxExt, MenuButtonExt, NativeDialogExt, NotebookExt, 
    PanedExt, PopoverExt, RangeExt, ScrollableExt, SelectionModelExt, 
    TextBufferExt, TextViewExt, ToggleButtonExt, TreeModelExt, 
    TreeSelectionExt, TreeSortableExt, TreeViewColumnExt, TreeViewExt, 
    WidgetExt
};

// GDK/GIO/GLib traits and types
pub use gdk::prelude::*;
pub use gio::prelude::*;
pub use glib::prelude::*;

// Common std imports
pub use std::rc::Rc;
pub use std::cell::{RefCell, Cell};
pub use std::path::{Path, PathBuf};

// Common error types
pub use crate::core::Error;
pub use log::{debug, info, warn, error};

// Common EventGhost types
pub use crate::core::config_manager::ConfigManager;

// Export path utilities
pub use crate::utils::path; 