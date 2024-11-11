use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut stream = tokio::net::TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to TCP server");

    let message = b"Hello, TCP server!";
    stream.write_all(message).await?;
    println!("Sent: {:?}", message);

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    println!("Received from server: {:?}", &buffer[..n]);

    Ok(())
}
