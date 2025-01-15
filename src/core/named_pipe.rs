use std::io::{self, Read, Write};
use std::sync::Arc;
use tokio::sync::Mutex;
use windows::Win32::Foundation::*;
use windows::Win32::System::Pipes::*;
use windows::core::*;
use crate::core::Error;

const PIPE_BUFFER_SIZE: u32 = 4096;
const PIPE_TIMEOUT: u32 = 5000; // 5 seconds

/// Error type for named pipe operations
#[derive(Debug)]
pub enum PipeError {
    /// Failed to create pipe
    Creation(String),
    /// Failed to connect to pipe
    Connection(String),
    /// Failed to read from pipe
    Read(String),
    /// Failed to write to pipe
    Write(String),
    /// Pipe timeout
    Timeout(String),
}

impl std::fmt::Display for PipeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipeError::Creation(msg) => write!(f, "Pipe creation failed: {}", msg),
            PipeError::Connection(msg) => write!(f, "Pipe connection failed: {}", msg),
            PipeError::Read(msg) => write!(f, "Pipe read failed: {}", msg),
            PipeError::Write(msg) => write!(f, "Pipe write failed: {}", msg),
            PipeError::Timeout(msg) => write!(f, "Pipe timeout: {}", msg),
        }
    }
}

impl std::error::Error for PipeError {}

impl From<PipeError> for Error {
    fn from(err: PipeError) -> Self {
        Error::Pipe(err.to_string())
    }
}

/// A named pipe server that can accept multiple client connections
pub struct NamedPipeServer {
    pipe_name: String,
    handle: Arc<Mutex<HANDLE>>,
}

impl NamedPipeServer {
    /// Create a new named pipe server
    pub fn new(name: &str) -> Result<Self, PipeError> {
        let pipe_name = format!(r"\\.\pipe\{}", name);
        
        unsafe {
            let handle = CreateNamedPipeA(
                PCSTR::from_raw(pipe_name.as_ptr()),
                PIPE_ACCESS_DUPLEX | FILE_FLAG_OVERLAPPED,
                PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
                PIPE_UNLIMITED_INSTANCES,
                PIPE_BUFFER_SIZE,
                PIPE_BUFFER_SIZE,
                PIPE_TIMEOUT,
                None,
            );

            if handle.is_invalid() {
                return Err(PipeError::Creation(format!(
                    "Failed to create pipe: {}",
                    io::Error::last_os_error()
                )));
            }

            Ok(Self {
                pipe_name,
                handle: Arc::new(Mutex::new(handle)),
            })
        }
    }

    /// Wait for a client to connect
    pub async fn wait_for_client(&self) -> Result<NamedPipeConnection, PipeError> {
        let mut handle = self.handle.lock().await;
        
        unsafe {
            if !ConnectNamedPipe(*handle, None).as_bool() {
                let error = io::Error::last_os_error();
                if error.raw_os_error() != Some(ERROR_PIPE_CONNECTED.0 as i32) {
                    return Err(PipeError::Connection(error.to_string()));
                }
            }
        }

        Ok(NamedPipeConnection {
            handle: *handle,
        })
    }

    /// Create a new instance of the pipe for another client
    pub async fn create_instance(&self) -> Result<Self, PipeError> {
        Self::new(&self.pipe_name)
    }
}

/// A connection to a named pipe
pub struct NamedPipeConnection {
    handle: HANDLE,
}

impl NamedPipeConnection {
    /// Read a message from the pipe
    pub fn read(&mut self) -> Result<Vec<u8>, PipeError> {
        let mut buffer = vec![0u8; PIPE_BUFFER_SIZE as usize];
        let mut bytes_read = 0;

        unsafe {
            if !ReadFile(
                self.handle,
                buffer.as_mut_ptr() as *mut _,
                PIPE_BUFFER_SIZE,
                Some(&mut bytes_read),
                None,
            ).as_bool() {
                return Err(PipeError::Read(
                    io::Error::last_os_error().to_string()
                ));
            }
        }

        buffer.truncate(bytes_read as usize);
        Ok(buffer)
    }

    /// Write a message to the pipe
    pub fn write(&mut self, data: &[u8]) -> Result<(), PipeError> {
        let mut bytes_written = 0;

        unsafe {
            if !WriteFile(
                self.handle,
                data.as_ptr() as *const _,
                data.len() as u32,
                Some(&mut bytes_written),
                None,
            ).as_bool() {
                return Err(PipeError::Write(
                    io::Error::last_os_error().to_string()
                ));
            }
        }

        Ok(())
    }
}

impl Drop for NamedPipeConnection {
    fn drop(&mut self) {
        unsafe {
            DisconnectNamedPipe(self.handle);
            CloseHandle(self.handle);
        }
    }
}

/// A client connection to a named pipe
pub struct NamedPipeClient {
    handle: HANDLE,
}

impl NamedPipeClient {
    /// Connect to a named pipe server
    pub fn connect(name: &str) -> Result<Self, PipeError> {
        let pipe_name = format!(r"\\.\pipe\{}", name);
        
        unsafe {
            let handle = CreateFileA(
                PCSTR::from_raw(pipe_name.as_ptr()),
                FILE_GENERIC_READ | FILE_GENERIC_WRITE,
                FILE_SHARE_NONE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            );

            if handle.is_invalid() {
                return Err(PipeError::Connection(format!(
                    "Failed to connect to pipe: {}",
                    io::Error::last_os_error()
                )));
            }

            Ok(Self { handle })
        }
    }

    /// Read a message from the pipe
    pub fn read(&mut self) -> Result<Vec<u8>, PipeError> {
        let mut buffer = vec![0u8; PIPE_BUFFER_SIZE as usize];
        let mut bytes_read = 0;

        unsafe {
            if !ReadFile(
                self.handle,
                buffer.as_mut_ptr() as *mut _,
                PIPE_BUFFER_SIZE,
                Some(&mut bytes_read),
                None,
            ).as_bool() {
                return Err(PipeError::Read(
                    io::Error::last_os_error().to_string()
                ));
            }
        }

        buffer.truncate(bytes_read as usize);
        Ok(buffer)
    }

    /// Write a message to the pipe
    pub fn write(&mut self, data: &[u8]) -> Result<(), PipeError> {
        let mut bytes_written = 0;

        unsafe {
            if !WriteFile(
                self.handle,
                data.as_ptr() as *const _,
                data.len() as u32,
                Some(&mut bytes_written),
                None,
            ).as_bool() {
                return Err(PipeError::Write(
                    io::Error::last_os_error().to_string()
                ));
            }
        }

        Ok(())
    }
}

impl Drop for NamedPipeClient {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_pipe_communication() {
        let server = NamedPipeServer::new("test_pipe").unwrap();
        
        // Spawn client in separate task
        let client_handle = tokio::spawn(async move {
            // Give server time to start
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            
            let mut client = NamedPipeClient::connect("test_pipe").unwrap();
            client.write(b"Hello from client!").unwrap();
            
            let response = client.read().unwrap();
            assert_eq!(&response, b"Hello from server!");
        });

        // Handle server connection
        let mut conn = server.wait_for_client().await.unwrap();
        let message = conn.read().unwrap();
        assert_eq!(&message, b"Hello from client!");
        
        conn.write(b"Hello from server!").unwrap();

        client_handle.await.unwrap();
    }
} 