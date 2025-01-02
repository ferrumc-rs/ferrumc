use ferrumc_ecs::entities::Entity;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = 0x42)]
pub struct RemoveEntitiesPacket {
    pub entity_ids: LengthPrefixedVec<VarInt>,
}

impl RemoveEntitiesPacket {
    pub fn from_entities<T>(entity_ids: T) -> Self
    where
        T: IntoIterator<Item = Entity>,
    {
        let entity_ids: Vec<VarInt> = entity_ids
            .into_iter()
            .map(|entity| VarInt::new(entity as i32))
            .collect();
        Self {
            entity_ids: LengthPrefixedVec::new(entity_ids),
        }
    }
}
