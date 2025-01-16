use std::io;
use windows::Win32::Foundation::{HANDLE, CloseHandle, INVALID_HANDLE_VALUE, BOOL};
use windows::Win32::Storage::FileSystem::{
    CreateFileA, ReadFile, WriteFile,
    FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_NONE,
    OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, FILE_FLAG_OVERLAPPED,
};
use windows::Win32::System::Pipes::{
    CreateNamedPipeA, ConnectNamedPipe,
    PIPE_ACCESS_DUPLEX, PIPE_TYPE_BYTE, PIPE_READMODE_BYTE,
    PIPE_WAIT, PIPE_UNLIMITED_INSTANCES,
};
use windows::core::{PCSTR, Result as WinResult};

use super::error::pipe::PipeError;
use super::error::Error;
use super::traits::{WindowsHandle, NamedPipeOps, NamedPipeFactory};

pub struct RealPipe {
    handle: HANDLE,
    name: String,
    connected: bool,
}

impl WindowsHandle for RealPipe {
    fn as_raw_handle(&self) -> HANDLE {
        self.handle
    }

    fn is_valid(&self) -> bool {
        self.handle != INVALID_HANDLE_VALUE
    }

    fn close(&mut self) -> io::Result<()> {
        if self.is_valid() {
            unsafe {
                CloseHandle(self.handle);
            }
        }
        Ok(())
    }
}

impl NamedPipeOps for RealPipe {
    fn connect(&mut self) -> Result<BOOL, Error> {
        unsafe {
            let result = ConnectNamedPipe(self.handle, None);
            self.connected = result.as_bool();
            Ok(result)
        }
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

        let mut bytes_read = 0u32;
        unsafe {
            let result = ReadFile(
                self.handle,
                Some(buf),
                Some(&mut bytes_read),
                None,
            );
            
            if !result.as_bool() {
                return Err(Error::Pipe(io::Error::last_os_error()));
            }
        }

        Ok(bytes_read as usize)
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        if !self.connected {
            return Err(Error::Pipe(io::Error::new(
                io::ErrorKind::NotConnected,
                "Pipe not connected"
            )));
        }

        let mut bytes_written = 0u32;
        unsafe {
            let result = WriteFile(
                self.handle,
                Some(buf),
                Some(&mut bytes_written),
                None,
            );
            
            if !result.as_bool() {
                return Err(Error::Pipe(io::Error::last_os_error()));
            }
        }

        Ok(bytes_written as usize)
    }

    fn wait_for_client(&mut self) -> Result<(), Error> {
        self.connect().map(|_| ())
    }
}

pub struct RealPipeFactory;

impl NamedPipeFactory for RealPipeFactory {
    type Pipe = RealPipe;

    fn create_server(&self, name: &str) -> Result<Self::Pipe, Error> {
        let pipe_name = format!("\\\\.\\pipe\\{}", name);
        unsafe {
            let handle = CreateNamedPipeA(
                PCSTR::from_raw(pipe_name.as_ptr()),
                PIPE_ACCESS_DUPLEX | FILE_FLAG_OVERLAPPED,
                PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
                PIPE_UNLIMITED_INSTANCES,
                4096,
                4096,
                0,
                None,
            );

            if handle == INVALID_HANDLE_VALUE {
                return Err(Error::Pipe(io::Error::last_os_error()));
            }

            Ok(RealPipe {
                handle,
                name: name.to_string(),
                connected: false,
            })
        }
    }

    fn connect_client(&self, name: &str) -> Result<Self::Pipe, Error> {
        let pipe_name = format!("\\\\.\\pipe\\{}", name);
        unsafe {
            let handle = CreateFileA(
                PCSTR::from_raw(pipe_name.as_ptr()),
                FILE_GENERIC_READ.0 | FILE_GENERIC_WRITE.0,
                FILE_SHARE_NONE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            );

            if handle == INVALID_HANDLE_VALUE {
                return Err(Error::Pipe(io::Error::last_os_error()));
            }

            Ok(RealPipe {
                handle,
                name: name.to_string(),
                connected: true,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use crate::win32::traits::mock::{MockPipe, MockPipeFactory};

    #[tokio::test]
    async fn test_pipe_mock_communication() {
        let factory = MockPipeFactory;
        
        // Create server and client
        let mut server = factory.create_server("test_pipe").unwrap();
        let mut client = factory.connect_client("test_pipe").unwrap();

        // Test data
        let test_data = b"Hello from client!";
        let response_data = b"Hello from server!";

        // Client writes data
        client.write(test_data).unwrap();
        
        // Server reads data
        let mut buf = vec![0u8; 1024];
        let n = server.read(&mut buf).unwrap();
        assert_eq!(&buf[..n], test_data);

        // Server responds
        server.write(response_data).unwrap();
        
        // Client reads response
        let mut buf = vec![0u8; 1024];
        let n = client.read(&mut buf).unwrap();
        assert_eq!(&buf[..n], response_data);
    }

    #[tokio::test]
    async fn test_pipe_mock_disconnected() {
        let factory = MockPipeFactory;
        let mut pipe = factory.create_server("test_pipe").unwrap();
        
        // Should fail when not connected
        assert!(pipe.write(b"test").is_err());
        
        // Connect and try again
        pipe.connect().unwrap();
        assert!(pipe.write(b"test").is_ok());
        
        // Disconnect and verify failure
        pipe.disconnect().unwrap();
        assert!(pipe.write(b"test").is_err());
    }
} 