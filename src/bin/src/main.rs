// Security or something like that
#![forbid(unsafe_code)]

use ferrumc_ecs::Universe;
use ferrumc_events::infrastructure::Event;
use ferrumc_net::{packets::outgoing::tick_event::TickEvent, ServerState};
use std::{sync::Arc, time::Duration};
use tokio::time::Instant;
use tracing::{debug, error, info};

pub(crate) mod errors;
mod packet_handlers;

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

async fn start_ticking(net_state: Arc<ServerState>) {
    // is game time saved in a file??
    let mut tick = 0;
    loop {
        let required_end = Instant::now() + Duration::from_millis(50);
        // TODO handle error
        let res = TickEvent::trigger(TickEvent::new(tick), net_state.clone()).await;

        if res.is_err() {
            debug!("error : {:?}", res);
        }
        let now = Instant::now();
        if required_end > now {
            tokio::time::sleep(required_end - now).await;
        } else {
            let time_debt = now - required_end;
            info!("running behind! by : {}ms", time_debt.as_millis());
        }

        tick += 1;
    }
}

async fn entry() -> Result<()> {
    let listener = ferrumc_net::server::create_server_listener().await?;

    let state = Arc::new(ServerState::new(Universe::new()));

    tokio::task::spawn(start_ticking(state.clone()));
    ferrumc_net::server::listen(state, listener).await?;

    Ok(())
}
