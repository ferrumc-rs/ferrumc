use std::sync::Arc;
use tracing::debug;
use ferrumc_macros::{packet, Event, NetDecode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_state::ServerState;
use crate::errors::NetError;
use crate::packets::IncomingPacket;

#[derive(Debug, NetDecode)]
#[packet(packet_id = "select_known_packs", state = "configuration")]
pub struct ServerBoundKnownPacks {
    pub packs: LengthPrefixedVec<PackOwned>,
}

#[derive(Debug, NetDecode)]
#[expect(dead_code)]
pub struct PackOwned {
    namespace: String,
    id: String,
    version: String,
}

impl IncomingPacket for ServerBoundKnownPacks {
    fn handle(self, conn_id: usize, state: Arc<ServerState>) -> Result<(), NetError> {
        //! No clue what this packet is for, but it's not used in the server.
        //! It's for data packs usually. But we're probably not gunna implement 'em anytime soon.
        debug!("Received known packs: {:#?}", self);
        Ok(())
    }
}