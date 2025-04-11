pub const CORE_PLUGIN_GUIDS: &[&str] = &[
    "{9D499A2C-72B6-40B0-8C8C-995831B10BB4}",  // "EventGhost"
    "{A21F443B-221D-44E4-8596-E1ED7100E0A4}",  // "System"
    "{E974D074-B0A3-4D0C-BBD1-992475DDD69D}",  // "Window"
    "{6B1751BF-F94E-4260-AB7E-64C0693FD959}",  // "Mouse"
];

pub const DEFAULT_DEBUG_LEVEL: i32 = 0;
pub const DEFAULT_ENCODING: &str = "utf-8";
pub const DEFAULT_PIPE_NAME: &str = r"\\.\pipe\EventGhost";
pub const DEFAULT_CONFIG_PATH: &str = "config.toml";

// Window message constants
pub const WM_TRAY_NOTIFY: u32 = 0x0401;
pub const WM_TASKBAR_CREATED: u32 = 0x0402; 
