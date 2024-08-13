use crate::database::Database;
use crate::ecs::world::World;
use crate::net::ConnectionList;
use std::sync::Arc;

pub struct ServerState {
    pub world: Arc<World>,
    pub connections: ConnectionList,
    pub database: Database,
    pub server_stream: tokio::net::TcpListener,
}

pub type GlobalState = Arc<ServerState>;
