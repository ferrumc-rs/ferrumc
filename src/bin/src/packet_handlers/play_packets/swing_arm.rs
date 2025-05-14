use bevy_ecs::prelude::{Entity, Query};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::entity_animation::{EntityAnimationEvent, EntityAnimationPacket};
use ferrumc_net::SwingArmPacketReceiver;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use std::sync::Arc;
use tracing::error;

pub fn handle(events: SwingArmPacketReceiver, query: Query<&PlayerIdentity>, conn_query: Query<&StreamWriter>) {
    for (event, eid) in events {
        let animation = {
            if event.hand == 0 {
                0
            } else {
                3
            }
        };
        let game_id = query.get(eid.0).unwrap();
        let packet = EntityAnimationPacket::new(VarInt::new(*game_id.uuid as i32), animation);
        for conn in conn_query.iter() {
            if let Err(e) = conn.send(packet.clone()) {
                error!("Failed to send packet: {}", e);
            }
        }
    }
}