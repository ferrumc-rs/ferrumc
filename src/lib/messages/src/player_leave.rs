use bevy_ecs::prelude::*;
use ferrumc_core::identity::player_identity::PlayerIdentity;

#[derive(Message, Clone)]
#[allow(unused)]
pub struct PlayerLeft(pub PlayerIdentity);
