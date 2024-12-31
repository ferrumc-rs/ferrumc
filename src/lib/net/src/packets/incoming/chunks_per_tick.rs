use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_state::ServerState;
use std::sync::Arc;

#[derive(NetDecode)]
#[packet(packet_id = 0x08, state = "play")]
pub struct ChunksPerTick {
    chunks_per_tick: f32,
}

impl IncomingPacket for ChunksPerTick {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        let mut chunk_recv = state.universe.get_mut::<ChunkReceiver>(conn_id)?;
        chunk_recv.chunks_per_tick = self.chunks_per_tick;
        Ok(())
    }
}
