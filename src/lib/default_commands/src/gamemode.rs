use bevy_ecs::prelude::*;
use ferrumc_commands::Sender;
use ferrumc_components::player::abilities::PlayerAbilities;
use ferrumc_components::player::gamemode::{GameMode, GameModeComponent};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::command;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::game_event::GameEventPacket; // For changing the client's UI
use ferrumc_net::packets::outgoing::player_abilities::PlayerAbilities as OutgoingPlayerAbilities;
use ferrumc_text::TextComponentBuilder;
use tracing::{error, info};

/// Sets the sender's gamemode.
#[command("gamemode")]
#[allow(unused_mut)] // For the `player_query`
fn gamemode_command(
    #[sender] sender: Sender,
    #[arg] new_gamemode: GameMode,
    mut player_query: Query<(
        Entity,
        &PlayerIdentity,
        &mut PlayerAbilities,
        &mut GameModeComponent,
        &StreamWriter,
    )>,
) {
    // 1. Ensure the sender is a player
    let player_entity = match sender {
        Sender::Server => {
            sender.send_message("Error: The server can't change gamemode.".into(), false);
            return;
        }
        Sender::Player(entity) => entity,
    };

    // 2. Get all components for that player
    let (_entity, identity, mut abilities, mut gamemode, writer) =
        match player_query.get_mut(player_entity) {
            Ok(components) => components,
            Err(e) => {
                error!(
                    "Gamemode: Failed to get components for player {:?}: {:?}",
                    player_entity, e
                );
                sender.send_message(
                    "Error: Could not find your player components.".into(),
                    false,
                );
                return;
            }
        };

    // 3. Update the GameMode component
    gamemode.0 = new_gamemode;

    // 4. Update PlayerAbilities to match the new gamemode
    new_gamemode.update_abilities(&mut abilities);

    // 5. Send packets to sync the client

    // 5a. Send the Game Event packet (this changes the UI)
    let gamemode_packet = GameEventPacket::new(3, new_gamemode as u8 as f32);
    if let Err(e) = writer.send_packet_ref(&gamemode_packet) {
        error!(
            "Failed to send gamemode change packet to {}: {:?}",
            identity.username, e
        );
    }

    // 5b. Send the Abilities packet (this allows/disallows flight)
    let abilities_packet = OutgoingPlayerAbilities::from_abilities(&abilities);
    if let Err(e) = writer.send_packet_ref(&abilities_packet) {
        error!(
            "Failed to send abilities sync packet to {}: {:?}",
            identity.username, e
        );
    }

    // 6. Send a confirmation message
    let mode_name = match new_gamemode {
        GameMode::Survival => "Survival",
        GameMode::Creative => "Creative",
        GameMode::Adventure => "Adventure",
        GameMode::Spectator => "Spectator",
    };

    sender.send_message(
        TextComponentBuilder::new(format!("Set gamemode to {}", mode_name)).build(),
        false,
    );

    info!(
        "Set gamemode for {} to {:?}",
        identity.username, new_gamemode
    );
}
