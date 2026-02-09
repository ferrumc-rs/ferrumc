use bevy_ecs::prelude::{Query, ResMut, Resource};
use ferrumc_blocks::BLOCK_MAPPINGS;
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::block_update::BlockUpdate;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::pos::BlockPos;
use tracing::error;
use ferrumc_components::player::client_information::ClientInformation;

#[derive(Resource, Default)]
pub struct BlockUpdates(Vec<BlockPos>);

impl BlockUpdates {
    pub fn queue_block_update(&mut self, pos: BlockPos) {
        self.0.push(pos);
    }
}

pub fn handle_block_updates(
    state: ResMut<GlobalStateResource>,
    mut updates: ResMut<BlockUpdates>,
    players: Query<(&StreamWriter, &Position, &ClientInformation)>,
) {
    let mut packets = Vec::new();

    for update_pos in updates.0.drain(..) {
        let Ok(block) = state.0.world.get_block_and_fetch(update_pos, "overworld") else {
            error!("Failed to fetch block at {} for block update", update_pos);
            continue;
        };

        let updated_block = BLOCK_MAPPINGS[block.raw() as usize].update(&state.0.world, update_pos);

        if block.raw() != updated_block {
            if let Err(err) = state.0.world.set_block_and_fetch(
                update_pos,
                "overworld",
                BlockStateId::new(updated_block),
            ) {
                error!(
                    "Failed to set block at {} for block update: {}",
                    update_pos, err
                );
                continue;
            }

            let packet = BlockUpdate {
                location: NetworkPosition {
                    x: update_pos.pos.x,
                    y: update_pos.pos.y as _,
                    z: update_pos.pos.z,
                },
                block_state_id: VarInt::new(updated_block as _),
            };

            packets.push((packet, update_pos));
        }
    }

    for (conn, player_pos, player_info) in players.iter() {
        let render_distance = get_global_config().chunk_render_distance.min(player_info.view_distance as _);

        for (packet, update_pos) in packets.iter() {
            let update_chunk = update_pos.chunk();
            let player_chunk = player_pos.chunk();

            if (update_chunk.x() - player_chunk.x).unsigned_abs() > render_distance
                && (update_chunk.z() - player_chunk.y).unsigned_abs() > render_distance
            {
                continue; // Player is out of render distance of the chunk so dont send the block update
            }

            if let Err(err) = conn.send_packet_ref(packet) {
                error!("Failed to send packet for block update: {}", err);
            }
        }
    }
}
