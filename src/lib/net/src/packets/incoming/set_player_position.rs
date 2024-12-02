use crate::packets::packet_events::TransformEvent;
use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_state::ServerState;
use std::sync::Arc;

#[derive(NetDecode)]
#[packet(packet_id = 0x1A, state = "play")]
pub struct SetPlayerPositionPacket {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub on_ground: bool,
}

impl IncomingPacket for SetPlayerPositionPacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        let transform_event = TransformEvent::new(conn_id)
            .position((self.x, self.feet_y, self.z).into())
            .on_ground(self.on_ground);

        TransformEvent::trigger(transform_event, state).await?;

        Ok(())
    }
}
