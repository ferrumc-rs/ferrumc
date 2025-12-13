//! ChunkSender component and ChunkCommand for async chunk streaming.
//!
//! Each player entity gets a `ChunkSender` component that holds a channel
//! to communicate with their dedicated async chunk loader task. This allows
//! the ECS tick to remain fast while chunk loading happens in the background.

use bevy_ecs::component::Component;
use tokio::sync::mpsc::Sender;

/// Component that holds a channel sender to communicate with the player's
/// dedicated async chunk loader task.
///
/// When a player moves or needs chunks recalculated, systems send `ChunkCommand`
/// messages through this channel. The async task processes these commands
/// without blocking the main game tick.
#[derive(Component)]
pub struct ChunkSender {
    /// Channel to send commands to the async chunk loader task.
    pub tx: Sender<ChunkCommand>,
}

/// Commands that can be sent to the async chunk loader task.
///
/// These commands are processed asynchronously, allowing chunk loading
/// to happen in the background without blocking the ECS tick.
#[derive(Debug)]
pub enum ChunkCommand {
    /// The player moved to a new chunk center. Load/unload chunks accordingly.
    ///
    /// Fields use primitive types (i32, u8) to avoid dependency complexity
    /// with ChunkPos from ferrumc-world.
    UpdateCenter {
        /// The chunk X coordinate the player is now in.
        chunk_x: i32,
        /// The chunk Z coordinate the player is now in.
        chunk_z: i32,
        /// The view distance radius in chunks.
        radius: u8,
    },
    /// The client acknowledged receiving a batch of chunks.
    /// The f32 indicates the client's desired chunks-per-tick rate.
    BatchReceived(f32),
    /// Stop the chunk loader task (e.g., on player disconnect).
    Stop,
}
