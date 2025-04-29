use crate::errors::NetError;
use crate::packets::outgoing::entity_animation::EntityAnimationEvent;
use crate::packets::IncomingPacket;

use ferrumc_ecs::entities::Entity;
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use std::sync::Arc;

#[derive(NetDecode)]
#[packet(packet_id = "swing", state = "play")]
pub struct SwingArmPacket {
    hand: VarInt,
}

impl IncomingPacket for SwingArmPacket {
    fn handle(self, conn_id: Entity, state: Arc<ServerState>) -> Result<(), NetError> {
        let animation = {
            if self.hand == 0 {
                0
            } else {
                3
            }
        };
        let event = EntityAnimationEvent::new(conn_id, animation);
        EntityAnimationEvent::trigger(event, state)?;
        Ok(())
    }
}
