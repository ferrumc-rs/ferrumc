use ferrumc_macros::bake_packet_registry;

use crate::state::GlobalState;
use crate::utils::prelude::*;

pub mod incoming;
pub mod outgoing;

pub type ConnectionId = usize;

pub trait IncomingPacket {
    #[allow(async_fn_in_trait)]
    async fn handle(self, conn_id: ConnectionId, state: GlobalState) -> Result<()>;
}

bake_packet_registry!("\\src\\net\\packets\\incoming");
