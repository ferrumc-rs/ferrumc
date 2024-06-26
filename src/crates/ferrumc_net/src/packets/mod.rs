use std::sync::Arc;

use tokio::sync::RwLock;

use ferrumc_macros::bake_packet_registry;
use ferrumc_utils::error::Error;

use crate::Connection;

pub mod incoming;
pub mod outgoing;

pub trait IncomingPacket {
    #[allow(async_fn_in_trait)]
    async fn handle(&self, conn: &mut tokio::sync::RwLockWriteGuard<Connection>) -> Result<(), Error>;
}


bake_packet_registry!("\\src\\packets\\incoming");