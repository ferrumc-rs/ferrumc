use crate::events::inventory_close::CloseInventoryEvent;
use crate::events::inventory_open::OpenInventoryEvent;
use crate::inventory::InventoryData;
use ferrumc_ecs::entities::Entity;
use ferrumc_ecs::errors::ECSError;
use ferrumc_events::infrastructure::Event;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::close_container::CloseContainerPacket;
use ferrumc_net::packets::outgoing::open_screen::OpenScreenPacket;
use ferrumc_net::packets::outgoing::set_container_slot::NetworkSlot;
use ferrumc_net::{
    connection::StreamWriter, packets::outgoing::set_container_content::SetContainerContentPacket,
};
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_state::ServerState;
use std::sync::Arc;

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
        inventory: &InventoryData,
        state: Arc<ServerState>,
        entity: Entity,
    ) -> Result<&mut Self, NetError> {
        let universe = &state.universe;
        let mut writer = universe.get_mut::<&mut StreamWriter>(entity)?;

        let viewers = &self.viewers;
        if viewers.contains(&entity) {
            return Ok(self);
        }

        self.viewers.push(entity);
        self.send_packet(inventory, *writer).await?;

        // handle event
        let event = OpenInventoryEvent::new(entity).inventory_id(*inventory.id);
        OpenInventoryEvent::trigger(event, state).await?;

        Ok(self)
    }

    pub async fn remove_viewer(
        &mut self,
        inventory: &InventoryData,
        state: Arc<ServerState>,
        entity: Entity,
    ) -> Result<&mut Self, NetError> {
        let universe = &state.universe;
        let mut writer = universe.get_mut::<&mut StreamWriter>(entity)?;

        let viewers = &mut self.viewers;
        if let Some(index) = viewers.iter().position(|&viewer| viewer == entity) {
            viewers.remove(index);
            self.send_close_packet(*inventory.id, *writer).await?;

            // handle event
            let event = CloseInventoryEvent::new(entity).inventory_id(*inventory.id);
            CloseInventoryEvent::trigger(event, state).await?;

            Ok(self)
        } else {
            Err(NetError::ECSError(ECSError::ComponentNotFound))?
        }
    }

    async fn send_close_packet(
        &self,
        inventory_id: i32,
        writer: &mut StreamWriter,
    ) -> Result<(), NetError> {
        writer
            .send_packet(
                &CloseContainerPacket::new(inventory_id as u8),
                &NetEncodeOpts::WithLength,
            )
            .await
    }

    async fn send_packet_to_viewers(&self, _packet: &impl NetEncode) -> Result<(), NetError> {
        Ok(())
    }

    async fn send_packet(
        &mut self,
        inventory: &InventoryData,
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

        let inventory_size = inventory.inventory_type.get_size() as usize;
        let container_content = inventory.contents.construct_container_vec(inventory_size);
        writer
            .send_packet(
                &SetContainerContentPacket::new(
                    *inventory.id as u8,
                    container_content,
                    NetworkSlot::empty(),
                ),
                &NetEncodeOpts::SizePrefixed,
            )
            .await?;

        Ok(())
    }
}
