use bevy_ecs::prelude::*;
use ferrumc_core::identity::player_identity::PlayerIdentity;

#[derive(Event, Clone)]
pub struct PlayerLeaveEvent(pub PlayerIdentity);
