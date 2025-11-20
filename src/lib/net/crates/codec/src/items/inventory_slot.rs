use crate::decode::{errors::NetDecodeError, NetDecode, NetDecodeOpts};
use crate::encode::{errors::NetEncodeError, NetEncode, NetEncodeOpts};
use crate::net_types::var_int::VarInt;
use ferrumc_core::items::inventory_slot::InventorySlot;
use ferrumc_core::items::item_id::ItemID;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

// --- NetDecode (Read from Client) ---
impl NetDecode for InventorySlot {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let count = VarInt::decode(reader, opts)?;

        if count.0 <= 0 {
            // Empty slot
            return Ok(InventorySlot {
                count: count.0,
                item_id: None,
                ..Default::default()
            });
        }

        let item_id_val = VarInt::decode(reader, opts)?;
        //   let add_count = VarInt::decode(reader, opts)?;
        //   let remove_count = VarInt::decode(reader, opts)?;

        // Skip/Read components (Placeholder logic - adjust based on actual component handling)
        // For now, we just read the VarInts to advance the cursor,
        // but we might need to read the actual component data arrays here.
        // (See your previous slot.rs logic for the specific loops)

        // ... (Insert component reading loops here if needed) ...

        Ok(InventorySlot {
            count: count.0,
            item_id: Some(ItemID::new(item_id_val.0)),
            // Store the component counts/data if you added fields for them
            ..Default::default()
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let count = VarInt::decode_async(reader, opts).await?;

        if count.0 <= 0 {
            return Ok(InventorySlot {
                count: count.0,
                item_id: None,
                ..Default::default()
            });
        }

        let item_id_val = VarInt::decode_async(reader, opts).await?;
        // ... (async component reading) ...

        Ok(InventorySlot {
            count: count.0,
            item_id: Some(ItemID::new(item_id_val.0)),
            ..Default::default()
        })
    }
}

// --- NetEncode (Write to Client) ---
impl NetEncode for InventorySlot {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        // 1. Write Count (i32 -> VarInt)
        VarInt(self.count).encode(writer, opts)?;

        if self.count <= 0 {
            return Ok(());
        }

        // 2. Write Item ID
        match self.item_id {
            Some(id) => VarInt(id.0).encode(writer, opts)?,
            None => VarInt(0).encode(writer, opts)?, // Should ideally not happen if count > 0
        }

        // 3. Write Component Counts (Simplification: 0 for now)
        // You should adapt this to write your actual component data
        VarInt(0).encode(writer, opts)?; // Add count
        VarInt(0).encode(writer, opts)?; // Remove count

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt(self.count).encode_async(writer, opts).await?;

        if self.count <= 0 {
            return Ok(());
        }

        match self.item_id {
            Some(id) => VarInt(id.0).encode_async(writer, opts).await?,
            None => VarInt(0).encode_async(writer, opts).await?,
        }

        VarInt(0).encode_async(writer, opts).await?;
        VarInt(0).encode_async(writer, opts).await?;

        Ok(())
    }
}
