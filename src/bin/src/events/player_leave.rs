use bevy_ecs::prelude::*;
use ferrumc_core::identity::player_identity::PlayerIdentity;

#[derive(Event, Clone)]
#[allow(unused)]
pub struct PlayerLeaveEvent(pub PlayerIdentity);
