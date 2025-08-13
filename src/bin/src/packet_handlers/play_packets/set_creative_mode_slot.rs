use bevy_ecs::prelude::Res;
use ferrumc_net::SetCreativeModeSlotReceiver;
use ferrumc_state::GlobalStateResource;
use tracing::debug;

pub fn handle(
    events: Res<SetCreativeModeSlotReceiver>,
    state: Res<GlobalStateResource>,
) {
    for (event, entity) in events.0.try_iter() {
        debug!("Slot {} placed at {} by player {}",
            event.slot, event.slot_index, entity);
    }
}