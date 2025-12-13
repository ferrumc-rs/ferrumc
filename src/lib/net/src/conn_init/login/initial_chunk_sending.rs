use ferrumc_world::block_state_id::BlockStateId;
use byteorder::{BigEndian, WriteBytesExt};
use crate::connection::StreamWriter;
use crate::errors::NetError;
use crate::packets::outgoing::set_center_chunk::SetCenterChunk;
use ferrumc_config::ServerConfig;
use ferrumc_state::GlobalState;
use tracing::info;
use ferrumc_macros::block;
use ferrumc_net_codec::net_types::bitset::BitSet;
use ferrumc_net_codec::net_types::byte_array::ByteArray;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use crate::packets::outgoing::chunk_and_light_data::ChunkAndLightData;

pub(super) fn send_initial_chunks(
    connection: &StreamWriter,
    _state: &GlobalState,
    config: &ServerConfig,
    client_view_distance: i8,
    _compressed: bool,
) -> Result<(), NetError> {
    info!("Sending initial chunks supposedly");

    // 1. Send center chunk
    connection.send_packet(SetCenterChunk::new(0, 0))?;

    // 2. Calculate the render distance
    let radius = calculate_render_distance(config, client_view_distance);
    info!("Calculated render distance: {}", radius);

    // -----------------------------------------------------------------------
    // PRE-CALCULATE CHUNK DATA (Fancy Maths / Hardcoding)
    // -----------------------------------------------------------------------

    // We need 24 sections for a world height of 384 (-64 to 320).
    const SECTION_COUNT: usize = 24;

    // Construct the raw data buffer for the chunk sections
    let mut chunk_sections_data = Vec::with_capacity(SECTION_COUNT * 8);

    for i in 0..SECTION_COUNT {
        // Section 0 is the bottom-most section (Y = -64). Make -64 ~ 64 = Bedrock.
        // All other sections are Air.
        // 8 sections = 384/24 * 8 = -64 (0) ~ 64 (7).
        let is_bottom = i < 4*2;

        // 1. Block Count (u16 Big Endian)
        // 16x16x16 = 4096 blocks.
        let block_count = if is_bottom { 4096 } else { 0 };
        chunk_sections_data.write_u16::<BigEndian>(block_count)?;

        // 2. Block States (Paletted Container)
        // Format: [BitsPerEntry(u8)] + [Palette(VarInt Array)] + [DataArray(Long Array)]

        // Bits Per Entry = 0 (Single Value Palette).
        // This means no Data Array follows, just the one palette value.
        chunk_sections_data.write_u8(0)?;

        // Palette Value (VarInt).
        // Use the block! macro and get the block ids for bedrock and air.
        let block_id = if is_bottom { block!("bedrock") } else { block!("air") };
        block_id.to_varint().write(&mut chunk_sections_data)?;

        // 3. Biomes (Paletted Container)
        // Bits Per Entry = 0 (Single Value)
        chunk_sections_data.write_u8(0)?;
        // Palette Value (VarInt). ID 1 = Plains.
        VarInt::new(1).write(&mut chunk_sections_data)?;
    }

    // Prepare empty lighting masks (Client will treat as full dark or recalculate)
    // Mask size = Sections + 2 (for Y-1 and Y+Max)
    let sky_mask = BitSet::new(SECTION_COUNT + 2);
    let block_mask = BitSet::new(SECTION_COUNT + 2);
    let mut empty_sky = BitSet::new(SECTION_COUNT + 2);
    let mut empty_block = BitSet::new(SECTION_COUNT + 2);

    // Mark all sections as having "empty" light data to prevent client waiting
    empty_sky.set_all(true);
    empty_block.set_all(true);

    // -----------------------------------------------------------------------
    // SEND LOOP
    // -----------------------------------------------------------------------

    for x in -radius..=radius {
        for z in -radius..=radius {

            // Create the struct using the pre-calculated data
            // We clone the `chunk_sections_data` for every chunk.
            // Since it's very small (due to single-value palette), this is extremely fast.
            let chunk_packet = ChunkAndLightData {
                chunk_x: x,
                chunk_z: z,
                // Empty heightmaps - client will default to min-height
                heightmaps: LengthPrefixedVec::new(vec![]),
                data: ByteArray::new(chunk_sections_data.clone()),
                block_entities: LengthPrefixedVec::new(vec![]),
                sky_light_mask: sky_mask.clone(),
                block_light_mask: block_mask.clone(),
                empty_sky_light_mask: empty_sky.clone(),
                empty_block_light_mask: empty_block.clone(),
                sky_light_arrays: LengthPrefixedVec::new(vec![]),
                block_light_arrays: LengthPrefixedVec::new(vec![]),
            };

            // StreamWriter automatically handles encryption and compression via `send_packet`
            if let Err(e) = connection.send_packet(chunk_packet) {
                return Err(NetError::Misc(format!("Failed to send chunk {},{}: {:?}", x, z, e)));
            }
        }
    }

    Ok(())
}

fn calculate_render_distance(
    server_config: &ServerConfig,
    client_view_distance: i8,
) -> i32 {
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
