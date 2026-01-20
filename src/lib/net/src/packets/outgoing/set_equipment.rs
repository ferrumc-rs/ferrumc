//! SetEquipment packet for broadcasting player equipment to other players.
//! Protocol: https://minecraft.wiki/w/Protocol#Set_Equipment

use ferrumc_inventories::slot::InventorySlot;
use ferrumc_inventories::sync::EquipmentSlot;
use ferrumc_macros::packet;
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;
use tokio::io::AsyncWrite;

/// Equipment entry for the SetEquipment packet.
#[derive(Clone, Debug)]
pub struct EquipmentEntry {
    pub slot: EquipmentSlot,
    pub item: InventorySlot,
}

/// Packet 0x5F: Set Equipment
/// Sent to update visible equipment on an entity (armor, held items).
#[packet(packet_id = "set_equipment", state = "play")]
pub struct SetEquipmentPacket {
    pub entity_id: VarInt,
    pub equipment: Vec<EquipmentEntry>,
}

impl SetEquipmentPacket {
    /// Creates a new SetEquipment packet.
    ///
    /// # Arguments
    /// * `entity_id` - The entity whose equipment changed (protocol entity ID, not bevy Entity)
    /// * `equipment` - List of equipment slots and their items
    pub fn new(entity_id: i32, equipment: Vec<EquipmentEntry>) -> Self {
        Self {
            entity_id: VarInt::new(entity_id),
            equipment,
        }
    }

    /// Creates a packet with a single equipment slot.
    pub fn single(entity_id: i32, slot: EquipmentSlot, item: InventorySlot) -> Self {
        Self::new(entity_id, vec![EquipmentEntry { slot, item }])
    }
}

impl NetEncode for SetEquipmentPacket {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        // 1. Entity ID
        self.entity_id.encode(writer, opts)?;

        // 2. Equipment entries
        // Format: [slot_byte] [item_data]
        // slot_byte has high bit (0x80) set if there are more entries following
        let len = self.equipment.len();
        for (i, entry) in self.equipment.iter().enumerate() {
            let is_last = i == len - 1;
            let slot_byte = if is_last {
                entry.slot.protocol_id()
            } else {
                entry.slot.protocol_id() | 0x80 // Set "has more" flag
            };

            writer.write_all(&[slot_byte])?;
            entry.item.encode(writer, opts)?;
        }

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        _writer: &mut W,
        _opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        unimplemented!("Async encoding not needed for server-to-client packets")
    }
}
