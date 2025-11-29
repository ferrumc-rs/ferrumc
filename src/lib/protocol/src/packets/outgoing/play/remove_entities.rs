use crate::codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use crate::codec::net_types::var_int::VarInt;
use crate::ids;
use ferrumc_components::player::identity::PlayerIdentity;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode)]
#[packet(id = ids::PLAY_CLIENTBOUND_REMOVE_ENTITIES, state = "play")]
pub struct RemoveEntitiesPacket {
    pub entity_ids: LengthPrefixedVec<VarInt>,
}

impl RemoveEntitiesPacket {
    pub fn from_entities<T>(entity_ids: T) -> Self
    where
        T: IntoIterator<Item = PlayerIdentity>,
    {
        let entity_ids: Vec<VarInt> = entity_ids
            .into_iter()
            .map(|entity| VarInt::new(entity.short_uuid))
            .collect();
        Self {
            entity_ids: LengthPrefixedVec::new(entity_ids),
        }
    }
}
