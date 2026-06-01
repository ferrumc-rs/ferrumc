use bevy_ecs::prelude::ResMut;
use ferrumc_core::tick::TickCounter;

/// Advances the global [`TickCounter`] by one.
///
/// Registered as the first system in the tick schedule so that every other
/// system running in the same tick observes a consistent, already-incremented
/// tick number. This is the authoritative clock for scheduled work such as
/// fluid spreading.
pub fn handle(mut tick: ResMut<TickCounter>) {
    tick.advance();
}
