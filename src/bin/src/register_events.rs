use bevy_ecs::event::EventRegistry;
use bevy_ecs::prelude::World;
use ferrumc_commands::events::{CommandDispatchEvent, ResolvedCommandDispatchEvent};
use ferrumc_core::chunks::cross_chunk_boundary_event::CrossChunkBoundaryEvent;
use ferrumc_core::conn::force_player_recount_event::ForcePlayerRecountEvent;
use ferrumc_core::conn::player_disconnect_event::PlayerDisconnectEvent;
use ferrumc_net::packets::packet_events::TransformEvent;

pub fn register_events(world: &mut World) {
    EventRegistry::register_event::<TransformEvent>(world);
    EventRegistry::register_event::<PlayerDisconnectEvent>(world);
    EventRegistry::register_event::<CrossChunkBoundaryEvent>(world);
    EventRegistry::register_event::<ForcePlayerRecountEvent>(world);
    EventRegistry::register_event::<CommandDispatchEvent>(world);
    EventRegistry::register_event::<ResolvedCommandDispatchEvent>(world);
}
