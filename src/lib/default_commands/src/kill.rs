#![expect(clippy::type_complexity)]
use bevy_ecs::prelude::{Commands, Entity, Query};
use ferrumc_commands::arg::entities::EntityArgument;
use ferrumc_commands::Sender;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::command;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::remove_entities::RemoveEntitiesPacket;
use ferrumc_net::packets::outgoing::system_message::SystemMessagePacket;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_text::{Color, NamedColor, TextComponentBuilder};
use tracing::debug;

#[command("kill")]
fn kill_command(
    #[sender] sender: Sender,
    #[arg] entity_argument: EntityArgument,
    args: (
        Query<(Entity, Option<&EntityIdentity>, Option<&PlayerIdentity>)>,
        Commands,
        Query<&StreamWriter>,
    ),
) {
    let (query, mut cmd, conn_query) = args;

    let selected_entities = entity_argument.resolve(query.iter());

    let mut removed_entities = Vec::new();

    let mut removed_count = 0;
    let killed_message = SystemMessagePacket {
        message: ferrumc_nbt::NBT::new(
            TextComponentBuilder::new("You have been killed. How sad :(")
                .bold()
                .color(Color::Named(NamedColor::Red))
                .build(),
        ),
        overlay: false,
    };
    for entity in selected_entities {
        if let Ok((ent, entity_id_opt, player_id_opt)) = query.get(entity) {
            if let Some(entity_id) = entity_id_opt {
                removed_entities.push(entity_id.entity_id.into());
                cmd.entity(ent).despawn();
                removed_count += 1;
            } else if let Some(_player_id) = player_id_opt {
                // Don't remove players, just send them a killed message
                if let Ok(conn) = conn_query.get(ent) {
                    if let Err(err) = conn.send_packet_ref(&killed_message) {
                        sender.send_message(
                            format!("Failed to send killed message: {}", err).into(),
                            false,
                        );
                    }
                }
            }
        }
    }

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
