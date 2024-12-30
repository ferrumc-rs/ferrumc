use crate::{inventory::Inventory, slot::Slot};
use ferrumc_ecs::entities::Entity;
use ferrumc_ecs::errors::ECSError;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::close_container::CloseContainerPacket;
use ferrumc_net::packets::outgoing::open_screen::OpenScreenPacket;
use ferrumc_net::packets::outgoing::set_container_slot::SetContainerSlotPacket;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};

#[derive(Debug, Clone)]
pub struct InventoryView {
    pub viewers: Vec<Entity>,
}

impl InventoryView {
    pub(crate) fn new() -> Self {
        Self {
            viewers: Vec::new(),
        }
    }

    pub async fn add_viewer(
        &mut self,
        inventory: &Inventory,
        mut entity: (Entity, &mut StreamWriter),
    ) -> Result<&mut Self, NetError> {
        let viewers = &self.viewers;
        if viewers.contains(&entity.0) {
            return Ok(self);
        }

        self.viewers.push(entity.0);
        self.send_packet(inventory, &mut entity.1).await?;
        Ok(self)
    }

    pub async fn remove_viewer(
        &mut self,
        inventory: &Inventory,
        entity: (Entity, &mut StreamWriter),
    ) -> Result<&mut Self, NetError> {
        let viewers = &mut self.viewers;
        if let Some(index) = viewers.iter().position(|&viewer| viewer == entity.0) {
            viewers.remove(index);
            entity
                .1
                .send_packet(
                    &CloseContainerPacket::new(*inventory.id as u8),
                    &NetEncodeOpts::WithLength,
                )
                .await?;
            Ok(self)
        } else {
            Err(NetError::ECSError(ECSError::ComponentNotFound))?
        }
    }

    pub async fn send_slot_update_packet(
        &self,
        inventory: &Inventory,
        slot: (i16, Slot),
    ) -> Result<(), NetError> {
        self.send_packet_to_viewers(&SetContainerSlotPacket::new(
            inventory.id,
            slot.0,
            slot.1.to_network_slot(),
        ))
        .await
    }

    async fn send_packet_to_viewers(&self, _packet: &impl NetEncode) -> Result<(), NetError> {
        Ok(())
    }

    async fn send_packet(
        &mut self,
        inventory: &Inventory,
        writer: &mut StreamWriter,
    ) -> Result<(), NetError> {
        let packet = OpenScreenPacket::new(
            inventory.id,
            inventory.inventory_type.get_id(),
            inventory.title.clone(),
        );

        writer
            .send_packet(&packet, &NetEncodeOpts::WithLength)
            .await?;

        // Temporary until i get container content setup
        for slot in inventory.contents.contents.iter() {
            let slot_packet = SetContainerSlotPacket::new(
                inventory.id,
                *slot.key() as i16,
                slot.to_network_slot(),
            );
            writer
                .send_packet(&slot_packet, &NetEncodeOpts::SizePrefixed)
                .await?;
        }

        Ok(())
    }
}
