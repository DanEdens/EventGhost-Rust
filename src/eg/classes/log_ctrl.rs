use crate::prelude::*;
use crate::prelude::{self, Box, TreeView, TreeViewColumn, CellRendererText, CellRendererPixbuf, ScrolledWindow};
use glib;
use gdk4;
use chrono::{DateTime, Local};
use std::collections::VecDeque;
use std::sync::{Mutex, Arc};
use std::fs::File;
use std::io::{self, Write, BufRead};
use std::fmt;
use std::cell::Cell;
use gtk4::ListStore;

const MAX_BUFFER_SIZE: usize = 2000;
const REMOVE_ON_MAX: usize = 200;

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Local>,
    pub level: LogLevel,
    pub message: String,
    pub source: Option<String>,
    pub indent: usize,
    pub show_time: bool,
    pub show_date: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Event,
}

impl LogLevel {
    fn icon_name(&self) -> &'static str {
        match self {
            LogLevel::Debug => "dialog-information",
            LogLevel::Info => "dialog-information",
            LogLevel::Warning => "dialog-warning",
            LogLevel::Error => "dialog-error",
            LogLevel::Event => "dialog-information",
        }
    }

    fn get_colors(&self) -> (&'static str, &'static str) {
        match self {
            LogLevel::Debug => ("#666666", "#ffffff"),
            LogLevel::Info => ("#000000", "#ffffff"),
            LogLevel::Warning => ("#7f4f00", "#ffffff"),
            LogLevel::Error => ("#cc0000", "#ffffff"),
            LogLevel::Event => ("#000099", "#ffffff"),
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level_str = match self {
            LogLevel::Debug => "Debug",
            LogLevel::Info => "Info",
            LogLevel::Warning => "Warning",
            LogLevel::Error => "Error",
            LogLevel::Event => "Event",
        };
        write!(f, "{}", level_str)
    }
}

#[derive(Debug)]
pub struct LogCtrl {
    pub container: ScrolledWindow,
    pub widget: TreeView,
    pub store: ListStore,
    pub entries: Arc<Mutex<VecDeque<LogEntry>>>,
    pub show_time: bool,
    pub show_date: bool,
    pub indent: bool,
    pub auto_scroll: bool,
    is_odd: Cell<bool>,
}

impl Clone for LogCtrl {
    fn clone(&self) -> Self {
        // Create a new TreeView with the same model
        let widget = TreeView::with_model(&self.store);
        widget.set_headers_visible(false);
        widget.set_enable_search(true);
        widget.set_search_column(1); // Search in the text column
        
        // Add icon column
        let icon_renderer = CellRendererPixbuf::new();
        let icon_column = TreeViewColumn::new();
        icon_column.pack_start(&icon_renderer, false);
        icon_column.add_attribute(&icon_renderer, "icon-name", 0);
        widget.append_column(&icon_column);
        
        // Add text column
        let text_renderer = CellRendererText::new();
        let text_column = TreeViewColumn::new();
        text_column.pack_start(&text_renderer, true);
        text_column.add_attribute(&text_renderer, "text", 1);
        text_column.add_attribute(&text_renderer, "foreground", 2);
        text_column.add_attribute(&text_renderer, "background", 3);
        text_column.set_expand(true);
        widget.append_column(&text_column);
        
        // Create scrolled window container
        let container = ScrolledWindow::new();
        container.set_child(Some(&widget));
        container.set_hexpand(true);
        container.set_vexpand(true);
        container.set_min_content_width(400);
        container.set_min_content_height(300);
        container.set_propagate_natural_height(true);
        container.set_propagate_natural_width(true);
            
        LogCtrl {
            container,
            widget,
            store: self.store.clone(),
            entries: self.entries.clone(),
            show_time: self.show_time,
            show_date: self.show_date,
            indent: self.indent,
            auto_scroll: self.auto_scroll,
            is_odd: Cell::new(self.is_odd.get()),
        }
    }
}

impl LogCtrl {
    pub fn new() -> Self {
        // Create list store with columns:
        // 0: icon name (String)
        // 1: text (String)
        // 2: foreground color (String)
        // 3: background color (String)
        let store = ListStore::new(&[
            glib::Type::STRING, // icon name
            glib::Type::STRING, // text
            glib::Type::STRING, // foreground color
            glib::Type::STRING, // background color
        ]);
        
        // Create tree view
        let widget = TreeView::with_model(&store);
        widget.set_headers_visible(false);
        widget.set_enable_search(true);
        widget.set_search_column(1); // Search in the text column
        
        // Add icon column
        let icon_renderer = CellRendererPixbuf::new();
        let icon_column = TreeViewColumn::new();
        icon_column.pack_start(&icon_renderer, false);
        icon_column.add_attribute(&icon_renderer, "icon-name", 0);
        widget.append_column(&icon_column);
        
        // Add text column
        let text_renderer = CellRendererText::new();
        let text_column = TreeViewColumn::new();
        text_column.pack_start(&text_renderer, true);
        text_column.add_attribute(&text_renderer, "text", 1);
        text_column.add_attribute(&text_renderer, "foreground", 2);
        text_column.add_attribute(&text_renderer, "background", 3);
        text_column.set_expand(true); // Allow text column to expand
        widget.append_column(&text_column);
            
        // Create scrolled window container with proper size and expansion
        let container = ScrolledWindow::new();
        container.set_child(Some(&widget));
        container.set_hexpand(true);
        container.set_vexpand(true);
        container.set_min_content_width(400);
        container.set_min_content_height(300);
        container.set_propagate_natural_height(true);
        container.set_propagate_natural_width(true);
            
        // Create context menu
        let menu = gio::Menu::new();
        
        // Add Copy action
        let copy_item = gio::MenuItem::new(Some("Copy"), Some("log.copy"));
        menu.append_item(&copy_item);
        
        // Add Select All action
        let select_all_item = gio::MenuItem::new(Some("Select All"), Some("log.select-all"));
        menu.append_item(&select_all_item);
        
        // Add separator
        menu.append(None, Some("log.separator"));
        
        // Add Filter submenu
        let filter_menu = gio::Menu::new();
        filter_menu.append(Some("Show All"), Some("log.filter.all"));
        filter_menu.append(Some("Show Info Only"), Some("log.filter.info"));
        filter_menu.append(Some("Show Warnings Only"), Some("log.filter.warning"));
        filter_menu.append(Some("Show Errors Only"), Some("log.filter.error"));
        filter_menu.append(Some("Show Events Only"), Some("log.filter.event"));
        filter_menu.append(Some("Show Debug Only"), Some("log.filter.debug"));
        menu.append_submenu(Some("Filter"), &filter_menu);
        
        // Add Clear action
        menu.append(Some("Clear Log"), Some("log.clear"));
        
        // Create popover menu
        let popover = gtk::PopoverMenu::from_model(Some(&menu));
        
        // Add context menu controller
        let gesture = gtk::GestureClick::new();
        gesture.set_button(3); // Right click
        gesture.connect_pressed(glib::clone!(@weak popover, @weak widget => move |gesture, _, x, y| {
            if gesture.current_button() == 3 {
                popover.set_parent(&widget);
                popover.set_pointing_to(Some(&gdk4::Rectangle::new(
                    x as i32,
                    y as i32,
                    1,
                    1
                )));
                popover.popup();
            }
        }));
        widget.add_controller(gesture);
        
        // Create the LogCtrl instance
        let log_ctrl = LogCtrl {
            container,
            widget,
            store,
            entries: Arc::new(Mutex::new(VecDeque::with_capacity(MAX_BUFFER_SIZE))),
            show_time: true,
            show_date: false,
            indent: true,
            auto_scroll: true,
            is_odd: Cell::new(false),
        };

        // Add actions
        let action_group = gio::SimpleActionGroup::new();
        
        // Copy action
        let copy_action = gio::SimpleAction::new("copy", None);
        let log_ctrl_copy = log_ctrl.clone();
        copy_action.connect_activate(move |_, _| {
            log_ctrl_copy.copy_selected_text();
        });
        action_group.add_action(&copy_action);
        
        // Select All action
        let select_all_action = gio::SimpleAction::new("select-all", None);
        let log_ctrl_select = log_ctrl.clone();
        select_all_action.connect_activate(move |_, _| {
            log_ctrl_select.select_all();
        });
        action_group.add_action(&select_all_action);
        
        // Filter actions
        let filter_all_action = gio::SimpleAction::new("filter.all", None);
        let log_ctrl_all = log_ctrl.clone();
        filter_all_action.connect_activate(move |_, _| {
            log_ctrl_all.filter_logs(None);
        });
        action_group.add_action(&filter_all_action);

        let filter_info_action = gio::SimpleAction::new("filter.info", None);
        let log_ctrl_info = log_ctrl.clone();
        filter_info_action.connect_activate(move |_, _| {
            log_ctrl_info.filter_logs(Some(LogLevel::Info));
        });
        action_group.add_action(&filter_info_action);

        let filter_warning_action = gio::SimpleAction::new("filter.warning", None);
        let log_ctrl_warning = log_ctrl.clone();
        filter_warning_action.connect_activate(move |_, _| {
            log_ctrl_warning.filter_logs(Some(LogLevel::Warning));
        });
        action_group.add_action(&filter_warning_action);

        let filter_error_action = gio::SimpleAction::new("filter.error", None);
        let log_ctrl_error = log_ctrl.clone();
        filter_error_action.connect_activate(move |_, _| {
            log_ctrl_error.filter_logs(Some(LogLevel::Error));
        });
        action_group.add_action(&filter_error_action);

        let filter_event_action = gio::SimpleAction::new("filter.event", None);
        let log_ctrl_event = log_ctrl.clone();
        filter_event_action.connect_activate(move |_, _| {
            log_ctrl_event.filter_logs(Some(LogLevel::Event));
        });
        action_group.add_action(&filter_event_action);

        let filter_debug_action = gio::SimpleAction::new("filter.debug", None);
        let log_ctrl_debug = log_ctrl.clone();
        filter_debug_action.connect_activate(move |_, _| {
            log_ctrl_debug.filter_logs(Some(LogLevel::Debug));
        });
        action_group.add_action(&filter_debug_action);
        
        // Clear action
        let clear_action = gio::SimpleAction::new("clear", None);
        let log_ctrl_clear = log_ctrl.clone();
        clear_action.connect_activate(move |_, _| {
            log_ctrl_clear.clear();
        });
        action_group.add_action(&clear_action);
        
        log_ctrl.widget.insert_action_group("log", Some(&action_group));
        
        // Add a welcome message
        log_ctrl.write(LogEntry {
            timestamp: Local::now(),
            level: LogLevel::Info,
            message: "Welcome to EventGhost".to_string(),
            source: None,
            indent: 0,
            show_time: true,
            show_date: false,
        });
        // Add a second message so we can test the window
        log_ctrl.write(LogEntry {
            timestamp: Local::now(),
            level: LogLevel::Info,
            message: "This is being implemented in rust, and is WIP ".to_string(),
            source: None,
            indent: 0,
            show_time: true,
            show_date: false,
        });
        

        log_ctrl
    }
    
    pub fn write(&self, entry: LogEntry) {
        let mut entries = self.entries.lock().unwrap();
        
        // Handle circular buffer
        if entries.len() >= MAX_BUFFER_SIZE {
            for _ in 0..REMOVE_ON_MAX {
                entries.pop_front();
                if let Some(iter) = self.store.iter_first() {
                    self.store.remove(&iter);
                }
            }
        }
        
        // Add the new entry
        entries.push_back(entry.clone());
        
        // Write the new entry to the list
        self.write_entry(&entry);
        
        // Toggle odd/even state safely using Cell
        self.is_odd.set(!self.is_odd.get());
        
        // Scroll to end
        self.scroll_to_end();
    }

    fn write_entry(&self, entry: &LogEntry) {
        let mut prefix = String::new();
        
        // Add timestamp if enabled
        if self.show_time || self.show_date {
            if self.show_date {
                prefix.push_str(&entry.timestamp.format("%Y-%m-%d ").to_string());
            }
            if self.show_time {
                prefix.push_str(&entry.timestamp.format("%H:%M:%S ").to_string());
            }
        }
        
        // Add indentation
        if self.indent {
            prefix.push_str(&"  ".repeat(entry.indent));
        }
        
        let full_text = format!("{}{}", prefix, entry.message);
        
        // Get colors based on log level and odd/even state
        let (fg_color, bg_color) = match entry.level {
            LogLevel::Error => {
                if self.is_odd.get() {
                    ("#FF0000", "#FFE8E8")
                } else {
                    ("#FF0000", "#FFE0E0")
                }
            }
            LogLevel::Warning => {
                if self.is_odd.get() {
                    ("#C04000", "#FFFFF0")
                } else {
                    ("#C04000", "#FFFFD0")
                }
            }
            LogLevel::Info => {
                if self.is_odd.get() {
                    ("#0000FF", "#F8F8F8")
                } else {
                    ("#0000FF", "#FFFFFF")
                }
            }
            LogLevel::Event => {
                if self.is_odd.get() {
                    ("#008000", "#F8F8F8")
                } else {
                    ("#008000", "#FFFFFF")
                }
            }
            LogLevel::Debug => {
                if self.is_odd.get() {
                    ("#808080", "#F8F8F8")
                } else {
                    ("#808080", "#FFFFFF")
                }
            }
        };
        
        // Add row to store using proper GTK4 API
        let iter = self.store.append();
        self.store.set_value(&iter, 0, &entry.level.icon_name().to_value());
        self.store.set_value(&iter, 1, &full_text.to_value());
        self.store.set_value(&iter, 2, &fg_color.to_value());
        self.store.set_value(&iter, 3, &bg_color.to_value());
    }
    
    pub fn clear(&self) {
        self.entries.lock().unwrap().clear();
        self.store.clear();
    }

    pub fn select_all(&self) {
        self.widget.selection().select_all();
    }
    
    pub fn set_time_logging(&mut self, enabled: bool) {
        self.show_time = enabled;
        self.refresh_view();
    }
    
    pub fn set_date_logging(&mut self, enabled: bool) {
        self.show_date = enabled;
        self.refresh_view();
    }
    
    pub fn set_indent(&mut self, enabled: bool) {
        self.indent = enabled;
        self.refresh_view();
    }

    fn refresh_view(&self) {
        // Clear store
        self.store.clear();
        
        // Rewrite all entries with new formatting
        let entries = self.entries.lock().unwrap();
        for entry in entries.iter() {
            self.write_entry(entry);
        }
    }

    pub fn filter_logs(&self, level: Option<LogLevel>) {
        self.store.clear();
        if let Ok(entries) = self.entries.lock() {
            for entry in entries.iter() {
                if level.is_none() || level.unwrap() == entry.level {
                    self.write_entry(entry);
                }
            }
        }
    }

    pub fn search_logs(&self, query: &str, case_sensitive: bool) {
        // Clear store
        self.store.clear();
        
        self.is_odd.set(false); // Reset alternating colors
        
        let query = if !case_sensitive {
            query.to_lowercase()
        } else {
            query.to_string()
        };
        
        let entries = self.entries.lock().unwrap();
        for entry in entries.iter() {
            let message = if !case_sensitive {
                entry.message.to_lowercase()
            } else {
                entry.message.clone()
            };
            
            let source_match = entry.source.as_ref().map_or(false, |s| {
                let source = if !case_sensitive {
                    s.to_lowercase()
                } else {
                    s.clone()
                };
                source.contains(&query)
            });
            
            if message.contains(&query) || source_match {
                self.write_entry(entry);
                self.is_odd.set(!self.is_odd.get());
            }
        }
    }

    pub fn copy_selected_text(&self) {
        let selection = self.widget.selection();
        if let Some((model, iter)) = selection.selected() {
            if let Ok(text) = model.get_value(&iter, 1).get::<String>() {
                let clipboard = self.widget.clipboard();
                clipboard.set_text(&text);
            }
        }
    }

    pub fn save_logs(&self, file_path: &str) -> io::Result<()> {
        let mut file = File::create(file_path)?;
        let entries = self.entries.lock().unwrap();
        
        for entry in entries.iter() {
            // Format: timestamp|level|source|indent|message
            let source = entry.source.as_deref().unwrap_or("");
            let line = format!(
                "{}|{}|{}|{}|{}\n",
                entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
                entry.level,
                source,
                entry.indent,
                entry.message
            );
            file.write_all(line.as_bytes())?;
        }
        Ok(())
    }

    pub fn load_logs(&self, file_path: &str) -> io::Result<()> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut entries = self.entries.lock().unwrap();
        
        // Clear existing entries
        entries.clear();
        self.store.clear();
        
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split('|').collect();
            
            if parts.len() == 5 {
                let timestamp = DateTime::parse_from_str(parts[0], "%Y-%m-%d %H:%M:%S")
                    .map(|dt| dt.with_timezone(&Local))
                    .unwrap_or_else(|_| Local::now());
                
                let level = match parts[1] {
                    "Error" => LogLevel::Error,
                    "Warning" => LogLevel::Warning,
                    "Info" => LogLevel::Info,
                    "Debug" => LogLevel::Debug,
                    "Event" => LogLevel::Event,
                    _ => LogLevel::Info,
                };
                
                let source = if parts[2].is_empty() {
                    None
                } else {
                    Some(parts[2].to_string())
                };
                
                let indent = parts[3].parse().unwrap_or(0);
                let message = parts[4].to_string();
                
                let entry = LogEntry {
                    timestamp,
                    level,
                    message,
                    source,
                    indent,
                    show_time: true,
                    show_date: false,
                };
                
                // Add entry to buffer
                entries.push_back(entry.clone());
                self.write_entry(&entry);
            }
        }
        
        Ok(())
    }

    pub fn scroll_to_end(&self) {
        let n_rows = self.store.iter_n_children(None);
        if n_rows > 0 {
            let path = gtk::TreePath::from_indices(&[(n_rows - 1) as i32]);
            gtk::prelude::TreeViewExt::set_cursor(&self.widget, &path, None, false);
        }
    }

    pub fn append_entry(&self, entry: LogEntry) {
        let full_text = entry.get_formatted_text();
        let (fg_color, bg_color) = entry.level.get_colors();
        
        let iter = self.store.append();
        let icon_name = entry.level.icon_name();
        self.store.set_value(&iter, 0, &icon_name.to_value());
        self.store.set_value(&iter, 1, &full_text.to_value());
        self.store.set_value(&iter, 2, &fg_color.to_value());
        self.store.set_value(&iter, 3, &bg_color.to_value());
        
        if self.auto_scroll {
            self.scroll_to_end();
        }
    }
}

impl super::UIComponent for LogCtrl {
    fn get_widget(&self) -> &gtk::Widget {
        self.container.upcast_ref()
    }
}

impl LogEntry {
    fn get_formatted_text(&self) -> String {
        let mut text = String::new();
        
        // Add timestamp if enabled
        if self.show_time {
            if self.show_date {
                text.push_str(&self.timestamp.format("%Y-%m-%d %H:%M:%S").to_string());
            } else {
                text.push_str(&self.timestamp.format("%H:%M:%S").to_string());
            }
            text.push_str(" - ");
        }
        
        // Add source if available
        if let Some(source) = &self.source {
            text.push_str(source);
            text.push_str(": ");
        }
        
        // Add indentation if enabled
        if self.indent > 0 {
            for _ in 0..self.indent {
                text.push_str("    ");
            }
        }
        
        // Add message
        text.push_str(&self.message);
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_log_ctrl() {
        gtk::init().expect("Failed to initialize GTK");
        
        let log_ctrl = LogCtrl::new();
        
        // Test writing entries
        log_ctrl.write(LogEntry {
            timestamp: Local::now(),
            level: LogLevel::Info,
            message: "Test info message".to_string(),
            source: None,
            indent: 0,
            show_time: true,
            show_date: false,
        });
        
        log_ctrl.write(LogEntry {
            timestamp: Local::now(),
            level: LogLevel::Error,
            message: "Test error message".to_string(),
            source: None,
            indent: 1,
            show_time: true,
            show_date: false,
        });
        
        // Verify entries were stored
        assert_eq!(log_ctrl.entries.lock().unwrap().len(), 2);
    }
} 
