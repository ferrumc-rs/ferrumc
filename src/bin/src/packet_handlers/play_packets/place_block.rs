use bevy_ecs::prelude::Query;
use ferrumc_core::collisions::bounds::CollisionBounds;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::block_change_ack::BlockChangeAck;
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net::PlaceBlockReceiver;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::ServerState;
use ferrumc_world::vanilla_chunk_format::BlockData;
use std::sync::Arc;
use tracing::{debug, trace};

pub fn handle(events: PlaceBlockReceiver, state: Arc<ServerState>, mut conn_q: Query<(&StreamWriter)>, pos_q: Query<(&Position, &CollisionBounds)>) {
    for (event, eid) in events.0 {
        let Ok(mut conn) = conn_q.get_mut(eid) else {
            debug!("Could not get connection for entity {:?}", eid);
            continue;
        };
        match event.hand.0 {
            0 => {
                debug!("Placing block at {:?}", event.position);
                let block_clicked = state.clone().world.get_block_and_fetch(
                    event.position.x,
                    event.position.y as i32,
                    event.position.z,
                    "overworld",
                )?;
                trace!("Block clicked: {:?}", block_clicked);
                // Use the face to determine the offset of the block to place
                let (x_block_offset, y_block_offset, z_block_offset) = match event.face.0 {
                    0 => (0, -1, 0),
                    1 => (0, 1, 0),
                    2 => (0, 0, -1),
                    3 => (0, 0, 1),
                    4 => (-1, 0, 0),
                    5 => (1, 0, 0),
                    _ => (0, 0, 0),
                };
                let (x, y, z) = (
                    event.position.x + x_block_offset,
                    event.position.y + y_block_offset,
                    event.position.z + z_block_offset,
                );
                // Check if the block collides with any entities
                let does_collide = {
                    pos_q.into_iter().any(|((pos, bounds))| {
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
                    let packet = BlockChangeAck {
                        sequence: event.sequence.clone(),
                    };
                    conn.send_packet(packet, &NetEncodeOpts::WithLength)?;
                }
                let mut chunk = state.world.load_chunk(x >> 4, z >> 4, "overworld")?;

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
                    sequence: event.sequence.clone(),
                };
                // Make this use the much more efficient block change packet
                let chunk_packet = ChunkAndLightData::from_chunk(&chunk)?;
                conn.send_packet(chunk_packet, &NetEncodeOpts::WithLength)?;
                conn.send_packet(ack_packet, &NetEncodeOpts::WithLength)?;

                state.world.save_chunk(chunk)?;
                state.world.sync()?;
            }
            1 => {
                trace!("Offhand block placement not implemented");
            }
            _ => {
                debug!("Invalid hand");
            }
        }
    }
}