use std::error::Error;
use std::net::SocketAddr;
use async_trait::async_trait;
use log::info;
use tokio::net::TcpListener;
use crate::conn::Conn;
use crate::listener::Listener;
use crate::tcp_connection::TcpConn;

pub struct TcpListen {
    pub(crate) listener: TcpListener,
}

#[async_trait]
impl Listener for TcpListen {
    async fn accept(&self) -> Result<Box<dyn Conn + Send + Sync>, Box<dyn Error + Send + Sync>> {
        let (stream, addr) = self.listener.accept().await?;
        println!("Tcp listener started at {}", addr); // Optional logging
        Ok(Box::new(TcpConn { stream }))
    }
}

impl TcpListen {
    pub async fn new(addr: SocketAddr) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self { listener })
    }
}
