use bevy_ecs::system::{Query, Res};
use ferrumc_core::mq;
use ferrumc_net::{
    connection::StreamWriter, packets::outgoing::system_message::SystemMessagePacket,
};
use ferrumc_state::GlobalStateResource;
use tracing::error;

pub fn process(query: Query<&StreamWriter>, state: Res<GlobalStateResource>) {
    while !mq::QUEUE.is_empty() {
        let entry = mq::QUEUE.pop().unwrap();

        if !state.0.players.is_connected(entry.receiver) {
            continue;
        }

        let Ok(writer) = query.get(entry.receiver) else {
            continue;
        };

        if let Err(err) = writer.send_packet(&SystemMessagePacket {
            message: entry.message,
            overlay: entry.overlay,
        }) {
            error!("failed sending queued message to player: {err}");
        }
    }
}
