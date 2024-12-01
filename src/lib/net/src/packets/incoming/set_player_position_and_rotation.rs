use std::sync::Arc;
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{packet, NetDecode};
use crate::packets::IncomingPacket;
use crate::{NetResult, ServerState};
use crate::packets::packet_events::TransformEvent;

#[derive(NetDecode)]
#[packet(packet_id = 0x1B, state = "play")]
pub struct SetPlayerPositionAndRotationPacket {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool
}

impl IncomingPacket for SetPlayerPositionAndRotationPacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        let event = TransformEvent::new(conn_id)
            .position((self.x, self.feet_y, self.z).into())
            .rotation((self.yaw, self.pitch).into())
            .on_ground(self.on_ground);

        TransformEvent::trigger(event, state).await?;
        
        Ok(())
    }
}