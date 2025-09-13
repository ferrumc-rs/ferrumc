use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_core::data::player::PlayerData;
use ferrumc_core::identity::player_identity::PlayerIdentity;
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
    mut query: Query<(Entity, &PlayerIdentity, &mut PlayerData, &StreamWriter)>,
) {
    for (_, player) in ev.0.try_iter() {
        let Ok((entity, player_identity, mut player_data, conn)) = query.get_mut(player) else {
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

        // Default player data
        *player_data = PlayerData::new(
            Position::default(),
            "overworld",
        );

        // Save the player's position in the world
        if let Ok(loaded) = state
            .0
            .world
            .load_player_state(player_identity.uuid.as_u128())
        {
            match loaded {
                Some(loaded_data) => {
                    *player_data = loaded_data;
                    tracing::info!(
                        "Loaded player state for {}: position=({}, {}, {}), dimension={}",
                        player_identity.uuid.as_u128(),
                        player_data.pos.x,
                        player_data.pos.y,
                        player_data.pos.z,
                        player_data.dimension
                    );
                }
                None => {
                    if let Err(e) = state.0.world.save_player_state(player_identity.uuid.as_u128(), &player_data) {
                        tracing::error!(
                            "Failed to save player state for {} ({}): {:?}",
                            player_identity.username,
                            player_identity.uuid.as_u128(),
                            e
                        );
                    }
                }
            }
        } else if let Err(e) = state.0.world.save_player_state(player_identity.uuid.as_u128(), &player_data) {
            tracing::error!(
                "Failed to save player state for {} ({}): {:?}",
                player_identity.username,
                player_identity.uuid.as_u128(),
                e
            );
        }
        let head_block = state.0.world.get_block_and_fetch(
            player_data.pos.x as i32,
            player_data.pos.y as i32,
            player_data.pos.z as i32,
            "overworld",
        );
        if let Ok(head_block) = head_block {
            if head_block == BlockId(0) {
                tracing::info!(
                    "Player {} loaded at position: ({}, {}, {})",
                    player,
                    player_data.pos.x,
                    player_data.pos.y,
                    player_data.pos.z
                );
            } else {
                tracing::info!(
                    "Player {} loaded at position: ({}, {}, {}) with head block: {:?}",
                    player,
                    player_data.pos.x,
                    player_data.pos.y,
                    player_data.pos.z,
                    head_block
                );
                // Teleport the player to the world center if their head block is not air
                let packet = SynchronizePlayerPositionPacket::default();
                if let Err(e) = conn.send_packet_ref(&packet) {
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
                player, player_data.pos.x, player_data.pos.y, player_data.pos.z
            );
        }
    }
}
