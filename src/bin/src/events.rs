use ferrumc_core::identity::player_identity::*;
use ferrumc_macros::Event;
use ferrumc_ecs::entities::Entity;

/// This event is triggered when the player attempts to log on to the server.
///
/// Beware that not all components on the entity may be set yet this event is mostly for:
/// a custom handshaking protocol before the player logs in using login plugin messages/etc.
///
#[derive(Event, Clone)]
pub struct PlayerStartLoginEvent {
    /// The entity that this event was fired for.
    pub entity: Entity,

    /// This profile can be changed and after the event is finished this will be the new profile.
    pub profile: PlayerIdentity,
}
