use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::fmt::Display;
use std::io::Read;
use tokio::io::AsyncRead;

#[derive(NetDecode)]
#[packet(packet_id = "set_creative_mode_slot", state = "play")]
pub struct SetCreativeModeSlot {
    pub slot_index: i16,
    pub slot: InventorySlot,
}

#[derive(Debug, Clone, Hash)]
pub struct InventorySlot {
    pub count: VarInt,
    pub item_id: Option<VarInt>,
    pub components_to_add_count: Option<VarInt>,
    pub components_to_remove_count: Option<VarInt>,
    pub components_to_add: Option<Vec<VarInt>>,
    pub components_to_remove: Option<Vec<VarInt>>,
    // https://minecraft.wiki/w/Java_Edition_protocol/Slot_data
}

impl Display for InventorySlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "InventorySlot {{ count: {}, item_id: {:?}, components_to_add_count: {:?}, components_to_remove_count: {:?} }}",
            self.count.0,
            self.item_id,
            self.components_to_add_count,
            self.components_to_remove_count
        )
    }
}

impl NetDecode for InventorySlot {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let count = VarInt::decode(reader, opts)?;
        if count.0 == 0 {
            Ok(Self {
                count,
                item_id: None,
                components_to_add_count: None,
                components_to_remove_count: None,
                components_to_add: None,
                components_to_remove: None,
            })
        } else {
            let item_id = VarInt::decode(reader, opts)?;
            let components_to_add_count = VarInt::decode(reader, opts)
                ?;
            let components_to_remove_count = VarInt::decode(reader, opts)
                ?;
            let components_to_add = {
                let mut components = Vec::with_capacity(components_to_add_count.0 as usize);
                for _ in 0..components_to_add_count.0 {
                    components.push(VarInt::decode(reader, opts)?);
                }
                Some(components)
            };
            let components_to_remove = {
                let mut components = Vec::with_capacity(components_to_remove_count.0 as usize);
                for _ in 0..components_to_remove_count.0 {
                    components.push(VarInt::decode(reader, opts)?);
                }
                Some(components)
            };
            Ok(Self {
                count,
                item_id: Some(item_id),
                components_to_add_count: Some(components_to_add_count),
                components_to_remove_count: Some(components_to_remove_count),
                components_to_add,
                components_to_remove,
            })
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let count = VarInt::decode_async(reader, opts).await?;
        if count.0 == 0 {
            Ok(Self {
                count,
                item_id: None,
                components_to_add_count: None,
                components_to_remove_count: None,
                components_to_add: None,
                components_to_remove: None,
            })
        } else {
            let item_id = PrefixedOptional::decode_async(reader, opts)
                .await?
                .to_option();
            let components_to_add_count: Option<VarInt> = PrefixedOptional::decode_async(reader, opts)
                .await?
                .to_option();
            let components_to_remove_count: Option<VarInt> = PrefixedOptional::decode_async(reader, opts)
                .await?
                .to_option();
            let components_to_add = if let Some(count) = components_to_add_count {
                let mut components = Vec::with_capacity(count.0 as usize);
                for _ in 0..count.0 {
                    components.push(VarInt::decode_async(reader, opts).await?);
                }
                Some(components)
            } else {
                None
            };
            let components_to_remove = if let Some(count) = components_to_remove_count {
                let mut components = Vec::with_capacity(count.0 as usize);
                for _ in 0..count.0 {
                    components.push(VarInt::decode_async(reader, opts).await?);
                }
                Some(components)
            } else {
                None
            };
            Ok(Self {
                count,
                item_id,
                components_to_add_count,
                components_to_remove_count,
                components_to_add,
                components_to_remove,
            })
        }
    }
}
