use std::sync::Arc;

use crate::database::Database;
use crate::ecs::world::World;
use crate::net::ConnectionList;

pub struct ServerState {
    pub world: Arc<World>,
    pub connections: ConnectionList,
    pub database: Database,
}

pub type GlobalState = Arc<ServerState>;
