use crate::net::packets::{ConnectionId, IncomingPacket};
use crate::net::systems::chunk_sender::{ChunkSender, CHUNK_RADIUS};
use crate::state::GlobalState;
use crate::utils::components::rotation::Rotation;
use crate::utils::encoding::position::Position;
use crate::utils::prelude::*;
use ferrumc_macros::{packet, NetDecode};
use tracing::{trace};

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

        let old_chunk_pos = (position.x >> 4, position.z >> 4);
        let new_chunk_pos = (self.x as i32 >> 4, self.z as i32 >> 4);

        if old_chunk_pos != new_chunk_pos {
            let state_clone = state.clone();
            tokio::spawn(
                async move {
                    ChunkSender::send_chunks_to_player(state_clone, my_entity_id).await?;

                    Ok::<(), Error>(())
                }
            );
        }

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
