use bevy_ecs::prelude::Component;
use typename::TypeName;

#[derive(Debug, Clone, Copy, Component, TypeName)]
pub struct EntityId(pub i32);

impl EntityId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }

    pub fn as_varint(&self) -> ferrumc_net_codec::net_types::var_int::VarInt {
        ferrumc_net_codec::net_types::var_int::VarInt::new(self.0)
    }
}
