//! Forget Level Chunk packet - tells the client to unload a chunk from memory.
//!
//! This packet is sent when a player moves far enough away from a chunk that
//! it should no longer be rendered. Proper unloading is important for client
//! performance, especially at high render distances.

use ferrumc_macros::{packet, NetEncode};

/// Packet sent to tell the client to unload a chunk.
///
/// This packet is used to inform the client that a specific chunk
/// should be removed from memory, typically because the player has
/// moved far enough away from it.
/// # Fields
/// - `chunk_x`: The X coordinate of the chunk to unload.
/// - `chunk_z`: The Z coordinate of the chunk to unload.
#[derive(NetEncode)]
#[packet(packet_id = "forget_level_chunk", state = "play")]
pub struct UnloadChunk {
    /// The Z coordinate of the chunk to unload.
    pub chunk_z: i32,
    /// The X coordinate of the chunk to unload.
    pub chunk_x: i32,
}

impl UnloadChunk {
    /// Create a new UnloadChunk packet for the given chunk coordinates.
    pub fn new(chunk_x: i32, chunk_z: i32) -> Self {
        Self { chunk_x, chunk_z }
    }
}
