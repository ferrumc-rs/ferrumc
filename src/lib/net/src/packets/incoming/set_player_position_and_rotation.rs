use crate::packets::packet_events::TransformEvent;
use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_state::ServerState;
use std::sync::Arc;

#[derive(NetDecode)]
#[packet(packet_id = "move_player_pos_rot", state = "play")]
pub struct SetPlayerPositionAndRotationPacket {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

impl IncomingPacket for SetPlayerPositionAndRotationPacket {
    fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        let event = TransformEvent::new(conn_id)
            .position((self.x, self.feet_y, self.z).into())
            .rotation((self.yaw, self.pitch).into())
            .on_ground(self.on_ground);

        TransformEvent::trigger(event, state)?;

        Ok(())
    }
}
