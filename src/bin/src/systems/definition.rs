use ferrumc_net::{GlobalState, NetResult};
use futures::stream::FuturesUnordered;
use tracing::{debug, debug_span, info, Instrument};
use async_trait::async_trait;
use crate::systems::keep_alive_system::KeepAliveSystem;
use crate::systems::tcp_listener_system::TcpListenerSystem;

use super::ticking_system::TickingSystem;

#[async_trait]
pub trait System: Send + Sync {
    async fn start(&self, state: GlobalState);
    async fn stop(&self, state: GlobalState);

    fn name(&self) -> &'static str;
}

pub static ALL_SYSTEMS: &[&dyn System] = &[
    &TcpListenerSystem,
    &KeepAliveSystem,
    &TickingSystem
];

pub async fn start_all_systems(state: GlobalState) -> NetResult<()> {
    let handles = FuturesUnordered::new();

    for system in ALL_SYSTEMS {
        let name = system.name();

        let handle = tokio::spawn(
            system
                .start(state.clone())
                .instrument(debug_span!("sys", %name)),
        );
        handles.push(handle);
    }

    futures::future::join_all(handles).await;

    Ok(())
}

pub async fn stop_all_systems(state: GlobalState) -> NetResult<()> {
    info!("Stopping all systems...");

    for system in ALL_SYSTEMS {
        debug!("Stopping system: {}", system.name());
        system.stop(state.clone()).await;
    }

    Ok(())
}