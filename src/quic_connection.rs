use std::{error::Error, fs, net::SocketAddr, sync::Arc};
use async_trait::async_trait;
use quinn::{ClientConfig, Connection as QuicConnection, Endpoint, SendStream};
use rustls::pki_types::CertificateDer;
use tokio::io::AsyncWriteExt;
use crate::conn::Conn;

pub struct QuicConn {
    pub(crate) connection: QuicConnection,
    pub(crate) send_stream: SendStream,
}

#[async_trait]
impl Conn for QuicConn {
    async fn send(&mut self, data: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("Sending from QUIC connection");
        self.send_stream.write_all(data).await?;
        Ok(())
    }

    async fn receive(&mut self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        // Since this is a unidirectional stream, we may not expect to receive data here
        Err("Receiving on unidirectional stream is not supported".into())
    }

    async fn close(&self) {
        self.connection.close(0u32.into(), b"");
    }
}

impl QuicConn {
    pub async fn new(bind_addr: SocketAddr, server_addr: SocketAddr) -> Result<Self, Box<dyn Error + Send + Sync>> {
        // Load server certificate from file
        let server_cert_bytes = fs::read("server_cert.der")?;
        let server_cert = CertificateDer::from(server_cert_bytes);

        // Create a client endpoint with the server certificate
        let endpoint = Self::make_client_endpoint(bind_addr, &[&server_cert])?;

        // Connect to the QUIC server
        println!("Attempting to connect to server at {}", server_addr);
        let connection = endpoint.connect(server_addr, "localhost")?.await?;
        println!("Connected to server");

        // Open a unidirectional send stream
        let send_stream = connection.open_uni().await?;

        Ok(Self {
            connection,
            send_stream,
        })
    }

    fn configure_client(
        server_certs: &[&[u8]],
    ) -> Result<ClientConfig, Box<dyn Error + Send + Sync>> {
        let mut certs = rustls::RootCertStore::empty();
        for cert in server_certs {
            certs.add(CertificateDer::from(*cert))?;
        }
        Ok(ClientConfig::with_root_certificates(Arc::new(certs))?)
    }

    fn make_client_endpoint(
        bind_addr: SocketAddr,
        server_certs: &[&[u8]],
    ) -> Result<Endpoint, Box<dyn Error + Send + Sync>> {
        let client_cfg = Self::configure_client(server_certs)?;
        let mut endpoint = Endpoint::client(bind_addr)?;
        endpoint.set_default_client_config(client_cfg);
        Ok(endpoint)
    }
}
