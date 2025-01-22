use gtk::prelude::*;
use gtk::{self, TextView, TextBuffer, TextTag, TextTagTable, ScrolledWindow};
use gdk;
use glib;
use chrono::{DateTime, Local};
use std::collections::VecDeque;
use std::sync::{Mutex, Arc};
use std::fs::File;
use std::io::{self, Write, BufRead};
use std::fmt;
use std::cell::Cell;

const MAX_BUFFER_SIZE: usize = 2000;
const REMOVE_ON_MAX: usize = 200;

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Local>,
    pub level: LogLevel,
    pub message: String,
    pub source: Option<String>,
    pub indent: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Event,
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
    pub widget: TextView,
    pub buffer: TextBuffer,
    pub entries: Arc<Mutex<VecDeque<LogEntry>>>,
    pub show_time: bool,
    pub show_date: bool,
    pub indent: bool,
    is_odd: Cell<bool>,
}

impl Clone for LogCtrl {
    fn clone(&self) -> Self {
        LogCtrl {
            container: ScrolledWindow::builder()
                .child(&self.widget)
                .build(),
            widget: TextView::builder()
                .buffer(&self.buffer)
                .editable(false)
                .monospace(true)
                .build(),
            buffer: self.buffer.clone(),
            entries: self.entries.clone(),
            show_time: self.show_time,
            show_date: self.show_date,
            indent: self.indent,
            is_odd: Cell::new(self.is_odd.get()),
        }
    }
}

impl LogCtrl {
    pub fn new() -> Self {
        // Create text tag table and buffer
        let tag_table = TextTagTable::new();
        
        // Create tags for different message types with alternating backgrounds
        let error_tag_odd = TextTag::builder()
            .name("error_odd")
            .foreground("red")
            .background("#FFE8E8")
            .build();
        tag_table.add(&error_tag_odd);
        
        let error_tag_even = TextTag::builder()
            .name("error_even")
            .foreground("red")
            .background("#FFE0E0")
            .build();
        tag_table.add(&error_tag_even);
        
        let warning_tag_odd = TextTag::builder()
            .name("warning_odd") 
            .foreground("#C04000")
            .background("#FFFFF0")
            .build();
        tag_table.add(&warning_tag_odd);
        
        let warning_tag_even = TextTag::builder()
            .name("warning_even") 
            .foreground("#C04000")
            .background("#FFFFD0")
            .build();
        tag_table.add(&warning_tag_even);
        
        let info_tag_odd = TextTag::builder()
            .name("info_odd")
            .foreground("blue")
            .background("#F8F8F8")
            .build();
        tag_table.add(&info_tag_odd);
        
        let info_tag_even = TextTag::builder()
            .name("info_even")
            .foreground("blue")
            .background("#FFFFFF")
            .build();
        tag_table.add(&info_tag_even);

        let event_tag_odd = TextTag::builder()
            .name("event_odd")
            .foreground("green")
            .background("#F8F8F8")
            .build();
        tag_table.add(&event_tag_odd);
        
        let event_tag_even = TextTag::builder()
            .name("event_even")
            .foreground("green")
            .background("#FFFFFF")
            .build();
        tag_table.add(&event_tag_even);
        
        let debug_tag_odd = TextTag::builder()
            .name("debug_odd")
            .foreground("gray")
            .background("#F8F8F8")
            .build();
        tag_table.add(&debug_tag_odd);
        
        let debug_tag_even = TextTag::builder()
            .name("debug_even")
            .foreground("gray")
            .background("#FFFFFF")
            .build();
        tag_table.add(&debug_tag_even);
            
        // Create buffer with tags
        let buffer = TextBuffer::builder()
            .tag_table(&tag_table)
            .build();
            
        // Create text view
        let widget = TextView::builder()
            .buffer(&buffer)
            .editable(false)
            .monospace(true)
            .build();

        // Create scrolled window container
        let container = ScrolledWindow::builder()
            .child(&widget)
            .build();
            
        // Enable scrolling
        widget.set_wrap_mode(gtk::WrapMode::Word);

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
                popover.set_pointing_to(Some(&gdk::Rectangle {
                    x: x as i32,
                    y: y as i32,
                    width: 1,
                    height: 1,
                }));
                popover.popup();
            }
        }));
        widget.add_controller(gesture);
        
        // Create the LogCtrl instance
        let log_ctrl = LogCtrl {
            container,
            widget: widget.clone(),
            buffer,
            entries: Arc::new(Mutex::new(VecDeque::with_capacity(MAX_BUFFER_SIZE))),
            show_time: true,
            show_date: false,
            indent: true,
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
        
        widget.insert_action_group("log", Some(&action_group));
        
        // Add a welcome message
        log_ctrl.write(LogEntry {
            timestamp: Local::now(),
            level: LogLevel::Info,
            message: "Welcome to EventGhost".to_string(),
            source: None,
            indent: 0,
        });

        log_ctrl
    }
    
    pub fn write(&self, entry: LogEntry) {
        let mut entries = self.entries.lock().unwrap();
        
        // Handle circular buffer
        if entries.len() >= MAX_BUFFER_SIZE {
            for _ in 0..REMOVE_ON_MAX {
                entries.pop_front();
            }
        }
        
        // Add the new entry
        entries.push_back(entry.clone());
        
        // Write the new entry to the buffer
        self.write_entry(&entry);
        
        // Toggle odd/even state safely using Cell
        self.is_odd.set(!self.is_odd.get());
        
        // Scroll to end
        let mut end_iter = self.buffer.end_iter();
        self.widget.scroll_to_iter(&mut end_iter, 0.0, false, 0.0, 0.0);
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
        
        let full_text = format!("{}{}\n", prefix, entry.message);
        
        // Get end iterator
        let mut end_iter = self.buffer.end_iter();
        
        // Insert text with appropriate tag
        let tag_name = match entry.level {
            LogLevel::Error => Some(if self.is_odd.get() { "error_odd" } else { "error_even" }),
            LogLevel::Warning => Some(if self.is_odd.get() { "warning_odd" } else { "warning_even" }),
            LogLevel::Info => Some(if self.is_odd.get() { "info_odd" } else { "info_even" }),
            LogLevel::Event => Some(if self.is_odd.get() { "event_odd" } else { "event_even" }),
            LogLevel::Debug => Some(if self.is_odd.get() { "debug_odd" } else { "debug_even" }),
        };
        
        if let Some(tag_name) = tag_name {
            if let Some(tag) = self.buffer.tag_table().lookup(tag_name) {
                self.buffer.insert_with_tags(&mut end_iter, &full_text, &[&tag]);
            }
        } else {
            self.buffer.insert(&mut end_iter, &full_text);
        }
    }
    
    pub fn clear(&self) {
        self.entries.lock().unwrap().clear();
        self.buffer.set_text("");
    }

    pub fn select_all(&self) {
        let start = self.buffer.start_iter();
        let end = self.buffer.end_iter();
        self.buffer.select_range(&start, &end);
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
        // Clear buffer
        self.buffer.set_text("");
        
        // Rewrite all entries with new formatting
        let entries = self.entries.lock().unwrap();
        for entry in entries.iter() {
            self.write_entry(entry);
        }
    }

    pub fn filter_logs(&self, level: Option<LogLevel>) {
        let entries = self.entries.lock().unwrap();
        self.buffer.set_text(""); // Clear current buffer
        self.is_odd.set(false); // Reset alternating colors
        
        for entry in entries.iter() {
            if level.map_or(true, |l| entry.level == l) {
                self.write_entry(entry);
                self.is_odd.set(!self.is_odd.get());
            }
        }
    }

    pub fn search_logs(&self, query: &str, case_sensitive: bool) {
        let entries = self.entries.lock().unwrap();
        self.buffer.set_text(""); // Clear current buffer
        self.is_odd.set(false); // Reset alternating colors
        
        let query = if !case_sensitive {
            query.to_lowercase()
        } else {
            query.to_string()
        };
        
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

    pub fn copy_selected_text(&self) -> Option<String> {
        if let Some((start, end)) = self.buffer.selection_bounds() {
            if let Some(text) = self.buffer.text(&start, &end, false) {
                if let Some(display) = self.widget.display() {
                    display.clipboard().set_text(&text);
                    return Some(text.to_string());
                }
            }
        }
        None
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
        self.buffer.set_text("");
        
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
                };
                
                // Add entry to buffer
                entries.push_back(entry.clone());
                self.write_entry(&entry);
            }
        }
        
        Ok(())
    }
}

impl super::UIComponent for LogCtrl {
    fn get_widget(&self) -> &gtk::Widget {
        self.container.upcast_ref()
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
        });
        
        log_ctrl.write(LogEntry {
            timestamp: Local::now(),
            level: LogLevel::Error,
            message: "Test error message".to_string(),
            source: None,
            indent: 1,
        });
        
        // Verify entries were stored
        assert_eq!(log_ctrl.entries.lock().unwrap().len(), 2);
    }
} 