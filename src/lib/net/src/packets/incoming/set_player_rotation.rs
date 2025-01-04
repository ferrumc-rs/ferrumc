use crate::packets::packet_events::TransformEvent;
use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_state::ServerState;
use std::sync::Arc;

#[derive(NetDecode)]
#[packet(packet_id = "move_player_rot", state = "play")]
pub struct SetPlayerRotationPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

impl IncomingPacket for SetPlayerRotationPacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        let event = TransformEvent::new(conn_id)
            .rotation((self.yaw, self.pitch).into())
            .on_ground(self.on_ground);

        TransformEvent::trigger(event, state).await?;

        Ok(())
    }
}
