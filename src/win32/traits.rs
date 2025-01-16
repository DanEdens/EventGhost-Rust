use std::io;
use windows::Win32::Foundation::{HANDLE, BOOL, HWND, LPARAM, RECT};
use crate::win32::error::Error;

/// Trait for abstracting Windows handle-based resources
pub trait WindowsHandle: Send + Sync {
    fn as_raw_handle(&self) -> HANDLE;
    fn is_valid(&self) -> bool;
    fn close(&mut self) -> io::Result<()>;
}

/// Trait for abstracting named pipe operations
pub trait NamedPipeOps: WindowsHandle {
    fn connect(&mut self) -> Result<BOOL, Error>;
    fn disconnect(&mut self) -> Result<(), Error>;
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn wait_for_client(&mut self) -> Result<(), Error>;
}

/// Trait for creating named pipes
pub trait NamedPipeFactory: Send + Sync {
    type Pipe: NamedPipeOps;
    
    fn create_server(&self, name: &str) -> Result<Self::Pipe, Error>;
    fn connect_client(&self, name: &str) -> Result<Self::Pipe, Error>;
}

/// Window information structure
#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub handle: HWND,
    pub title: String,
    pub class_name: String,
    pub rect: RECT,
    pub visible: bool,
    pub enabled: bool,
    pub pid: u32,
}

/// Trait for window operations
pub trait WindowOps: Send + Sync {
    fn get_info(&self) -> Result<WindowInfo, Error>;
    fn set_foreground(&self) -> Result<(), Error>;
    fn show(&self) -> Result<(), Error>;
    fn hide(&self) -> Result<(), Error>;
    fn maximize(&self) -> Result<(), Error>;
    fn minimize(&self) -> Result<(), Error>;
    fn restore(&self) -> Result<(), Error>;
    fn move_window(&self, x: i32, y: i32, width: i32, height: i32) -> Result<(), Error>;
    fn send_message(&self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> Result<isize, Error>;
    fn post_message(&self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> Result<(), Error>;
}

/// Trait for window enumeration and finding
pub trait WindowManager: Send + Sync {
    type Window: WindowOps;

    fn find_window(&self, class_name: Option<&str>, window_name: Option<&str>) -> Result<Option<Self::Window>, Error>;
    fn find_window_ex(&self, parent: Option<HWND>, child: Option<HWND>, class: Option<&str>, title: Option<&str>) -> Result<Option<Self::Window>, Error>;
    fn enum_windows(&self) -> Result<Vec<Self::Window>, Error>;
    fn enum_child_windows(&self, parent: HWND) -> Result<Vec<Self::Window>, Error>;
    fn get_foreground_window(&self) -> Result<Option<Self::Window>, Error>;
    fn get_desktop_window(&self) -> Result<Self::Window, Error>;
}

/// Trait for window event hooks
pub trait WindowEventHook: Send + Sync {
    fn start(&mut self) -> Result<(), Error>;
    fn stop(&mut self) -> Result<(), Error>;
    fn is_active(&self) -> bool;
}

/// Callback types for window events
pub type WindowEventCallback = Box<dyn Fn(HWND, u32, WPARAM, LPARAM) + Send + Sync>;
pub type WindowEnumCallback = Box<dyn Fn(HWND) -> bool + Send + Sync>;

/// Trait for creating window hooks
pub trait WindowHookFactory: Send + Sync {
    type Hook: WindowEventHook;

    fn create_wnd_proc_hook(&self, callback: WindowEventCallback) -> Result<Self::Hook, Error>;
    fn create_shell_hook(&self, callback: WindowEventCallback) -> Result<Self::Hook, Error>;
    fn create_foreground_hook(&self, callback: WindowEventCallback) -> Result<Self::Hook, Error>;
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::collections::VecDeque;

    /// Mock pipe for testing
    pub struct MockPipe {
        handle: HANDLE,
        read_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
        write_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
        connected: bool,
    }

    impl MockPipe {
        pub fn new() -> Self {
            Self {
                handle: HANDLE(1), // Mock handle
                read_queue: Arc::new(Mutex::new(VecDeque::new())),
                write_queue: Arc::new(Mutex::new(VecDeque::new())),
                connected: false,
            }
        }

        pub fn push_read_data(&self, data: Vec<u8>) {
            self.read_queue.lock().unwrap().push_back(data);
        }

        pub fn get_written_data(&self) -> Option<Vec<u8>> {
            self.write_queue.lock().unwrap().pop_front()
        }
    }

    impl WindowsHandle for MockPipe {
        fn as_raw_handle(&self) -> HANDLE {
            self.handle
        }

        fn is_valid(&self) -> bool {
            self.handle.0 != 0
        }

        fn close(&mut self) -> io::Result<()> {
            self.handle = HANDLE(0);
            Ok(())
        }
    }

    impl NamedPipeOps for MockPipe {
        fn connect(&mut self) -> Result<BOOL, Error> {
            self.connected = true;
            Ok(BOOL(1))
        }

        fn disconnect(&mut self) -> Result<(), Error> {
            self.connected = false;
            Ok(())
        }

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
            if !self.connected {
                return Err(Error::Pipe(io::Error::new(
                    io::ErrorKind::NotConnected,
                    "Pipe not connected"
                )));
            }

            if let Some(data) = self.read_queue.lock().unwrap().pop_front() {
                let len = data.len().min(buf.len());
                buf[..len].copy_from_slice(&data[..len]);
                Ok(len)
            } else {
                Ok(0)
            }
        }

        fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
            if !self.connected {
                return Err(Error::Pipe(io::Error::new(
                    io::ErrorKind::NotConnected,
                    "Pipe not connected"
                )));
            }

            self.write_queue.lock().unwrap().push_back(buf.to_vec());
            Ok(buf.len())
        }

        fn wait_for_client(&mut self) -> Result<(), Error> {
            self.connected = true;
            Ok(())
        }
    }

    /// Mock factory for creating test pipes
    pub struct MockPipeFactory;

    impl NamedPipeFactory for MockPipeFactory {
        type Pipe = MockPipe;

        fn create_server(&self, _name: &str) -> Result<Self::Pipe, Error> {
            Ok(MockPipe::new())
        }

        fn connect_client(&self, _name: &str) -> Result<Self::Pipe, Error> {
            let mut pipe = MockPipe::new();
            pipe.connect()?;
            Ok(pipe)
        }
    }
} 