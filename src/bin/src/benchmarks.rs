//! Benchmark utilities for measuring chunk generation and serialization throughput.
//!
//! This module provides tools to measure the performance of the chunk pipeline,
//! helping identify bottlenecks in generation, serialization, and compression.

use crate::errors::BinaryError;
use crate::launch;
use ferrumc_net::compression::compress_packet;
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_world::pos::ChunkPos;
use std::time::{Duration, Instant};
use tracing::info;

/// Run chunk generation and serialization benchmark.
///
/// This benchmark measures the throughput of:
/// 1. Chunk generation (terrain noise calculations)
/// 2. Packet creation (ChunkAndLightData serialization)
/// 3. Compression (Zlib)
/// 4. Optionally: Database persistence (bitcode + yazi + LMDB)
///
/// # Arguments
///
/// * `count` - Number of chunks to generate
/// * `save` - Whether to save chunks to the database
///
/// # Returns
///
/// Prints benchmark results and returns Ok on success.
pub fn run_chunk_benchmark(count: usize, save: bool) -> Result<(), BinaryError> {
    info!("=== Chunk Benchmark ===");
    info!("Chunks to generate: {}", count);
    info!("Save to DB: {}", save);
    info!("");

    // Initialize state (World + Generator)
    let start_time = Instant::now();
    let state = launch::create_state(start_time)?;
    info!("State initialized in {:?}", start_time.elapsed());

    // Track individual phase timings
    let mut generation_time = Duration::ZERO;
    let mut save_time = Duration::ZERO;
    let mut packet_time = Duration::ZERO;
    let mut compress_time = Duration::ZERO;
    let mut total_compressed_bytes: usize = 0;

    // Generate chunks in a grid pattern around origin
    let grid_size = (count as f64).sqrt().ceil() as i32;
    let half = grid_size / 2;

    info!("Starting benchmark...");
    let benchmark_start = Instant::now();

    let mut generated = 0;
    'outer: for x in -half..=half {
        for z in -half..=half {
            if generated >= count {
                break 'outer;
            }

            let chunk_pos = ChunkPos::new(x, z);

            // 1. Generate chunk
            let gen_start = Instant::now();
            let chunk = state
                .terrain_generator
                .generate_chunk(chunk_pos)
                .map_err(|e| BinaryError::Custom(format!("Generation failed: {:?}", e)))?;
            generation_time += gen_start.elapsed();

            // 2. Save to DB (if enabled)
            if save {
                let save_start = Instant::now();
                state
                    .world
                    .save_ferrumc_chunk(chunk_pos, "overworld", &chunk)
                    .map_err(|e| BinaryError::Custom(format!("Save failed: {:?}", e)))?;
                save_time += save_start.elapsed();
            }

            // 3. Create packet
            let packet_start = Instant::now();
            let packet = ChunkAndLightData::from_ferrumc_chunk(&chunk)
                .map_err(|e| BinaryError::Custom(format!("Packet creation failed: {:?}", e)))?;
            packet_time += packet_start.elapsed();

            // 4. Compress
            let compress_start = Instant::now();
            let compressed =
                compress_packet(&packet, true, &NetEncodeOpts::WithLength, 512).map_err(|e| {
                    BinaryError::Custom(format!("Compression failed: {:?}", e))
                })?;
            compress_time += compress_start.elapsed();
            total_compressed_bytes += compressed.len();

            generated += 1;

            // Progress report every 100 chunks
            if generated % 100 == 0 {
                info!("Progress: {}/{} chunks...", generated, count);
            }
        }
    }

    let total_time = benchmark_start.elapsed();

    // Calculate stats
    let chunks_per_sec = generated as f64 / total_time.as_secs_f64();
    let avg_time_per_chunk = total_time / generated as u32;
    let avg_compressed_size = total_compressed_bytes / generated;

    // Print results
    info!("");
    info!("=== Benchmark Results ===");
    info!("Chunks generated:    {}", generated);
    info!("Total time:          {:?}", total_time);
    info!("Throughput:          {:.2} chunks/sec", chunks_per_sec);
    info!("Avg time per chunk:  {:?}", avg_time_per_chunk);
    info!("");
    info!("=== Phase Breakdown ===");
    info!(
        "Generation:          {:?} ({:.1}%)",
        generation_time,
        100.0 * generation_time.as_secs_f64() / total_time.as_secs_f64()
    );
    if save {
        info!(
            "DB Save:             {:?} ({:.1}%)",
            save_time,
            100.0 * save_time.as_secs_f64() / total_time.as_secs_f64()
        );
    }
    info!(
        "Packet Creation:     {:?} ({:.1}%)",
        packet_time,
        100.0 * packet_time.as_secs_f64() / total_time.as_secs_f64()
    );
    info!(
        "Compression:         {:?} ({:.1}%)",
        compress_time,
        100.0 * compress_time.as_secs_f64() / total_time.as_secs_f64()
    );
    info!("");
    info!("=== Size Stats ===");
    info!("Total compressed:    {} bytes", total_compressed_bytes);
    info!("Avg chunk size:      {} bytes", avg_compressed_size);
    info!("");

    // Sync DB if we saved
    if save {
        info!("Syncing database...");
        state.world.sync()?;
        info!("Database synced.");
    }

    info!("Benchmark complete!");
    Ok(())
}
