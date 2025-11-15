use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::incoming::player_command::PlayerCommandAction;
use ferrumc_net::packets::outgoing::entity_metadata::{EntityMetadata, EntityMetadataPacket};
use ferrumc_net::PlayerCommandPacketReceiver;
use ferrumc_state::GlobalStateResource;
use tracing::error;

pub fn handle(
    receiver: Res<PlayerCommandPacketReceiver>,
    query: Query<(Entity, &StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    for (event, _) in receiver.0.try_iter() {
        match event.action {
            PlayerCommandAction::StartSneaking => {
                let packet = EntityMetadataPacket::new(
                    event.entity_id,
                    [
                        EntityMetadata::entity_sneaking_visual(),
                        EntityMetadata::entity_sneaking_pressed(),
                    ],
                );

                // TODO: Don't clone
                for (entity, conn) in query {
                    if !state.0.players.is_connected(entity) {
                        continue;
                    }
                    if let Err(err) = conn.send_packet_ref(&packet) {
                        error!("Failed to send start sneaking packet: {:?}", err);
                    }
                }
            }
            PlayerCommandAction::StopSneaking => {
                let packet =
                    EntityMetadataPacket::new(event.entity_id, [EntityMetadata::entity_standing()]);

                for (entity, conn) in query {
                    if !state.0.players.is_connected(entity) {
                        continue;
                    }
                    if let Err(err) = conn.send_packet_ref(&packet) {
                        error!("Failed to send stop sneaking packet: {:?}", err);
                    }
                }
            }
            _ => {}
        }
    }
}
