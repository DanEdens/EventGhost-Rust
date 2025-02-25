use gtk::prelude::*;
use gtk::{self, Dialog, Button, Entry, Label, ResponseType, Grid, Window, HeaderBar, Box};
use gtk::glib::MainContext;
use std::rc::Rc;
use std::cell::RefCell;
use std::result::Result;
use std::collections::HashMap;
use uuid::Uuid;
use crate::core::Error;
use super::property_grid::PropertyGrid;
use super::UIComponent;
use crate::eg::config::{Plugin, Config};

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
        let header_bar = HeaderBar::new();
        header_bar.set_title(Some(&format!("{} Configuration", plugin_name)));
        
        widget.set_titlebar(Some(&header_bar));
        
        PluginConfigDialog {
            widget,
            pages: Vec::new(),
            config: None,
            plugin_id: None,
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
        
        // Update UI with plugin config
        for page in &self.pages {
            // Set properties based on plugin config
            for (key, value) in &plugin.config {
                page.property_grid.set_property(key, value, "string");
            }
        }
    }
    
    pub fn run(&self) -> ResponseType {
        let future = self.widget.run_future();
        MainContext::default().block_on(future)
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
                    // Update the plugin in the config
                    for plugin in &mut config.plugins {
                        if plugin.id == plugin_id {
                            *plugin = updated_plugin.clone();
                            break;
                        }
                    }
                }
            }
            
            Ok(updated_plugin)
        } else {
            Err(Error::new("Plugin configuration cancelled"))
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