//! Prelude module for EventGhost
//!
//! This module exports common traits and types used throughout the codebase
//! to simplify imports and ensure consistency.

// Re-export GTK modules
pub use gtk4 as gtk;
pub use glib;
pub use gdk4 as gdk;
pub use gio;

// Common GTK traits
pub use gtk4::prelude::*;

// Common GTK types
pub use gtk4::{
    Application, ApplicationWindow, Box, Button, CheckButton, Dialog, Entry,
    FileChooserDialog, Grid, Label, ListBox, MessageDialog, 
    MessageType, Notebook, Orientation, Paned, PopoverMenu, PopoverMenuBar, 
    ResponseType, ScrolledWindow, Separator, Statusbar, TreeIter, TreePath, 
    TreeSelection, TreeStore, TreeView, TreeViewColumn, Window,
    CssProvider, StyleContext, ShortcutController, EventControllerKey, Widget,
    AboutDialog, License, Image, FileChooserAction, FileFilter, Frame,
    HeaderBar, Align, CellRendererText, CellRendererPixbuf, SelectionMode,
    DragSource, DropTarget
};

// Additional specific GTK traits that might not be included in prelude
pub use gtk4::prelude::{
    ApplicationExt, ApplicationWindowExt, BoxExt, ButtonExt, 
    CellLayoutExt, CellRendererExt, DialogExt, EditableExt, 
    EntryExt, FileChooserExt, GridExt, GtkWindowExt, 
    WidgetExt, DialogExtManual, FileChooserExtManual, GestureSingleExt,
    TreeModelExtManual, TreeViewExt, TreeModelExt, ObjectExt
};

// TreeView related traits that might need direct imports
pub use gtk4::{TreeSelectionExt, TreeViewColumnExt};

// GDK/GIO/GLib traits and types
pub use gdk::prelude::*;
pub use gdk::{Display, Key, ModifierType};
pub use gio::prelude::*;
pub use gio::Menu;
pub use glib::prelude::*;
pub use glib::{self, Propagation, clone};

// Common std imports
pub use std::rc::Rc;
pub use std::cell::{RefCell, Cell};
pub use std::path::{Path, PathBuf};
pub use std::borrow::Borrow;

// Common error types
pub use crate::core::Error;
pub use log::{debug, info, warn, error};

// Common EventGhost types
pub use crate::core::config_manager::ConfigManager;

// Export path utilities
pub use crate::utils::path; 
