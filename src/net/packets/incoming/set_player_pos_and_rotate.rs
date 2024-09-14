use crate::net::packets::{ConnectionId, IncomingPacket};
use crate::net::systems::chunk_sender::ChunkSender;
use crate::state::GlobalState;
use crate::utils::components::rotation::Rotation;
use crate::utils::encoding::position::Position;
use crate::utils::prelude::*;
use ferrumc_macros::{packet, NetDecode};
use tracing::trace;

#[derive(NetDecode, Debug)]
#[packet(packet_id = 0x15, state = "play")]
pub struct SetPlayerPosAndRotate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

impl IncomingPacket for SetPlayerPosAndRotate {
    async fn handle(self, conn_id: ConnectionId, state: GlobalState) -> Result<()> {
        let my_entity_id = conn_id;

        let component_storage = state.world.get_component_storage();

        let mut position = component_storage.get_mut::<Position>(my_entity_id).await?;
        let mut rotation = component_storage.get_mut::<Rotation>(my_entity_id).await?;

        ChunkSender::send_chunks_to_player_if_needed(
            state.clone(),
            my_entity_id,
            (position.x >> 4, position.z >> 4),
        )
        .await?;

        *position = Position {
            x: self.x as i32,
            y: self.y as i16,
            z: self.z as i32,
        };

        *rotation = Rotation {
            yaw: self.yaw,
            pitch: self.pitch,
        };

        trace!("SetPlayerPosAndRotate packet received: {:?}", self);

        Ok(())
    }
}
