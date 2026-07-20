#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! # Overview
//!
//! This milestone delivers the deterministic *skeleton* of the simulation
//! layer: the messages crossing the sim boundary, the tick counter that drives
//! it, and shards that apply inputs at tick boundaries. The application still
//! drives one authoritative shard; a crate-internal, non-default shadow
//! scheduler prepares deterministic multi-shard worker ownership without
//! changing that runtime. The same internal seam owns a bounded typed
//! cross-shard queue whose messages apply only at the next tick boundary; this
//! is transport preparation, not an enabled entity-transfer feature. There is
//! no networking, no storage handle, and no plugin dispatch here yet.
//!
//! The pieces fit together as a one-way pipeline driven entirely by explicit
//! `tick` calls (never a wall clock):
//!
//! ```text
//!   GameInput  --enqueue-->  SimShard inbox  --run_tick-->  GameOutput
//!                                  ^                 ^
//!                                  |                 |
//!                          (bounded, reject     (TickCoordinator
//!                           backpressure)        advances the Tick)
//! ```
//!
//! - [`GameInput`] / [`GameOutput`] — the minimal typed messages.
//! - [`TickCoordinator`] / [`TickRate`] — the authoritative [`Tick`] counter,
//!   advanced one tick at a time with **no catch-up**.
//! - [`SimShard`] — owns a bounded inbox, the player set, and the resident
//!   chunks, applying inputs only at tick boundaries.
//! - [`ShardPartitioner`] / [`ShardOwnershipMap`] — deterministic typed 8x8
//!   region partitioning and overlap-free runtime ownership claims.
//! - [`ShardLifecycle`] — the explicit eligibility lifecycle used by the
//!   crate-internal shadow scheduler's logical worker plan.
//! - The crate-internal cross-shard envelope queue — canonical reject-newest
//!   admission after tick N and application as a separate prefix in tick N+1.
//! - Crate-internal per-shard persist batches — move-owned overlay prefixes
//!   bounded to the storage trait ceiling and emitted only after a successful
//!   scheduler tick.
//! - [`SimHarness`] / [`TickOutcome`] — a deterministic, wall-clock-free driver
//!   tying a coordinator to one shard, used by tests and replay.
//!
//! # Chunk residency
//!
//! A shard owns its chunks in a [`LoadedChunkMap`], governed by
//! [`ChunkTicket`]s: a chunk is resident exactly while it holds a ticket.
//! [`LoadedChunkMap::acquire`] runs the load-or-generate flow
//! ([`load_or_generate`]: try the [`WorldStore`](ferrumc_storage::WorldStore),
//! else generate with [`FlatWorldGenerator`](ferrumc_world::FlatWorldGenerator)),
//! [`LoadedChunkMap::release`] drops tickets and unloads, and
//! [`LoadedChunkMap::take_dirty`] hands changed chunks off as full save records,
//! while persist-dirty gameplay edits become overlay records. The
//! [`SpawnChunkTickets`] set keeps the world spawn resident. The map and
//! scheduler collect records but never persist them — flush *policy* is the
//! caller's.
//!
//! [`Tick`]: ferrumc_core::Tick

mod coordinator;
mod cross_shard;
mod error;
mod harness;
mod loaded;
mod message;
mod mutation;
mod ownership;
mod physics;
mod region;
#[allow(
    dead_code,
    reason = "the owner-gated shadow scheduler intentionally has no app wiring yet"
)]
pub(crate) mod scheduler;
mod shard;
mod spawn;
mod ticket;
mod time;

pub use coordinator::{TickCoordinator, TickRate};
pub use error::{SimError, SimResult};
pub use harness::{SimHarness, TickOutcome};
pub use loaded::{
    load_or_generate, ChunkAcquired, ChunkProvenance, LoadedChunkMap, TicketRelease,
    CHUNK_SCHEMA_VERSION, OVERLAY_SCHEMA_VERSION,
};
pub use message::{GameInput, GameOutput, SpawnedEntityKind};
pub use physics::{AIR_DRAG_Y, GRAVITY_ARROW, GRAVITY_ITEM, GRAVITY_LIVING, GRAVITY_THROWN};
// Only the cause is public: the structured `MutationResult` stays internal to
// the shard so the app never confuses it with `ferrumc_observability::MutationResult`.
pub use mutation::{MutationCause, PendingMutation};
pub use ownership::{
    ShardId, ShardLifecycle, ShardLifecycleState, ShardOwnership, ShardOwnershipMap,
    ShardPartitioner, ShardRegion, MAX_SHARD_COORD, MIN_SHARD_COORD, SHARD_WIDTH_CHUNKS,
};
pub use region::{RegionLimits, RegionOp};
pub use shard::SimShard;
pub use time::{WorldTime, DAY_LENGTH_TICKS, TIME_DAY, TIME_MIDNIGHT, TIME_NIGHT, TIME_NOON};

/// Re-exported from [`ferrumc_world`] so callers can name the block-state ids
/// carried by [`GameOutput::BlockChanged`] / produced by block-edit inputs
/// without taking a direct dependency on the world crate.
pub use ferrumc_world::BlockStateId;
/// Re-exported from [`ferrumc_world`] so the session layer can name the sign
/// block-entity carried by [`GameOutput::SignUpdated`] (to encode its network
/// NBT) without depending on the world crate directly.
pub use ferrumc_world::{Sign, SignFace, SignKind, SIGN_LINES};
pub use spawn::SpawnChunkTickets;
pub use ticket::{ChunkTicket, TicketLevel, TicketReason};
