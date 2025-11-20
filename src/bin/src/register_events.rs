use bevy_ecs::event::EventRegistry;
use bevy_ecs::prelude::World;
use ferrumc_commands::events::{CommandDispatchEvent, ResolvedCommandDispatchEvent};
use ferrumc_messages::connection::monitoring::ForcePlayerRecountEvent;
use ferrumc_messages::player::lifecycle::{PlayerJoinEvent, PlayerLeaveEvent};
use ferrumc_messages::world::chunk::CrossChunkBoundaryEvent;
use ferrumc_messages::*;
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
}
