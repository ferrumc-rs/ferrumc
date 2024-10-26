use std::sync::Arc;
use ferrumc_net::{GlobalState, NetResult};
use futures::stream::FuturesUnordered;
use tracing::{debug, debug_span, info, Instrument};
use async_trait::async_trait;
use crate::systems::keep_alive_system::KeepAliveSystem;
use crate::systems::tcp_listener_system::TcpListenerSystem;
use crate::systems::ticking_system::TickingSystem;

#[async_trait]
pub trait System: Send + Sync {
    async fn start(self: Arc<Self>, state: GlobalState);
    async fn stop(self: Arc<Self>, state: GlobalState);

    fn name(&self) -> &'static str;
}


pub fn create_systems() -> Vec<Arc<dyn System>> {
    vec![
        Arc::new(TcpListenerSystem),
        Arc::new(KeepAliveSystem::new()),
        Arc::new(TickingSystem),
    ]
}
pub async fn start_all_systems(state: GlobalState) -> NetResult<()> {
    let systems = create_systems();
    let handles = FuturesUnordered::new();

    for system in systems {
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
    let systems = create_systems();
    info!("Stopping all systems...");

    for system in systems {
        debug!("Stopping system: {}", system.name());
        system.stop(state.clone()).await;
    }

    Ok(())
}