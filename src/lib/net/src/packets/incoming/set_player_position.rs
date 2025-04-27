use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_state::ServerState;
use std::sync::Arc;

#[derive(NetDecode)]
#[packet(packet_id = "move_player_pos", state = "play")]
pub struct SetPlayerPositionPacket {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub on_ground: bool,
}

impl IncomingPacket for SetPlayerPositionPacket {
    fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        // let transform_event = TransformEvent::new(conn_id)
        //     .position((self.x, self.feet_y, self.z).into())
        //     .on_ground(self.on_ground);
        //
        // TransformEvent::trigger(transform_event, state)?;

        {
            let mut pos = state.universe.get_mut::<&mut Position>(conn_id)?;
            pos.x = self.x;
            pos.y = self.feet_y;
            pos.z = self.z;
        }
        {
            let mut on_ground = state.universe.get_mut::<&mut OnGround>(conn_id)?;
            on_ground.0 = self.on_ground;
        }

        Ok(())
    }
}
