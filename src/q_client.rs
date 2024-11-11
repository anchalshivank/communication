use std::{error::Error, net::SocketAddr, sync::Arc};
use quinn::{ClientConfig, Endpoint};
use rustls::pki_types::CertificateDer;
use std::fs;
use tokio::io::AsyncWriteExt;

pub async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let server_addr: SocketAddr = "127.0.0.1:5000".parse()?;

    let server_cert_bytes = fs::read("server_cert.der")?;
    let server_cert = CertificateDer::from(server_cert_bytes);

    // Create client endpoint and connect to server
    let endpoint = make_client_endpoint("0.0.0.0:0".parse().unwrap(), &[&server_cert])?;
    let connection = endpoint.connect(server_addr, "localhost")?.await?;
    println!("Connected to QUIC server at {}", server_addr);

    // Open a unidirectional stream and send a message
    let mut send = connection.open_uni().await?;
    let message = b"Hello, QUIC server!";
    send.write_all(message).await?;
    println!("Sent message: {:?}", message);

    Ok(())
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

pub fn make_client_endpoint(
    bind_addr: SocketAddr,
    server_certs: &[&[u8]],
) -> Result<Endpoint, Box<dyn Error + Send + Sync>> {
    let client_cfg = configure_client(server_certs)?;
    let mut endpoint = Endpoint::client(bind_addr)?;
    endpoint.set_default_client_config(client_cfg);
    Ok(endpoint)
}
