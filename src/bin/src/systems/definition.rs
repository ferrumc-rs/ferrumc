use crate::systems::keep_alive_system::KeepAliveSystem;
use crate::systems::tcp_listener_system::TcpListenerSystem;
use crate::systems::ticking_system::TickingSystem;
use async_trait::async_trait;
use ferrumc_net::NetResult;
use ferrumc_state::GlobalState;
use futures::stream::FuturesUnordered;
use std::sync::{Arc, LazyLock};
use tracing::{debug, debug_span, info, Instrument};

#[async_trait]
pub trait System: Send + Sync {
    async fn start(self: Arc<Self>, state: GlobalState);
    async fn stop(self: Arc<Self>, state: GlobalState);

    fn name(&self) -> &'static str;
}

static SYSTEMS: LazyLock<Vec<Arc<dyn System>>> = LazyLock::new(create_systems);
pub fn create_systems() -> Vec<Arc<dyn System>> {
    vec![
        Arc::new(TcpListenerSystem),
        Arc::new(KeepAliveSystem::new()),
        Arc::new(TickingSystem),
    ]
}
pub async fn start_all_systems(state: GlobalState) -> NetResult<()> {
    let handles = FuturesUnordered::new();

    for system in SYSTEMS.iter() {
        let name = system.name();

        let handle = tokio::spawn(
            system
                .clone()
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

    for system in SYSTEMS.iter() {
        debug!("Stopping system: {}", system.name());
        system.clone().stop(state.clone()).await;
    }

    Ok(())
}
