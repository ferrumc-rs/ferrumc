use ferrumc_macros::{Decode, packet};

use crate::net::Connection;
use crate::net::packets::IncomingPacket;
use crate::state::GlobalState;
use crate::utils::components::rotation::Rotation;

#[derive(Decode)]
#[packet(packet_id = 0x16, state = "play")]
pub struct SetPlayerRotation {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

impl IncomingPacket for SetPlayerRotation {
    async fn handle(self, conn: &mut Connection, state: GlobalState) -> crate::utils::prelude::Result<()> {
        let my_entity_id = conn.metadata.entity;

        let component_storage = state.world.get_component_storage();

        let mut rotation = component_storage
            .get_mut_or_insert_with(my_entity_id, || Rotation::new(0.0, 0.0))
            .await;

        rotation.yaw = self.yaw;
        rotation.pitch = self.pitch;

        Ok(())
    }
}