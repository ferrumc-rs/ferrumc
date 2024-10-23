use std::sync::Arc;

use ferrumc_ecs::components::ComponentRefMut;
use ferrumc_ecs::query::Query;
use ferrumc_macros::event_handler;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::tick_event::TickEvent;
use ferrumc_net::packets::outgoing::update_time::UpdateTimePacket;
use ferrumc_net::GlobalState;
use ferrumc_net_codec::encode::NetEncodeOpts;
use tracing::{debug, error};

pub async fn update_time(
    mut writer: ComponentRefMut<'_, StreamWriter>,
    packet: Arc<UpdateTimePacket>,
) {
    match writer
        .send_packet(packet.as_ref(), &NetEncodeOpts::WithLength)
        .await
    {
        Ok(_) => {
            // debug!("Sent update time packet");
        }
        Err(err) => {
            error!("Failed to send update time packet: {}", err);
        }
    };
}

#[event_handler]
async fn handle_tick(event: TickEvent, state: GlobalState) -> Result<TickEvent, NetError> {
    // info!("Tick {} ", event.tick);
    let packet = Arc::new(UpdateTimePacket::new(event.tick, event.tick % 24000));

    let entities: Query<'_, StreamWriter> = state.universe.query();

    for writer in entities {
        update_time(writer, packet.clone()).await; //TODO make this async
    }

    Ok(event)
}
