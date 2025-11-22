use bevy_ecs::event::EventRegistry;
use bevy_ecs::prelude::World;
use ferrumc_commands::events::{CommandDispatchEvent, ResolvedCommandDispatchEvent};
use ferrumc_core::chunks::cross_chunk_boundary_event::CrossChunkBoundaryEvent;
use ferrumc_core::conn::force_player_recount_event::ForcePlayerRecountEvent;
use ferrumc_events::*;
use ferrumc_net::packets::packet_events::TransformEvent;

pub fn register_events(world: &mut World) {
    EventRegistry::register_event::<TransformEvent>(world);
    EventRegistry::register_event::<CrossChunkBoundaryEvent>(world);
    EventRegistry::register_event::<ForcePlayerRecountEvent>(world);
    EventRegistry::register_event::<CommandDispatchEvent>(world);
    EventRegistry::register_event::<ResolvedCommandDispatchEvent>(world);
    // classic events
    EventRegistry::register_event::<PlayerLeaveEvent>(world);
    EventRegistry::register_event::<PlayerJoinEvent>(world);
    EventRegistry::register_event::<PlayerDamageEvent>(world);
    EventRegistry::register_event::<PlayerDeathEvent>(world);
    EventRegistry::register_event::<PlayerStartDiggingEvent>(world);
    EventRegistry::register_event::<PlayerCancelDiggingEvent>(world);
    EventRegistry::register_event::<PlayerFinishDiggingEvent>(world);
    EventRegistry::register_event::<PlayerEatEvent>(world);
    EventRegistry::register_event::<PlayerXPGainEvent>(world);
    EventRegistry::register_event::<PlayerLevelUpEvent>(world);
    EventRegistry::register_event::<ChangeGameModeEvent>(world);
    EventRegistry::register_event::<SpawnEntityEvent>(world);
}
