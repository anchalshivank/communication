use std::env;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use communication::conn::Conn;
use communication::factory::{create_client, create_listener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mode = env::args().nth(1).expect("Specify 'listener' or 'conn'");
    let protocol = env::args().nth(2).expect("Specify 'tcp' or 'quic'");

    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);
    let conn_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6000);

    match mode.as_str() {
        "listener" => {
            let listener = create_listener(&protocol, server_addr).await?;
            println!("Running as {} listener on {:?}", protocol, server_addr);

            loop {
                let mut connection = listener.accept().await?;
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(&mut connection).await {
                        eprintln!("Connection error: {:?}", e);
                    }
                });
            }
        }
        "conn" => {
            let mut client = create_client(&protocol, conn_addr, server_addr).await?;
            println!("Running as {} client connecting to {:?}", protocol, server_addr);

            client.send(b"Hello from client").await?;
            let response = client.receive().await?;
            println!("Received from server: {:?}", response);
        }
        _ => {
            eprintln!("Invalid mode. Use 'listener' or 'conn'.");
            return Err("Invalid mode".into());
        }
    }

    Ok(())
}

async fn handle_connection(
    connection: &mut Box<dyn Conn + Send + Sync>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    connection.send(b"Hello from server").await?;
    let response = connection.receive().await?;
    println!("Received from client: {:?}", response);
    Ok(())
}
