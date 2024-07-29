use crate::database::Database;
use crate::ecs::world::World;
use crate::net::ConnectionList;

pub struct ServerState {
    pub world: World,
    pub connections: ConnectionList,
    pub database: Database,
}

pub type GlobalState = std::sync::Arc<tokio::sync::RwLock<ServerState>>;
