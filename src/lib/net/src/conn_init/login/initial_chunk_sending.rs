use crate::connection::StreamWriter;
use crate::errors::NetError;
use crate::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use crate::packets::outgoing::set_center_chunk::SetCenterChunk;
use ferrumc_config::ServerConfig;
use ferrumc_state::GlobalState;

pub(super) fn send_initial_chunks(
    connection: &StreamWriter,
    _state: &GlobalState,
    config: &ServerConfig,
    client_view_distance: i8,
    _compressed: bool,
) -> Result<(), NetError> {
    // 1. Send center chunk
    connection.send_packet(SetCenterChunk::new(0, 0))?;

    // 2. Calculate render distance
    let radius = calculate_render_distance(config, client_view_distance);

    // 3. Send Flat Chunks
    // Fill the bottom 8 sections (Y -64 to 64) with bedrock
    let floor_sections = 8;

    for x in -radius..=radius {
        for z in -radius..=radius {
            // Use the new helper!
            let packet = ChunkAndLightData::flat(x, z, floor_sections)?;
            connection.send_packet(packet)?;
        }
    }

    Ok(())
}

fn calculate_render_distance(server_config: &ServerConfig, client_view_distance: i8) -> i32 {
    let server_render_distance = server_config.chunk_render_distance as i32;
    let client_view_distance = client_view_distance as i32;
    // Return the minimum of both,
    // Because: The server should not send more chunks than the client can handle,
    // and the client should not request more chunks than the server is willing to send.
    server_render_distance.min(client_view_distance)
}

// /// Sends initial chunks to the player.
// /// TODO: rework to use events/bevy ecs to send chunks instead of writing the logic here. Unify the logic essentially.
// fn send_initial_chunks(
//     conn_write: &StreamWriter,
//     state: &GlobalState,
//     config: &ServerConfig,
//     client_view_distance: i8,
//     compressed: bool,
// ) -> Result<(), NetError> {
//
//     // Send center chunk
//     conn_write.send_packet(SetCenterChunk::new(0, 0))?;
//
//     // Calculate render distance
//     let server_render_distance = config.chunk_render_distance as i32;
//     let client_view_distance = client_view_distance as i32;
//     let radius = server_render_distance.min(client_view_distance);
//
//     // Generate/load chunks in parallel
//     let mut batch = state.thread_pool.batch();
//
//     for x in -radius..=radius {
//         for z in -radius..=radius {
//             batch.execute({
//                 let state = state.clone();
//                 move || -> Result<Vec<u8>, NetError> {
//                     let chunk = state.world.load_chunk(ChunkPos::new(x,z), "overworld").unwrap_or(
//                         state
//                             .terrain_generator
//                             .generate_chunk(ChunkPos::new(x, z))
//                             .expect("Could not generate chunk")
//                             .into(),
//                     );
//                     let chunk_data =
//                         crate::packets::outgoing::chunk_and_light_data::ChunkAndLightData::from_chunk(
//                         ChunkPos::new(x,z),
//                             &chunk,
//                         )?;
//                     compress_packet(&chunk_data, compressed, &NetEncodeOpts::WithLength, 64)
//                 }
//             });
//         }
//     }
//
//     // Send all chunks
//     for packet in batch.wait() {
//         match packet {
//             Ok(data) => conn_write.send_raw_packet(data)?,
//             Err(err) => {
//                 error!("Failed to send chunk data: {:?}", err);
//                 return Err(NetError::Misc(format!(
//                     "Failed to send chunk data: {:?}",
//                     err
//                 )));
//             }
//         }
//     }
//
//     Ok(())
// }
