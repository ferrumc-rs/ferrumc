extern crate core;

use tokio::net::TcpListener;
use ferrumc_ecs::Universe;
use ferrumc_macros::bake_packet_registry;
use std::sync::{Arc};
use ferrumc_world::World;

pub mod connection;
pub mod errors;
pub mod packets;
pub mod server;
pub mod utils;
pub type NetResult<T> = Result<T, errors::NetError>;

pub struct ServerState {
    pub universe: Universe,
    pub tcp_listener: TcpListener,
    pub world: World
}

pub type GlobalState = Arc<ServerState>;


bake_packet_registry!("\\src\\packets\\incoming");
