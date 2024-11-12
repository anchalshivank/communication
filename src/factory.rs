use crate::listener::Listener;
use crate::quic_listener::QuicListen;
use crate::tcp_listener::TcpListen;
use std::error::Error;
use std::net::SocketAddr;
use crate::conn::Conn;
use crate::quic_connection::QuicConn;
use crate::tcp_connection::TcpConn;

pub async fn create_listener(protocol: &str, bind_addr: SocketAddr) -> Result<Box<dyn Listener + Send + Sync>, Box<dyn Error + Send + Sync>>{


    match protocol {
        "tcp" => {
            let listener = TcpListen::new(bind_addr).await?;
            Ok(Box::new(listener))

        }
        "quic" => {
            let listener = QuicListen::new(bind_addr)?;
            Ok(Box::new(listener))

        }
        _ => Err("Unknown protocol".into()),
    }

}

pub async fn create_client(protocol: &str, bind_addr: SocketAddr, server_addr: SocketAddr) -> Result<Box<dyn Conn + Send + Sync>, Box<dyn Error + Send + Sync>> {
    match protocol {
        "tcp" => {
            let mut client = TcpConn::new(server_addr).await?;
            Ok(Box::new(client))
        }
        "quic" => {
            let mut client = QuicConn::new(bind_addr, server_addr).await?;
            Ok(Box::new(client))
        }
        _ => Err("Unknown protocol".into()),
    }
}