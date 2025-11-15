use bevy_ecs::prelude::{Query, Res};
use ferrumc_inventories::hotbar::Hotbar;
use ferrumc_net::SetHeldItemReceiver;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, error};

pub fn handle(
    receiver: Res<SetHeldItemReceiver>,
    state: Res<GlobalStateResource>,
    mut query: Query<&mut Hotbar>,
) {
    for (event, entity) in receiver.0.try_iter() {
        if state.0.players.is_connected(entity) {
            if 0 <= event.slot_index && event.slot_index < 9 {
                if let Ok(mut hotbar) = query.get_mut(entity) {
                    hotbar.selected_slot = event.slot_index as u8;
                    debug!(
                        "Set held item for player {} to slot {}",
                        entity, event.slot_index
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
