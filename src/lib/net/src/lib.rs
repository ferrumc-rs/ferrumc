use ferrumc_macros::bake_packet_registry;

pub mod errors;
pub mod packets;
pub mod connection;
pub mod server;
pub type NetResult<T> = Result<T, errors::NetError>;




bake_packet_registry!("\\src\\packets\\incoming");