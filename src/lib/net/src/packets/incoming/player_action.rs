use crate::connection::StreamWriter;
use crate::errors::NetError;
use crate::packets::outgoing::block_change_ack::BlockChangeAck;
use crate::packets::outgoing::block_update::BlockUpdate;
use crate::packets::IncomingPacket;

use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use ferrumc_world::chunk_format::BLOCK2ID;
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
    fn handle(self, conn_id: usize, state: Arc<ServerState>) -> Result<(), NetError> {
        // https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol?oldid=2773393#Player_Action
        match self.status.0 {
            0 => {
                let mut chunk = state.clone().world.load_chunk(
                    self.location.x >> 4,
                    self.location.z >> 4,
                    "overworld",
                )?;
                let block =
                    chunk.get_block(self.location.x, self.location.y as i32, self.location.z)?;
                debug!("Block: {:?}", block);
                let (relative_x, relative_y, relative_z) = (
                    self.location.x & 0xF,
                    self.location.y as i32,
                    self.location.z & 0xF,
                );
                chunk.set_block(
                    relative_x & 0xf,
                    relative_y,
                    relative_z & 0xf,
                    BlockData::default(),
                )?;
                // Save the chunk to disk
                state.world.save_chunk(chunk.clone())?;
                state.world.sync()?;
                {
                    // Send the block update packet to all players
                    let query = state
                        .universe
                        .query::<(&mut StreamWriter, &mut ChunkReceiver)>()
                        .into_entities();
                    for entity_id in query {
                        if let Ok(mut connection) = state.universe.get_mut::<StreamWriter>(conn_id)
                        {
                            // Don't send the block update packet if the player can't see the chunk
                            if let Ok(chunk_recv) = state.universe.get::<ChunkReceiver>(entity_id) {
                                if chunk_recv.needs_reload.contains(&(
                                    self.location.x >> 4,
                                    self.location.z >> 4,
                                    "overworld".to_string(),
                                )) {
                                    let block_update_packet = BlockUpdate {
                                        location: self.location.clone(),
                                        block_id: VarInt::from(*BLOCK2ID.get(&BlockData::default()).expect(
                                            "BlockData::default() should always have a corresponding block ID",
                                        )),
                                    };
                                    connection.send_packet(
                                        block_update_packet,
                                        &NetEncodeOpts::WithLength,
                                    )?;
                                }
                            }

                            // If the player is the one who placed the block, send the BlockChangeAck packet
                            // We do this here to avoid locking the streamwriter multiple times
                            if entity_id == conn_id {
                                let ack_packet = BlockChangeAck {
                                    sequence: self.sequence.clone(),
                                };
                                connection.send_packet(ack_packet, &NetEncodeOpts::WithLength)?;
                            }
                        } else {
                            debug!("Player disconnected before we could send the BlockChangeAck packet");
                        }
                    }
                }
            }
            1 => {
                debug!("You shouldn't be seeing this in creative mode.");
            }
            _ => {}
        };
        Ok(())
    }
}
