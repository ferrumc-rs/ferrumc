use crate::errors::BinaryError;
use bevy_ecs::prelude::{Query, Res};
use ferrumc_core::collisions::bounds::CollisionBounds;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::block_change_ack::BlockChangeAck;
use ferrumc_net::packets::outgoing::block_update::BlockUpdate;
use ferrumc_net::PlaceBlockReceiver;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_id::BlockId;
use tracing::{debug, trace};

// Cobblestone block ID for testing purposes
const DUMMY_BLOCK: BlockId = BlockId(14);

pub fn handle(
    events: Res<PlaceBlockReceiver>,
    state: Res<GlobalStateResource>,
    conn_q: Query<&StreamWriter>,
    pos_q: Query<(&Position, &CollisionBounds)>,
) {
    'ev_loop: for (event, eid) in events.0.try_iter() {
        let res: Result<(), BinaryError> = try {
            let Ok(conn) = conn_q.get(eid) else {
                debug!("Could not get connection for entity {:?}", eid);
                continue;
            };
            if !conn.running.load(std::sync::atomic::Ordering::Relaxed) {
                debug!("Connection for entity {:?} is not running", eid);
                continue;
            }
            match event.hand.0 {
                0 => {
                    let mut chunk = match state.0.world.load_chunk(
                        event.position.x >> 4,
                        event.position.z >> 4,
                        "overworld",
                    ) {
                        Ok(chunk) => chunk,
                        Err(e) => {
                            debug!("Failed to load chunk: {:?}", e);
                            continue 'ev_loop;
                        }
                    };
                    let block_clicked = chunk.get_block(
                        event.position.x,
                        event.position.y as i32,
                        event.position.z,
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
                        pos_q.into_iter().any(|(pos, bounds)| {
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
                        continue 'ev_loop;
                    }
                    let packet = BlockChangeAck {
                        sequence: event.sequence,
                    };
                    conn.send_packet(packet)?;

                    chunk.set_block(x & 0xF, y as i32, z & 0xF, DUMMY_BLOCK)?;
                    let ack_packet = BlockChangeAck {
                        sequence: event.sequence,
                    };

                    let chunk_packet = BlockUpdate {
                        location: NetworkPosition { x, y, z },
                        block_id: VarInt::from(DUMMY_BLOCK),
                    };
                    conn.send_packet(chunk_packet)?;
                    conn.send_packet(ack_packet)?;

                    state.0.world.save_chunk(chunk)?;
                }
                1 => {
                    trace!("Offhand block placement not implemented");
                }
                _ => {
                    debug!("Invalid hand");
                }
            }
        };
        if let Err(e) = &res {
            debug!("Failed to handle place block: {:?}", e);
        }
    }
}
