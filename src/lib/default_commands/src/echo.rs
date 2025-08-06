use bevy_ecs::prelude::*;
use ferrumc_commands::arg::primitive::string::QuotableString;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::command;
use ferrumc_net::{
    connection::StreamWriter, packets::outgoing::system_message::SystemMessagePacket,
};
use ferrumc_text::TextComponentBuilder;
use tracing::error;

#[command("echo")]
fn test_command(
    #[arg] message: QuotableString,
    #[sender] sender: Entity,
    query: Query<(&StreamWriter, &PlayerIdentity)>,
) {
    let (writer, identity) = query.get(sender).expect("sender has no stream writer");
    if let Err(err) = writer.send_packet(&SystemMessagePacket::new(
        TextComponentBuilder::new(format!("{} said: ", identity.username))
            .extra(TextComponentBuilder::new(message.clone()).build())
            .build(),
        false,
    )) {
        error!("failed sending command error to player: {err}");
    }
}
