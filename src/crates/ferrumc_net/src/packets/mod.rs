use ferrumc_macros::bake_packet_registry;
use ferrumc_utils::error::Error;
use crate::Connection;

pub mod incoming;
pub mod outgoing;

pub trait IncomingPacket {
    async fn handle(&self, conn: &mut Connection) -> Result<(), Error>;
}


bake_packet_registry!("\\src\\packets\\incoming");