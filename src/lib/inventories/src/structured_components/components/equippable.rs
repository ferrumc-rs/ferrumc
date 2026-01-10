use ferrumc_macros::{NetDecode, NetEncode};
use crate::structured_components::data::{IdOr, IdSet, Identifier, SoundEvent};
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct Equippable {
    pub slot: VarInt,
    pub equip_sound: IdOr<SoundEvent>,
    pub model: PrefixedOptional<Identifier>,
    pub camera_overlay: PrefixedOptional<Identifier>,
    pub allowed_entities: PrefixedOptional<IdSet>,
    pub dispensable: bool,
    pub swappable: bool,
    pub damage_on_hurt: bool,
}
