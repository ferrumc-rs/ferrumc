use ferrumc_macros::event_handler;
use ferrumc_net::connection::ConnectionState;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::update_time::TickEvent;
use ferrumc_net::packets::outgoing::update_time::UpdateTimePacket;
use ferrumc_net::GlobalState;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use futures::StreamExt;
use tracing::error;

#[event_handler]
async fn handle_tick(event: TickEvent, state: GlobalState) -> Result<TickEvent, NetError> {
    // info!("Tick {} ", event.tick);
    // TODO: Handle tick in terms of game logic here
    // this should call a function in world which handles the world state and calls the appropriate events which send their respective packets

    ///////

    let packet = UpdateTimePacket::new(event.tick, event.tick % 24000);
    let packet = {
        let mut buffer = Vec::new();
        packet.encode(&mut buffer, &NetEncodeOpts::WithLength)?;
        buffer
    };

    let query = state
        .universe
        .query::<(&mut StreamWriter, &ConnectionState)>()
        .into_entities()
        .into_iter()
        .filter_map(|entity| {
            let conn_state = state.universe.get::<ConnectionState>(entity).ok()?;
            if matches!(*conn_state, ConnectionState::Play) {
                Some(entity)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    tokio::spawn(
        futures::stream::iter(query.into_iter())
            .fold((state, packet), move |(state, packet), entity| {
                async move {
                    if let Ok(mut writer) = state.universe.get_mut::<StreamWriter>(entity) {
                        if let Err(e) = writer
                            .send_packet(&packet.as_slice(), &NetEncodeOpts::None)
                            .await
                        {
                            error!("Error sending update_time packet: {}", e);
                        }
                    }

                    (state, packet)
                }
            })
    );
    Ok(event)
}
