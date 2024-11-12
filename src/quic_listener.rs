use quinn::{Endpoint, ServerConfig};
use rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};
use std::{error::Error, fs, net::SocketAddr, sync::Arc};
use tokio::io::AsyncReadExt;
use crate::conn::Conn;
use crate::listener::Listener;

pub struct QuicListen {
    pub(crate) endpoint: Endpoint,
}

#[async_trait::async_trait]
impl Listener for QuicListen {
    async fn accept(&self) -> Result<Box<dyn Conn + Send + Sync>, Box<dyn Error + Send + Sync>> {
        let conn = self.endpoint.accept().await;
        let connection = conn.unwrap().await?;
        println!("Connection received from {:?}", connection.remote_address());
        let connection_clone = connection.clone();

        tokio::spawn(async move {
            if let Ok(mut recv_stream) = connection_clone.accept_uni().await {
                println!("Accepted unidirectional stream");
                let mut buffer = [0; 1024];
                match recv_stream.read(&mut buffer).await {
                    Ok(Some(n)) => {
                        println!("Received message from client: {:?}", &buffer[..n]);
                    }
                    Ok(None) => println!("Client closed the stream"),
                    Err(e) => eprintln!("Error receiving message: {:?}", e),
                }
            } else {
                eprintln!("Failed to accept unidirectional stream");
            }
        });

        Ok(Box::new(QuicDummyConn {
            connection,
        }))
    }
}


impl QuicListen {
    pub fn new(bind_addr: SocketAddr) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let (server_config, server_cert) = Self::configure_server()?;
        let endpoint = Endpoint::server(server_config, bind_addr)?;
        println!("QUIC server started on {}", bind_addr);

        fs::write("server_cert.der", server_cert.as_ref()).expect("Cannot save certificate");

        Ok(Self { endpoint })
    }

    fn configure_server() -> Result<(ServerConfig, CertificateDer<'static>), Box<dyn Error + Send + Sync>> {
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()])?;
        let cert_der = CertificateDer::from(cert.cert);
        let priv_key = PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der());

        let mut server_config = ServerConfig::with_single_cert(vec![cert_der.clone()], priv_key.into())?;
        let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
        transport_config.max_concurrent_uni_streams(0_u8.into());

        Ok((server_config, cert_der))
    }
}

pub struct QuicDummyConn {
    pub(crate) connection: quinn::Connection,
}

#[async_trait::async_trait]
impl Conn for QuicDummyConn {
    async fn send(&mut self, _data: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

    async fn receive(&mut self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        Ok(Vec::new())
    }

    async fn close(&self) {
        self.connection.close(0u32.into(), b"");
    }
}
