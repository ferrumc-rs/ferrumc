use bevy_ecs::message::MessageWriter;
use bevy_ecs::prelude::{DetectChanges, Entity, Query, Ref};
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::velocity::Velocity;
use ferrumc_messages::entity_update::SendEntityUpdate;

pub fn handle(
    mut query: Query<(Entity, Ref<Velocity>, &mut Position)>,
    mut writer: MessageWriter<SendEntityUpdate>,
) {
    for (eid, vel, mut pos) in query.iter_mut() {
        if vel.is_changed() || pos.is_changed() {
            pos.coords += vel.as_dvec3();
            writer.write(SendEntityUpdate(eid));
        }
    }
}
