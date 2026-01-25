use bevy_ecs::prelude::{MessageWriter, Query, Res};
use ferrumc_inventories::hotbar::Hotbar;
use ferrumc_messages::inventory::HeldItemChanged;
use ferrumc_net::SetHeldItemReceiver;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, error};

pub fn handle(
    receiver: Res<SetHeldItemReceiver>,
    state: Res<GlobalStateResource>,
    mut query: Query<&mut Hotbar>,
    mut held_events: MessageWriter<HeldItemChanged>,
) {
    for (event, entity) in receiver.0.try_iter() {
        if state.0.players.is_connected(entity) {
            if 0 <= event.slot_index && event.slot_index < 9 {
                if let Ok(mut hotbar) = query.get_mut(entity) {
                    let old_slot = hotbar.selected_slot;
                    let new_slot = event.slot_index as u8;

                    hotbar.selected_slot = new_slot;

                    // Fire the HeldItemChanged message for plugins and equipment broadcast
                    held_events.write(HeldItemChanged {
                        player: entity,
                        old_slot,
                        new_slot,
                    });

                    debug!(
                        "Set held item for player {} to slot {} (was {})",
                        entity, new_slot, old_slot
                    );
                } else {
                    error!("Could not find hotbar for player {}", entity);
                }
            } else {
                error!(
                    "Invalid slot index {} for player {}",
                    event.slot_index, entity
                );
            }
        } else {
            error!("Player {} is not connected", entity);
        }
    }
}
