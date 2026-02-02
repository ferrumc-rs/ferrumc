use bevy_ecs::prelude::Query;
use ferrumc_commands::arg::primitive::string::GreedyString;
use ferrumc_commands::Sender;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::command;
use ferrumc_nbt::NBT;
use ferrumc_net::connection::StreamWriter;

#[command("say")]
fn say_command(
    #[sender] sender: Sender,
    #[arg] message: GreedyString,
    query: Query<(&StreamWriter, &PlayerIdentity)>,
) {
    let full_message = match sender {
        Sender::Server => format!("<Server> {}", message.as_str()),
        Sender::Player(entity) => {
            let player_identity = query.get(entity).expect("sender does not exist").1;
            format!("<{}> {}", player_identity.username, message.as_str())
        }
    };

    let packet = ferrumc_net::packets::outgoing::system_message::SystemMessagePacket {
        message: NBT::new(full_message.into()),
        overlay: false,
    };

    for (conn, _) in query.iter() {
        conn.send_packet_ref(&packet)
            .expect("Failed to send packet");
    }
}
