use bevy_ecs::prelude::{Query, Res};
use ferrumc_components::player::teleport_tracker::TeleportTracker;
use ferrumc_net::ConfirmPlayerTeleportReceiver;

pub fn handle(
    receiver: Res<ConfirmPlayerTeleportReceiver>,
    mut query: Query<&mut TeleportTracker>,
) {
    for (_, eid) in receiver.0.try_iter() {
        let Ok(mut tracker) = query.get_mut(eid) else {
            continue;
        };
        tracker.waiting_for_confirm = false;
    }
}
