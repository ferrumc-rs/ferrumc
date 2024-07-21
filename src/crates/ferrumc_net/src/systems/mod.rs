use async_trait::async_trait;
use ferrumc_utils::prelude::*;
use tracing::{info_span, Instrument};

pub mod tick_system;
pub mod keep_alive_system;

#[async_trait]
pub trait System : Send + Sync
{
    async fn run(&self);
    fn name(&self) -> &'static str;
    async fn kill(&self) {}
}


pub static ALL_SYSTEMS: &[&dyn System] = &[
    &tick_system::TickSystem,
];

pub async fn start_all_systems()  -> Result<()> {
    for system in ALL_SYSTEMS {
        let system_name = system.name();
        tokio::spawn(system.run().instrument(info_span!("system", %system_name)));
    }
    Ok(())
}

pub async fn kill_all_systems() -> Result<()> {
    for system in ALL_SYSTEMS {
        system.kill().await?;
    }
    Ok(())
}