use bevy_ecs::prelude::{Query, Res};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::incoming::player_command::PlayerCommandAction;
use ferrumc_net::packets::outgoing::entity_metadata::{EntityMetadata, EntityMetadataPacket};
use ferrumc_net::PlayerCommandPacketReceiver;
use tracing::error;

pub fn handle(events: Res<PlayerCommandPacketReceiver>, query: Query<&StreamWriter>) {
    if events.0.is_empty() {
        return;
    }
    for (event, entity) in events.0.try_iter() {
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
                for stream in query {
                    if let Err(err) = stream.send_packet(packet.clone()) {
                        error!("Failed to send packet: {:?}", err);
                    }
                }
            }
            PlayerCommandAction::StopSneaking => {
                let packet =
                    EntityMetadataPacket::new(event.entity_id, [EntityMetadata::entity_standing()]);

                for stream in query {
                    if let Err(err) = stream.send_packet(packet.clone()) {
                        error!("Failed to send packet: {:?}", err);
                    }
                }
            }
            _ => {}
        }
    }
}
