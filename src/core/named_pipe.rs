use std::io;
use windows::Win32::Foundation::{HANDLE, BOOL};
use windows::Win32::Storage::FileSystem::{
    CreateFileA, ReadFile, WriteFile,
    FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_NONE,
    OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, FILE_FLAG_OVERLAPPED,
    FILE_FLAGS_AND_ATTRIBUTES,
};
use windows::Win32::System::Pipes::{
    CreateNamedPipeA, ConnectNamedPipe, DisconnectNamedPipe,
    NAMED_PIPE_MODE,
};
use windows::core::{PCSTR, Result as WinResult, Error as WinError};

const PIPE_ACCESS_DUPLEX: u32 = 0x00000003;
const PIPE_TYPE_MESSAGE: u32 = 0x00000004;
const PIPE_READMODE_MESSAGE: u32 = 0x00000002;
const PIPE_WAIT: u32 = 0x00000000;
const PIPE_UNLIMITED_INSTANCES: u32 = 255;

#[derive(Debug, thiserror::Error)]
pub enum PipeError {
    #[error("Failed to create pipe: {0}")]
    Creation(String),
    #[error("Failed to connect pipe: {0}")]
    Connection(String),
    #[error("Failed to read from pipe: {0}")]
    Read(String),
    #[error("Failed to write to pipe: {0}")]
    Write(String),
}

impl From<WinError> for PipeError {
    fn from(err: WinError) -> Self {
        PipeError::Creation(err.to_string())
    }
}

pub struct NamedPipe {
    handle: HANDLE,
}

impl NamedPipe {
    pub fn create(name: &str) -> Result<Self, PipeError> {
        let pipe_name = format!("\\\\.\\pipe\\{}", name);
        unsafe {
            let handle = CreateNamedPipeA(
                PCSTR::from_raw(pipe_name.as_ptr()),
                FILE_FLAGS_AND_ATTRIBUTES(PIPE_ACCESS_DUPLEX | FILE_FLAG_OVERLAPPED.0),
                NAMED_PIPE_MODE(PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT),
                PIPE_UNLIMITED_INSTANCES,
                4096,
                4096,
                0,
                None,
            )?;
            Ok(Self { handle })
        }
    }

    pub fn connect(&self) -> Result<(), PipeError> {
        unsafe {
            ConnectNamedPipe(self.handle, None)
                .map_err(|e| PipeError::Connection(e.to_string()))?;
            Ok(())
        }
    }

    pub fn read(&self, buffer: &mut [u8]) -> Result<usize, PipeError> {
        let mut bytes_read = 0;
        unsafe {
            ReadFile(
                self.handle,
                Some(buffer),
                Some(&mut bytes_read),
                None,
            )
            .map_err(|e| PipeError::Read(e.to_string()))?;
            Ok(bytes_read as usize)
        }
    }

    pub fn write(&self, buffer: &[u8]) -> Result<usize, PipeError> {
        let mut bytes_written = 0;
        unsafe {
            WriteFile(
                self.handle,
                Some(buffer),
                Some(&mut bytes_written),
                None,
            )
            .map_err(|e| PipeError::Write(e.to_string()))?;
            Ok(bytes_written as usize)
        }
    }
}

impl Drop for NamedPipe {
    fn drop(&mut self) {
        unsafe {
            let _ = DisconnectNamedPipe(self.handle);
        }
    }
}

pub struct NamedPipeClient {
    handle: HANDLE,
}

impl NamedPipeClient {
    pub fn connect(name: &str) -> Result<Self, PipeError> {
        let pipe_name = format!("\\\\.\\pipe\\{}", name);
        unsafe {
            let handle = CreateFileA(
                PCSTR::from_raw(pipe_name.as_ptr()),
                (FILE_GENERIC_READ | FILE_GENERIC_WRITE).0,
                FILE_SHARE_NONE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            )?;
            Ok(Self { handle })
        }
    }

    pub fn read(&self, buffer: &mut [u8]) -> Result<usize, PipeError> {
        let mut bytes_read = 0;
        unsafe {
            ReadFile(
                self.handle,
                Some(buffer),
                Some(&mut bytes_read),
                None,
            )
            .map_err(|e| PipeError::Read(e.to_string()))?;
            Ok(bytes_read as usize)
        }
    }

    pub fn write(&self, buffer: &[u8]) -> Result<usize, PipeError> {
        let mut bytes_written = 0;
        unsafe {
            WriteFile(
                self.handle,
                Some(buffer),
                Some(&mut bytes_written),
                None,
            )
            .map_err(|e| PipeError::Write(e.to_string()))?;
            Ok(bytes_written as usize)
        }
    }
}

impl Drop for NamedPipeClient {
    fn drop(&mut self) {
        unsafe {
            let _ = DisconnectNamedPipe(self.handle);
        }
    }
} 