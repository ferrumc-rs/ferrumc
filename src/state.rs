use crate::database::Database;
use crate::ecs::world::World;
use crate::net::ConnectionList;
use std::sync::Arc;
use crate::events::creation::dispatcher::EventDispatcher;

pub struct ServerState {
    pub world: Arc<World>,
    pub connections: ConnectionList,
    pub database: Database,
    pub server_stream: tokio::net::TcpListener,
    pub event_dispatcher: Arc<EventDispatcher>,
}

pub type GlobalState = Arc<ServerState>;
