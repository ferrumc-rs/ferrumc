use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_core::data::player::PlayerData;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::synchronize_player_position::SynchronizePlayerPositionPacket;
use ferrumc_net::PlayerLoadedReceiver;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_state_id::BlockStateId;
use tracing::warn;

pub fn handle(
    ev: Res<PlayerLoadedReceiver>,
    state: Res<GlobalStateResource>,
    mut query: Query<(Entity, &PlayerIdentity, &StreamWriter, &mut Position)>,
) {
    for (_, player) in ev.0.try_iter() {
        let Ok((entity, player_identity, conn, mut position)) = query.get_mut(player) else {
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

        // Save the player's position in the world
        match state
            .0
            .world
            .load_player_state(player_identity.uuid.as_u128())
        {
            Ok(loaded) => match loaded {
                Some(loaded_data) => {
                    *position =
                        Position::new(loaded_data.pos.x, loaded_data.pos.y, loaded_data.pos.z);
                    tracing::info!(
                        "Loaded player state for {}: {}",
                        player_identity.uuid.as_u128(),
                        loaded_data,
                    );
                }
                None => {
                    if let Err(e) = state
                        .0
                        .world
                        .save_player_state(player_identity.uuid.as_u128(), &PlayerData::default())
                    // First time saving player data
                    {
                        tracing::error!(
                            "Failed to save player state for {:?}: {:?}",
                            player_identity,
                            e
                        );
                    }
                }
            },
            Err(e) => {
                tracing::error!(
                    "Failed to load player state for {:?}: {:?}",
                    player_identity,
                    e
                );
                if let Err(e) = state
                    .0
                    .world
                    .save_player_state(player_identity.uuid.as_u128(), &PlayerData::default())
                // First time saving player data
                {
                    tracing::error!(
                        "Failed to save player state for {:?}: {:?}",
                        player_identity,
                        e
                    );
                }
            }
        }
        let head_block = state.0.world.get_block_and_fetch(
            position.x as i32,
            position.y as i32,
            position.z as i32,
            "overworld",
        );
        if let Ok(head_block) = head_block {
            if head_block == BlockId(0) {
                tracing::info!("Player {} loaded at position: {:?}", player, position);
            } else {
                tracing::info!(
                    "Player {} loaded at position: {:?} with head block: {:?}",
                    player,
                    position,
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
                "Failed to fetch head block for player {} at position: {:?}",
                player, position
            );
        }
    }
}
