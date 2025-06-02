use bevy_ecs::prelude::Resource;
use bevy_ecs::world::World;
use crossbeam_channel::{Receiver, Sender};
use ferrumc_macros::setup_packet_handling;
use std::sync::Arc;

mod conn_init;
pub mod connection;
pub mod errors;
pub mod packets;
pub mod server;

setup_packet_handling!("\\src\\packets\\incoming");
