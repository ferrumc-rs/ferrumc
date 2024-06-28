use ferrumc_macros::bake_packet_registry;
use ferrumc_utils::prelude::*;

use crate::Connection;

pub mod incoming;
pub mod outgoing;

pub trait IncomingPacket {
    #[allow(async_fn_in_trait)]
    async fn handle(&self, conn: &mut Connection) -> Result<()>;
}


bake_packet_registry!("\\src\\packets\\incoming");