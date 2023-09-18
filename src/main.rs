extern crate ferrumc;
pub use ferrumc_world as world;

use anyhow::Result;
use ferrumc::server::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::new(25565).await?;
    server.run().await?;

    Ok(())
}