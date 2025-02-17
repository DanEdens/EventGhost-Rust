use gtk::prelude::*;
use gtk::{self, Dialog, Box, Label, Entry, Grid, Button, ResponseType};
use uuid::Uuid;
use std::collections::HashMap;

use crate::eg::config::{Plugin, Folder, Macro, Event, Action};

/// Base dialog for configuration items
pub struct ConfigDialog {
    dialog: Dialog,
    name_entry: Entry,
}

impl ConfigDialog {
    fn new(title: &str) -> Self {
        let dialog = Dialog::builder()
            .title(title)
            .modal(true)
            .build();

        dialog.add_button("Cancel", ResponseType::Cancel);
        dialog.add_button("OK", ResponseType::Ok);

        let content_area = dialog.content_area();
        
        let grid = Grid::new();
        grid.set_row_spacing(6);
        grid.set_column_spacing(6);
        grid.set_margin_top(6);
        grid.set_margin_bottom(6);
        grid.set_margin_start(6);
        grid.set_margin_end(6);
        
        let name_label = Label::new(Some("Name:"));
        grid.attach(&name_label, 0, 0, 1, 1);
        
        let name_entry = Entry::new();
        grid.attach(&name_entry, 1, 0, 1, 1);
        
        content_area.append(&grid);

        ConfigDialog {
            dialog,
            name_entry,
        }
    }

    fn get_name(&self) -> String {
        self.name_entry.text().to_string()
    }

    fn set_name(&self, name: &str) {
        self.name_entry.set_text(name);
    }

    fn run(&self) -> ResponseType {
        self.dialog.run()
    }
}

/// Dialog for adding/editing plugins
pub struct PluginDialog {
    base: ConfigDialog,
}

impl PluginDialog {
    pub fn new() -> Self {
        let base = ConfigDialog::new("Plugin");
        PluginDialog { base }
    }

    pub fn run_for_new(&self) -> Option<Plugin> {
        if self.base.run() == ResponseType::Ok {
            Some(Plugin {
                id: Uuid::new_v4(),
                name: self.base.get_name(),
                config: HashMap::new(),
            })
        } else {
            None
        }
    }

    pub fn run_for_edit(&self, plugin: &Plugin) -> Option<Plugin> {
        self.base.set_name(&plugin.name);
        if self.base.run() == ResponseType::Ok {
            Some(Plugin {
                id: plugin.id,
                name: self.base.get_name(),
                config: plugin.config.clone(),
            })
        } else {
            None
        }
    }
}

/// Dialog for adding/editing folders
pub struct FolderDialog {
    base: ConfigDialog,
}

impl FolderDialog {
    pub fn new() -> Self {
        let base = ConfigDialog::new("Folder");
        FolderDialog { base }
    }

    pub fn run_for_new(&self) -> Option<Folder> {
        if self.base.run() == ResponseType::Ok {
            Some(Folder {
                id: Uuid::new_v4(),
                name: self.base.get_name(),
            })
        } else {
            None
        }
    }

    pub fn run_for_edit(&self, folder: &Folder) -> Option<Folder> {
        self.base.set_name(&folder.name);
        if self.base.run() == ResponseType::Ok {
            Some(Folder {
                id: folder.id,
                name: self.base.get_name(),
            })
        } else {
            None
        }
    }
}

/// Dialog for adding/editing macros
pub struct MacroDialog {
    base: ConfigDialog,
}

impl MacroDialog {
    pub fn new() -> Self {
        let base = ConfigDialog::new("Macro");
        MacroDialog { base }
    }

    pub fn run_for_new(&self) -> Option<Macro> {
        if self.base.run() == ResponseType::Ok {
            Some(Macro {
                id: Uuid::new_v4(),
                name: self.base.get_name(),
                events: Vec::new(),
                actions: Vec::new(),
            })
        } else {
            None
        }
    }

    pub fn run_for_edit(&self, macro_: &Macro) -> Option<Macro> {
        self.base.set_name(&macro_.name);
        if self.base.run() == ResponseType::Ok {
            Some(Macro {
                id: macro_.id,
                name: self.base.get_name(),
                events: macro_.events.clone(),
                actions: macro_.actions.clone(),
            })
        } else {
            None
        }
    }
}

/// Dialog for adding/editing events
pub struct EventDialog {
    base: ConfigDialog,
}

impl EventDialog {
    pub fn new() -> Self {
        let base = ConfigDialog::new("Event");
        EventDialog { base }
    }

    pub fn run_for_new(&self) -> Option<Event> {
        if self.base.run() == ResponseType::Ok {
            Some(Event {
                id: Uuid::new_v4(),
                name: self.base.get_name(),
                parameters: HashMap::new(),
            })
        } else {
            None
        }
    }

    pub fn run_for_edit(&self, event: &Event) -> Option<Event> {
        self.base.set_name(&event.name);
        if self.base.run() == ResponseType::Ok {
            Some(Event {
                id: event.id,
                name: self.base.get_name(),
                parameters: event.parameters.clone(),
            })
        } else {
            None
        }
    }
}

/// Dialog for adding/editing actions
pub struct ActionDialog {
    base: ConfigDialog,
}

impl ActionDialog {
    pub fn new() -> Self {
        let base = ConfigDialog::new("Action");
        ActionDialog { base }
    }

    pub fn run_for_new(&self) -> Option<Action> {
        if self.base.run() == ResponseType::Ok {
            Some(Action {
                id: Uuid::new_v4(),
                name: self.base.get_name(),
                parameters: HashMap::new(),
            })
        } else {
            None
        }
    }

    pub fn run_for_edit(&self, action: &Action) -> Option<Action> {
        self.base.set_name(&action.name);
        if self.base.run() == ResponseType::Ok {
            Some(Action {
                id: action.id,
                name: self.base.get_name(),
                parameters: action.parameters.clone(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plugin_dialog() {
        gtk::init().expect("Failed to initialize GTK");
        
        let dialog = PluginDialog::new();
        
        // Test creating new plugin
        dialog.base.set_name("Test Plugin");
        if let Some(plugin) = dialog.run_for_new() {
            assert_eq!(plugin.name, "Test Plugin");
            assert!(plugin.config.is_empty());
        }
        
        // Test editing existing plugin
        let existing = Plugin {
            id: Uuid::new_v4(),
            name: "Existing Plugin".to_string(),
            config: HashMap::new(),
        };
        if let Some(edited) = dialog.run_for_edit(&existing) {
            assert_eq!(edited.id, existing.id);
            assert_eq!(edited.name, "Test Plugin");
        }
    }
} 