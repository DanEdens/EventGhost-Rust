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
        }
    }
}

impl LogCtrl {
    pub fn new() -> Self {
        // Create text tag table and buffer
        let tag_table = TextTagTable::new();
        
        // Create tags for different message types
        let error_tag = TextTag::builder()
            .name("error")
            .foreground("red")
            .background("#FFE0E0")
            .build();
        tag_table.add(&error_tag);
        
        let warning_tag = TextTag::builder()
            .name("warning") 
            .foreground("#C04000")
            .background("#FFFFD0")
            .build();
        tag_table.add(&warning_tag);
        
        let info_tag = TextTag::builder()
            .name("info")
            .foreground("blue")
            .build();
        tag_table.add(&info_tag);

        let event_tag = TextTag::builder()
            .name("event")
            .foreground("green")
            .build();
        tag_table.add(&event_tag);
            
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
        let copy_item = gio::MenuItem::new(Some("Copy"), Some("log.copy"));
        let select_all_item = gio::MenuItem::new(Some("Select All"), Some("log.select-all"));
        let clear_item = gio::MenuItem::new(Some("Clear Log"), Some("log.clear"));

        menu.append_item(&copy_item);
        menu.append_item(&select_all_item);
        menu.append_item(&clear_item);

        // let popover = PopoverMenu::from_model(Some(&menu));

        // // Add context menu controller
        // let gesture = gtk::GestureClick::new();
        // gesture.set_button(3); // Right click
        // gesture.connect_pressed(glib::clone!(@weak popover, @weak widget => move |gesture, _, x, y| {
        //     if gesture.current_button() == 3 {
        //         popover.set_parent(&widget);
        //         popover.set_pointing_to(Some(&gdk::Rectangle::new(
        //             x as i32,
        //             y as i32,
        //             1,
        //             1
        //         )));
        //         popover.popup();
        //     }
        // }));
        // widget.add_controller(gesture);
        
        // Create the LogCtrl instance
        let log_ctrl = LogCtrl {
            container,
            widget,
            buffer,
            entries: Arc::new(Mutex::new(VecDeque::with_capacity(MAX_BUFFER_SIZE))),
            show_time: true,
            show_date: false,
            indent: true,
        };

        // Add actions for menu items
        // let action_group = gio::SimpleActionGroup::new();
        
        
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
            LogLevel::Error => Some("error"),
            LogLevel::Warning => Some("warning"),
            LogLevel::Info => Some("info"),
            LogLevel::Event => Some("event"),
            LogLevel::Debug => None,
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

    // pub fn copy_selected_text(&self) {
    //     if let Some((start, end)) = self.buffer.selection_bounds() {
    //         let text = self.buffer.text(&start, &end, false);
    //         let display = self.widget.display();
    //         let clipboard = display.clipboard();
    //         clipboard.set_text(&text);
    //     }
    // }

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

    // pub fn filter_logs(&self, level: LogLevel) {
    //     let entries = self.entries.lock().unwrap();
    //     self.buffer.set_text(""); // Clear current buffer
    //     for entry in entries.iter() {
    //         if entry.level == level {
    //             self.write_entry(entry);
    //         }
    //     }
    // }

    // pub fn save_logs(&self, file_path: &str) -> io::Result<()> {
    //     let entries = self.entries.lock().unwrap();
    //     let mut file = File::create(file_path)?;
    //     for entry in entries.iter() {
    //         writeln!(file, "{} - {}: {}", entry.timestamp, entry.level, entry.message)?;
    //     }
    //     Ok(())
    // }

    // pub fn load_logs(&self, file_path: &str) -> io::Result<()> {
    //     let file = File::open(file_path)?;
    //     let reader = io::BufReader::new(file);
    //     let mut entries = self.entries.lock().unwrap();
        
    //     for line in reader.lines() {
    //         let line = line?;
    //         // Assuming the log format is "timestamp - level: message"
    //         let parts: Vec<&str> = line.split(" - ").collect();
    //         if parts.len() == 2 {
    //             let timestamp = parts[0].to_string(); // Parse timestamp
    //             let level = parts[1].split(":").next().unwrap(); // Parse level
    //             let message = parts[1].split(": ").nth(1).unwrap().to_string(); // Parse message
                
    //             // Create LogEntry and push to entries
    //             let entry = LogEntry {
    //                 timestamp: Local::now(), // Replace with actual parsed timestamp
    //                 level: match level {
    //                     "Error" => LogLevel::Error,
    //                     "Warning" => LogLevel::Warning,
    //                     "Info" => LogLevel::Info,
    //                     "Debug" => LogLevel::Debug,
    //                     "Event" => LogLevel::Event,
    //                     _ => LogLevel::Info, // Default level
    //                 },
    //                 message,
    //                 source: None,
    //                 indent: 0,
    //             };
    //             entries.push_back(entry);
    //         }
    //     }
    //     Ok(())
    // }

    // pub fn search_logs(&self, query: &str) {
    //     let entries = self.entries.lock().unwrap();
    //     self.buffer.set_text(""); // Clear current buffer
    //     for entry in entries.iter() {
    //         if entry.message.contains(query) {
    //             self.write_entry(entry);
    //         }
    //     }
    // }
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