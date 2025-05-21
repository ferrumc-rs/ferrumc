use crate::errors::BinaryError;
use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::block_change_ack::BlockChangeAck;
use ferrumc_net::packets::outgoing::block_update::BlockUpdate;
use ferrumc_net::PlayerActionReceiver;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::chunk_format::BLOCK2ID;
use ferrumc_world::vanilla_chunk_format::BlockData;
use tracing::{debug, error};

pub fn handle(
    events: Res<PlayerActionReceiver>,
    state: Res<GlobalStateResource>,
    query: Query<(Entity, &StreamWriter, &ChunkReceiver)>,
) {
    if events.0.is_empty() {
        return;
    }
    // https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol?oldid=2773393#Player_Action
    for (event, trigger_eid) in events.0.try_iter() {
        let res: Result<(), BinaryError> = try {
            match event.status.0 {
                0 => {
                    let mut chunk = state.0.clone().world.load_chunk(
                        event.location.x >> 4,
                        event.location.z >> 4,
                        "overworld",
                    )?;
                    let block =
                        chunk.get_block(event.location.x, event.location.y as i32, event.location.z)?;
                    debug!("Block: {:?}", block);
                    let (relative_x, relative_y, relative_z) = (
                        event.location.x & 0xF,
                        event.location.y as i32,
                        event.location.z & 0xF,
                    );
                    chunk.set_block(
                        relative_x & 0xf,
                        relative_y,
                        relative_z & 0xf,
                        BlockData::default(),
                    )?;
                    // Save the chunk to disk
                    state.0.world.save_chunk(chunk.clone())?;
                    state.0.world.sync()?;
                    for (eid, conn, chunk_recv) in query {
                        // Don't send the block update packet if the player can't see the chunk
                        if chunk_recv.needs_reload.contains(&(
                            event.location.x >> 4,
                            event.location.z >> 4,
                            "overworld".to_string(),
                        )) {
                            let block_update_packet = BlockUpdate {
                                location: event.location.clone(),
                                block_id: VarInt::from(*BLOCK2ID.get(&BlockData::default()).expect(
                                    "BlockData::default() should always have a corresponding block ID",
                                )),
                            };
                            conn.send_packet(block_update_packet)?;
                        }

                        // If the player is the one who placed the block, send the BlockChangeAck packet
                        // We do this here to avoid locking the streamwriter multiple times
                        if trigger_eid == eid {
                            let ack_packet = BlockChangeAck {
                                sequence: event.sequence.clone(),
                            };
                            conn.send_packet(ack_packet)?;
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
