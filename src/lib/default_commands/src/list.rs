use bevy_ecs::system::Query;
use ferrumc_commands::Sender;
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::command;
use ferrumc_text::TextComponent;

#[command("list")]
fn list_command(
    #[sender] sender: Sender,
    player_names: Query<&PlayerIdentity>,
) {
    let mut players = Vec::new();
    let mut playerlist = String::new();
    for player in player_names.iter() {
        players.push(&player.username);
        playerlist.push_str(&player.username);
        playerlist.push_str(", ");
    };
    let max = get_global_config().max_players;
    let total = players.len();
    let msg = if playerlist.is_empty() {
        TextComponent::from(format!("There are no players online, out of a maximum of {max}."))
    } else {
        playerlist.pop(); // remove the trailing ', '
        playerlist.pop();
        TextComponent::from(format!("There are {total} of a max of {max} players online: {playerlist}"))
    };

    sender.send_message(msg, false);
}