use bevy_ecs::prelude::*;
use ferrumc_commands::arg::primitive::int::Integer;
use ferrumc_commands::Sender;
use ferrumc_components::player::experience::Experience;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::command;
use ferrumc_messages::PlayerGainedXP;
use ferrumc_text::TextComponent;

/// Adds experience points to the sender.
#[command("experience add")]
fn experience_add_command(
    #[sender] sender: Sender,
    #[arg] amount: Integer,
    mut gained_xp_events: MessageWriter<PlayerGainedXP>,
) {
    // 1. Ensure the sender is a player
    let player_entity = match sender {
        Sender::Server => {
            sender.send_message("Error: The server can't change its XP level.".into(), false);
            return;
        },
        Sender::Player(entity) => entity,
    };

    let amount = *amount as u32;

    // 2. Fire the event
    gained_xp_events.write(PlayerGainedXP {
        player: player_entity,
        amount,
    });

    let msg = TextComponent::from(format!("Added {amount} experience points to player."));
    sender.send_message(msg, false);
}

/// Returns information about the experience points & levels of the sender.
#[command("experience query")]
fn experience_query_command(
    #[sender] sender: Sender,
    xp_players: Query<&Experience, With<PlayerIdentity>>,
) {
    let player_entity = match sender {
        Sender::Server => {
            sender.send_message("Error: The server doesn't have an XP level.".into(), false);
            return;
        },
        Sender::Player(entity) => entity,
    };

    let Ok(xp) = xp_players.get(player_entity) else {return};
    let levels = xp.level;
    let progress = xp.progress * 100.0;

    let msg = TextComponent::from(format!("You have {levels} experience levels and are {progress:.0}% to the next level."));
    sender.send_message(msg, false);
}

#[command("xp add")]
fn xp_add_command(
    #[sender] sender: Sender,
    #[arg] amount: Integer,
    gained_xp_events: MessageWriter<PlayerGainedXP>,
) {
    experience_add_command(sender, amount, gained_xp_events);
}

#[command("xp query")]
fn xp_query_command(
    #[sender] sender: Sender,
    xp_players: Query<&Experience, With<PlayerIdentity>>,
) {
    experience_query_command(sender, xp_players);
}

