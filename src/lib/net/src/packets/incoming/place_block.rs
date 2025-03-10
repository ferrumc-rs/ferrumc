use crate::connection::StreamWriter;
use crate::packets::outgoing::block_change_ack::BlockChangeAck;
use crate::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::collisions::bounds::CollisionBounds;
use ferrumc_core::transform::position::Position;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use ferrumc_world::vanilla_chunk_format::BlockData;
use std::sync::Arc;
use tracing::{debug, trace};

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
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        match self.hand.val {
            0 => {
                debug!("Placing block at {:?}", self.position);
                let block_clicked = state
                    .clone()
                    .world
                    .get_block_and_fetch(
                        self.position.x,
                        self.position.y as i32,
                        self.position.z,
                        "overworld",
                    )
                    .await?;
                trace!("Block clicked: {:?}", block_clicked);
                // Use the face to determine the offset of the block to place
                let (x_block_offset, y_block_offset, z_block_offset) = match self.face.val {
                    0 => (0, -1, 0),
                    1 => (0, 1, 0),
                    2 => (0, 0, -1),
                    3 => (0, 0, 1),
                    4 => (-1, 0, 0),
                    5 => (1, 0, 0),
                    _ => (0, 0, 0),
                };
                let (x, y, z) = (
                    self.position.x + x_block_offset,
                    self.position.y + y_block_offset,
                    self.position.z + z_block_offset,
                );
                // Check if the block collides with any entities
                let does_collide = {
                    let q = state.universe.query::<(&Position, &CollisionBounds)>();
                    q.into_iter().any(|(_, (pos, bounds))| {
                        bounds.collides(
                            (pos.x, pos.y, pos.z),
                            &CollisionBounds {
                                x_offset_start: 0.0,
                                x_offset_end: 1.0,
                                y_offset_start: 0.0,
                                y_offset_end: 1.0,
                                z_offset_start: 0.0,
                                z_offset_end: 1.0,
                            },
                            (x as f64, y as f64, z as f64),
                        )
                    })
                };
                if does_collide {
                    trace!("Block placement collided with entity");
                    return Ok(());
                }
                {
                    if let Ok(mut conn) = state.universe.get_mut::<StreamWriter>(conn_id) {
                        let packet = BlockChangeAck {
                            sequence: self.sequence.clone(),
                        };
                        conn.send_packet(packet, &NetEncodeOpts::WithLength)?;
                    } else {
                        debug!("Could not get StreamWriter");
                    }
                }
                let mut chunk = state.world.load_chunk(x >> 4, z >> 4, "overworld").await?;

                chunk.set_block(
                    x & 0xF,
                    y as i32,
                    z & 0xF,
                    BlockData {
                        name: "minecraft:stone".to_string(),
                        properties: None,
                    },
                )?;
                let ack_packet = BlockChangeAck {
                    sequence: self.sequence.clone(),
                };
                let chunk_packet = ChunkAndLightData::from_chunk(&chunk)?;
                let mut conn = state.universe.get_mut::<StreamWriter>(conn_id)?;
                conn.send_packet(chunk_packet, &NetEncodeOpts::WithLength)?;
                conn.send_packet(ack_packet, &NetEncodeOpts::WithLength)?;

                state.world.save_chunk(chunk).await?;
                state.world.sync().await?;
            }
            1 => {
                trace!("Offhand block placement not implemented");
            }
            _ => {
                debug!("Invalid hand");
            }
        }
        Ok(())
    }
}
