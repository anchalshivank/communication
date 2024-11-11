use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("TCP server running on 127.0.0.1:8080");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("TCP connection received from {}", addr);

        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            if let Ok(n) = socket.read(&mut buffer).await {
                println!("TCP received: {:?}", &buffer[..n]);
                let _ = socket.write_all(&buffer[..n]).await;
            }
        });
    }
}
