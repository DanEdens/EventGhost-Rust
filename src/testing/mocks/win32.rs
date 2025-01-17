use std::sync::{Arc, Mutex};
use std::collections::{HashMap, VecDeque};
use windows::Win32::Foundation::{HWND, BOOL, LPARAM, WPARAM, RECT};
use crate::win32::{
    error::{Error, window::WindowError},
    traits::{
        WindowInfo, WindowOps, WindowManager,
        WindowEventHook, WindowEventCallback, WindowHookFactory,
    },
};

/// Mock window for testing
#[derive(Clone)]
pub struct MockWindow {
    info: Arc<Mutex<WindowInfo>>,
    messages: Arc<Mutex<VecDeque<(u32, WPARAM, LPARAM)>>>,
}

impl MockWindow {
    pub fn new(title: &str, class_name: &str) -> Self {
        let info = WindowInfo {
            handle: HWND(1), // Mock handle
            title: title.to_string(),
            class_name: class_name.to_string(),
            rect: RECT { left: 0, top: 0, right: 100, bottom: 100 },
            visible: true,
            enabled: true,
            pid: 1234,
        };

        Self {
            info: Arc::new(Mutex::new(info)),
            messages: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Get the last message sent to this window
    pub fn get_last_message(&self) -> Option<(u32, WPARAM, LPARAM)> {
        self.messages.lock().unwrap().pop_front()
    }

    /// Simulate window state changes
    pub fn set_visibility(&self, visible: bool) {
        self.info.lock().unwrap().visible = visible;
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.info.lock().unwrap().enabled = enabled;
    }

    pub fn move_to(&self, x: i32, y: i32, width: i32, height: i32) {
        let mut info = self.info.lock().unwrap();
        info.rect.left = x;
        info.rect.top = y;
        info.rect.right = x + width;
        info.rect.bottom = y + height;
    }
}

impl WindowOps for MockWindow {
    fn get_info(&self) -> Result<WindowInfo, Error> {
        Ok(self.info.lock().unwrap().clone())
    }

    fn set_foreground(&self) -> Result<(), Error> {
        // Simulate foreground window change
        Ok(())
    }

    fn show(&self) -> Result<(), Error> {
        self.set_visibility(true);
        Ok(())
    }

    fn hide(&self) -> Result<(), Error> {
        self.set_visibility(false);
        Ok(())
    }

    fn maximize(&self) -> Result<(), Error> {
        self.move_to(0, 0, 1920, 1080); // Simulate maximized state
        Ok(())
    }

    fn minimize(&self) -> Result<(), Error> {
        self.move_to(-32000, -32000, 100, 100); // Simulate minimized state
        Ok(())
    }

    fn restore(&self) -> Result<(), Error> {
        self.move_to(100, 100, 800, 600); // Simulate restored state
        Ok(())
    }

    fn move_window(&self, x: i32, y: i32, width: i32, height: i32) -> Result<(), Error> {
        self.move_to(x, y, width, height);
        Ok(())
    }

    fn send_message(&self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> Result<isize, Error> {
        self.messages.lock().unwrap().push_back((msg, wparam, lparam));
        Ok(0)
    }

    fn post_message(&self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> Result<(), Error> {
        self.messages.lock().unwrap().push_back((msg, wparam, lparam));
        Ok(())
    }
}

/// Mock window manager for testing
pub struct MockWindowManager {
    windows: Arc<Mutex<HashMap<String, MockWindow>>>,
    foreground: Arc<Mutex<Option<MockWindow>>>,
}

impl MockWindowManager {
    pub fn new() -> Self {
        Self {
            windows: Arc::new(Mutex::new(HashMap::new())),
            foreground: Arc::new(Mutex::new(None)),
        }
    }

    /// Add a window to the mock system
    pub fn add_window(&self, title: &str, class_name: &str) -> MockWindow {
        let window = MockWindow::new(title, class_name);
        self.windows.lock().unwrap().insert(title.to_string(), window.clone());
        window
    }

    /// Set the foreground window
    pub fn set_foreground(&self, window: Option<MockWindow>) {
        *self.foreground.lock().unwrap() = window;
    }
}

impl WindowManager for MockWindowManager {
    type Window = MockWindow;

    fn find_window(&self, class_name: Option<&str>, window_name: Option<&str>) -> Result<Option<Self::Window>, Error> {
        let windows = self.windows.lock().unwrap();
        
        if let Some(name) = window_name {
            Ok(windows.get(name).cloned())
        } else if let Some(class) = class_name {
            Ok(windows.values()
                .find(|w| w.info.lock().unwrap().class_name == class)
                .cloned())
        } else {
            Ok(None)
        }
    }

    fn find_window_ex(&self, _parent: Option<HWND>, _child: Option<HWND>, class: Option<&str>, title: Option<&str>) -> Result<Option<Self::Window>, Error> {
        self.find_window(class, title)
    }

    fn enum_windows(&self) -> Result<Vec<Self::Window>, Error> {
        Ok(self.windows.lock().unwrap().values().cloned().collect())
    }

    fn enum_child_windows(&self, _parent: HWND) -> Result<Vec<Self::Window>, Error> {
        Ok(Vec::new()) // Mock implementation doesn't support child windows
    }

    fn get_foreground_window(&self) -> Result<Option<Self::Window>, Error> {
        Ok(self.foreground.lock().unwrap().clone())
    }

    fn get_desktop_window(&self) -> Result<Self::Window, Error> {
        Ok(MockWindow::new("Desktop", "Desktop"))
    }
}

/// Mock window hook for testing
pub struct MockWindowHook {
    active: Arc<Mutex<bool>>,
    callback: Arc<WindowEventCallback>,
}

impl MockWindowHook {
    fn new(callback: WindowEventCallback) -> Self {
        Self {
            active: Arc::new(Mutex::new(false)),
            callback: Arc::new(callback),
        }
    }

    /// Simulate a window event
    pub fn simulate_event(&self, hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) {
        if *self.active.lock().unwrap() {
            (self.callback)(hwnd, msg, wparam, lparam);
        }
    }
}

impl WindowEventHook for MockWindowHook {
    fn start(&mut self) -> Result<(), Error> {
        *self.active.lock().unwrap() = true;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Error> {
        *self.active.lock().unwrap() = false;
        Ok(())
    }

    fn is_active(&self) -> bool {
        *self.active.lock().unwrap()
    }
}

/// Mock hook factory for testing
pub struct MockHookFactory;

impl MockHookFactory {
    pub fn new() -> Self {
        Self
    }
}

impl WindowHookFactory for MockHookFactory {
    type Hook = MockWindowHook;

    fn create_wnd_proc_hook(&self, callback: WindowEventCallback) -> Result<Self::Hook, Error> {
        Ok(MockWindowHook::new(callback))
    }

    fn create_shell_hook(&self, callback: WindowEventCallback) -> Result<Self::Hook, Error> {
        Ok(MockWindowHook::new(callback))
    }

    fn create_foreground_hook(&self, callback: WindowEventCallback) -> Result<Self::Hook, Error> {
        Ok(MockWindowHook::new(callback))
    }
} 