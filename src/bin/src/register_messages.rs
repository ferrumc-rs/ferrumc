use bevy_ecs::message::MessageRegistry;
use bevy_ecs::prelude::World;
use ferrumc_core::chunks::cross_chunk_boundary_event::CrossChunkBoundaryEvent;
use ferrumc_core::conn::force_player_recount_event::ForcePlayerRecountEvent;
use ferrumc_net::packets::packet_events::TransformEvent;

pub fn register_messages(world: &mut World) {
    MessageRegistry::register_message::<TransformEvent>(world);
    MessageRegistry::register_message::<CrossChunkBoundaryEvent>(world);
    MessageRegistry::register_message::<ForcePlayerRecountEvent>(world);
}
