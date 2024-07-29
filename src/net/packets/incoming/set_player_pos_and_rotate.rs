use tracing::trace;

use ferrumc_macros::{Decode, packet};

use crate::Connection;
use crate::net::packets::IncomingPacket;
use crate::utils::encoding::position::Position;
use crate::state::GlobalState;

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
    async fn handle(
        &self,
        _: &mut Connection,
        _: GlobalState,
    ) -> crate::utils::prelude::Result<()> {
        trace!("SetPlayerPosAndRotate packet received");
        trace!("X: {}", self.x);
        trace!("Y: {}", self.y);
        trace!("Z: {}", self.z);
        trace!("Yaw: {}", self.yaw);
        trace!("Pitch: {}", self.pitch);

        let my_entity_id = conn.metadata.entity.id();

        let mut world = crate::GET_WORLD().write().await;
        let component_storage = world.get_component_storage_mut();
        let position =  match component_storage.get_mut::<Position>(my_entity_id as usize) {
            Some(pos) => pos,
            None => {
                return Err(Error::ComponentNotFound(String::from("Position"), my_entity_id))
            }
        };

        *position = Position {
            // x: self.x as i32,
            x: -189,
            y: self.y as i16,
            // z: self.z as i32,
            z: -548
        };

        Ok(())
    }
}
