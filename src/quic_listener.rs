use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use quinn::{Connection, ConnectionError, Endpoint, ServerConfig};
use rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};
use crate::conn::Conn;
use crate::listener::Listener;
use crate::quic_connection::QuicConn;

pub struct QuicListen {

    pub(crate) endpoint: Endpoint,

}

#[async_trait::async_trait]
impl Listener for QuicListen {

    async fn accept(&self) -> Result<Box<dyn Conn + Send + Sync>, Box<dyn Error + Send + Sync>> {

        let conn = self.endpoint.accept().await;
        let connection= conn.unwrap().await?;
        let  (send_stream, recv_stream) = connection.open_bi().await?;

        Ok(Box::new(QuicConn {
            connection,
            send_stream,
            recv_stream
        }))
    }

}

impl QuicListen {
    pub fn new(bind_addr: SocketAddr) -> Result<Self, Box<dyn Error + Send + Sync>> {

        let (server_config, server_cert) = Self::configure_server()?;
        let endpoint = Endpoint::server(server_config, bind_addr)?;

        Ok(Self {
            endpoint
        })

    }

    fn configure_server() -> Result<(ServerConfig, CertificateDer<'static>), Box<dyn Error + Send + Sync>> {

        let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()])?;
        let cert_der = CertificateDer::from(cert.cert);
        let priv_key = PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der());

        let mut server_config = ServerConfig::with_single_cert(
            vec![cert_der.clone()],priv_key.into()
        )?;

        let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
        transport_config.max_concurrent_uni_streams(0_u8.into());

        Ok((server_config, cert_der))


    }



}