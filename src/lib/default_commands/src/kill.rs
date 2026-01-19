use bevy_ecs::prelude::{Commands, Entity, Query};
use ferrumc_commands::Sender;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_macros::command;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::remove_entities::RemoveEntitiesPacket;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;

#[command("kill")]
fn kill_command(
    #[sender] sender: Sender,
    args: (
        Query<(Entity, &EntityIdentity)>,
        Commands,
        Query<&StreamWriter>,
    ),
) {
    let (query, mut cmd, conn_query) = args;

    let mut removed_entities = Vec::new();

    for (entity, entity_id) in query.iter() {
        removed_entities.push(entity_id.entity_id.into());
        cmd.entity(entity).despawn();
    }

    let removed_count = removed_entities.len();

    let packet = RemoveEntitiesPacket {
        entity_ids: LengthPrefixedVec::new(removed_entities),
    };

    for conn in conn_query.iter() {
        if let Err(err) = conn.send_packet_ref(&packet) {
            sender.send_message(
                format!("Failed to send RemoveEntitiesPacket: {}", err).into(),
                false,
            );
        }
    }

    sender.send_message(
        format!("Killed {} entities (excluding players).", removed_count).into(),
        false,
    );
}
