use bevy_ecs::prelude::*;
use ferrumc_core::mq;
use ferrumc_net::{
    connection::StreamWriter, packets::outgoing::system_message::SystemMessagePacket,
};
use ferrumc_state::GlobalStateResource;
use tracing::error;

fn send(
    writer: &StreamWriter,
    receiver: Entity,
    state: &GlobalStateResource,
    entry: ferrumc_core::mq::QueueEntry,
) {
    if !state.0.players.is_connected(receiver) {
        return;
    }

    if let Err(err) = writer.send_packet(SystemMessagePacket {
        message: entry.message.into(),
        overlay: entry.overlay,
    }) {
        error!("failed sending queued message to player: {err}");
    }
}

pub fn process(query: Query<(Entity, &StreamWriter)>, state: Res<GlobalStateResource>) {
    while !mq::QUEUE.is_empty() {
        let entry = mq::QUEUE.pop().unwrap();

        match entry.receiver {
            Some(receiver) => {
                let Ok((_, writer)) = query.get(receiver) else {
                    continue;
                };

                send(writer, receiver, &state, entry);
            }

            None => {
                for (receiver, writer) in query {
                    send(writer, receiver, &state, entry.clone());
                }
            }
        }
    }
}
