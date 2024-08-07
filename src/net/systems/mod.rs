use async_trait::async_trait;
use tracing::{info_span, Instrument};

use crate::state::GlobalState;
use crate::utils::prelude::*;

pub mod chunk_sender;
pub mod keep_alive_system;
pub mod tick_system;

#[async_trait]
pub trait System: Send + Sync {
    async fn run(&self, state: GlobalState);
    fn name(&self) -> &'static str;
    async fn kill(&self) {}
}

pub static ALL_SYSTEMS: &[&dyn System] = &[
    &tick_system::TickSystem,
    &keep_alive_system::KeepAliveSystem,
    &chunk_sender::ChunkSender,
];

pub async fn start_all_systems(state: GlobalState) -> Result<()> {
    for system in ALL_SYSTEMS {
        let system_name = system.name();
        tokio::spawn(
            system
                .run(state.clone())
                .instrument(info_span!("system", %system_name)),
        );
    }
    Ok(())
}

pub async fn kill_all_systems() -> Result<()> {
    for system in ALL_SYSTEMS {
        system.kill().await;
    }
    Ok(())
}
