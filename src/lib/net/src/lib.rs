use bevy_ecs::prelude::Resource;
use bevy_ecs::world::World;
use crossbeam_channel::{Receiver, Sender};
use ferrumc_macros::bake_packet_registry;

mod conn_init;
pub mod connection;
pub mod errors;
pub mod packets;
pub mod server;
pub mod utils;

bake_packet_registry!("\\src\\packets\\incoming");
