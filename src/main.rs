mod server;

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind the listener to an address
    let listener = TcpListener::bind("127.0.0.1:25565").await?;

    server::run_server(listener).await?;

    Ok(())
}
