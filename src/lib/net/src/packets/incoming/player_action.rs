use crate::connection::StreamWriter;
use crate::packets::outgoing::block_change_ack::BlockChangeAck;
use crate::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use ferrumc_world::vanilla_chunk_format::BlockData;
use std::sync::Arc;
use tracing::debug;

#[derive(NetDecode)]
#[packet(packet_id = "player_action", state = "play")]
pub struct PlayerAction {
    pub status: VarInt,
    pub location: NetworkPosition,
    pub face: u8,
    pub sequence: VarInt,
}

impl IncomingPacket for PlayerAction {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        // https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol?oldid=2773393#Player_Action
        match self.status.val {
            0 => {
                let mut chunk = state
                    .clone()
                    .world
                    .load_chunk(self.location.x >> 4, self.location.z >> 4, "overworld")
                    .await?;
                let block =
                    chunk.get_block(self.location.x, self.location.y as i32, self.location.z)?;
                debug!("Block: {:?}", block);
                let (relative_x, relative_y, relative_z) = (
                    self.location.x & 0xF,
                    self.location.y as i32,
                    self.location.z & 0xF,
                );
                chunk.set_block(relative_x, relative_y, relative_z, BlockData::default())?;
                // debug!(chunk = ?chunk, "Chunk after block placement");
                state.world.save_chunk(chunk.clone()).await?;
                state.world.sync().await?;
                {
                    let ack_packet = BlockChangeAck {
                        sequence: self.sequence,
                    };
                    if let Ok(mut conn) = state.universe.get_mut::<StreamWriter>(conn_id) {
                        let chunk_packet = ChunkAndLightData::from_chunk(&chunk)?;
                        conn.send_packet(chunk_packet, &NetEncodeOpts::WithLength)?;
                        conn.send_packet(ack_packet, &NetEncodeOpts::WithLength)?;
                    } else {
                        debug!(
                            "Player disconnected before we could send the BlockChangeAck packet"
                        );
                    }
                }
                // {
                //     let q = state.universe.query::<&mut ChunkReceiver>();
                //     for (_, mut chunk_receiver) in q {
                //         debug!("Queueing chunk resend");
                //         chunk_receiver.queue_chunk_resend(
                //             self.location.x >> 4,
                //             self.location.z >> 4,
                //             "overworld".to_string(),
                //         );
                //     }
                // }
            }
            1 => {
                debug!("You shouldn't be seeing this in creative mode.");
            }
            _ => {}
        };
        Ok(())
    }
}
