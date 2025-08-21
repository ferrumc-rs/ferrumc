use bevy_ecs::event::EventRegistry;
use bevy_ecs::prelude::World;
use ferrumc_core::chunks::cross_chunk_boundary_event::CrossChunkBoundaryEvent;
use ferrumc_core::conn::conn_kill_event::ConnectionKillEvent;
use ferrumc_core::conn::force_player_recount_event::ForcePlayerRecountEvent;
use ferrumc_entities::events::SpawnEntityEvent;
use ferrumc_net::packets::packet_events::TransformEvent;

pub fn register_events(world: &mut World) {
    EventRegistry::register_event::<TransformEvent>(world);
    EventRegistry::register_event::<ConnectionKillEvent>(world);
    EventRegistry::register_event::<CrossChunkBoundaryEvent>(world);
    EventRegistry::register_event::<ForcePlayerRecountEvent>(world);

    // idk if they should be added here, but just for testing purposes:
    EventRegistry::register_event::<SpawnEntityEvent>(world);
}
