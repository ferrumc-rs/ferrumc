use std::sync::Arc;
use tokio::net::TcpListener;
use ferrumc_ecs::Universe;

pub struct ServerState {
    pub universe: Universe,
    pub tcp_listener: TcpListener
}

pub type GlobalState = Arc<ServerState>;