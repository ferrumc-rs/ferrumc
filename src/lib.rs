#![feature(box_into_inner)]

use std::sync::{atomic::AtomicU32, Arc};

use dashmap::DashMap;
use ecs::world::World;
use net::ConnectionList;
use state::{GlobalState, ServerState};
use tokio::net::TcpListener;
use utils::prelude::*;
use crate::events::creation::dispatcher::EventDispatcher;

extern crate core;
#[macro_use]
extern crate macro_rules_attribute;

pub mod ecs;
pub mod net;
pub mod setup;
#[cfg(test)]
mod tests;
pub mod utils;

pub mod database;
pub mod state;
pub mod world;
pub mod events;

pub async fn create_state(tcp_listener: TcpListener) -> Result<GlobalState> {
    Ok(Arc::new(ServerState {
        world: Arc::new(World::new()),
        connections: ConnectionList {
            connections: DashMap::new(),
            connection_count: AtomicU32::new(0),
        },
        database: database::start_database().await?,
        server_stream: tcp_listener,
        event_dispatcher: Arc::new(EventDispatcher::new()),
    }))
}
