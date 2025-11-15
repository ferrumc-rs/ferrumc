use bevy_ecs::message::MessageRegistry;
use bevy_ecs::prelude::World;
use ferrumc_commands::messages::{CommandDispatched, ResolvedCommandDispatched};
use ferrumc_core::chunks::cross_chunk_boundary_event::ChunkBoundaryCrossed;
use ferrumc_core::conn::force_player_recount_event::ForcePlayerRecount;
use ferrumc_net::packets::packet_messages::Movement;

pub fn register_messages(world: &mut World) {
    MessageRegistry::register_message::<Movement>(world);
    MessageRegistry::register_message::<ChunkBoundaryCrossed>(world);
    MessageRegistry::register_message::<ForcePlayerRecount>(world);
    MessageRegistry::register_message::<CommandDispatched>(world);
    MessageRegistry::register_message::<ResolvedCommandDispatched>(world);
}
