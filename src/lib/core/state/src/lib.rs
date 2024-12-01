use ferrumc_ecs::Universe;
use ferrumc_world::World;
use std::sync::Arc;
use tokio::net::TcpListener;
pub struct ServerState {
    pub universe: Universe,
    pub tcp_listener: TcpListener,
    pub world: World,
}

pub type GlobalState = Arc<ServerState>;
