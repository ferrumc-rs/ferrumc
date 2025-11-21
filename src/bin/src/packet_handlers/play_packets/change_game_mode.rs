use bevy_ecs::prelude::*;
use ferrumc_components::player::gamemode::GameMode;
use ferrumc_net::ChangeGameModeReceiver;
use tracing::warn;

use ferrumc_messages::PlayerGameModeChanged;

pub fn handle(
    events: Res<ChangeGameModeReceiver>,
    mut gamemode_events: MessageWriter<PlayerGameModeChanged>,
) {
    for (packet, sender_entity) in events.0.try_iter() {
        // 1. Parse the gamemode ID from the packet
        let new_mode = match packet.gamemode.0 {
            0 => GameMode::Survival,
            1 => GameMode::Creative,
            2 => GameMode::Adventure,
            3 => GameMode::Spectator,
            _ => {
                warn!(
                    "Player {:?} sent an invalid gamemode ID: {}",
                    sender_entity, packet.gamemode.0
                );
                continue;
            }
        };

        // 2. Fire the event
        gamemode_events.write(PlayerGameModeChanged {
            player: sender_entity,
            new_mode,
        });
    }
}
