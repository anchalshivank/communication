use communication::{q_client, q_server, tcp_client, tcp_server, common};
use std::env;
use std::error::Error;
use communication::factory::create_listener;
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
//     // Parse command-line arguments to check if we run as TCP/QUIC server or client
//     let args: Vec<String> = env::args().collect();
//     if args.len() < 2 {
//         eprintln!("Usage: {} <server|client|qserver|qclient>", args[0]);
//         return Ok(());
//     }
//
//     match args[1].as_str() {
//         "server" => tcp_server::run().await?,
//         "client" => tcp_client::run().await?,
//         "qserver" => q_server::run().await?,
//         "qclient" => q_client::run().await?,
//         _ => eprintln!("Invalid argument: use 'server', 'client', 'qserver' or 'qclient'"),
//     }
//
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

    let protocol = env::args().nth(1).expect("Specify tcp or quic");
    let listener = create_listener(&protocol, "127.0.0.1:8080").await?;

    loop {
        let mut connection = listener.accept().await?;
        connection.send(b"Hello").await?;
        let response = connection.receive().await?;
        println!("{:?}", response);
    }


}