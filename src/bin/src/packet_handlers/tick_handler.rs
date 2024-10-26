use std::sync::Arc;

use ferrumc_macros::event_handler;
use ferrumc_net::connection::ConnectionState;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::update_time::TickEvent;
use ferrumc_net::packets::outgoing::update_time::UpdateTimePacket;
use ferrumc_net::GlobalState;
use ferrumc_net_codec::encode::NetEncodeOpts;
use tracing::error;

#[event_handler]
async fn handle_tick(event: TickEvent, state: GlobalState) -> Result<TickEvent, NetError> {
    // info!("Tick {} ", event.tick);
    // TODO: Handle tick in terms of game logic here
    // this should call a function in world which handles the world state and calls the appropriate events which send their respective packets

    ///////

    let packet = Arc::new(UpdateTimePacket::new(event.tick, event.tick % 24000));

    let query = state
        .universe
        .query::<(&mut StreamWriter, &ConnectionState)>();

    for (mut writer, connection_state) in query {
        match *connection_state {
            ConnectionState::Play => {
                if let Err(e) = writer
                    .send_packet(packet.as_ref(), &NetEncodeOpts::WithLength)
                    .await
                {
                    error!("Error sending update_time packet: {}", e);
                }
            }
            _ => {}
        }
    }

    Ok(event)
}
