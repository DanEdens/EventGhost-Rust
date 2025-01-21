use std::io;
use tokio::net::windows::named_pipe::{ClientOptions, ServerOptions};
use std::path::PathBuf;

pub struct NamedPipe {
    path: PathBuf,
    server: Option<tokio::net::windows::named_pipe::NamedPipeServer>,
    client: Option<tokio::net::windows::named_pipe::NamedPipeClient>,
}

impl NamedPipe {
    pub async fn create(name: &str) -> io::Result<Self> {
        let path = PathBuf::from(format!("\\\\.\\pipe\\{}", name));
        let server = ServerOptions::new()
            .create(&path)?;
            
        Ok(NamedPipe {
            path,
            server: Some(server),
            client: None,
        })
    }
    
    pub async fn connect(name: &str) -> io::Result<Self> {
        let path = PathBuf::from(format!("\\\\.\\pipe\\{}", name));
        let client = ClientOptions::new()
            .open(&path)?;
            
        Ok(NamedPipe {
            path,
            server: None,
            client: Some(client),
        })
    }
    
    pub async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(server) = &mut self.server {
            tokio::io::AsyncReadExt::read(server, buf).await
        } else if let Some(client) = &mut self.client {
            tokio::io::AsyncReadExt::read(client, buf).await
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "Pipe not connected"))
        }
    }
    
    pub async fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Some(server) = &mut self.server {
            tokio::io::AsyncWriteExt::write(server, buf).await
        } else if let Some(client) = &mut self.client {
            tokio::io::AsyncWriteExt::write(client, buf).await
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "Pipe not connected"))
        }
    }
}

impl Drop for NamedPipe {
    fn drop(&mut self) {
        if self.path.exists() {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;
    
    #[test]
    fn test_named_pipe() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut server = NamedPipe::create("test").await.unwrap();
            let mut client = NamedPipe::connect("test").await.unwrap();
            
            let msg = b"Hello, world!";
            client.write(msg).await.unwrap();
            
            let mut buf = [0u8; 13];
            server.read(&mut buf).await.unwrap();
            
            assert_eq!(&buf, msg);
        });
    }
} 