use tracing::trace;

use ferrumc_macros::{packet, NetDecode};

use crate::net::packets::{ConnectionId, IncomingPacket};
use crate::net::systems::chunk_sender::ChunkSender;
use crate::state::GlobalState;
use crate::utils::encoding::position::Position;

/// The set player position packet is sent by the client to the server to update the player's position.
#[derive(NetDecode)]
#[packet(packet_id = 0x14, state = "play")]
pub struct SetPlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub on_ground: bool,
}

impl IncomingPacket for SetPlayerPosition {
    async fn handle(
        self,
        conn_id: ConnectionId,
        state: GlobalState,
    ) -> crate::utils::prelude::Result<()> {
        trace!("SetPlayerPosition packet received");
        trace!("X: {}", self.x);
        trace!("Y: {}", self.y);
        trace!("Z: {}", self.z);

        let my_entity_id = conn_id;

        let component_storage = state.world.get_component_storage();

        let mut position = component_storage.get_mut::<Position>(my_entity_id).await?;

        ChunkSender::send_chunks_to_player_if_needed(
            state.clone(),
            my_entity_id,
            (position.x >> 4, position.z >> 4),
        )
        .await?;

        /*let old_chunk_pos = (position.x >> 4, position.z >> 4);
        let new_chunk_pos = (self.x as i32 >> 4, self.z as i32 >> 4);

        if old_chunk_pos != new_chunk_pos {
            let state_clone = state.clone();
            tokio::spawn(
                async move {
                    ChunkSender::send_chunks_to_player(state_clone, my_entity_id).await?;

                    Ok::<(), Error>(())
                }
            );
        }*/

        *position = Position {
            x: self.x as i32,
            y: self.y as i16,
            z: self.z as i32,
        };

        Ok(())
    }
}
