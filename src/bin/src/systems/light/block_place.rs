use bevy_ecs::message::MessageReader;
use bevy_ecs::prelude::Query;
use bevy_ecs::system::Res;
use tracing::{debug, error};
use ferrumc_messages::block_place::BlockPlaceEvent;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::update_light::UpdateLightPacket;
use ferrumc_state::GlobalStateResource;

pub fn handle(
    mut events: MessageReader<BlockPlaceEvent>,
    broadcast_query: Query<&StreamWriter>,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        let placed_pos = event.position;
        let chunk_pos = placed_pos.chunk();

        debug!("[Lighting] Block Placed.");

        for stream in broadcast_query.iter() {
            let world = &state.0.world;
            let mut chunk = world.load_chunk_mut(chunk_pos, "overworld").unwrap();
            {
                let mut lighting = world.light_engine.lock().unwrap();
                if let Err(e) = lighting.sky.on_block_changed(&mut chunk, &chunk_pos, placed_pos, event.old_id, event.new_id) {
                    debug!("Failed to change block: {}", e);
                }
            }

            if let Err(e) = stream.send_packet(UpdateLightPacket::from_chunk(chunk_pos, &chunk)) {
                error!("Failed to send update light: {}", e);
            }
        }
    }
}