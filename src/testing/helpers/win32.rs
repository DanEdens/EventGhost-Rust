use std::sync::Arc;
use windows::Win32::Foundation::{HWND, WPARAM, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::testing::mocks::win32::{
    MockWindow, MockWindowManager, MockHookFactory,
    MockWindowHook,
};

/// Helper to create a test window environment
pub struct TestWindowEnv {
    pub manager: MockWindowManager,
    pub hook_factory: MockHookFactory,
}

impl TestWindowEnv {
    pub fn new() -> Self {
        Self {
            manager: MockWindowManager::new(),
            hook_factory: MockHookFactory::new(),
        }
    }

    /// Create a test window with a title
    pub fn create_window(&self, title: &str) -> MockWindow {
        self.manager.add_window(title, "TestClass")
    }

    /// Create multiple test windows
    pub fn create_windows(&self, titles: &[&str]) -> Vec<MockWindow> {
        titles.iter()
            .map(|title| self.create_window(title))
            .collect()
    }

    /// Set up a window hook and return it
    pub fn setup_hook<F>(&self, callback: F) -> Result<MockWindowHook, crate::win32::error::Error>
    where
        F: Fn(HWND, u32, WPARAM, LPARAM) + Send + Sync + 'static
    {
        self.hook_factory.create_wnd_proc_hook(Box::new(callback))
    }
}

impl Default for TestWindowEnv {
    fn default() -> Self {
        Self::new()
    }
}

/// Common window messages for testing
pub mod messages {
    use super::*;

    pub const WM_TEST_CUSTOM: u32 = WM_USER + 1000;

    pub struct TestMessage {
        pub msg: u32,
        pub wparam: WPARAM,
        pub lparam: LPARAM,
    }

    impl TestMessage {
        pub fn new(msg: u32) -> Self {
            Self {
                msg,
                wparam: WPARAM(0),
                lparam: LPARAM(0),
            }
        }

        pub fn with_params(msg: u32, wparam: isize, lparam: isize) -> Self {
            Self {
                msg,
                wparam: WPARAM(wparam),
                lparam: LPARAM(lparam),
            }
        }
    }
}

/// Assertions for window testing
pub mod assertions {
    use super::*;
    use crate::win32::traits::WindowInfo;

    pub fn assert_window_visible(window: &MockWindow) {
        let info = window.get_info().unwrap();
        assert!(info.visible, "Window should be visible");
    }

    pub fn assert_window_hidden(window: &MockWindow) {
        let info = window.get_info().unwrap();
        assert!(!info.visible, "Window should be hidden");
    }

    pub fn assert_window_position(window: &MockWindow, x: i32, y: i32, width: i32, height: i32) {
        let info = window.get_info().unwrap();
        assert_eq!(info.rect.left, x, "Window x position mismatch");
        assert_eq!(info.rect.top, y, "Window y position mismatch");
        assert_eq!(info.rect.right - info.rect.left, width, "Window width mismatch");
        assert_eq!(info.rect.bottom - info.rect.top, height, "Window height mismatch");
    }

    pub fn assert_received_message(window: &MockWindow, expected_msg: u32) {
        if let Some((msg, _, _)) = window.get_last_message() {
            assert_eq!(msg, expected_msg, "Unexpected message received");
        } else {
            panic!("No message received");
        }
    }

    pub fn assert_received_message_with_params(
        window: &MockWindow,
        expected_msg: u32,
        expected_wparam: isize,
        expected_lparam: isize
    ) {
        if let Some((msg, wparam, lparam)) = window.get_last_message() {
            assert_eq!(msg, expected_msg, "Unexpected message received");
            assert_eq!(wparam.0, expected_wparam, "Unexpected wparam");
            assert_eq!(lparam.0, expected_lparam, "Unexpected lparam");
        } else {
            panic!("No message received");
        }
    }
} 
