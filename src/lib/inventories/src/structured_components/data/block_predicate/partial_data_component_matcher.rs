use std::io::Read;
use tokio::io::AsyncRead;
use ferrumc_nbt::de::borrow::NbtTag;
use ferrumc_nbt::de::converter::readers::async_reader::read_nbt_content_async;
use ferrumc_nbt::de::converter::readers::sync_reader::read_nbt_content_recursive;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use crate::structured_components::data::utils::create_compound_buffer;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct PartialDataComponentMatcher {
    pub predicate_type: PartialDataComponentType,
    pub nbt_predicate: Vec<u8>, //owning nbt tape would be great
}

#[repr(i32)]
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum PartialDataComponentType {
    Damage = 0,
    Enchantments = 1,
    StoredEnchantments = 2,
    PotionContents = 3,
    CustomData = 4,
    Container = 5,
    BundleContents = 6,
    FireworkExplosion = 7,
    Fireworks = 8,
    WritableBookContent = 9,
    WrittenBookContent = 10,
    AttributeModifiers = 11,
    Trim = 12,
    JukeboxPlayable = 13,
    Unknown(i32),
}

impl From<i32> for PartialDataComponentType {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Damage,
            1 => Self::Enchantments,
            2 => Self::StoredEnchantments,
            3 => Self::PotionContents,
            4 => Self::CustomData,
            5 => Self::Container,
            6 => Self::BundleContents,
            7 => Self::FireworkExplosion,
            8 => Self::Fireworks,
            9 => Self::WritableBookContent,
            10 => Self::WrittenBookContent,
            11 => Self::AttributeModifiers,
            12 => Self::Trim,
            13 => Self::JukeboxPlayable,
            other => Self::Unknown(other),
        }
    }
}

impl NetDecode for PartialDataComponentMatcher {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let type_raw = i32::decode(reader, opts)?;
        let predicate_type = PartialDataComponentType::from(type_raw);

        let mut buffer = create_compound_buffer();
        read_nbt_content_recursive(&NbtTag::Compound, reader, &mut buffer)
            .map_err(|e| NetDecodeError::ExternalError(Box::new(e)))?;

        Ok(Self {
            predicate_type,
            nbt_predicate: buffer,
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let type_raw = i32::decode_async(reader, opts).await?;
        let predicate_type = PartialDataComponentType::from(type_raw);

        let mut buffer = create_compound_buffer();
        read_nbt_content_async(&NbtTag::Compound, reader, &mut buffer)
            .await
            .map_err(|e| NetDecodeError::ExternalError(Box::new(e)))?;

        Ok(Self {
            predicate_type,
            nbt_predicate: buffer,
        })
    }
}