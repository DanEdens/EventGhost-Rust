use gtk::prelude::*;
use gtk::{self, Dialog, Button, Entry, Label, ResponseType, Grid, Window, HeaderBar, Box};
use gtk::glib;
use std::rc::Rc;
use std::cell::RefCell;
use std::result::Result;
use std::collections::HashMap;
use uuid::Uuid;
use crate::core::Error;
use super::property_grid::PropertyGrid;
use super::UIComponent;
use crate::eg::config::{Plugin, Config, ConfigItem};

pub struct PluginPage {
    pub name: String,
    pub widget: Box,
    pub property_grid: PropertyGrid,
}

impl PluginPage {
    pub fn new(name: &str) -> Self {
        let widget = Box::new(gtk::Orientation::Vertical, 6);
        widget.set_margin_top(12);
        widget.set_margin_bottom(12);
        widget.set_margin_start(12);
        widget.set_margin_end(12);
        
        let property_grid = PropertyGrid::new();
        
        // Add property grid to page
        widget.append(property_grid.get_widget());
        
        PluginPage {
            name: name.to_string(),
            widget,
            property_grid,
        }
    }
}

pub struct PluginConfigDialog {
    widget: Dialog,
    pages: Vec<PluginPage>,
    config: Option<Rc<RefCell<Config>>>,
    plugin_id: Option<Uuid>,
    plugin_values: HashMap<String, String>,
}

impl PluginConfigDialog {
    pub fn new(parent: &Window, plugin_name: &str) -> Self {
        // Create dialog
        let widget = Dialog::builder()
            .title(&format!("{} Configuration", plugin_name))
            .modal(true)
            .default_width(500)
            .default_height(400)
            .transient_for(parent)
            .build();
        
        // Set up dialog buttons
        widget.add_button("Cancel", ResponseType::Cancel);
        widget.add_button("OK", ResponseType::Ok);
        widget.add_button("Apply", ResponseType::Apply);
        
        // Set up dialog UI
        let header_bar = gtk::HeaderBar::new();
        header_bar.set_title_widget(Some(&gtk::Label::new(Some(&format!("{} Configuration", plugin_name)))));
        widget.set_titlebar(Some(&header_bar));
        
        PluginConfigDialog {
            widget,
            pages: Vec::new(),
            config: None,
            plugin_id: None,
            plugin_values: HashMap::new(),
        }
    }
    
    pub fn add_page(&mut self, page: PluginPage) {
        // Add the page widget to the dialog
        let content_area = self.widget.content_area();
        content_area.append(&page.widget);
        
        // Store the page
        self.pages.push(page);
    }
    
    pub fn set_plugin(&mut self, plugin: &Plugin, config: Rc<RefCell<Config>>) {
        self.plugin_id = Some(plugin.id);
        self.config = Some(config);
        self.plugin_values = plugin.config.clone();
        
        // Update UI with plugin config
        for page in &self.pages {
            // Set properties based on plugin config
            for (key, value) in &plugin.config {
                page.property_grid.set_property(key, value, "string");
            }
        }
    }
    
    pub fn run(&self) -> ResponseType {
        // Create a channel to receive the dialog response
        let (sender, receiver) = gtk::glib::MainContext::channel(glib::Priority::DEFAULT);
        
        // Connect to the response signal to send the response through the channel
        self.widget.connect_response(move |dialog, response| {
            sender.send(response).expect("Failed to send response");
            dialog.close();
        });
        
        // Show the dialog
        self.widget.show();
        
        // Use a new main context to wait for the response
        let context = gtk::glib::MainContext::new();
        let _acquire = context.acquire();
        
        // Use a boolean to track response
        let mut response_received = false;
        let mut response_value = ResponseType::Cancel;
        
        // Install source to wait for the response
        let _source_id = receiver.attach(Some(&context), move |response| {
            response_value = response;
            response_received = true;
            gtk::glib::ControlFlow::Break // Remove source after first response
        });
        
        // Run the main loop until we get a response
        while !response_received {
            context.iteration(true);
        }
        
        response_value
    }
    
    pub fn run_for_plugin(&mut self, plugin: &Plugin, config: Rc<RefCell<Config>>) -> Result<Plugin, Error> {
        self.set_plugin(plugin, config);
        
        let response = self.run();
        
        if response == ResponseType::Ok || response == ResponseType::Apply {
            // Collect properties from all pages
            let mut new_config = HashMap::new();
            
            for page in &self.pages {
                // Get properties from the grid
                if let Some(model) = page.property_grid.tree_view.model() {
                    if let Some(iter) = model.iter_first() {
                        let mut current_iter = iter;
                        loop {
                            let name: String = model.get(&current_iter, 0);
                            let value: String = model.get(&current_iter, 1);
                            
                            new_config.insert(name, value);
                            
                            if !model.iter_next(&current_iter) {
                                break;
                            }
                        }
                    }
                }
            }
            
            // Create new plugin config
            let updated_plugin = Plugin {
                id: plugin.id,
                name: plugin.name.clone(),
                config: new_config,
            };
            
            // If we have a config and plugin_id, update the config
            if let Some(config) = &self.config {
                if let Some(plugin_id) = self.plugin_id {
                    let mut config = config.borrow_mut();
                    // Find the plugin in the config items
                    for item in &mut config.items {
                        if let ConfigItem::Plugin(plugin) = item {
                            if plugin.id == self.plugin_id.unwrap() {
                                // Update the plugin values
                                plugin.config = self.plugin_values.clone();
                                break;
                            }
                        }
                    }
                }
            }
            
            Ok(updated_plugin)
        } else {
            Err(Error::Other("Plugin configuration cancelled".to_string()))
        }
    }
}

impl UIComponent for PluginConfigDialog {
    fn get_widget(&self) -> &gtk::Widget {
        self.widget.upcast_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plugin_config_dialog() {
        gtk::init().expect("Failed to initialize GTK");
        
        let window = Window::new();
        let mut dialog = PluginConfigDialog::new(&window, "Test Plugin");
        
        let page = PluginPage::new("General");
        dialog.add_page(page);
        
        assert!(dialog.widget.is_visible());
    }
} 