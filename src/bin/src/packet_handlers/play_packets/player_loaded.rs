use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::synchronize_player_position::SynchronizePlayerPositionPacket;
use ferrumc_net::PlayerLoadedReceiver;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_id::BlockId;
use tracing::warn;

pub fn handle(
    ev: Res<PlayerLoadedReceiver>,
    state: Res<GlobalStateResource>,
    query: Query<(Entity, &Position, &StreamWriter)>,
) {
    for (_, player) in ev.0.try_iter() {
        let Ok((entity, player_pos, conn)) = query.get(player) else {
            warn!("Player position not found in query.");
            continue;
        };
        if !state.0.players.is_connected(entity) {
            warn!(
                "Player {} is not connected, skipping position synchronization.",
                player
            );
            continue;
        }
        let Ok(chunk) = state.0.world.load_chunk(
            player_pos.x as i32 >> 4,
            player_pos.z as i32 >> 4,
            "overworld",
        ) else {
            warn!(
                "Failed to load chunk for player {} at position: ({}, {}, {})",
                player, player_pos.x, player_pos.y, player_pos.z
            );
            continue;
        };
        let head_block = chunk.get_block(
            player_pos.x as i32 % 16,
            player_pos.y as i32,
            player_pos.z as i32 % 16,
        );
        if let Ok(head_block) = head_block {
            if head_block == BlockId(0) {
                tracing::info!(
                    "Player {} loaded at position: ({}, {}, {})",
                    player,
                    player_pos.x,
                    player_pos.y,
                    player_pos.z
                );
            } else {
                tracing::info!(
                    "Player {} loaded at position: ({}, {}, {}) with head block: {:?}",
                    player,
                    player_pos.x,
                    player_pos.y,
                    player_pos.z,
                    head_block
                );
                // get the lowest block we can teleport the player to

                let lowest_y = chunk.real_heightmap[player_pos.x.abs() as usize % 16][player_pos.z.abs() as usize % 16];

                // Teleport the player to the world center if their head block is not air
                let mut packet = SynchronizePlayerPositionPacket::default();
                packet.y = lowest_y as f64 + 1.0; // Adjusting y to be above the ground
                if let Err(e) = conn.send_packet(&packet) {
                    tracing::error!(
                        "Failed to send synchronize player position packet for player {}: {:?}",
                        player,
                        e
                    );
                } else {
                    tracing::info!(
                        "Sent synchronize player position packet for player {}",
                        player
                    );
                }
            }
        } else {
            warn!(
                "Failed to fetch head block for player {} at position: ({}, {}, {})",
                player, player_pos.x, player_pos.y, player_pos.z
            );
        }
    }
}
