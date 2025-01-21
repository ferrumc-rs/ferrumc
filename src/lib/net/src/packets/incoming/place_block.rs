use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use ferrumc_world::vanilla_chunk_format::BlockData;
use std::sync::Arc;
use tracing::debug;

#[derive(NetDecode, Debug)]
#[packet(packet_id = "use_item_on", state = "play")]
pub struct PlaceBlock {
    pub hand: VarInt,
    pub position: NetworkPosition,
    pub face: VarInt,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub cursor_z: f32,
    pub inside_block: bool,
    pub sequence: VarInt,
}

impl IncomingPacket for PlaceBlock {
    async fn handle(self, _conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        let block_clicked = state
            .clone()
            .world
            .get_block(
                self.position.x,
                self.position.y as i32,
                self.position.z,
                "overworld",
            )
            .await?;
        debug!("Block clicked: {:?}", block_clicked);
        state
            .world
            .set_block(
                self.position.x,
                self.position.y as i32,
                self.position.z,
                "overworld",
                BlockData {
                    name: "minecraft:stone".to_string(),
                    properties: None,
                },
            )
            .await?;
        let q = state.universe.query::<&mut ChunkReceiver>();
        for (_, mut chunk_recv) in q {
            chunk_recv.queue_chunk_resend(
                self.position.x >> 4,
                self.position.z >> 4,
                "overworld".to_string(),
            );
        }
        Ok(())
    }
}
