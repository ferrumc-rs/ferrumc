use tracing::trace;

use ferrumc_macros::{Decode, packet};

use crate::Connection;
use crate::net::packets::IncomingPacket;
use crate::state::GlobalState;
use crate::utils::encoding::position::Position;
use crate::utils::prelude::*;

#[derive(Decode)]
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
    async fn handle(&self, conn: &mut Connection, state: GlobalState) -> Result<()> {
        trace!("SetPlayerPosAndRotate packet received");
        trace!("X: {}", self.x);
        trace!("Y: {}", self.y);
        trace!("Z: {}", self.z);
        trace!("Yaw: {}", self.yaw);
        trace!("Pitch: {}", self.pitch);

        let my_entity_id = conn.metadata.entity;

        let component_storage = state.world.get_component_storage();

        let mut position = component_storage
            .get_mut::<Position>(my_entity_id)
            .await
            .ok_or(Error::from(crate::ecs::error::Error::ComponentNotFound))?;

        *position = Position {
            x: self.x as i32,
            y: self.y as i16,
            z: self.z as i32,
        };

        Ok(())
    }
}
