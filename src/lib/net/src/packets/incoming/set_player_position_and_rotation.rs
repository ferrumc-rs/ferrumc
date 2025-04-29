use crate::packets::packet_events::TransformEvent;
use crate::packets::IncomingPacket;

use crate::errors::NetError;
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
    fn handle(self, conn_id: usize, state: Arc<ServerState>) -> Result<(), NetError> {
        let event = TransformEvent::new(conn_id)
            .position((self.x, self.feet_y, self.z).into())
            .rotation((self.yaw, self.pitch).into())
            .on_ground(self.on_ground);

        TransformEvent::trigger(event, state)?;

        // {
        //     let mut on_ground = state.universe.get_mut::<&mut OnGround>(conn_id)?;
        //     on_ground.0 = self.on_ground;
        // }
        // {
        //     let mut pos = state.universe.get_mut::<&mut Position>(conn_id)?;
        //     pos.x = self.x;
        //     pos.y = self.feet_y;
        //     pos.z = self.z;
        // }
        // {
        //     let mut rotation = state
        //         .universe
        //         .get_mut::<&mut ferrumc_core::transform::rotation::Rotation>(conn_id)?;
        //     rotation.yaw = self.yaw;
        //     rotation.pitch = self.pitch;
        // }

        Ok(())
    }
}
