extern crate core;

use ferrumc_macros::bake_packet_registry;

pub mod connection;
pub mod errors;
pub mod packets;
pub mod server;
pub mod utils;
pub type NetResult<T> = Result<T, errors::NetError>;




bake_packet_registry!("\\src\\packets\\incoming");
