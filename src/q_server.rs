use quinn::{Endpoint, ServerConfig};
use rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};
use std::{error::Error, fs, net::SocketAddr, sync::Arc};
use std::net::{IpAddr, Ipv4Addr};
use tokio::io::AsyncReadExt;

pub async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);
    let (endpoint, _cert) = make_server_endpoint(server_addr)?;
    println!("QUIC server running on {}", server_addr);
    
    fs::write("server_cert.der", _cert.as_ref()).expect("Cannot save certificate");

    while let Some(conn) = endpoint.accept().await {
        let connection = conn.await?;
        println!("QUIC connection received from {:?}", connection.remote_address());

        tokio::spawn(async move {
            if let Ok(mut recv) = connection.accept_uni().await {
                let mut buffer = [0; 1024];
                if let Ok(Some(n)) = recv.read(&mut buffer).await {
                    println!("QUIC received: {:?}", &buffer[..n]);
                }
            }
        });
    }
    Ok(())
}

#[allow(unused)]
pub fn make_server_endpoint(
    bind_addr: SocketAddr,
) -> Result<(Endpoint, CertificateDer<'static>), Box<dyn Error + Send + Sync>> {
    let (server_config, server_cert) = configure_server()?;
    let endpoint = Endpoint::server(server_config, bind_addr)?;
    Ok((endpoint, server_cert))
}

fn configure_server(
) -> Result<(ServerConfig, CertificateDer<'static>), Box<dyn Error + Send + Sync>> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = CertificateDer::from(cert.cert);
    let priv_key = PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der());

    let mut server_config =
        ServerConfig::with_single_cert(vec![cert_der.clone()], priv_key.into())?;
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());

    Ok((server_config, cert_der))
}
