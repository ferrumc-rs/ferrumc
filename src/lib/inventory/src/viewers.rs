use crate::inventory::Inventory;
use ferrumc_ecs::entities::Entity;
use ferrumc_ecs::errors::ECSError;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::close_container::CloseContainerPacket;
use ferrumc_net::packets::outgoing::open_screen::OpenScreenPacket;
use ferrumc_net::packets::outgoing::set_container_slot::SetContainerSlotPacket;
use ferrumc_net_codec::encode::NetEncodeOpts;

pub struct InventoryView {
    pub viewers: Vec<Entity>,
}

impl InventoryView {
    pub fn new() -> Self {
        Self {
            viewers: Vec::new(),
        }
    }

    pub async fn add_viewer(
        &mut self,
        inventory: &Inventory,
        mut entity: (Entity, &mut StreamWriter),
    ) -> Result<&mut Self, NetError> {
        let mut viewers = &self.viewers;
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
        mut entity: (Entity, &mut StreamWriter),
    ) -> Result<&mut Self, NetError> {
        let mut viewers = &mut self.viewers;
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

    async fn send_packet(
        &mut self,
        inventory: &Inventory,
        writer: &mut StreamWriter,
    ) -> Result<(), NetError> {
        let id = &inventory.id;
        let packet = OpenScreenPacket::new(
            id.clone(),
            inventory.inventory_type.get_id(),
            inventory.title.clone(),
        );

        writer
            .send_packet(&packet, &NetEncodeOpts::WithLength)
            .await?;

        // Temporary until i get container content setup
        for slot in inventory.contents.contents.iter() {
            let slot_packet =
                SetContainerSlotPacket::new(id.clone(), *slot.key() as i16, slot.to_network_slot());
            writer
                .send_packet(&slot_packet, &NetEncodeOpts::SizePrefixed)
                .await?;
        }

        Ok(())
    }
}
