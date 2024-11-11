use std::error::Error;
use async_trait::async_trait;
use quinn::{Connection as QuicConnection, RecvStream, SendStream};
use crate::conn::Conn;

pub struct QuicConn {

    pub(crate) connection: QuicConnection,
    pub(crate) send_stream: SendStream,
    pub(crate) recv_stream: RecvStream,

}

#[async_trait]
impl Conn for QuicConn {
    async fn send(&mut self, data: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.send_stream.write_all(data).await?;
        Ok(())
    }

    async fn receive(&mut self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let mut buffer = vec![0u8; 1024];
        let n = self.recv_stream.read(&mut buffer).await?.unwrap();
        buffer.truncate(n);
        Ok(buffer)
    }

    async fn close(&self) {
        self.connection.close(0u32.into(), b"");
    }
}