use bevy_ecs::message::MessageReader;
use bevy_ecs::query::With;
use bevy_ecs::system::Query;
use ferrumc_components::player::abilities::PlayerAbilities;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_messages::ClearPlayerInventory;

pub fn handle_clear_player_inventory(
    mut events: MessageReader<ClearPlayerInventory>,
    mut player_inventories: Query<&mut Inventory, With<PlayerAbilities>>, // querying for PlayerAbilities in order to check if player??
) {
    for event in events.read() {
        let Ok(mut inventory) = player_inventories.get_mut(event.player) else {return};
        inventory.clear_with_update(event.player);
        dbg!(&inventory);
    }
}