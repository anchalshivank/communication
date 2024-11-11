use crate::listener::Listener;
use crate::quic_listener::QuicListen;
use crate::tcp_listener::TcpListen;
use quinn::Endpoint;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

pub async fn create_listener(protocol: &str, addr: &str) -> Result<Box<dyn Listener + Send + Sync>, Box<dyn Error + Send + Sync>>{

    let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);

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