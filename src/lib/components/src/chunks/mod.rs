//! Chunk-related components for player chunk streaming.
//!
//! This module provides the async chunk loading architecture where each player
//! has a dedicated Tokio task that handles chunk loading/unloading operations
//! completely decoupled from the main game tick.

mod chunk_sender;

pub use chunk_sender::{ChunkCommand, ChunkSender};
