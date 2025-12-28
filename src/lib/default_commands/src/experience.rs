use bevy_ecs::prelude::*;
use ferrumc_commands::Sender;
use ferrumc_components::player::experience::Experience;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::command;
use ferrumc_text::TextComponent;

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

    let msg = TextComponent::from(format!("You have {levels} experience levels and are {progress}% to the next level."));
    sender.send_message(msg, false);
}
