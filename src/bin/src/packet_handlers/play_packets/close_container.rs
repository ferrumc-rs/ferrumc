use bevy_ecs::system::{Query, Res};
use ferrumc_inventories::defined_slots;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_net::CloseContainerReceiver;
use tracing::{debug, error};

/// Called when a player closes a container.
///
/// For now, all it does is check if that container is the player inventory,
/// and then clears the player's crafting grid if so.
pub fn handle(receiver: Res<CloseContainerReceiver>, mut inventories: Query<&mut Inventory>) {
    for (event, eid) in receiver.0.try_iter() {
        // 0 is the player's inventory
        if event.window_id.0 == 0 {
            debug!("Clearing crafting grid");

            if let Ok(mut inventory) = inventories.get_mut(eid) {
                // CRAFT_SLOT_OUTPUT is slot 0 and CRAFT_SLOT 1-4 are slots 1-4, this clears all the survival inventory crafting grid slots
                for slot_id in
                    defined_slots::player::CRAFT_SLOT_OUTPUT..=defined_slots::player::CRAFT_SLOT_4
                {
                    inventory
                        .clear_slot_with_update(slot_id as _, eid)
                        .unwrap_or_else(|err| error!("Failed to clear player inventory: {}", err))
                }
            }
        }
    }
}
