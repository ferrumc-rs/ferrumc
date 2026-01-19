use crate::item::ItemID;
use bitcode_derive::{Decode, Encode};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::fmt::Display;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone, Hash, Default, PartialEq, Decode, Encode)]
pub struct InventorySlot {
    pub count: VarInt,
    pub item_id: Option<ItemID>,
    pub components_to_add_count: Option<VarInt>,
    pub components_to_remove_count: Option<VarInt>,
    pub components_to_add: Option<Vec<VarInt>>,
    pub components_to_remove: Option<Vec<VarInt>>,
    // https://minecraft.wiki/w/Java_Edition_protocol/Slot_data
}

impl InventorySlot {
    pub fn empty() -> Self {
        Self {
            count: VarInt(0),
            item_id: None,
            components_to_add_count: None,
            components_to_add: None,
            components_to_remove: None,
            components_to_remove_count: None,
        }
    }
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
                ..Default::default()
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
        // 1. Always encode the count
        self.count.encode(writer, opts)?;

        // If count is 0, stop immediately
        if self.count.0 == 0 {
            return Ok(());
        }

        let zero_varint = VarInt::new(0);

        // 2. Encode ItemID
        match &self.item_id {
            Some(item_id) => item_id.0.encode(writer, opts)?,
            None => zero_varint.encode(writer, opts)?,
        }

        // 3. Get add_count and remove_count
        let add_count = self
            .components_to_add_count
            .as_ref()
            .unwrap_or(&zero_varint);
        let remove_count = self
            .components_to_remove_count
            .as_ref()
            .unwrap_or(&zero_varint);

        // 4. Encode components_to_add_count
        add_count.encode(writer, opts)?;

        // 5. Encode components_to_remove_count
        remove_count.encode(writer, opts)?;

        // 6. Encode components_to_add list (if any)
        if add_count.0 > 0
            && let Some(components) = &self.components_to_add
        {
            for component in components {
                component.encode(writer, opts)?;
            }
        }

        // 7. Encode components_to_remove list (if any)
        if remove_count.0 > 0
            && let Some(components) = &self.components_to_remove
        {
            for component in components {
                component.encode(writer, opts)?;
            }
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
    use ferrumc_net_codec::encode::NetEncodeOpts;
    use ferrumc_net_codec::net_types::var_int::VarInt;
    use std::io::Cursor;

    // This helper function runs the encode/decode cycle
    fn run_roundtrip_test(slot_in: &InventorySlot) -> InventorySlot {
        let mut buffer = Vec::new();

        // Create both types of options explicitly.
        let encode_opts = NetEncodeOpts::default();
        let decode_opts = NetDecodeOpts::default();

        // 1. Encode
        slot_in
            .encode(&mut buffer, &encode_opts)
            .expect("Encode failed");

        // 2. Decode
        let mut reader = Cursor::new(&buffer);
        let slot_out = InventorySlot::decode(&mut reader, &decode_opts).expect("Decode failed");

        // 3. Check that all bytes were read
        assert_eq!(
            reader.position() as usize,
            buffer.len(),
            "Decoder did not read the entire buffer"
        );

        slot_out
    }

    #[test]
    fn test_slot_encode_decode_roundtrip() {
        // --- Test Case 1: The Empty Slot ---

        let simple_slot = InventorySlot {
            count: VarInt::new(10),
            item_id: Some(ItemID::new(1)),
            components_to_add_count: Some(VarInt::new(0)),
            components_to_remove_count: Some(VarInt::new(0)),
            components_to_add: Some(vec![]),
            components_to_remove: Some(vec![]),
        };

        let decoded_simple = run_roundtrip_test(&simple_slot);
        assert_eq!(simple_slot, decoded_simple, "Simple slot roundtrip failed");

        // --- Test Case 2: The Full NBT/Component Slot ---
        let complex_slot = InventorySlot {
            count: VarInt::new(1),
            item_id: Some(ItemID::new(872)),
            components_to_add_count: Some(VarInt::new(2)),
            components_to_remove_count: Some(VarInt::new(1)),
            components_to_add: Some(vec![VarInt::new(10), VarInt::new(11)]),
            components_to_remove: Some(vec![VarInt::new(20)]),
        };
        let decoded_complex = run_roundtrip_test(&complex_slot);
        assert_eq!(
            complex_slot, decoded_complex,
            "Complex slot roundtrip failed"
        );
    }
}
