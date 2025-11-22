use bevy_ecs::prelude::Component;
use typename::TypeName;

/// Entity ID stored as i64 internally to reduce collision risk on large servers.
/// The network protocol only supports i32, so we truncate when sending to clients.
#[derive(Debug, Clone, Copy, Component, TypeName)]
pub struct EntityId(pub i64);

impl EntityId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the network-safe i32 representation of this ID.
    /// The protocol only supports 32-bit entity IDs, so we truncate.
    pub fn to_network_id(&self) -> i32 {
        self.0 as i32
    }

    pub fn as_varint(&self) -> ferrumc_net_codec::net_types::var_int::VarInt {
        ferrumc_net_codec::net_types::var_int::VarInt::new(self.to_network_id())
    }
}
