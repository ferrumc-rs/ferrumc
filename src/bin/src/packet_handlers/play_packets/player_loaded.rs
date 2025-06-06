use bevy_ecs::prelude::{Query, Res};
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::synchronize_player_position::SynchronizePlayerPositionPacket;
use ferrumc_net::PlayerLoadedReceiver;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_id::BlockId;
use std::sync::atomic::Ordering;

pub fn handle(
    ev: Res<PlayerLoadedReceiver>,
    state: Res<GlobalStateResource>,
    query: Query<(&Position, &StreamWriter)>,
) {
    for (_, player) in ev.0.try_iter() {
        let Ok((player_pos, conn)) = query.get(player) else {
            tracing::warn!("Player position not found in query.");
            continue;
        };
        if !conn.running.load(Ordering::Relaxed) {
            tracing::warn!("Connection for player {} is not running.", player);
            continue;
        }
        let head_block = state.0.world.get_block_and_fetch(
            player_pos.x as i32,
            player_pos.y as i32,
            player_pos.z as i32,
            "overworld",
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
                // Teleport the player to the world center if their head block is not air
                let packet = SynchronizePlayerPositionPacket::default();
                if let Err(e) = conn.send_packet(packet) {
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
            tracing::warn!(
                "Failed to fetch head block for player {} at position: ({}, {}, {})",
                player,
                player_pos.x,
                player_pos.y,
                player_pos.z
            );
        }
    }
}
