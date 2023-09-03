use anyhow::Result;
use tokio::net::TcpListener;
use tokio::spawn;

use crate::network::connection::handle_connection;



pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn new(port: u16) -> Result<Self> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        Ok(Self { listener})
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            // server loop
            let (socket, _) = self.listener.accept().await?;
            spawn(handle_connection(socket));
        }
    }
}

