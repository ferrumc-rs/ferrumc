use std::sync::Arc;

use crate::errors::BinaryError;
use bevy_ecs::prelude::{Entity, Query, Res};
use bevy_math::{IVec2, IVec3};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::block_change_ack::BlockChangeAck;
use ferrumc_net::packets::outgoing::block_update::BlockUpdate;
use ferrumc_net::PlayerActionReceiver;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_id::BlockId;
use ferrumc_world::vanilla_chunk_format::BlockData;
use tracing::{debug, error, trace};

pub fn handle(
    events: Res<PlayerActionReceiver>,
    state: Res<GlobalStateResource>,
    query: Query<(Entity, &StreamWriter)>,
) {
    // https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol?oldid=2773393#Player_Action
    for (event, trigger_eid) in events.0.try_iter() {
        let res: Result<(), BinaryError> = try {
            match event.status.0 {
                0 => {
                    let mut chunk = match state.0.clone().world.load_chunk_owned(
                        IVec2::new(event.location.x >> 4, event.location.z >> 4),
                        "overworld",
                    ) {
                        Ok(chunk) => chunk,
                        Err(e) => {
                            trace!("Chunk not found, generating new chunk: {:?}", e);
                            state
                                .0
                                .clone()
                                .terrain_generator
                                .generate_chunk(IVec2::new(
                                    event.location.x >> 4,
                                    event.location.z >> 4,
                                ))?
                        }
                    };
                    let (relative_x, relative_y, relative_z) = (
                        event.location.x.abs() % 16,
                        event.location.y as i32,
                        event.location.z.abs() % 16,
                    );
                    chunk.set_block(
                        IVec3::new(relative_x, relative_y, relative_z),
                        BlockId::default(),
                    )?;
                    // Save the chunk to disk
                    state.0.world.save_chunk(Arc::new(chunk))?;
                    for (eid, conn) in query {
                        if !state.0.players.is_connected(eid) {
                            continue;
                        }
                        // If the player is the one who placed the block, send the BlockChangeAck packet
                        let block_update_packet = BlockUpdate {
                            location: event.location.clone(),
                            block_id: VarInt::from(BlockId::default()),
                        };
                        conn.send_packet_ref(&block_update_packet)?;
                        if eid == trigger_eid {
                            let ack_packet = BlockChangeAck {
                                sequence: event.sequence,
                            };
                            conn.send_packet_ref(&ack_packet)?;
                        }
                    }
                }

                1 => {
                    debug!("You shouldn't be seeing this in creative mode.");
                }
                _ => {}
            };
        };
        if res.is_err() {
            error!("Error handling player action: {:?}", res);
        }
    }
}
