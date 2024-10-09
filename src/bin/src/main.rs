// Security or something like that
#![forbid(unsafe_code)]

use std::sync::Arc;
use tracing::{error, info};
use ferrumc_ecs::Universe;
use ferrumc_events::infrastructure::Event;
use ferrumc_net::ServerState;
use ferrumc_net_codec::net_types::var_int::VarInt;

pub(crate)mod errors;

pub type Result<T> = std::result::Result<T, errors::BinaryError>;

#[tokio::main]
async fn main() {
    ferrumc_logging::init_logging();

    println!("good day to ya. enjoy your time with ferrumc!");

    if let Err(e) = entry().await {
        error!("Server exited with the following error;");
        error!("{:?}", e);
    } else {
        info!("Server exited successfully.");
    }
}

async fn entry() -> Result<()> {
    let listener = ferrumc_net::server::create_server_listener().await?;

    let state = ServerState::new(Universe::new());

    ferrumc_net::server::listen(Arc::new(state), listener).await?;

    Ok(())
}