use super::*;
use crate::testing::win32::{
    TestWindowEnv,
    messages::{TestMessage, WM_TEST_CUSTOM},
    assertions::*,
};
use windows::Win32::UI::WindowsAndMessaging::*;

#[tokio::test]
async fn test_window_basic_operations() {
    let env = TestWindowEnv::new();
    let window = env.create_window("Test Window");

    // Test visibility
    assert_window_visible(&window);
    window.hide().unwrap();
    assert_window_hidden(&window);
    window.show().unwrap();
    assert_window_visible(&window);

    // Test window position
    window.move_window(100, 100, 800, 600).unwrap();
    assert_window_position(&window, 100, 100, 800, 600);

    // Test window state
    window.maximize().unwrap();
    assert_window_position(&window, 0, 0, 1920, 1080);
    
    window.restore().unwrap();
    assert_window_position(&window, 100, 100, 800, 600);
}

#[tokio::test]
async fn test_window_messaging() {
    let env = TestWindowEnv::new();
    let window = env.create_window("Test Window");

    // Test basic message
    let msg = TestMessage::new(WM_TEST_CUSTOM);
    window.send_message(msg.msg, msg.wparam, msg.lparam).unwrap();
    assert_received_message(&window, WM_TEST_CUSTOM);

    // Test message with parameters
    let msg = TestMessage::with_params(WM_SETTEXT, 42, 123);
    window.post_message(msg.msg, msg.wparam, msg.lparam).unwrap();
    assert_received_message_with_params(&window, WM_SETTEXT, 42, 123);
}

#[tokio::test]
async fn test_window_manager() {
    let env = TestWindowEnv::new();
    
    // Create test windows
    let windows = env.create_windows(&[
        "Window 1",
        "Window 2",
        "Window 3"
    ]);

    // Test window finding
    let found = env.manager.find_window(None, Some("Window 2")).unwrap();
    assert!(found.is_some());
    assert_eq!(
        found.unwrap().get_info().unwrap().title,
        "Window 2"
    );

    // Test window enumeration
    let all_windows = env.manager.enum_windows().unwrap();
    assert_eq!(all_windows.len(), 3);

    // Test foreground window
    assert!(env.manager.get_foreground_window().unwrap().is_none());
    env.manager.set_foreground(Some(windows[0].clone()));
    assert!(env.manager.get_foreground_window().unwrap().is_some());
}

#[tokio::test]
async fn test_window_hooks() {
    let env = TestWindowEnv::new();
    let window = env.create_window("Test Window");

    // Create a hook to track window messages
    let received = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let received_clone = received.clone();

    let mut hook = env.setup_hook(move |hwnd, msg, wparam, lparam| {
        if msg == WM_TEST_CUSTOM {
            received_clone.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }).unwrap();

    // Start the hook
    hook.start().unwrap();
    assert!(hook.is_active());

    // Simulate window message
    let msg = TestMessage::new(WM_TEST_CUSTOM);
    window.send_message(msg.msg, msg.wparam, msg.lparam).unwrap();

    // Verify hook received message
    assert!(received.load(std::sync::atomic::Ordering::SeqCst));

    // Stop the hook
    hook.stop().unwrap();
    assert!(!hook.is_active());
} 