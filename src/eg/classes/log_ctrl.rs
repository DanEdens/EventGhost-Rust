use gtk::prelude::*;
use gtk::{self, TextView, TextBuffer, TextTag, TextTagTable};
use glib;
use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Local>,
    pub level: LogLevel,
    pub message: String,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

pub struct LogCtrl {
    pub widget: TextView,
    buffer: TextBuffer,
    show_time: bool,
    show_date: bool,
    indent: bool,
}

impl LogCtrl {
    pub fn new() -> Self {
        // Create text tag table and buffer
        let tag_table = TextTagTable::new();
        
        // Create tags for different message types
        let error_tag = TextTag::builder()
            .name("error")
            .foreground("red")
            .build();
        tag_table.add(&error_tag);
        
        let warning_tag = TextTag::builder()
            .name("warning")
            .foreground("orange")
            .build();
        tag_table.add(&warning_tag);
        
        let info_tag = TextTag::builder()
            .name("info")
            .foreground("blue")
            .build();
        tag_table.add(&info_tag);
        
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
            
        // Enable scrolling
        widget.set_wrap_mode(gtk::WrapMode::Word);
        
        LogCtrl {
            widget,
            buffer,
            show_time: true,
            show_date: false,
            indent: true,
        }
    }
    
    pub fn write(&self, text: &str, level: LogLevel) {
        let mut prefix = String::new();
        
        // Add timestamp if enabled
        if self.show_time || self.show_date {
            let now = Local::now();
            if self.show_date {
                prefix.push_str(&now.format("%Y-%m-%d ").to_string());
            }
            if self.show_time {
                prefix.push_str(&now.format("%H:%M:%S ").to_string());
            }
        }
        
        // Add indentation if enabled
        if self.indent {
            prefix.push_str("  ");
        }
        
        let full_text = format!("{}{}\n", prefix, text);
        
        // Get end iterator
        let mut end_iter = self.buffer.end_iter();
        
        // Insert text with appropriate tag
        let tag_name = match level {
            LogLevel::Error => Some("error"),
            LogLevel::Warning => Some("warning"),
            LogLevel::Info => Some("info"),
            LogLevel::Debug => None,
        };
        
        if let Some(tag_name) = tag_name {
            if let Some(tag) = self.buffer.tag_table().lookup(tag_name) {
                self.buffer.insert_with_tags(&mut end_iter, &full_text, &[&tag]);
            }
        } else {
            self.buffer.insert(&mut end_iter, &full_text);
        }
        
        // Scroll to end - Fixed to use mutable iterator
        let mut end_iter = self.buffer.end_iter();
        self.widget.scroll_to_iter(&mut end_iter, 0.0, false, 0.0, 0.0);
    }
    
    pub fn clear(&self) {
        self.buffer.set_text("");
    }
    
    pub fn set_time_logging(&mut self, enabled: bool) {
        self.show_time = enabled;
    }
    
    pub fn set_date_logging(&mut self, enabled: bool) {
        self.show_date = enabled;
    }
    
    pub fn set_indent(&mut self, enabled: bool) {
        self.indent = enabled;
    }
} 