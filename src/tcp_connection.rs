use std::error::Error;
use std::net::SocketAddr;
use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::conn::Conn;

pub struct TcpConn{

    pub(crate) stream: TcpStream,

}

#[async_trait]
impl Conn for TcpConn{
    async fn send(&mut self, data: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.stream.write_all(data).await?;
        Ok(())
    }

    async fn receive(&mut self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let mut buffer = vec![0; 1024];
        let n = self.stream.read(&mut buffer).await?;
        buffer.truncate(n);
        Ok(buffer)
    }

    async fn close(&self) {
        todo!()
    }
}

impl TcpConn {
    pub async fn new(server_addr: SocketAddr) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let stream = TcpStream::connect(server_addr).await?;
        Ok(Self { stream })
    }
}