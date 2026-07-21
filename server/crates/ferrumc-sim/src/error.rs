//! Error types for the simulation layer.

use std::io::ErrorKind;

use ferrumc_core::{ServerError, Tick};
use ferrumc_math::{ChunkPos, ShardPos};
use ferrumc_storage::ChunkKey;

use crate::ownership::{ShardId, ShardLifecycleState, ShardRegion};

/// A classifying error returned by simulation operations.
///
/// Each variant names the *kind* of failure so callers can react
/// programmatically rather than parse message strings, per the project's
/// error-classification rule. Convert one into the server-wide [`ServerError`]
/// with the provided [`From`] impl.
///
/// The enum is `#[non_exhaustive]`: more variants will appear as the simulation
/// grows, so downstream `match`es must include a wildcard arm.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum SimError {
    /// A [`SimShard`](crate::SimShard) inbox was at capacity and rejected an
    /// input.
    ///
    /// The inbox applies *reject* backpressure: it never blocks (it runs on a
    /// sim worker that must not stall) and never silently drops (that would
    /// desync clients). The caller — upstream, the session router — decides what
    /// to do on rejection (retry next tick, coalesce, or disconnect a flooding
    /// client).
    #[error("simulation shard inbox is full (capacity {capacity})")]
    InboxFull {
        /// The fixed inbox capacity that was reached.
        capacity: usize,
    },

    /// The tick counter could not advance because it would overflow [`u64`].
    ///
    /// At 20 ticks per second this cannot occur for roughly 29 billion years; it
    /// is surfaced as a classified error rather than a panic so the counter can
    /// never wrap silently and the no-panic rule still holds.
    #[error("simulation tick counter overflowed u64")]
    TickOverflow,

    /// A [`SimShard`](crate::SimShard) could not allocate an entity id because
    /// its per-shard counter would overflow [`i32`].
    ///
    /// Entity ids are signed 32-bit on the wire, so a shard can hand out at most
    /// `i32::MAX` of them over its lifetime. Exhausting that range is unreachable
    /// for a real shard (bounded to a handful of chunks), but it is surfaced as a
    /// classified error rather than a panic so the counter can never wrap and
    /// reissue an id that still refers to a live entity.
    #[error("simulation shard exhausted its entity id range")]
    EntityIdExhausted,

    /// Loading a chunk from the [`WorldStore`](ferrumc_storage::WorldStore)
    /// failed.
    ///
    /// Raised by the load-or-generate flow (see
    /// [`crate::load_or_generate`]) when the store read itself errors — a
    /// *missing* chunk is not a failure, it is the generate path. The underlying
    /// [`ServerError`] carries the storage classification.
    #[error("failed to load chunk at ({},{})", pos.x(), pos.z())]
    ChunkLoad {
        /// The position of the chunk whose load failed.
        pos: ChunkPos,
        /// The underlying storage error.
        #[source]
        source: ServerError,
    },

    /// A shard-grid position cannot describe a complete 8x8 region in the
    /// [`ChunkPos`] coordinate domain.
    #[error("shard position ({},{}) is outside the complete-region range", position.x(), position.z())]
    ShardRegionOutOfRange {
        /// The invalid typed shard position.
        position: ShardPos,
    },

    /// The same canonical shard region was claimed more than once.
    #[error("shard region {region} is already claimed by logical shard {shard}")]
    OverlappingShardOwnership {
        /// The fixed world/dimension region claimed twice.
        region: ShardRegion,
        /// The canonical logical shard whose duplicate claim was rejected.
        shard: ShardId,
    },

    /// A shard lifecycle attempted a same-state, backward, skipped, or
    /// post-terminal transition.
    #[error("shard {shard} cannot transition from {from} to {to}")]
    InvalidShardLifecycleTransition {
        /// The logical shard whose transition was rejected.
        shard: ShardId,
        /// The state retained after rejection.
        from: ShardLifecycleState,
        /// The rejected target state.
        to: ShardLifecycleState,
    },

    /// A scheduler operation named a shard that is not registered.
    #[error("logical shard {shard} is not registered with the scheduler")]
    UnknownScheduledShard {
        /// The unregistered logical shard.
        shard: ShardId,
    },

    /// More than one runnable shard was requested while the multi-shard switch
    /// was off.
    #[error("multi-shard scheduling is disabled, but {scheduled} runnable shards were requested")]
    MultipleScheduledShardsDisabled {
        /// Number of runnable shards the rejected operation would create.
        scheduled: usize,
    },

    /// The same shard appeared more than once in one scheduler execution plan.
    #[error("logical shard {shard} was dispatched more than once in one tick")]
    DuplicateShardDispatch {
        /// The duplicate logical shard.
        shard: ShardId,
    },

    /// An internally generated scheduler plan did not match owned shard state.
    #[error("internal scheduler plan did not match logical shard {shard}")]
    InvalidSchedulerPlan {
        /// The first shard whose plan entry was missing, extra, or misplaced.
        shard: ShardId,
    },

    /// A persistent shard worker thread could not be created.
    #[error("failed to spawn simulation shard worker slot {slot}: {kind}")]
    ShardWorkerSpawnFailed {
        /// Zero-based worker slot that could not be created.
        slot: usize,
        /// Operating-system error classification returned by thread creation.
        kind: ErrorKind,
    },

    /// A worker-pool request exceeded the defensive OS-thread ceiling.
    #[error("requested {requested} simulation shard workers, maximum is {maximum}")]
    TooManyShardWorkers {
        /// Requested persistent worker count.
        requested: usize,
        /// Fixed maximum persistent worker count.
        maximum: usize,
    },

    /// A persistent shard worker panicked after a tick was dispatched.
    #[error("simulation shard worker slot {slot} panicked during tick {tick}")]
    ShardWorkerPanicked {
        /// Zero-based worker slot whose thread panicked.
        slot: usize,
        /// Tick that may have been partially applied and makes retry unsafe.
        tick: Tick,
    },

    /// A persistent worker exited without returning its dispatched shard batch.
    #[error("simulation shard worker slot {slot} stopped during tick {tick}")]
    ShardWorkerStopped {
        /// Zero-based worker slot that stopped.
        slot: usize,
        /// Tick whose owned shard batch was not returned.
        tick: Tick,
    },

    /// A worker returned a result stamped with the wrong global tick.
    #[error("simulation shard worker slot {slot} returned tick {actual}, expected {expected}")]
    ShardWorkerWrongTick {
        /// Zero-based worker slot that returned the invalid result.
        slot: usize,
        /// Global tick dispatched by the scheduler.
        expected: Tick,
        /// Tick returned by the worker.
        actual: Tick,
    },

    /// Shadow mode was constructed without its required worker pool.
    #[error("shadow shard scheduler has no worker pool")]
    ShardWorkerPoolUnavailable,

    /// A prior worker failure made the scheduler unsafe to retry.
    #[error("simulation shard scheduler is fail-stop after partial tick {tick}")]
    ShardSchedulerPoisoned {
        /// Potentially partial tick that permanently poisoned the scheduler.
        tick: Tick,
    },

    /// A scheduler input targeted a shard whose lifecycle rejects new work.
    #[error("logical shard {shard} does not accept input while {state}")]
    ShardInputNotAccepted {
        /// Logical shard that rejected the input.
        shard: ShardId,
        /// Lifecycle state that disallows admission.
        state: ShardLifecycleState,
    },

    /// A draining shard still owns admitted work and cannot stop yet.
    #[error("logical shard {shard} cannot stop before its admitted tick work drains")]
    ShardDrainIncomplete {
        /// Draining logical shard that still has tick work.
        shard: ShardId,
    },

    /// Two registered shard containers held the same storage chunk key.
    ///
    /// This is rejected before tick advancement so two owners can never publish
    /// competing persist records for one world/dimension/chunk identity.
    #[error("persist chunk {key:?} is resident in both logical shards {first} and {second}")]
    PersistChunkAliased {
        /// Full world/dimension/chunk key claimed by both containers.
        key: ChunkKey,
        /// Canonically first registered container holding the key.
        first: ShardId,
        /// Later conflicting registered container.
        second: ShardId,
    },

    /// One shard's bounded per-tick cross-shard outbox rejected a new intent.
    #[error("logical shard {shard} cross-shard outbox is full (capacity {capacity})")]
    CrossShardOutboxFull {
        /// Source shard whose local outbox reached its fixed ceiling.
        shard: ShardId,
        /// Fixed per-shard outbox capacity.
        capacity: usize,
    },

    /// A prepared cross-shard boundary changed before its successful tick could
    /// commit it.
    #[error("cross-shard boundary for tick {tick} changed before commit")]
    CrossShardBoundaryCommitFailed {
        /// Boundary whose unchanged snapshot could not be removed.
        tick: Tick,
    },

    /// A shard emitted a cross-shard intent on the final representable tick.
    #[error("logical shard {shard} emitted cross-shard work at terminal tick {tick}")]
    CrossShardEnvelopeTickOverflow {
        /// Source shard whose owned intent could not receive a next boundary.
        shard: ShardId,
        /// Completed terminal tick.
        tick: Tick,
    },
}

impl From<SimError> for ServerError {
    fn from(error: SimError) -> Self {
        match error {
            SimError::InboxFull { capacity } => {
                ServerError::capacity(format!("simulation shard inbox full (capacity {capacity})"))
            }
            SimError::TickOverflow => {
                ServerError::internal("simulation tick counter overflowed u64")
            }
            SimError::EntityIdExhausted => {
                ServerError::internal("simulation shard exhausted its entity id range")
            }
            // Preserve the storage classification rather than flatten it.
            SimError::ChunkLoad { source, .. } => source,
            SimError::ShardRegionOutOfRange { position } => ServerError::invalid_state(format!(
                "shard position ({},{}) is outside the complete-region range",
                position.x(),
                position.z(),
            )),
            SimError::OverlappingShardOwnership { region, shard } => ServerError::invalid_state(
                format!("shard region {region} is already claimed by logical shard {shard}"),
            ),
            SimError::InvalidShardLifecycleTransition { shard, from, to } => {
                ServerError::invalid_state(format!(
                    "shard {shard} cannot transition from {from} to {to}",
                ))
            }
            SimError::UnknownScheduledShard { shard } => ServerError::invalid_state(format!(
                "logical shard {shard} is not registered with the scheduler"
            )),
            SimError::MultipleScheduledShardsDisabled { scheduled } => {
                ServerError::invalid_state(format!(
                    "multi-shard scheduling is disabled, but {scheduled} runnable shards were requested"
                ))
            }
            SimError::DuplicateShardDispatch { shard } => ServerError::internal(format!(
                "logical shard {shard} was dispatched more than once in one tick"
            )),
            SimError::InvalidSchedulerPlan { shard } => ServerError::internal(format!(
                "internal scheduler plan did not match logical shard {shard}"
            )),
            SimError::ShardWorkerSpawnFailed { slot, kind } => ServerError::internal(format!(
                "failed to spawn simulation shard worker slot {slot}: {kind}"
            )),
            SimError::TooManyShardWorkers { requested, maximum } => {
                ServerError::capacity(format!(
                    "requested {requested} simulation shard workers, maximum is {maximum}"
                ))
            }
            SimError::ShardWorkerPanicked { slot, tick } => ServerError::internal(format!(
                "simulation shard worker slot {slot} panicked during tick {tick}"
            )),
            SimError::ShardWorkerStopped { slot, tick } => ServerError::internal(format!(
                "simulation shard worker slot {slot} stopped during tick {tick}"
            )),
            SimError::ShardWorkerWrongTick {
                slot,
                expected,
                actual,
            } => ServerError::internal(format!(
                "simulation shard worker slot {slot} returned tick {actual}, expected {expected}"
            )),
            SimError::ShardWorkerPoolUnavailable => {
                ServerError::internal("shadow shard scheduler has no worker pool")
            }
            SimError::ShardSchedulerPoisoned { tick } => ServerError::internal(format!(
                "simulation shard scheduler is fail-stop after partial tick {tick}"
            )),
            SimError::ShardInputNotAccepted { shard, state } => ServerError::invalid_state(
                format!("logical shard {shard} does not accept input while {state}"),
            ),
            SimError::ShardDrainIncomplete { shard } => ServerError::invalid_state(format!(
                "logical shard {shard} cannot stop before its admitted tick work drains"
            )),
            SimError::PersistChunkAliased { key, first, second } => {
                ServerError::invalid_state(format!(
                    "persist chunk {key:?} is resident in both logical shards {first} and {second}"
                ))
            }
            SimError::CrossShardOutboxFull { shard, capacity } => ServerError::capacity(format!(
                "logical shard {shard} cross-shard outbox full (capacity {capacity})"
            )),
            SimError::CrossShardBoundaryCommitFailed { tick } => ServerError::internal(format!(
                "cross-shard boundary for tick {tick} changed before commit"
            )),
            SimError::CrossShardEnvelopeTickOverflow { shard, tick } => {
                ServerError::internal(format!(
                    "logical shard {shard} emitted cross-shard work at terminal tick {tick}"
                ))
            }
        }
    }
}

/// Convenience result alias whose error type is [`SimError`].
///
/// Prefer this over spelling out `Result<T, SimError>` by hand.
pub type SimResult<T> = core::result::Result<T, SimError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_strings_are_classified() {
        assert_eq!(
            SimError::InboxFull { capacity: 8 }.to_string(),
            "simulation shard inbox is full (capacity 8)"
        );
        assert_eq!(
            SimError::TickOverflow.to_string(),
            "simulation tick counter overflowed u64"
        );
    }

    #[test]
    fn inbox_full_maps_to_capacity_server_error() {
        let server: ServerError = SimError::InboxFull { capacity: 4 }.into();
        assert!(matches!(server, ServerError::Capacity(_)));
    }

    #[test]
    fn tick_overflow_maps_to_internal_server_error() {
        let server: ServerError = SimError::TickOverflow.into();
        assert!(matches!(server, ServerError::Internal { .. }));
    }

    #[test]
    fn entity_id_exhausted_is_classified_and_maps_to_internal() {
        assert_eq!(
            SimError::EntityIdExhausted.to_string(),
            "simulation shard exhausted its entity id range"
        );
        let server: ServerError = SimError::EntityIdExhausted.into();
        assert!(matches!(server, ServerError::Internal { .. }));
    }

    #[test]
    fn chunk_load_displays_position_and_preserves_source() {
        let err = SimError::ChunkLoad {
            pos: ChunkPos::new(-3, 4),
            source: ServerError::not_found("chunk record"),
        };
        assert_eq!(err.to_string(), "failed to load chunk at (-3,4)");
        // Converting back to a ServerError keeps the original classification.
        let server: ServerError = err.into();
        assert!(matches!(server, ServerError::NotFound(_)));
    }

    #[test]
    fn variants_compare_by_value() {
        assert_eq!(
            SimError::InboxFull { capacity: 1 },
            SimError::InboxFull { capacity: 1 }
        );
        assert_ne!(
            SimError::InboxFull { capacity: 1 },
            SimError::InboxFull { capacity: 2 }
        );
    }
}
