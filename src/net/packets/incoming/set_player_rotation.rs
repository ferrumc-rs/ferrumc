use ferrumc_macros::{NetDecode, packet};

use crate::net::packets::{ConnectionId, IncomingPacket};
use crate::state::GlobalState;
use crate::utils::components::rotation::Rotation;

#[derive(NetDecode)]
#[packet(packet_id = 0x16, state = "play")]
pub struct SetPlayerRotation {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

impl IncomingPacket for SetPlayerRotation {
    async fn handle(
        self,
        conn_id: ConnectionId,
        state: GlobalState,
    ) -> crate::utils::prelude::Result<()> {
        let my_entity_id = conn_id;

        let component_storage = state.world.get_component_storage();

        let mut rotation = component_storage
            .get_mut_or_insert_with(my_entity_id, || Rotation::new(0.0, 0.0))
            .await;

        rotation.yaw = self.yaw;
        rotation.pitch = self.pitch;

        Ok(())
    }
}
