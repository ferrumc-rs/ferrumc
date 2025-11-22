use bevy_ecs::prelude::{Query, Res};
use ferrumc_inventories::inventory::Inventory;
use ferrumc_net::SetCreativeModeSlotReceiver;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, error};

pub fn handle(
    receiver: Res<SetCreativeModeSlotReceiver>,
    state: Res<GlobalStateResource>,
    mut query: Query<&mut Inventory>,
) {
    for (event, entity) in receiver.0.try_iter() {
        debug!(
            "Slot {} placed at {} by player {}",
            event.slot, event.slot_index, entity
        );
        if state.0.players.is_connected(entity) {
            if let Ok(mut inventory) = query.get_mut(entity) {
                if event.slot.count.0 == 0 {
                    if let Err(e) =
                        inventory.clear_slot_with_update(event.slot_index as usize, entity)
                    {
                        error!(
                            "Failed to clear slot {} for player {}: {:?}",
                            event.slot_index, entity, e
                        );
                    }
                } else if let Err(e) =
                    inventory.set_item_with_update(event.slot_index as usize, event.slot, entity)
                {
                    error!(
                        "Failed to set item in slot {} for player {}: {:?}",
                        event.slot_index, entity, e
                    );
                }
            }
        }
    }
}
