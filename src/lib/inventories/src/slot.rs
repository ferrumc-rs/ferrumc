use crate::components::Component;
use crate::item::ItemID;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::fmt::Display;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

/// Represents an inventory slot with item data and components.
/// See: https://minecraft.wiki/w/Java_Edition_protocol/Slot_data
#[derive(Debug, Clone, Default)]
pub struct InventorySlot {
    /// Item count (0 = empty slot)
    pub count: VarInt,
    /// Item type ID (None if count is 0)
    pub item_id: Option<ItemID>,
    /// Components to add to the item (overrides default components)
    pub components_to_add: Vec<Component>,
    /// Component type IDs to remove from the item
    pub components_to_remove: Vec<VarInt>,
}

impl InventorySlot {
    /// Creates an empty inventory slot.
    pub fn empty() -> Self {
        Self {
            count: VarInt(0),
            item_id: None,
            components_to_add: Vec::new(),
            components_to_remove: Vec::new(),
        }
    }

    /// Creates a simple slot with just an item and count, no components.
    pub fn new(item_id: i32, count: i32) -> Self {
        Self {
            count: VarInt(count),
            item_id: Some(ItemID::new(item_id)),
            components_to_add: Vec::new(),
            components_to_remove: Vec::new(),
        }
    }

    /// Creates a slot with components.
    pub fn with_components(item_id: i32, count: i32, components: Vec<Component>) -> Self {
        Self {
            count: VarInt(count),
            item_id: Some(ItemID::new(item_id)),
            components_to_add: components,
            components_to_remove: Vec::new(),
        }
    }

    /// Returns true if this slot is empty.
    pub fn is_empty(&self) -> bool {
        self.count.0 == 0
    }
}

impl Display for InventorySlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "InventorySlot {{ count: {}, item_id: {:?}, components_add: {}, components_remove: {} }}",
            self.count.0,
            self.item_id,
            self.components_to_add.len(),
            self.components_to_remove.len()
        )
    }
}

impl NetDecode for InventorySlot {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let count = VarInt::decode(reader, opts)?;
        if count.0 == 0 {
            Ok(Self::empty())
        } else {
            let item_id = VarInt::decode(reader, opts)?;
            let add_count = VarInt::decode(reader, opts)?;
            let remove_count = VarInt::decode(reader, opts)?;

            // Decode components to add (each component reads its own type ID)
            let mut components_to_add = Vec::with_capacity(add_count.0 as usize);
            for _ in 0..add_count.0 {
                components_to_add.push(Component::decode(reader, opts)?);
            }

            // Decode component IDs to remove
            let mut components_to_remove = Vec::with_capacity(remove_count.0 as usize);
            for _ in 0..remove_count.0 {
                components_to_remove.push(VarInt::decode(reader, opts)?);
            }

            Ok(Self {
                count,
                item_id: Some(ItemID(item_id)),
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
        // 1. Always encode the count
        self.count.encode(writer, opts)?;

        // If count is 0, stop immediately (empty slot)
        if self.count.0 == 0 {
            return Ok(());
        }

        // 2. Encode ItemID
        match &self.item_id {
            Some(item_id) => item_id.0.encode(writer, opts)?,
            None => VarInt(0).encode(writer, opts)?,
        }

        // 3. Encode component counts
        VarInt(self.components_to_add.len() as i32).encode(writer, opts)?;
        VarInt(self.components_to_remove.len() as i32).encode(writer, opts)?;

        // 4. Encode components to add (type ID + data for each)
        for component in &self.components_to_add {
            // Write component type ID first
            component.id().encode(writer, opts)?;
            // Then write component data (derived NetEncode handles the fields)
            component.encode(writer, opts)?;
        }

        // 5. Encode component IDs to remove
        for component_id in &self.components_to_remove {
            component_id.encode(writer, opts)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{Component, Rarity};
    use ferrumc_net_codec::encode::NetEncodeOpts;
    use std::io::Cursor;

    #[test]
    fn test_empty_slot_roundtrip() {
        let slot = InventorySlot::empty();
        let mut buffer = Vec::new();
        slot.encode(&mut buffer, &NetEncodeOpts::default())
            .expect("Encode failed");

        // Empty slot should just be a single VarInt(0)
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer[0], 0);

        let mut reader = Cursor::new(&buffer);
        let decoded = InventorySlot::decode(&mut reader, &NetDecodeOpts::default())
            .expect("Decode failed");

        assert!(decoded.is_empty());
    }

    #[test]
    fn test_simple_slot_roundtrip() {
        let slot = InventorySlot::new(1, 64); // 64 stone

        let mut buffer = Vec::new();
        slot.encode(&mut buffer, &NetEncodeOpts::default())
            .expect("Encode failed");

        let mut reader = Cursor::new(&buffer);
        let decoded = InventorySlot::decode(&mut reader, &NetDecodeOpts::default())
            .expect("Decode failed");

        assert_eq!(decoded.count.0, 64);
        assert_eq!(decoded.item_id.unwrap().0.0, 1);
        assert!(decoded.components_to_add.is_empty());
        assert!(decoded.components_to_remove.is_empty());
    }

    #[test]
    fn test_slot_with_single_component_roundtrip() {
        // Test with just Unbreakable (no data)
        let slot = InventorySlot::with_components(1, 1, vec![Component::Unbreakable]);

        let mut buffer = Vec::new();
        slot.encode(&mut buffer, &NetEncodeOpts::default())
            .expect("Encode failed");

        println!("Encoded bytes: {:?}", buffer);

        let mut reader = Cursor::new(&buffer);
        let decoded = InventorySlot::decode(&mut reader, &NetDecodeOpts::default())
            .expect("Decode failed");

        assert_eq!(decoded.count.0, 1);
        assert_eq!(decoded.components_to_add.len(), 1);
        assert_eq!(decoded.components_to_add[0].id().0, 4); // Unbreakable
    }

    #[test]
    fn test_slot_with_max_stack_component_roundtrip() {
        // Test with MaxStackSize
        let slot = InventorySlot::with_components(1, 1, vec![Component::MaxStackSize(VarInt(99))]);

        let mut buffer = Vec::new();
        slot.encode(&mut buffer, &NetEncodeOpts::default())
            .expect("Encode failed");

        println!("Encoded bytes: {:?}", buffer);

        let mut reader = Cursor::new(&buffer);
        let decoded = InventorySlot::decode(&mut reader, &NetDecodeOpts::default())
            .expect("Decode failed");

        assert_eq!(decoded.count.0, 1);
        assert_eq!(decoded.components_to_add.len(), 1);
        assert_eq!(decoded.components_to_add[0].id().0, 1); // MaxStackSize
    }

    #[test]
    fn test_slot_with_rarity_component_roundtrip() {
        // Test with Rarity
        let slot = InventorySlot::with_components(1, 1, vec![Component::Rarity(Rarity::Epic)]);

        let mut buffer = Vec::new();
        slot.encode(&mut buffer, &NetEncodeOpts::default())
            .expect("Encode failed");

        println!("Encoded bytes: {:?}", buffer);

        let mut reader = Cursor::new(&buffer);
        let decoded = InventorySlot::decode(&mut reader, &NetDecodeOpts::default())
            .expect("Decode failed");

        assert_eq!(decoded.count.0, 1);
        assert_eq!(decoded.components_to_add.len(), 1);
        assert_eq!(decoded.components_to_add[0].id().0, 9); // Rarity
    }
}
