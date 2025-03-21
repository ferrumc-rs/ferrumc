use crate::systems::keep_alive_system::KeepAliveSystem;
use crate::systems::tcp_listener_system::TcpListenerSystem;
use crate::systems::ticking_system::TickingSystem;
use ferrumc_net::NetResult;
use ferrumc_state::GlobalState;
use rayon::prelude::*;
use std::sync::{Arc, LazyLock};
use tracing::{debug, debug_span, info, Instrument};

pub trait System: Send + Sync {
    fn start(self: Arc<Self>, state: GlobalState);
    fn stop(self: Arc<Self>, state: GlobalState);

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
pub fn start_all_systems(state: GlobalState) -> NetResult<()> {
    SYSTEMS.iter().par_bridge().for_each(|system| {
        let name = system.name();
        let _ = system
            .clone()
            .start(state.clone())
            .instrument(debug_span!("sys", %name));
    });

    Ok(())
}

pub fn stop_all_systems(state: GlobalState) -> NetResult<()> {
    info!("Stopping all systems...");

    for system in SYSTEMS.iter() {
        debug!("Stopping system: {}", system.name());
        system.clone().stop(state.clone());
    }

    Ok(())
}
