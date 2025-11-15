use bevy_ecs::prelude::*;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::player::abilities::PlayerAbilities;
use ferrumc_core::player::gamemode::{GameMode, GameModeComponent};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::game_event::GameEventPacket;
use ferrumc_net::packets::outgoing::player_abilities::PlayerAbilities as OutgoingPlayerAbilities;
use ferrumc_net::ChangeGameModeReceiver;
use tracing::{error, info, warn};

pub fn handle(
    events: Res<ChangeGameModeReceiver>,
    mut player_query: Query<(
        Entity,
        &PlayerIdentity,
        &mut PlayerAbilities,
        &mut GameModeComponent,
        &StreamWriter,
    )>,
) {
    for (packet, sender_entity) in events.0.try_iter() {
        // 3. Get all components for the player who sent the packet
        let (_entity, identity, mut abilities, mut gamemode, writer) =
            match player_query.get_mut(sender_entity) {
                Ok(components) => components,
                Err(e) => {
                    warn!(
                        "Received ChangeGameMode from entity {:?} without components: {:?}",
                        sender_entity, e
                    );
                    continue; // Skip this event
                }
            };

        // Parse the gamemode ID from the packet
        let new_mode = match packet.gamemode.0 {
            0 => GameMode::Survival,
            1 => GameMode::Creative,
            2 => GameMode::Adventure,
            3 => GameMode::Spectator,
            _ => {
                warn!(
                    "Player {} sent and invalid gamemode ID: {}",
                    identity.username, packet.gamemode.0
                );
                continue;
            }
        };

        // Update the server-side GameMode component
        gamemode.0 = new_mode;

        // Update the server-side PlayerAbilities to match
        new_mode.update_abilities(&mut abilities);

        // Send confirmation packets back to the client
        let gamemode_packet = GameEventPacket::new(3, new_mode as u8 as f32);
        if let Err(e) = writer.send_packet_ref(&gamemode_packet) {
            error!(
                "Failed to send gamemode change packet to {}: {:?}",
                identity.username, e
            );
        }

        // Abilities packet (allows/disallows flight)
        let abilities_packet = OutgoingPlayerAbilities::from_abilities(&abilities);
        if let Err(e) = writer.send_packet_ref(&abilities_packet) {
            error!(
                "Failed to send abilities sync packet to {}: {:?}",
                identity.username, e
            );
        }

        info!(
            "Set gamemode for {} to {:?} via packet",
            identity.username, new_mode
        );
    }
}
