use crate::item::ItemID;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::fmt::Display;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

/// Represents a single inventory slot in a Minecraft player's inventory.
///
/// This struct models the server-to-client or client-to-server slot data as defined in the
/// [Minecraft Protocol](https://minecraft.wiki/w/Java_Edition_protocol/Slot_data).
///
/// Each slot can optionally contain an item and additional NBT or component data.
/// Empty slots are represented by `item_id: None`.
#[derive(Debug, Clone, Hash, Default)]
pub struct InventorySlot {
    /// The number of items in this slot.
    ///
    /// Represented as a [`VarInt`] according to the Minecraft protocol.
    /// Only meaningful if `item_id` is `Some(_)`.
    pub count: VarInt,
    /// The unique identifier of the item stored in this slot.
    ///
    /// `None` means the slot is empty.
    pub item_id: Option<ItemID>,
    /// The number of item components to add to this item stack.
    ///
    /// Present only when the protocol indicates component-based modifications.
    pub components_to_add_count: Option<VarInt>,
    /// The number of item components to remove from this item stack.
    ///
    /// Present only when the protocol indicates component-based modifications.
    pub components_to_remove_count: Option<VarInt>,
    /// The list of component IDs (as [`VarInt`]) to add to this item.
    ///
    /// Each value corresponds to a specific item component or modifier.
    /// Only present if `components_to_add_count` is `Some(_)`.
    pub components_to_add: Option<Vec<VarInt>>,
    /// The list of component IDs (as [`VarInt`]) to remove from this item.
    ///
    /// Each value corresponds to a specific item component or modifier.
    /// Only present if `components_to_remove_count` is `Some(_)`.
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
            let components_to_add_count = VarInt::decode(reader, opts)?;
            let components_to_remove_count = VarInt::decode(reader, opts)?;
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
                item_id: Some(ItemID(item_id)),
                components_to_add_count: Some(components_to_add_count),
                components_to_remove_count: Some(components_to_remove_count),
                components_to_add,
                components_to_remove,
            })
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        _reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        todo!()
    }
}

impl NetEncode for InventorySlot {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.count.encode(writer, opts)?;
        if let Some(item_id) = &self.item_id {
            item_id.encode(writer, opts)?;
        } else {
            VarInt::new(0).encode(writer, opts)?; // Encode 0 for empty slot
        }
        if let Some(components_to_add_count) = &self.components_to_add_count {
            components_to_add_count.encode(writer, opts)?;
            if let Some(components) = &self.components_to_add {
                for component in components {
                    component.encode(writer, opts)?;
                }
            }
        } else {
            VarInt::new(0).encode(writer, opts)?; // Encode 0 for no components to add
        }
        if let Some(components_to_remove_count) = &self.components_to_remove_count {
            components_to_remove_count.encode(writer, opts)?;
            if let Some(components) = &self.components_to_remove {
                for component in components {
                    component.encode(writer, opts)?;
                }
            }
        } else {
            VarInt::new(0).encode(writer, opts)?; // Encode 0 for no components to remove
        }
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        _writer: &mut W,
        _opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        todo!()
    }
}
