use gtk::prelude::*;
use gtk::{self, Dialog, Box, Label, Entry, Grid, Button, ResponseType};
use gtk::glib::{self, clone, MainContext};
use uuid::Uuid;
use std::collections::HashMap;

use crate::eg::config::{Plugin, Folder, Macro, Event, Action};

/// Base dialog for configuration items
#[derive(Clone)]
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
        let future = self.dialog.run_future();
        MainContext::default().block_on(future)
    }
}

/// Dialog for adding/editing plugins
#[derive(Clone)]
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
#[derive(Clone)]
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
#[derive(Clone)]
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
#[derive(Clone)]
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
#[derive(Clone)]
pub struct ActionDialog {
    base: ConfigDialog,
    advanced_button: Button,
    config_dialog: Option<crate::eg::classes::action_config_dialog::ActionConfigDialog>,
}

impl ActionDialog {
    pub fn new() -> Self {
        let base = ConfigDialog::new("Action");
        
        // Add a button for advanced configuration
        let advanced_button = Button::with_label("Advanced Configuration...");
        base.dialog.content_area().append(&advanced_button);
        
        ActionDialog { 
            base,
            advanced_button,
            config_dialog: None,
        }
    }

    pub fn run_for_new(&self) -> Option<Action> {
        let mut action = None;
        let parameters = std::rc::Rc::new(std::cell::RefCell::new(HashMap::new()));
        
        // Connect button click handler
        let advanced_clicked = self.advanced_button.connect_clicked(clone!(@weak self.base.dialog as dialog, @strong parameters => move |_| {
            // Hide the simple dialog temporarily
            dialog.hide();
            
            // Create and show the advanced configuration dialog
            let config_dialog = crate::eg::classes::action_config_dialog::ActionConfigDialog::new();
            if let Some(new_action) = config_dialog.run_for_new_config_action() {
                // Copy the values back to our parameters
                let params_clone = parameters.clone();
                let mut params = params_clone.borrow_mut();
                params.clear();
                params.extend(new_action.parameters.clone());
            }
            
            // Show the simple dialog again
            dialog.show();
        }));
        
        if self.base.run() == ResponseType::Ok {
            action = Some(Action {
                id: Uuid::new_v4(),
                name: self.base.get_name(),
                parameters: parameters.borrow().clone(),
            });
        }
        
        // Disconnect signal handler
        self.advanced_button.disconnect(advanced_clicked);
        
        action
    }

    pub fn run_for_edit(&self, action: &Action) -> Option<Action> {
        self.base.set_name(&action.name);
        
        let mut result = None;
        let parameters = std::rc::Rc::new(std::cell::RefCell::new(action.parameters.clone()));
        
        // Connect button click handler
        let advanced_clicked = self.advanced_button.connect_clicked(clone!(@weak self.base.dialog as dialog, @strong parameters, @strong action => move |_| {
            // Hide the simple dialog temporarily
            dialog.hide();
            
            // Create the action for advanced configuration with current values
            let config_action = Action {
                id: action.id,
                name: action.name.clone(),
                parameters: parameters.borrow().clone(),
            };
            
            // Create and show the advanced configuration dialog
            let config_dialog = crate::eg::classes::action_config_dialog::ActionConfigDialog::new();
            if let Some(updated_action) = config_dialog.run_for_config_action(&config_action) {
                // Copy the values back to our parameters
                let params_clone = parameters.clone();
                let mut params = params_clone.borrow_mut();
                params.clear();
                params.extend(updated_action.parameters.clone());
            }
            
            // Show the simple dialog again
            dialog.show();
        }));
        
        if self.base.run() == ResponseType::Ok {
            result = Some(Action {
                id: action.id,
                name: self.base.get_name(),
                parameters: parameters.borrow().clone(),
            });
        }
        
        // Disconnect signal handler
        self.advanced_button.disconnect(advanced_clicked);
        
        result
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