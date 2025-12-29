use bevy_ecs::message::MessageWriter;
use bevy_ecs::system::Query;
use ferrumc_commands::arg::primitive::int::Integer;
use ferrumc_commands::Sender;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_data::items::Item;
use ferrumc_macros::command;
use ferrumc_messages::GiveItemToPlayer;
use ferrumc_text::TextComponent;

#[command("give")]
fn give_command(
    #[sender] sender: Sender,
    #[arg] item: Item,
    #[arg] quantity: Integer,
    args: (Query<&PlayerIdentity>, MessageWriter<GiveItemToPlayer>),
) {
    let player_identities = args.0;
    let mut give_events = args.1;

    let item_name = item.registry_key;
    let player_entity = match sender {
        Sender::Server => {
            sender.send_message("Error: server does not have an inventory.".into(), false);
            return;
        }
        Sender::Player(entity) => entity,
    };
    let quantity = *quantity as u32;
    let username = &player_identities.get(player_entity).unwrap().username;
    let msg = TextComponent::from(format!("Gave {quantity} {item_name} to {username}"));
    sender.send_message(msg, false);
    give_events.write(GiveItemToPlayer {
        item_id: item.id,
        player: player_entity,
        quantity,
    });
}
