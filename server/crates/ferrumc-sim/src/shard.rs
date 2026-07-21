//! A single simulation shard: bounded inbox in, outputs out, at tick
//! boundaries.

use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::num::NonZeroUsize;

use ferrumc_core::{DimensionId, EntityId, GameMode, PlayerId, WorldId};
use ferrumc_items::{left_click_exchange, ItemStack};
use ferrumc_math::{BlockPos, Cuboid, Direction, ShardPos, Vec3};
use ferrumc_placement::{
    compute_fence_connection_state, compute_placement, is_water_source, NeighborQuery,
    PlacementContext, PlacementResult, PlacementRule,
};
use ferrumc_registry::block_state::{block_metadata, collision_top_y, state_id_to_block_name};
use ferrumc_registry::dimension;
use ferrumc_world::{
    is_chest_state, sign_kind_for_state, BlockEntity, BlockStateId, ChestInventory, Sign,
    SIGN_LINES,
};

use crate::cross_shard::{CrossShardIntent, CrossShardPayload};
use crate::error::SimError;
use crate::loaded::LoadedChunkMap;
use crate::message::{GameInput, GameOutput, SpawnedEntityKind};
use crate::mutation::{MutationCause, MutationResult, PendingMutation, RejectionReason};
use crate::physics::{is_gravity_affected, is_replaceable, AIR_DRAG_Y, GRAVITY_ITEM};
use crate::region::{RegionLimits, RegionOp};
use crate::scheduler::{CrossShardOutboxRestore, ScheduledTickInputs};

/// Maximum absolute value allowed for any player position coordinate.
///
/// Mirrors the vanilla server's move-packet sanity bound: a client may not place
/// itself beyond +/-3.0e7 blocks on any axis (just past the maximum world
/// border). Anything larger — or non-finite — is a malformed or hostile position
/// and is rejected at the tick boundary rather than corrupting shard state.
const MAX_POSITION_MAGNITUDE: f64 = 3.0e7;

/// Returns `true` if `position` is a finite, in-range player position.
///
/// Rejects NaN, infinities, and any coordinate whose magnitude exceeds
/// [`MAX_POSITION_MAGNITUDE`]. This is the simulation's only movement check this
/// milestone: no collision, no speed limit, just finite/range sanity so a bad
/// packet can never poison player state.
fn is_valid_position(position: Vec3) -> bool {
    let in_range = |value: f64| value.is_finite() && value.abs() <= MAX_POSITION_MAGNITUDE;
    in_range(position.x) && in_range(position.y) && in_range(position.z)
}

/// Maximum distance, in blocks, between a player and a block they may break or
/// place.
///
/// Measured from the player's position to the centre of the target block. Set a
/// little above vanilla's ~4.5-block interaction range so creative-mode reach is
/// comfortably covered. This is the milestone's only interaction-range check:
/// there is no eye-height offset, line-of-sight, or per-gamemode tuning yet.
const MAX_REACH: f64 = 6.0;

/// A fixed `minecraft:stone` block-state (id `1` in the pinned flat-world
/// registry), used only by the shard's block-edit tests now that an accepted
/// place writes the held item's resolved state threaded through
/// [`GameInput::BlockPlace`] rather than a hardcoded default.
#[cfg(test)]
const DEFAULT_PLACED_STATE: BlockStateId = BlockStateId::new(1);

/// Returns `true` if the block at `block` is within [`MAX_REACH`] of an actor
/// positioned at `actor`.
///
/// Distance is measured from `actor` to the centre of the target block and
/// compared squared to avoid a square root. A non-finite actor position (which
/// movement validation already rejects before it can be stored) can never make
/// this return `true`, so it fails closed.
fn within_reach(actor: Vec3, block: BlockPos) -> bool {
    let centre = Vec3::new(
        f64::from(block.x()) + 0.5,
        f64::from(block.y()) + 0.5,
        f64::from(block.z()) + 0.5,
    );
    (actor - centre).length_squared() <= MAX_REACH * MAX_REACH
}

/// Builds a [`NonZeroUsize`] in const context, falling back to `1` for a zero
/// input.
const fn non_zero_usize(value: usize) -> NonZeroUsize {
    match NonZeroUsize::new(value) {
        Some(v) => v,
        None => NonZeroUsize::MIN,
    }
}

/// Default inbox capacity used by [`SimShard::new`].
///
/// 1024 queued inputs per shard is far above the per-tick volume a well-behaved
/// session router produces (a handful of inputs per player per tick), so
/// reaching it signals upstream misbehaviour or a stall — exactly when reject
/// backpressure should kick in.
const DEFAULT_INBOX_CAPACITY: NonZeroUsize = non_zero_usize(1024);

/// Maximum number of cross-shard intents one shard may emit in one tick.
///
/// This matches the default input bound: the current seam permits at most one
/// intent per admitted input, and the scheduler drains the outbox after every
/// completed tick. A future system that can fan one input out further must
/// handle reject-newest ownership from [`SimShard::emit_cross_shard`] instead of
/// growing this buffer.
const CROSS_SHARD_OUTBOX_CAPACITY: usize = 1024;

/// Default world a shard owns chunks for when none is specified.
///
/// The current milestone runs a single overworld shard, so [`SimShard::new`]
/// and friends default to world `0`. Use [`SimShard::in_dimension`] to place a
/// shard in an explicit world/dimension.
const DEFAULT_WORLD: WorldId = WorldId::new(0);

/// Default dimension a shard owns chunks for when none is specified (the
/// overworld, index `0`).
const DEFAULT_DIMENSION: DimensionId = DimensionId::new(0);

/// Upper bound on buffered [`PendingMutation`]s between drains.
///
/// The driver drains the buffer with [`SimShard::take_mutations`] every tick, and
/// at most one mutation is produced per queued block-edit input, so the buffer
/// never exceeds one inbox's worth in normal operation. This cap is a defensive
/// ceiling for the pathological case where the driver stalls without draining:
/// past it, new journal entries are dropped (the journal is best-effort and the
/// authoritative overlay still persists the block) rather than growing unbounded.
const MUTATION_LOG_CAP: usize = 4096;

/// The prior block-states a single region edit overwrote, captured so the edit
/// can be undone.
///
/// Each pair is `(position, state_before_the_edit)`. Restoring re-applies every
/// state through the same block-edit funnel. Bounded indirectly: the edit that
/// produced it could touch at most [`RegionLimits::max_volume`] cells, and a
/// player keeps at most [`RegionLimits::max_undo_entries`] of these.
#[derive(Debug, Clone)]
struct RegionUndoEntry {
    blocks: Vec<(BlockPos, BlockStateId)>,
}

/// Maximum number of region edits/undos a shard buffers in flight at once.
///
/// Each item is drained incrementally under [`RegionLimits::max_blocks_per_tick`],
/// so the queue length is the number of *distinct* region commands still in
/// progress. Bounding it stops a flood of region commands from growing shard
/// memory without limit; past the cap a new region command is dropped
/// (best-effort backpressure, the same posture the inbox uses).
const MAX_PENDING_REGION_WORK: usize = 256;

/// A region operation in progress, applied incrementally across ticks under the
/// per-tick block budget.
#[derive(Debug, Clone)]
struct PendingRegionWork {
    /// The player the work is attributed to (the undo-history key).
    player: PlayerId,
    /// What the work does, plus the cursor it resumes from.
    kind: RegionWorkKind,
}

/// The two kinds of in-flight region work and the progress cursor each resumes
/// from across ticks.
#[derive(Debug, Clone)]
enum RegionWorkKind {
    /// A `/fill` or `/replace`: walk `region` from `cursor`, applying `op`, and
    /// accumulate the prior states into `prior`. A completed edit pushes `prior`
    /// onto the acting player's undo history.
    Edit {
        /// The cuboid being edited.
        region: Cuboid,
        /// How each cell changes.
        op: RegionOp,
        /// The next linear index into `region` to process.
        cursor: u64,
        /// Prior states captured so far, for the undo entry recorded on completion.
        prior: Vec<(BlockPos, BlockStateId)>,
    },
    /// A `/undo`: re-apply `restore[cursor..]` (the prior states captured by the
    /// edit being undone) verbatim through the funnel.
    Undo {
        /// The `(position, prior_state)` pairs to restore.
        restore: Vec<(BlockPos, BlockStateId)>,
        /// The next index into `restore` to apply.
        cursor: usize,
    },
}

/// World `y` below which a falling entity is removed from the simulation.
///
/// An entity that falls past the bottom of the world — off the edge of the
/// loaded region, over the void — would otherwise fall forever: its vertical
/// velocity is capped at terminal, but its position decreases without bound,
/// leaking the entity and emitting a move every tick. Vanilla despawns entities
/// roughly 64 blocks below the build floor; this mirrors that, deterministically
/// bounding the lifetime of a voided entity.
const VOID_DESPAWN_Y: f64 = dimension::MIN_Y as f64 - 64.0;

/// Client sequence stamped on a server-originated block change that no client
/// requested (a falling block restoring itself on landing). Player edits carry a
/// real ack sequence; a physics restore has none, so it uses zero.
const NO_CLIENT_SEQUENCE: i32 = 0;

/// The first entity id a shard hands out from its per-shard counter.
///
/// Starts at `1` so `0` stays free as a reserved "no entity" sentinel for later
/// code that needs a null-like id distinct from any live entity. The protocol
/// permits `0`; reserving it here is a defensive convention, not a requirement.
/// Ids only increase from here (see [`SimShard::spawn_entity`]), so an id is
/// never reused within a shard's lifetime.
const FIRST_ENTITY_ID: i32 = 1;

/// What a non-player entity *is*, beyond its shared physical state.
///
/// The store is otherwise type-agnostic — every entity has a position, velocity,
/// gravity, and ground flag regardless of kind. `EntityKind` adds only the
/// behaviour that differs: what happens when the entity lands. New kinds
/// (projectiles, dropped items) slot in here without touching the physics step.
#[derive(Debug, Clone, Copy, PartialEq)]
enum EntityKind {
    /// A plain entity with no on-land behaviour. It falls, lands, and rests.
    Simple,
    /// A falling block (sand, gravel, …). On landing it turns back into `block`
    /// at its resting cell and despawns — the vanilla falling-block lifecycle.
    FallingBlock {
        /// The block-state to restore when the entity lands.
        block: BlockStateId,
    },
}

/// Non-player entity state owned exclusively by the shard.
///
/// The minimal physical footprint every simulated entity carries: where it is,
/// how fast it is moving, and whether it is resting on a block. Later milestones
/// (gravity, drag, velocity integration, collision) read and write
/// `velocity`/`position` and set `on_ground`; this milestone only stores the
/// fields so those systems have a deterministic place to act. Player state is
/// tracked separately in [`PlayerState`] — this store is for non-player entities
/// (dropped items, projectiles, falling blocks, mobs).
///
/// `PartialEq` compares the `f64` fields of `Vec3` with `==` (bit-exact).
/// Approximate equality would be wrong here: the crate's determinism invariant
/// requires two shards driven by identical inputs to reach *bit-identical*
/// state, and this impl is what tests assert that on.
#[derive(Debug, Clone, Copy, PartialEq)]
struct EntityState {
    /// World-space position in blocks.
    position: Vec3,
    /// Velocity in blocks per tick. Gravity accelerates it each tick while the
    /// entity is airborne; air drag decays it toward terminal velocity.
    velocity: Vec3,
    /// Downward acceleration in blocks per tick² applied while airborne, seeded
    /// at spawn from a [`crate::physics`] category constant (`-0.08` living,
    /// `-0.04` item, …). Stored per entity because vanilla gravity differs by
    /// category.
    gravity: f64,
    /// Whether the entity is resting on a solid block. `false` on spawn
    /// (airborne until a later collision pass proves otherwise). While `true`,
    /// gravity is not applied.
    on_ground: bool,
    /// What the entity is, which decides its on-land behaviour (see
    /// [`EntityKind`]). Physics treats every kind identically.
    kind: EntityKind,
}

/// Per-player state owned exclusively by the shard.
#[derive(Debug, Clone, Copy)]
struct PlayerState {
    position: Vec3,
    /// Body yaw in degrees, seeded to `0.0` on join and updated by a
    /// [`GameInput::PlayerMove`] carrying rotation. Broadcast to viewers so a
    /// remote player faces the right way instead of always facing north.
    yaw: f32,
    /// Pitch in degrees, seeded to `0.0` on join and updated by a
    /// [`GameInput::PlayerMove`] carrying rotation.
    pitch: f32,
    /// The authoritative server-side game mode. Seeded to [`GameMode::default`] on
    /// join and mutated by [`GameInput::SetGameMode`]; later milestones read it to
    /// enforce mode-specific rules (creative no-decrement, break speed, flight).
    game_mode: GameMode,
}

/// A movement coalesced within one tick: the latest valid position and/or
/// rotation a player's [`GameInput::PlayerMove`]s carried this tick.
///
/// Each field merges independently — a later input's `Some` component overwrites
/// an earlier one, while a `None` leaves the earlier value — so a position-only
/// move followed by a rotation-only move in the same tick applies both. A
/// `position` of `None` means no (valid) position arrived this tick, so the apply
/// pass emits a rotation-only [`GameOutput::PlayerMoved`].
#[derive(Debug, Clone, Copy, Default)]
struct PendingMove {
    position: Option<Vec3>,
    yaw: Option<f32>,
    pitch: Option<f32>,
}

/// One simulation shard.
///
/// A shard exclusively owns its players, a bounded inbox, and the chunks
/// resident for its world/dimension (a [`LoadedChunkMap`]). It applies queued
/// [`GameInput`]s **only** at tick boundaries and returns the resulting
/// [`GameOutput`]s. Entity ownership arrives in later milestones.
///
/// # Chunk ownership
///
/// The shard owns chunk *data* through [`loaded_chunks`](SimShard::loaded_chunks)
/// / [`loaded_chunks_mut`](SimShard::loaded_chunks_mut) but never a database
/// handle: chunk loading is driven by passing a borrowed
/// [`WorldStore`](ferrumc_storage::WorldStore) to the map's
/// [`acquire`](LoadedChunkMap::acquire). Which chunks are resident is governed
/// entirely by tickets.
///
/// # Tick-boundary application
///
/// [`enqueue`](SimShard::enqueue) only appends to the inbox; it never mutates
/// shard state. State changes happen exclusively inside
/// [`run_tick`](SimShard::run_tick), which drains the whole inbox in FIFO order.
/// An input enqueued after a `run_tick` returns is therefore applied at the
/// *next* tick, never mid-tick.
///
/// # Movement coalescing and validation
///
/// Multiple [`GameInput::PlayerMove`]s for the same player in one tick are
/// *coalesced*: only the latest valid position is applied at the boundary, and a
/// single [`GameOutput::PlayerMoved`] is emitted (overload handling step one —
/// coalesce movement). Coordinates are sanity-checked with [`is_valid_position`]:
/// a non-finite or out-of-range move is rejected without touching state, and if
/// no valid move supersedes it the shard emits a
/// [`GameOutput::PlayerPositionCorrected`] so the desynced client can snap back.
/// A move for an absent player is ignored, and a [`GameInput::PlayerLeave`]
/// cancels any pending move/correction for that player.
///
/// # Block edits
///
/// [`GameInput::BlockBreak`] and [`GameInput::BlockPlace`] mutate the resident
/// chunk at the tick boundary. Each is validated first (see
/// [`apply_block_edit`](SimShard::apply_block_edit)): the actor must be present,
/// the target chunk must be resident in this shard (which also pins the edit to
/// the shard's dimension), and the target must be within [`MAX_REACH`]. Reach is
/// measured against the actor's position *as of the start of the tick* — a move
/// queued in the same tick is coalesced and applied afterwards, so it does not
/// extend reach for an edit earlier in the same inbox. An accepted break writes
/// [`BlockStateId::AIR`]; an accepted place writes the held item's resolved
/// block-state, carried on [`GameInput::BlockPlace`].
/// Either way the owning section is marked dirty (by
/// [`Chunk::set_block`](ferrumc_world::Chunk::set_block)) and a single
/// [`GameOutput::BlockChanged`] is emitted, in inbox order, for the session layer
/// to broadcast and acknowledge. A rejected edit mutates nothing; if there is a
/// client to heal (an in-reach edit refused in a resident chunk) it emits a
/// [`GameOutput::BlockChangeRejected`] so the actor can resync, otherwise (absent
/// actor, unloaded chunk) it emits nothing.
///
/// # Backpressure
///
/// The inbox is bounded to a fixed capacity. When it is full,
/// [`enqueue`](SimShard::enqueue) returns [`SimError::InboxFull`] and leaves the
/// inbox untouched: it neither blocks (the shard runs on a sim worker that must
/// never stall) nor silently drops (that would desync clients). Deciding what to
/// do on rejection is the caller's responsibility.
///
/// # Determinism
///
/// Given the same starting state and the same sequence of enqueued inputs, a
/// shard produces an identical sequence of outputs. The inbox is strictly FIFO,
/// and both player state and the non-player entity store live in ordered
/// [`BTreeMap`]s (the entity store keyed by [`EntityId`], handed out from a
/// monotonic per-shard counter), so no iteration order or hashing randomness can
/// leak into results.
#[derive(Debug, Clone)]
pub struct SimShard {
    shard_pos: ShardPos,
    inbox: VecDeque<GameInput>,
    inbox_capacity: usize,
    players: BTreeMap<PlayerId, PlayerState>,
    /// Non-player entities the shard owns, keyed by [`EntityId`] in an ordered
    /// map so iteration order is deterministic (see the type-level determinism
    /// note).
    ///
    /// The CLAUDE.md simulation model describes the target store as
    /// `SlotMap + ComponentVecs` (O(1) lookup, cache-friendly per-tick
    /// iteration for physics). This milestone deliberately uses a `BTreeMap`
    /// for two reasons: determinism falls out for free from the ordered key,
    /// and a `SlotMap` would reuse its keys — which would collide with the
    /// invariant that an [`EntityId`] is never reused within a shard's
    /// lifetime unless we separate the protocol id from the storage key. The
    /// migration point is when the per-tick physics step lands: split
    /// [`EntityId`] (protocol, immutable) from the storage key (reusable),
    /// then swap to `SlotMap + ComponentVecs` for the iteration win.
    entities: BTreeMap<EntityId, EntityState>,
    /// Monotonic source of entity ids for this shard: the next id to hand out,
    /// or `None` once the `i32` range is exhausted. Only ever advances, so an id
    /// is never reused within the shard's lifetime.
    next_entity_id: Option<i32>,
    chunks: LoadedChunkMap,
    /// Accepted gameplay mutations buffered for the storage journal, drained each
    /// tick by the driver. Bounded by [`MUTATION_LOG_CAP`].
    mutation_log: Vec<PendingMutation>,
    /// Per-player history of region edits (newest at the back), each recording the
    /// prior states it overwrote so `/undo` can restore them. Bounded per player by
    /// [`RegionLimits::max_undo_entries`]; an entry is dropped when its player
    /// leaves the shard.
    undo_history: BTreeMap<PlayerId, VecDeque<RegionUndoEntry>>,
    /// Caps bounding region edits and the per-player undo history. Set from
    /// configuration by the app via [`set_region_limits`](SimShard::set_region_limits).
    region_limits: RegionLimits,
    /// Region edits/undos awaiting (or mid-) application, drained a budgeted number
    /// of cells per tick (see [`RegionLimits::max_blocks_per_tick`]) so a large fill
    /// spreads across ticks instead of stalling one. Bounded by
    /// [`MAX_PENDING_REGION_WORK`].
    pending_region_work: VecDeque<PendingRegionWork>,
    /// Typed intents produced by this shard during its current tick. The
    /// scheduler drains them only after every worker returns, then stamps source
    /// identity and the completed tick before bounded canonical admission.
    cross_shard_outbox: Vec<CrossShardIntent>,
}

impl SimShard {
    /// Creates an empty shard for `shard_pos` with the default inbox capacity in
    /// the default single overworld (world `0`, dimension `0`). Use
    /// [`in_dimension`](SimShard::in_dimension) to place the shard elsewhere.
    pub fn new(shard_pos: ShardPos) -> Self {
        Self::build(
            shard_pos,
            DEFAULT_WORLD,
            DEFAULT_DIMENSION,
            DEFAULT_INBOX_CAPACITY,
        )
    }

    /// Creates an empty shard for `shard_pos` with an explicit inbox `capacity`
    /// in the default world/dimension.
    ///
    /// `capacity` is a [`NonZeroUsize`] so a zero-capacity (permanently full)
    /// inbox is unrepresentable. The inbox pre-allocates this capacity once and
    /// never grows beyond it.
    pub fn with_inbox_capacity(shard_pos: ShardPos, capacity: NonZeroUsize) -> Self {
        Self::build(shard_pos, DEFAULT_WORLD, DEFAULT_DIMENSION, capacity)
    }

    /// Creates an empty shard for `shard_pos` owning chunks in an explicit
    /// `world` and `dimension`, with the default inbox capacity.
    pub fn in_dimension(shard_pos: ShardPos, world: WorldId, dimension: DimensionId) -> Self {
        Self::build(shard_pos, world, dimension, DEFAULT_INBOX_CAPACITY)
    }

    /// Shared constructor: builds a shard with every field initialized.
    fn build(
        shard_pos: ShardPos,
        world: WorldId,
        dimension: DimensionId,
        capacity: NonZeroUsize,
    ) -> Self {
        Self {
            shard_pos,
            inbox: VecDeque::with_capacity(capacity.get()),
            inbox_capacity: capacity.get(),
            players: BTreeMap::new(),
            entities: BTreeMap::new(),
            next_entity_id: Some(FIRST_ENTITY_ID),
            chunks: LoadedChunkMap::new(world, dimension),
            mutation_log: Vec::new(),
            undo_history: BTreeMap::new(),
            region_limits: RegionLimits::default(),
            pending_region_work: VecDeque::new(),
            cross_shard_outbox: Vec::new(),
        }
    }

    /// Replaces the region-edit caps with operator-configured `limits`.
    ///
    /// Called once at startup by the app from configuration; tests rely on the
    /// [`RegionLimits::default`] a freshly built shard carries.
    pub fn set_region_limits(&mut self, limits: RegionLimits) {
        self.region_limits = limits;
    }

    /// Returns the position of this shard in shard coordinates.
    pub const fn shard_pos(&self) -> ShardPos {
        self.shard_pos
    }

    /// Returns the chunks this shard currently owns in memory.
    pub const fn loaded_chunks(&self) -> &LoadedChunkMap {
        &self.chunks
    }

    /// Returns a mutable handle to the shard's chunks, used to acquire/release
    /// tickets and collect dirty chunks for saving.
    pub fn loaded_chunks_mut(&mut self) -> &mut LoadedChunkMap {
        &mut self.chunks
    }

    /// Returns the fixed inbox capacity.
    pub const fn inbox_capacity(&self) -> usize {
        self.inbox_capacity
    }

    /// Returns the number of inputs currently queued in the inbox.
    pub fn inbox_len(&self) -> usize {
        self.inbox.len()
    }

    /// Returns whether no admitted input or incremental region operation still
    /// needs a future tick.
    ///
    /// Used by the crate-internal scheduler before completing a draining
    /// lifecycle. Buffered mutation records and dirty chunks are completed
    /// outputs for later persistence phases, not tick work, so they do not make
    /// the shard non-quiescent.
    #[allow(
        dead_code,
        reason = "the owner-gated shadow scheduler intentionally has no app wiring yet"
    )]
    pub(crate) fn is_tick_quiescent(&self) -> bool {
        self.inbox.is_empty()
            && self.pending_region_work.is_empty()
            && self.cross_shard_outbox.is_empty()
    }

    /// Returns `true` if the inbox is at capacity and will reject new inputs.
    pub fn is_inbox_full(&self) -> bool {
        self.inbox.len() >= self.inbox_capacity
    }

    /// Returns the number of players currently present in the shard.
    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    /// Returns `true` if any accepted gameplay mutations are buffered for the
    /// storage journal.
    pub fn has_pending_mutations(&self) -> bool {
        !self.mutation_log.is_empty()
    }

    /// Iterates canonical complete pending-mutation records for determinism tests.
    ///
    /// Each item pairs its typed position (for coordinate-derived logical-owner
    /// grouping) with a record carrying world and dimension scope, the complete
    /// mutation cause (including the player UUID when present), block position,
    /// and exact old/new block-state ids. Iteration preserves the journal
    /// buffer's deterministic insertion order.
    #[cfg(test)]
    pub(crate) fn canonical_pending_mutation_records(
        &self,
    ) -> impl Iterator<Item = (BlockPos, Vec<u8>)> + '_ {
        let world = self.chunks.world();
        let dimension = self.chunks.dimension();
        self.mutation_log.iter().map(move |mutation| {
            let mut record = Vec::with_capacity(45);
            record.extend_from_slice(&world.get().to_be_bytes());
            record.extend_from_slice(&dimension.get().to_be_bytes());
            match mutation.cause() {
                MutationCause::PlayerCreative { player } => {
                    record.push(0);
                    record.extend_from_slice(player.as_uuid().as_bytes());
                }
                MutationCause::Command => record.push(1),
                MutationCause::Plugin => record.push(2),
                MutationCause::Test => record.push(3),
            }
            let position = mutation.position();
            record.extend_from_slice(&position.x().to_be_bytes());
            record.extend_from_slice(&position.y().to_be_bytes());
            record.extend_from_slice(&position.z().to_be_bytes());
            record.extend_from_slice(&mutation.old_state().as_u32().to_be_bytes());
            record.extend_from_slice(&mutation.new_state().as_u32().to_be_bytes());
            (position, record)
        })
    }

    /// Returns whether any player owns at least one undo-history entry.
    #[cfg(test)]
    pub(crate) fn has_undo_history(&self) -> bool {
        self.undo_history
            .values()
            .any(|history| !history.is_empty())
    }

    /// Drains and returns the buffered gameplay mutations for the storage
    /// journal, leaving the buffer empty.
    ///
    /// Called by the driver each tick; it stamps each entry with the current tick
    /// and a monotonic id when building the journal records, so the deterministic
    /// shard never reads a clock or allocates an id itself.
    #[must_use]
    pub fn take_mutations(&mut self) -> Vec<PendingMutation> {
        std::mem::take(&mut self.mutation_log)
    }

    /// Returns `true` if `player` is currently present in the shard.
    pub fn contains_player(&self, player: PlayerId) -> bool {
        self.players.contains_key(&player)
    }

    /// Returns the current position of `player`, or `None` if absent.
    pub fn player_position(&self, player: PlayerId) -> Option<Vec3> {
        self.players.get(&player).map(|state| state.position)
    }

    /// Returns the authoritative game mode of `player`, or `None` if absent.
    pub fn player_game_mode(&self, player: PlayerId) -> Option<GameMode> {
        self.players.get(&player).map(|state| state.game_mode)
    }

    /// Iterates canonical complete-player records for determinism tests.
    ///
    /// This remains test-only so the shadow harness can digest every currently
    /// modelled player field without expanding the production simulation API.
    /// World and dimension are semantic identity; physical shard position is
    /// deliberately absent because the compared topologies partition owners
    /// differently.
    #[cfg(test)]
    pub(crate) fn canonical_player_state_records(&self) -> impl Iterator<Item = Vec<u8>> + '_ {
        let world = self.chunks.world();
        let dimension = self.chunks.dimension();
        self.players.iter().map(move |(&player, state)| {
            let mut record = Vec::with_capacity(57);
            record.extend_from_slice(&world.get().to_be_bytes());
            record.extend_from_slice(&dimension.get().to_be_bytes());
            record.extend_from_slice(player.as_uuid().as_bytes());
            record.extend_from_slice(&state.position.x.to_bits().to_be_bytes());
            record.extend_from_slice(&state.position.y.to_bits().to_be_bytes());
            record.extend_from_slice(&state.position.z.to_bits().to_be_bytes());
            record.extend_from_slice(&state.yaw.to_bits().to_be_bytes());
            record.extend_from_slice(&state.pitch.to_bits().to_be_bytes());
            record.push(state.game_mode.as_id());
            record
        })
    }

    /// Spawns a non-player entity at `position` with an initial `velocity` and
    /// per-tick `gravity`, returning its freshly allocated [`EntityId`].
    ///
    /// `gravity` is the downward acceleration in blocks per tick² (negative),
    /// chosen from a [`crate::physics`] category constant — `GRAVITY_ITEM` for a
    /// dropped item or falling block, `GRAVITY_LIVING` for a mob, and so on.
    ///
    /// Ids come from a per-shard counter that only ever increases, so the same
    /// spawn sequence yields the same ids on any run and an id is never reused
    /// within the shard's lifetime — the store stays deterministic. The new
    /// entity starts `on_ground = false` (airborne until a later collision pass
    /// proves otherwise). Spawning reads no clock and applies to the store
    /// immediately; the tick-boundary rule is upheld because the public spawn
    /// path is a [`GameInput::SpawnEntity`] applied inside
    /// [`run_tick`](SimShard::run_tick), and this method is otherwise reachable
    /// only from crate-internal callers on the tick path.
    ///
    /// # Errors
    ///
    /// Returns [`SimError::EntityIdExhausted`] once the shard has already handed
    /// out every id in the `i32` range. This is unreachable for a real shard,
    /// but is surfaced as a classified error rather than a panic so the counter
    /// can never wrap and reissue a live id.
    pub fn spawn_entity(
        &mut self,
        position: Vec3,
        velocity: Vec3,
        gravity: f64,
    ) -> Result<EntityId, SimError> {
        self.spawn_with_kind(position, velocity, gravity, EntityKind::Simple)
    }

    /// Spawns a falling block carrying `block`, which is restored at the entity's
    /// resting cell when it lands (then the entity despawns).
    ///
    /// The same allocation, determinism, and error rules as
    /// [`spawn_entity`](Self::spawn_entity) apply. Use [`GRAVITY_ITEM`] for the
    /// gravity: vanilla falling blocks share the item acceleration.
    ///
    /// [`GRAVITY_ITEM`]: crate::physics::GRAVITY_ITEM
    ///
    /// # Errors
    ///
    /// Returns [`SimError::EntityIdExhausted`] on id-range exhaustion, exactly as
    /// [`spawn_entity`](Self::spawn_entity).
    pub fn spawn_falling_block(
        &mut self,
        position: Vec3,
        velocity: Vec3,
        gravity: f64,
        block: BlockStateId,
    ) -> Result<EntityId, SimError> {
        self.spawn_with_kind(
            position,
            velocity,
            gravity,
            EntityKind::FallingBlock { block },
        )
    }

    /// Shared spawn path: reserves an id, inserts an entity of `kind`, advances
    /// the counter. All public spawn methods funnel through here.
    fn spawn_with_kind(
        &mut self,
        position: Vec3,
        velocity: Vec3,
        gravity: f64,
        kind: EntityKind,
    ) -> Result<EntityId, SimError> {
        // Take the reserved id; `None` means a prior spawn exhausted the range.
        // Fail before touching the store so a rejected spawn is a no-op.
        let raw = self.next_entity_id.ok_or(SimError::EntityIdExhausted)?;
        let id = EntityId::new(raw);
        self.entities.insert(
            id,
            EntityState {
                position,
                velocity,
                gravity,
                on_ground: false,
                kind,
            },
        );
        // Advance for the next spawn. checked_add yields `None` at i32::MAX,
        // marking the range exhausted so `raw` is the last id ever issued and is
        // never reused — no panic, no silent wrap (see TickOverflow).
        self.next_entity_id = raw.checked_add(1);
        Ok(id)
    }

    /// Removes the entity with `id`, returning `true` if it was present.
    ///
    /// The id is not returned to the counter: ids are never reused within a
    /// shard's lifetime, so a removed id can never collide with a future spawn.
    pub fn remove_entity(&mut self, id: EntityId) -> bool {
        self.entities.remove(&id).is_some()
    }

    /// Returns the number of non-player entities the shard currently owns.
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Returns `true` if an entity with `id` is present in the shard.
    pub fn contains_entity(&self, id: EntityId) -> bool {
        self.entities.contains_key(&id)
    }

    /// Returns the current position of the entity with `id`, or `None` if absent.
    pub fn entity_position(&self, id: EntityId) -> Option<Vec3> {
        self.entities.get(&id).map(|state| state.position)
    }

    /// Returns the current velocity of the entity with `id`, or `None` if absent.
    pub fn entity_velocity(&self, id: EntityId) -> Option<Vec3> {
        self.entities.get(&id).map(|state| state.velocity)
    }

    /// Returns whether the entity with `id` is resting on the ground, or `None`
    /// if absent.
    pub fn entity_on_ground(&self, id: EntityId) -> Option<bool> {
        self.entities.get(&id).map(|state| state.on_ground)
    }

    /// Returns an iterator over the ids of all non-player entities, in ascending
    /// id order.
    ///
    /// Ordering is deterministic because the store is a [`BTreeMap`] keyed by
    /// [`EntityId`], so iterating it to build spawn/despawn/movement outputs never
    /// leaks nondeterminism into results.
    pub fn entity_ids(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.entities.keys().copied()
    }

    /// Enqueues `input` for application at the next tick boundary.
    ///
    /// Returns [`SimError::InboxFull`] without modifying the inbox if it is
    /// already at capacity (reject backpressure — see the type docs).
    pub fn enqueue(&mut self, input: GameInput) -> Result<(), SimError> {
        if self.inbox.len() >= self.inbox_capacity {
            return Err(SimError::InboxFull {
                capacity: self.inbox_capacity,
            });
        }
        self.inbox.push_back(input);
        Ok(())
    }

    /// Adds one owned cross-shard intent to this tick's bounded outbox.
    ///
    /// Reject-newest backpressure returns the intact intent when the fixed
    /// outbox is full. The source shard never blocks and never mutates a
    /// destination directly; the scheduler alone stamps and routes accepted
    /// intents after this shard's tick completes.
    #[allow(
        dead_code,
        reason = "Packet 42 adds the internal carrier before a gameplay system emits transfers"
    )]
    fn emit_cross_shard(&mut self, intent: CrossShardIntent) -> Result<(), CrossShardIntent> {
        if self.cross_shard_outbox.len() >= CROSS_SHARD_OUTBOX_CAPACITY {
            return Err(intent);
        }
        self.cross_shard_outbox.push(intent);
        Ok(())
    }

    /// Drains this completed tick's cross-shard intents in source-local FIFO
    /// order.
    fn take_cross_shard_outbox(&mut self) -> Vec<CrossShardIntent> {
        std::mem::take(&mut self.cross_shard_outbox)
    }

    /// Atomically restores scheduler-drained intents after terminal-tick
    /// stamping fails.
    ///
    /// The restore carrier has no public constructor, so sibling modules cannot
    /// use this rollback seam to manufacture out-of-tick emissions.
    pub(crate) fn restore_cross_shard_outbox(
        &mut self,
        restore: CrossShardOutboxRestore,
    ) -> Result<(), CrossShardOutboxRestore> {
        if restore.len() > CROSS_SHARD_OUTBOX_CAPACITY.saturating_sub(self.cross_shard_outbox.len())
        {
            return Err(restore);
        }
        self.cross_shard_outbox.extend(restore.into_intents());
        Ok(())
    }

    /// Returns the fixed per-shard cross-shard outbox capacity.
    pub(crate) const fn cross_shard_outbox_capacity() -> usize {
        CROSS_SHARD_OUTBOX_CAPACITY
    }

    /// Applies every queued input at this tick boundary and returns the outputs.
    ///
    /// This public path and the capability-gated scheduler path share one
    /// private tick body, so queued inputs mutate player state only at those
    /// boundaries. The inbox is empty on return.
    ///
    /// Joins and leaves apply in FIFO order; movement is coalesced (latest valid
    /// position per player) and validated, then applied after the drain — see the
    /// type-level docs. Spawn/despawn outputs (both player and entity) are emitted
    /// in inbox order; move/correction outputs follow, ordered by [`PlayerId`] so
    /// the result is fully deterministic for a given inbox. Entity spawn ids come
    /// from a monotonic per-shard counter (see [`spawn_entity`](Self::spawn_entity)),
    /// so the same inbox on any run produces the same [`EntityId`] sequence.
    #[allow(clippy::too_many_lines)] // one tick drain: join/leave/move + every block-edit input arm
    pub fn run_tick(&mut self) -> Vec<GameOutput> {
        self.run_tick_with_boundary_inputs(Vec::new())
    }

    /// Runs one scheduler-authorized tick and returns its bounded source
    /// emissions with the ordinary outputs.
    ///
    /// [`ScheduledTickInputs`] is an unforgeable capability: only the scheduler
    /// module can construct it. Consequently, no sibling module can invoke this
    /// path to apply a cross-shard prefix between tick boundaries.
    pub(crate) fn run_scheduled_tick(
        &mut self,
        mut tick_inputs: ScheduledTickInputs,
    ) -> (Vec<GameOutput>, Vec<CrossShardIntent>) {
        let boundary_inputs = tick_inputs.take_boundary_inputs();
        let outputs = self.run_tick_with_boundary_inputs(boundary_inputs);

        #[cfg(test)]
        for intent in tick_inputs.take_test_emissions() {
            self.emit_cross_shard(intent)
                .expect("bounded test hook was preflighted");
        }

        (outputs, self.take_cross_shard_outbox())
    }

    /// Applies a scheduler-owned cross-shard prefix, then this shard's ordinary
    /// inbox, in one tick-boundary execution.
    ///
    /// The prefix is already bounded by the central cross-shard queue. Keeping
    /// it separate from the ordinary inbox prevents accepted cross-shard work
    /// from competing with session-input capacity or being delayed to a later
    /// tick. This private method is reachable from outside the shard module only
    /// through [`SimShard::run_scheduled_tick`], whose input capability can be
    /// constructed only by the scheduler.
    #[allow(clippy::too_many_lines)] // same explicit tick drain as `run_tick`
    fn run_tick_with_boundary_inputs(
        &mut self,
        boundary_inputs: Vec<CrossShardPayload>,
    ) -> Vec<GameOutput> {
        let mut outputs = Vec::new();
        // Coalesce movement: keep only the latest *valid* position and rotation
        // per player (each component merged independently).
        let mut pending_moves: BTreeMap<PlayerId, PendingMove> = BTreeMap::new();
        // Players whose move was rejected this tick and still need a snap-back
        // correction (a later valid move removes them again).
        let mut corrections: BTreeSet<PlayerId> = BTreeSet::new();
        let mut boundary_inputs = boundary_inputs
            .into_iter()
            .map(CrossShardPayload::into_input);

        loop {
            let input = boundary_inputs.next().or_else(|| self.inbox.pop_front());
            let Some(input) = input else {
                break;
            };
            match input {
                GameInput::PlayerJoin { player, position } => {
                    // A duplicate join for an already-present player is ignored:
                    // the first join wins and re-joining produces no output,
                    // keeping the result deterministic regardless of retries.
                    if let Entry::Vacant(slot) = self.players.entry(player) {
                        slot.insert(PlayerState {
                            position,
                            yaw: 0.0,
                            pitch: 0.0,
                            game_mode: GameMode::default(),
                        });
                        outputs.push(GameOutput::PlayerSpawned { player, position });
                    }
                }
                GameInput::SetGameMode { player, mode } => {
                    // Mutate the authoritative mode in place; a mode change for an
                    // absent player is a silent no-op and emits nothing.
                    if let Some(state) = self.players.get_mut(&player) {
                        state.game_mode = mode;
                    }
                }
                GameInput::SpawnEntity {
                    position,
                    velocity,
                    gravity,
                } => {
                    // Applied in FIFO order so the output stream matches the inbox.
                    // Id-range exhaustion is a silent skip: run_tick is infallible
                    // by design (a rejected input yields no output, never a panic),
                    // and the ceiling is unreachable for a real shard (see
                    // SimError::EntityIdExhausted).
                    if let Ok(entity) = self.spawn_entity(position, velocity, gravity) {
                        outputs.push(GameOutput::EntitySpawned {
                            entity,
                            position,
                            velocity,
                            kind: SpawnedEntityKind::Simple,
                        });
                    }
                }
                GameInput::DespawnEntity { entity } => {
                    // Despawn of an absent entity is a silent no-op: outputs must
                    // describe real state transitions, so a retry never doubles the
                    // client-visible removal.
                    if self.remove_entity(entity) {
                        outputs.push(GameOutput::EntityDespawned { entity });
                    }
                }
                GameInput::PlayerMove {
                    player,
                    position,
                    yaw,
                    pitch,
                } => {
                    // Movement for an unknown player is ignored rather than
                    // implicitly spawning one; there is also nothing to correct.
                    if !self.players.contains_key(&player) {
                        continue;
                    }
                    // A position is accepted only if finite and in range.
                    let valid_position = position.filter(|p| is_valid_position(*p));
                    if position.is_some() && valid_position.is_none() {
                        // Reject out-of-range / non-finite coords. Request a
                        // correction only if no valid position is queued to
                        // override the client's bad one.
                        let valid_pending = pending_moves
                            .get(&player)
                            .is_some_and(|m| m.position.is_some());
                        if !valid_pending {
                            corrections.insert(player);
                        }
                    }
                    // Coalesce each component independently: a later input's
                    // `Some` overwrites, a `None` leaves the earlier value. Only
                    // touch the entry when there is something to record so a
                    // purely-invalid move leaves no empty entry to apply.
                    if valid_position.is_some() || yaw.is_some() || pitch.is_some() {
                        let merged = pending_moves.entry(player).or_default();
                        if valid_position.is_some() {
                            merged.position = valid_position;
                            // A valid move supersedes a queued correction.
                            corrections.remove(&player);
                        }
                        if yaw.is_some() {
                            merged.yaw = yaw;
                        }
                        if pitch.is_some() {
                            merged.pitch = pitch;
                        }
                    }
                }
                GameInput::PlayerLeave { player } => {
                    // A leave cancels any queued movement/correction: there is no
                    // point moving or correcting a player who is gone.
                    pending_moves.remove(&player);
                    corrections.remove(&player);
                    // Drop the player's undo history so it cannot leak past their
                    // session (it is keyed by player and bounded only per-player).
                    self.undo_history.remove(&player);
                    if self.players.remove(&player).is_some() {
                        outputs.push(GameOutput::PlayerDespawned { player });
                    }
                }
                GameInput::BlockBreak {
                    player,
                    position,
                    sequence,
                } => {
                    // Break -> air. Applied in FIFO order during the drain so the
                    // output keeps the inbox ordering.
                    let cause = MutationCause::PlayerCreative { player };
                    let result = self.apply_block_edit(cause, position, BlockStateId::AIR);
                    if let Some(output) =
                        block_change_output(cause, sequence, position, BlockStateId::AIR, result)
                    {
                        outputs.push(output);
                    }
                    // The block resting on the one just broken may now be
                    // unsupported: if it is a gravity block, it starts falling.
                    if let MutationResult::Applied { .. } = result {
                        self.settle_falling_block(position.offset(Direction::Up), &mut outputs);
                    }
                }
                GameInput::BlockPlace {
                    player,
                    position,
                    sequence,
                    state,
                    clicked_face,
                    cursor_position,
                    player_yaw,
                } => {
                    // Refine the held item's default state into the correct placed
                    // state (rotation/facing/half/fence connectivity, plus any
                    // relocated merge or extra door/bed cell) against an immutable
                    // view of the resident chunks. The borrow ends before the
                    // mutable writes below. The same refinement backs the off-tick
                    // `preview_placement` the driver uses to report the final state
                    // to the after-hook, so the two never diverge.
                    let computed = self.refine_placement(
                        state,
                        clicked_face,
                        cursor_position,
                        player_yaw,
                        position,
                    );
                    self.apply_player_placement(
                        &mut outputs,
                        player,
                        position,
                        sequence,
                        state,
                        computed.as_ref(),
                    );
                    // A placed gravity block with nothing beneath it falls at once.
                    // `settle_falling_block` is self-gating (it acts only on an
                    // unsupported gravity block at `position`), so it is safe to run
                    // unconditionally after the placement helper.
                    self.settle_falling_block(position, &mut outputs);
                }
                GameInput::SetBlockExact {
                    player,
                    position,
                    sequence,
                    state,
                } => {
                    // An authoritative plugin/command exact write: store `state`
                    // verbatim through the same edit funnel, with NO
                    // compute_placement refinement and NO fence-neighbour pass. The
                    // plugin already chose the final state (e.g. a rotated
                    // `oak_log axis=x`), so re-deriving it would corrupt it.
                    let cause = MutationCause::PlayerCreative { player };
                    let result = self.apply_block_edit(cause, position, state);
                    if let Some(output) =
                        block_change_output(cause, sequence, position, state, result)
                    {
                        outputs.push(output);
                    }
                }
                GameInput::RejectBlockEdit {
                    player,
                    position,
                    sequence,
                    requested_state,
                } => {
                    // An edit refused upstream (plugin Deny / veto): the world is
                    // never touched. Read the authoritative state at the target and
                    // emit the same rejection output an in-sim refusal produces, so
                    // the actor is healed (mandatory resync + ack) through one
                    // funnel. Read-only: no `set_block`, no journal entry, so the
                    // tick stays deterministic.
                    outputs.push(GameOutput::BlockChangeRejected {
                        player,
                        position,
                        sequence,
                        requested_state,
                        authoritative_state: self.authoritative_state(position),
                    });
                }
                GameInput::RegionEdit { player, region, op } => {
                    // Queue the edit; it is drained a budgeted number of cells per
                    // tick (starting this tick) by `drive_region_work` below.
                    self.enqueue_region_edit(player, region, op);
                }
                GameInput::RegionUndo { player } => {
                    self.enqueue_region_undo(player);
                }
                GameInput::UpdateSign {
                    player,
                    position,
                    is_front,
                    lines,
                } => {
                    // Validate and apply the sign-text edit (actor present, chunk
                    // resident, in reach, a non-waxed sign present). On acceptance,
                    // broadcast the new text; a failed validation is a silent no-op
                    // (net never writes the world directly).
                    if let Some(sign) = self.apply_sign_update(player, position, is_front, lines) {
                        outputs.push(GameOutput::SignUpdated {
                            position,
                            sign: Box::new(sign),
                        });
                    }
                }
            }
        }

        // Apply the coalesced moves at the boundary, in deterministic player
        // order. Every player here was present at coalesce time and cannot have
        // left (a leave clears the entry), so the lookup always succeeds. Every
        // entry carries at least one component, so each yields a PlayerMoved.
        for (player, merged) in pending_moves {
            if let Some(state) = self.players.get_mut(&player) {
                let position_changed = merged.position.is_some();
                if let Some(position) = merged.position {
                    state.position = position;
                }
                if let Some(yaw) = merged.yaw {
                    state.yaw = yaw;
                }
                if let Some(pitch) = merged.pitch {
                    state.pitch = pitch;
                }
                outputs.push(GameOutput::PlayerMoved {
                    player,
                    position: state.position,
                    yaw: state.yaw,
                    pitch: state.pitch,
                    position_changed,
                });
            }
        }

        // Snap clients back for rejected moves with no superseding valid move.
        for player in corrections {
            if let Some(state) = self.players.get(&player) {
                outputs.push(GameOutput::PlayerPositionCorrected {
                    player,
                    position: state.position,
                });
            }
        }

        // Apply a budgeted slice of any in-flight region edits/undos. Runs after
        // the inbox drain so an edit enqueued this tick begins applying this tick,
        // while a large fill spreads its remaining cells over later ticks.
        self.drive_region_work(&mut outputs);

        // Integrate non-player entity motion for this tick. Runs last so any
        // entity spawned by this tick's input drain (or by region work above) also
        // gets integrated — matters once spawns carry non-zero velocity (falling
        // blocks, projectiles). Outputs are appended in ascending EntityId order
        // (see apply_entity_physics), keeping the tick fully deterministic.
        self.apply_entity_physics(&mut outputs);

        outputs
    }

    /// Applies one tick of gravity, integration, ground collision, and air drag
    /// to every non-player entity, appending a [`GameOutput::EntityMoved`] for
    /// each one that moved.
    ///
    /// Per entity, in vanilla order:
    /// 1. **Gravity** — while airborne, `velocity.y += gravity` (the category
    ///    acceleration seeded at spawn). A grounded entity (`on_ground`) is not
    ///    accelerated.
    /// 2. **Integration** — the tentative next position is `position + velocity`.
    /// 3. **Ground collision** — a downward move whose landing point enters a
    ///    solid block is clamped to that block's top face: the vertical velocity
    ///    is zeroed and `on_ground` is set. This is the only collision axis
    ///    handled here.
    /// 4. **Air drag** — while still airborne, `velocity.y *= AIR_DRAG_Y`, which
    ///    makes the fall speed converge to terminal velocity without a hardcoded
    ///    clamp (see [`crate::physics`]).
    ///
    /// A grounded entity that still has a floor is skipped, so a store of
    /// resting entities produces no output. All arithmetic is `f64` in a fixed
    /// order, so the result is bit-identical across runs (the determinism
    /// invariant); iteration over the `entities` [`BTreeMap`] appends outputs in
    /// ascending [`EntityId`] order.
    ///
    /// A grounded entity re-checks the block beneath it each tick: if that floor
    /// is gone (broken this tick, for instance) it un-grounds and gravity resumes
    /// next — no separate event path is needed to notice a vanished floor.
    ///
    /// # Collision
    ///
    /// Each entity has an axis-aligned box ([`entity_dimensions`]) that is swept
    /// the full tick and clamped against every solid world block along its path,
    /// one axis at a time ([`sweep_move`]). Sweeping the whole path — not just the
    /// destination cell — means a fast mover near terminal velocity (~3.92
    /// blocks/tick) can never tunnel through a one-block floor or wall, and the
    /// per-axis clip resolves horizontal collision (a wall, a piston-pushed block)
    /// as well as landing. A blocked axis has its velocity zeroed; a downward stop
    /// grounds the entity.
    ///
    /// # Remaining
    ///
    /// Sub-block support shapes (slabs, stairs, fences) still collide as full
    /// cubes — [`is_solid_block`] tests `is_solid_cube`, so an entity rests on a
    /// slab's full-cube top rather than its half height. Refining per-shape AABBs
    /// is a later step.
    fn apply_entity_physics(&mut self, outputs: &mut Vec<GameOutput>) {
        // Process entities one at a time in ascending id order (a snapshot of the
        // keys, so the map can be mutated inside the loop). Handling each entity
        // fully — including writing a landed falling block back to the world and
        // despawning it — before the next means a lower entity's restored block is
        // already solid
        // when a higher entity in the same column sweeps this tick, so a collapsing
        // stack re-stacks correctly instead of piling into one cell. Ascending id
        // order keeps the output stream deterministic.
        if self.entities.is_empty() {
            return;
        }
        let ids: Vec<EntityId> = self.entities.keys().copied().collect();
        // Broadphase scratch reused across every entity this tick: grows to the
        // largest sweep's footprint once, then only `clear()`s — no per-entity
        // allocation. A tick-local, so it never crosses the `&self.chunks` /
        // `&mut self.entities` borrow and keeps the step allocation-free after
        // warmup.
        let mut solids: Vec<BlockPos> = Vec::new();
        for entity in ids {
            let Some(state) = self.entities.get(&entity).copied() else {
                continue;
            };

            // An entity that fell into the void is removed rather than integrated
            // forever (bounded lifetime, see VOID_DESPAWN_Y).
            if state.position.y < VOID_DESPAWN_Y {
                self.entities.remove(&entity);
                outputs.push(GameOutput::EntityDespawned { entity });
                continue;
            }

            // A grounded entity rests only while the block beneath it is solid.
            // Grounded positions are clamped to an integer top face, so the cell
            // one below is exactly the supporting block. If it is still there the
            // entity stays put and emits nothing; otherwise it un-grounds and falls.
            if state.on_ground {
                let here = block_cell(state.position);
                let below = BlockPos::new(here.x(), here.y().saturating_sub(1), here.z());
                if is_solid_block(&self.chunks, below) {
                    continue;
                }
            }

            // 1. Gravity accelerates the (now certainly airborne) entity.
            let velocity = Vec3::new(
                state.velocity.x,
                state.velocity.y + state.gravity,
                state.velocity.z,
            );
            // Nothing to integrate for a gravity-free, motionless entity, but a
            // just-un-grounded one still needs its ground flag cleared.
            if velocity == Vec3::ZERO {
                if state.on_ground {
                    if let Some(st) = self.entities.get_mut(&entity) {
                        st.on_ground = false;
                    }
                }
                continue;
            }

            // 2-3. Swept integration + collision. The entity's box is swept the
            // full tick and clamped against every solid block along the path, one
            // axis at a time (Y, then X, then Z — the vanilla order), so a fast
            // mover can never tunnel through a thin floor or wall. A blocked axis
            // has its velocity zeroed; a downward stop sets `grounded`.
            let start = state.position;
            let (half_width, height) = entity_dimensions(state.kind);
            let swept = sweep_move(
                &self.chunks,
                &mut solids,
                start,
                half_width,
                height,
                velocity,
            );
            let next = swept.position;
            let grounded = swept.on_ground;

            // 4. Vertical air drag, only while still falling (a grounded entity has
            // already had its vertical velocity zeroed by the sweep).
            let velocity = if grounded {
                swept.velocity
            } else {
                Vec3::new(
                    swept.velocity.x,
                    swept.velocity.y * AIR_DRAG_Y,
                    swept.velocity.z,
                )
            };

            if let Some(st) = self.entities.get_mut(&entity) {
                st.position = next;
                st.velocity = velocity;
                st.on_ground = grounded;
            }

            // Emit only for a real position change (a landing clamp that leaves the
            // entity exactly where it was produces no output).
            if next != start {
                outputs.push(GameOutput::EntityMoved {
                    entity,
                    position: next,
                });
            }

            // A falling block that just landed either restores its block or breaks,
            // depending on what already occupies its resting cell (the cell directly
            // above the support). A wall torch on a side block lives in a different
            // cell, so it never triggers this — only an object sitting on the
            // support does.
            if grounded {
                if let EntityKind::FallingBlock { block } = state.kind {
                    let cell = block_cell(next);
                    let support = BlockPos::new(cell.x(), cell.y().saturating_sub(1), cell.z());
                    // A faller breaks instead of settling in two vanilla cases:
                    // (1) its resting cell already holds a non-air, non-replaceable
                    // block it fell through — a torch, sapling, sign, plate, … (an
                    // `"empty"`-box object) — so it cannot occupy the cell; or (2) it
                    // came to rest on top of a solid but non-full-height support — a
                    // slab, soul sand, farmland, chest, … — whose real collision top
                    // is below a full cube, so vanilla sinks the faller into that
                    // block's cell and breaks it. Air or a replaceable fluid/plant in
                    // the resting cell, over a genuine full cube, settles.
                    if is_non_replaceable_block(&self.chunks, cell)
                        || breaks_on_support(&self.chunks, support)
                    {
                        // Breaks and vanishes (the dropped item is deferred until
                        // item entities exist).
                        self.entities.remove(&entity);
                        outputs.push(GameOutput::EntityDespawned { entity });
                    } else {
                        // Restore the block and despawn the entity the same tick
                        // (through the same edit funnel as every mutation — a
                        // `Command` cause: no actor, no ack), so a higher entity
                        // landing later this tick sees it as solid. Because entities
                        // are handled in ascending id order and the block is written
                        // before the next is processed, a collapsing column re-stacks
                        // correctly.
                        let result = self.apply_block_edit(MutationCause::Command, cell, block);
                        if let Some(output) = block_change_output(
                            MutationCause::Command,
                            NO_CLIENT_SEQUENCE,
                            cell,
                            block,
                            result,
                        ) {
                            outputs.push(output);
                        }
                        self.entities.remove(&entity);
                        outputs.push(GameOutput::EntityDespawned { entity });
                    }
                }
            }
        }
    }

    /// If the block at `pos` is a gravity block that has lost its support,
    /// converts it — and the unbroken column of gravity blocks stacked directly
    /// above it — into falling-block entities.
    ///
    /// Called after a block edit that could unsupport a gravity block: after a
    /// place (the placed block may hang in the air) and above a break (the block
    /// resting on the broken one may now be unsupported). A block is unsupported
    /// when the block directly below it is not a solid cube.
    ///
    /// A no-op unless `pos` holds a resident, gravity-affected, unsupported
    /// block. When it does, the whole gravity column from `pos` upward converts
    /// at once (removing the bottom block unsupports every gravity block above
    /// it, so they all fall together, as in vanilla): each conversion removes
    /// the block through the edit funnel (`Command` cause: broadcast, no ack) and
    /// spawns an entity that falls on the same tick's physics pass. Entities are
    /// spawned bottom-up, so lower ids sit lower in the column and re-stack in the
    /// right order when they land.
    fn settle_falling_block(&mut self, pos: BlockPos, outputs: &mut Vec<GameOutput>) {
        // The bottom block only falls if it is a gravity block with no support.
        let Some(state) = self.gravity_block_at(pos) else {
            return;
        };
        let below = BlockPos::new(pos.x(), pos.y().saturating_sub(1), pos.z());
        if is_non_replaceable_block(&self.chunks, below) {
            return;
        }

        // Convert this block and every gravity block directly above it. Removing
        // the one below unsupports the next, so the column collapses as a unit.
        let mut cell = pos;
        let mut carried = state;
        loop {
            self.convert_block_to_falling_entity(cell, carried, outputs);
            let above = BlockPos::new(cell.x(), cell.y().saturating_add(1), cell.z());
            match self.gravity_block_at(above) {
                Some(next) => {
                    cell = above;
                    carried = next;
                }
                None => break,
            }
        }
    }

    /// Returns the block-state at `pos` if it is a resident, gravity-affected
    /// block, or `None` otherwise (air, non-gravity, unknown, or non-resident).
    fn gravity_block_at(&self, pos: BlockPos) -> Option<BlockStateId> {
        let state = self.authoritative_state(pos);
        if state.is_air() {
            return None;
        }
        let name = state_id_to_block_name(state.as_u32())?;
        is_gravity_affected(name).then_some(state)
    }

    /// Removes the block at `pos` and spawns a falling-block entity carrying
    /// `state`, emitting the block removal and the entity spawn. Assumes the
    /// caller has already established that `pos` should fall.
    fn convert_block_to_falling_entity(
        &mut self,
        pos: BlockPos,
        state: BlockStateId,
        outputs: &mut Vec<GameOutput>,
    ) {
        // Only convert if the removal actually landed (chunk resident, applied).
        let result = self.apply_block_edit(MutationCause::Command, pos, BlockStateId::AIR);
        if !matches!(result, MutationResult::Applied { .. }) {
            return;
        }
        if let Some(output) =
            block_change_output(MutationCause::Command, 0, pos, BlockStateId::AIR, result)
        {
            outputs.push(output);
        }
        // Spawn at the cell's horizontal centre and vertical base, so `floor`
        // maps the entity back to this column when it lands.
        let spawn_pos = Vec3::new(
            f64::from(pos.x()) + 0.5,
            f64::from(pos.y()),
            f64::from(pos.z()) + 0.5,
        );
        if let Ok(entity) = self.spawn_falling_block(spawn_pos, Vec3::ZERO, GRAVITY_ITEM, state) {
            outputs.push(GameOutput::EntitySpawned {
                entity,
                position: spawn_pos,
                velocity: Vec3::ZERO,
                kind: SpawnedEntityKind::FallingBlock { block: state },
            });
        }
    }

    /// Validates and applies a single block edit at the tick boundary — the one
    /// and only block-write funnel.
    ///
    /// Returns the structured [`MutationResult`]: [`Applied`](MutationResult::Applied)
    /// (the write happened and `set_block` marked the section dirty) or
    /// [`Rejected`](MutationResult::Rejected) with a [`RejectionReason`] and the
    /// authoritative state the client must heal to. An edit is rejected, in this
    /// precedence, when:
    /// - the acting player is not present in the shard ([`ActorAbsent`](RejectionReason::ActorAbsent));
    /// - the target chunk is not resident in this shard ([`ChunkNotLoaded`](RejectionReason::ChunkNotLoaded),
    ///   which also covers another dimension, since the map is dimension-scoped) —
    ///   checked before reach so an edit aimed at an absent chunk is rejected
    ///   silently rather than healed to a fabricated air state;
    /// - the target is beyond [`MAX_REACH`] of the actor ([`OutOfReach`](RejectionReason::OutOfReach)); or
    /// - the target `y` is outside the buildable range ([`YOutOfBounds`](RejectionReason::YOutOfBounds),
    ///   rejected by [`Chunk::set_block`](ferrumc_world::Chunk::set_block) without panic).
    ///
    /// Only [`MutationCause::PlayerCreative`] is reach-checked (it carries the
    /// actor); other causes bypass the actor/reach checks. A rejected edit never
    /// mutates chunk state.
    fn apply_block_edit(
        &mut self,
        cause: MutationCause,
        position: BlockPos,
        requested_state: BlockStateId,
    ) -> MutationResult {
        // Only a player edit has an actor; other causes (command/plugin/test) are
        // authoritative and skip the actor/reach checks. Resolve the actor first so
        // an edit by an absent player is rejected before anything else.
        let actor = match cause {
            MutationCause::PlayerCreative { player } => {
                let Some(actor_position) = self.players.get(&player).map(|state| state.position)
                else {
                    return MutationResult::Rejected {
                        reason: RejectionReason::ActorAbsent,
                        authoritative_state: self.authoritative_state(position),
                    };
                };
                Some(actor_position)
            }
            _ => None,
        };

        // The target chunk must be resident: it is where the write lands and the
        // only source of the authoritative state a rejection heals to. Checked
        // *before* reach so an edit aimed at an absent chunk is rejected silently
        // (no client to heal — `block_change_output` drops it) rather than
        // "healing" the client to a fabricated air state. The real column corrects
        // the client when it streams in.
        if !self.chunks.is_loaded(position.to_chunk_pos()) {
            return MutationResult::Rejected {
                reason: RejectionReason::ChunkNotLoaded,
                authoritative_state: BlockStateId::AIR,
            };
        }

        // Reach is validated only for a player edit, against the actor's
        // start-of-tick position; the chunk is resident, so the resync carries the
        // real authoritative state.
        if let Some(actor) = actor {
            if !within_reach(actor, position) {
                return MutationResult::Rejected {
                    reason: RejectionReason::OutOfReach,
                    authoritative_state: self.authoritative_state(position),
                };
            }
        }

        let Some(chunk) = self.chunks.get_mut(position.to_chunk_pos()) else {
            // Unreachable: residency was confirmed above. Fail closed rather than
            // panic if the invariant ever changes.
            return MutationResult::Rejected {
                reason: RejectionReason::ChunkNotLoaded,
                authoritative_state: BlockStateId::AIR,
            };
        };
        let old_state = chunk.get_block(position).unwrap_or(BlockStateId::AIR);
        // The chunk was looked up by `position`'s own column, so `set_block` can
        // only fail on an out-of-range `y`; treat that as a rejected edit rather
        // than mutating or panicking.
        match chunk.set_block(position, requested_state) {
            Ok(()) => {
                // Reconcile the block-entity with the new block: a sign block keeps
                // (or, if absent/of a different kind, gains) a blank sign
                // block-entity; any non-sign block clears a stale one. An existing
                // sign of the same kind is preserved so re-placing the same sign
                // keeps its text. The map is bounded; an at-capacity insert is
                // dropped best-effort (the block itself is still placed).
                match sign_kind_for_state(requested_state.as_u32()) {
                    Some(kind) => {
                        let needs_fresh = !matches!(
                            chunk.block_entity(position),
                            Some(BlockEntity::Sign(sign)) if sign.kind() == kind
                        );
                        if needs_fresh {
                            let _ = chunk
                                .set_block_entity(position, BlockEntity::Sign(Sign::new(kind)));
                        }
                    }
                    None => {
                        // A chest block gains (or, if absent/of a different kind,
                        // gains a fresh) empty container; an existing chest is
                        // preserved so re-placing the same chest keeps its
                        // contents. Any other block clears a stale block-entity.
                        if is_chest_state(requested_state.as_u32()) {
                            if !matches!(chunk.block_entity(position), Some(BlockEntity::Chest(_)))
                            {
                                let _ = chunk.set_block_entity(
                                    position,
                                    BlockEntity::Chest(ChestInventory::new()),
                                );
                            }
                        } else {
                            chunk.remove_block_entity(position);
                        }
                    }
                }
                // A non-test gameplay edit drives the *persistence* signal: mark
                // the owning section persist-dirty (so only player-modified chunks
                // ever produce an overlay) and journal the mutation. A `Test` cause
                // is excluded so deterministic test/replay edits never persist.
                // `set_block` already marked the network dirty mask for everyone.
                if !matches!(cause, MutationCause::Test) {
                    chunk.mark_persist_dirty(position);
                    // Defensive bound only: the driver drains this every tick, so
                    // it is cleared long before reaching the cap. Past the cap a
                    // journal entry is dropped (best-effort; the overlay still
                    // persists the block) rather than growing the buffer unbounded.
                    if self.mutation_log.len() < MUTATION_LOG_CAP {
                        self.mutation_log.push(PendingMutation::new(
                            cause,
                            position,
                            old_state,
                            requested_state,
                        ));
                    }
                }
                MutationResult::Applied {
                    new_state: requested_state,
                }
            }
            Err(_) => MutationResult::Rejected {
                reason: RejectionReason::YOutOfBounds,
                authoritative_state: old_state,
            },
        }
    }

    /// Applies a player block placement at the tick boundary, honouring the
    /// placement engine's full result: a relocated **merge** write, a
    /// water-replaceable target, and **multi-cell** doors/beds — atomically.
    ///
    /// `computed` is the [`PlacementResult`] from
    /// [`refine_placement`](Self::refine_placement), or `None` for an unrecognised
    /// block (which falls back to writing the held `state` at `position`). The write
    /// plan is:
    ///
    /// - **Primary cell** — [`PlacementResult::place_at`] when set (a double-slab or
    ///   candle *merge*, which intentionally replaces the block already there),
    ///   otherwise `position`. Written under [`MutationCause::PlayerCreative`] so the
    ///   placing client is acked/resynced.
    /// - **Extra cells** — [`PlacementResult::extra_blocks`] (a door's `upper` half,
    ///   a bed's `head`). Written under [`MutationCause::Command`]: broadcast-only,
    ///   no per-cell ack, no reach check (vanilla does not reach-check the second
    ///   cell).
    ///
    /// Every cell that must be newly occupied (the primary of a non-merge placement,
    /// and every extra) is first checked to be *free* — air or a replaceable water
    /// source (see [`is_placement_clear`](Self::is_placement_clear)). If any is
    /// obstructed (a ceiling above a door, a wall where a bed head would go) the
    /// **whole** placement is rejected and the actor healed — never a half-door. A
    /// merge's primary cell is exempt: it completes the matching block already there,
    /// which the engine validated.
    fn apply_player_placement(
        &mut self,
        outputs: &mut Vec<GameOutput>,
        player: PlayerId,
        position: BlockPos,
        sequence: i32,
        held: BlockStateId,
        computed: Option<&PlacementResult>,
    ) {
        let cause = MutationCause::PlayerCreative { player };
        // A merge relocates the single write onto the clicked cell.
        let primary_pos = computed.and_then(|r| r.place_at).unwrap_or(position);
        let is_merge = computed.is_some_and(|r| r.place_at.is_some());
        // Unsupported/unrecognised -> safe default (the held state).
        let primary_state = computed.map_or(held, |r| BlockStateId::new(r.state_id));
        let is_fence = computed.is_some_and(|r| r.rule == PlacementRule::FenceLike);

        // Replaceability / multi-cell gate: validate every cell that must be newly
        // occupied BEFORE writing anything, so an obstructed door/bed never lands a
        // half-block. A merge's primary cell is exempt (it completes the block
        // already there). A non-resident extra chunk counts as obstructed.
        let primary_blocked = !is_merge && !self.is_placement_clear(primary_pos);
        let extra_blocked = computed.is_some_and(|r| {
            r.extra_blocks
                .iter()
                .any(|&(epos, _)| !self.is_placement_clear(epos))
        });
        if primary_blocked || extra_blocked {
            // Heal the actor's prediction at the clicked cell, but only when the
            // actor is present (an absent actor has no session to resync).
            if self.players.contains_key(&player) {
                outputs.push(GameOutput::BlockChangeRejected {
                    player,
                    position,
                    sequence,
                    requested_state: held,
                    authoritative_state: self.authoritative_state(position),
                });
            }
            return;
        }

        // Apply the primary cell first. If it is refused (absent actor, unloaded
        // chunk, out of reach, y-out-of-bounds) nothing else is written, so a
        // rejected door never lands its upper half.
        let result = self.apply_block_edit(cause, primary_pos, primary_state);
        let MutationResult::Applied { .. } = result else {
            if let Some(output) =
                block_change_output(cause, sequence, position, primary_state, result)
            {
                outputs.push(output);
            }
            return;
        };
        if let Some(output) =
            block_change_output(cause, sequence, primary_pos, primary_state, result)
        {
            outputs.push(output);
        }
        // An accepted sign placement opens the editor for the placer (ordered after
        // the BlockChanged so the block exists client-side first).
        if sign_kind_for_state(primary_state.as_u32()).is_some() {
            outputs.push(GameOutput::OpenSignEditor {
                player,
                position: primary_pos,
            });
        }
        // Apply the pre-validated extra cells (door upper / bed head). They are part
        // of the same placement: broadcast-only (Command cause -> no extra ack) and
        // persisted like any gameplay edit.
        if let Some(r) = computed {
            for &(epos, estate) in &r.extra_blocks {
                let estate = BlockStateId::new(estate);
                let eresult = self.apply_block_edit(MutationCause::Command, epos, estate);
                if let Some(output) =
                    block_change_output(MutationCause::Command, sequence, epos, estate, eresult)
                {
                    outputs.push(output);
                }
            }
        }
        // A placed fence updates its same-fence cardinal neighbours so they connect
        // back to it (broadcast-only, no extra ack). The reverse lookup yields a
        // `'static` name, so it does not borrow `self`.
        if is_fence {
            if let Some(fence_name) = state_id_to_block_name(primary_state.as_u32()) {
                self.update_fence_neighbors(outputs, primary_pos, fence_name);
            }
        }
    }

    /// Returns `true` if `position` is a resident, in-range cell a placement may
    /// occupy: it must hold air or a replaceable water *source*.
    ///
    /// A non-resident chunk or an out-of-range `y` returns `false` (the placement
    /// cannot safely land there). A water source counts as replaceable so a
    /// waterloggable block can be placed into it; every other block (flowing water
    /// included, and any solid) blocks the placement, preserving the usual "cannot
    /// place into an occupied cell" rule.
    fn is_placement_clear(&self, position: BlockPos) -> bool {
        let Some(chunk) = self.chunks.get(position.to_chunk_pos()) else {
            return false;
        };
        let Some(state) = chunk.get_block(position) else {
            return false;
        };
        state.is_air() || is_water_source(state.as_u32())
    }

    /// Queues a region (cuboid) block edit for incremental application after a
    /// defensive volume re-check.
    ///
    /// The command layer already rejected an over-cap region with a user-facing
    /// error; re-checking [`RegionLimits::max_volume`] here guards every other
    /// caller from stalling a tick. The edit is appended to the bounded pending
    /// queue and drained a budgeted number of cells per tick by
    /// [`drive_region_work`](Self::drive_region_work); when the queue is full it is
    /// dropped (best-effort backpressure, like the inbox).
    fn enqueue_region_edit(&mut self, player: PlayerId, region: Cuboid, op: RegionOp) {
        if region.volume() > self.region_limits.max_volume {
            return;
        }
        // Best-effort backpressure: past the (generous) in-flight bound a new edit
        // is dropped rather than growing memory. The per-tick budget drains the
        // queue continuously, so this is only reachable under an absurd command
        // flood; the deterministic shard has no logger, so the drop is silent.
        if self.pending_region_work.len() >= MAX_PENDING_REGION_WORK {
            return;
        }
        self.pending_region_work.push_back(PendingRegionWork {
            player,
            kind: RegionWorkKind::Edit {
                region,
                op,
                cursor: 0,
                prior: Vec::new(),
            },
        });
    }

    /// Pops `player`'s most recent undo entry and queues its restoration as
    /// incremental work, or does nothing if they have no recorded edits.
    ///
    /// The entry is popped now (deterministically, at the tick boundary) so a
    /// later `/undo` walks back to the previous edit; the restoration itself is
    /// drained across ticks like any region edit and is *not* re-recorded. When
    /// the pending queue is full the undo is dropped without popping (the operator
    /// can retry).
    fn enqueue_region_undo(&mut self, player: PlayerId) {
        // Best-effort backpressure, as in `enqueue_region_edit`: drop the undo
        // (without popping the history) when the in-flight queue is saturated.
        if self.pending_region_work.len() >= MAX_PENDING_REGION_WORK {
            return;
        }
        // Pop the newest entry; the mutable borrow ends before anything else.
        let entry = match self.undo_history.get_mut(&player) {
            Some(stack) => stack.pop_back(),
            None => None,
        };
        // Drop an emptied stack so the map does not retain idle players.
        if self
            .undo_history
            .get(&player)
            .is_some_and(VecDeque::is_empty)
        {
            self.undo_history.remove(&player);
        }
        let Some(entry) = entry else {
            return;
        };
        self.pending_region_work.push_back(PendingRegionWork {
            player,
            kind: RegionWorkKind::Undo {
                restore: entry.blocks,
                cursor: 0,
            },
        });
    }

    /// Pushes a completed region edit's captured prior states onto `player`'s undo
    /// history, evicting the oldest entry once [`RegionLimits::max_undo_entries`]
    /// is exceeded. A cap of zero disables history entirely.
    fn push_undo(&mut self, player: PlayerId, entry: RegionUndoEntry) {
        let cap = self.region_limits.max_undo_entries;
        if cap == 0 {
            return;
        }
        let stack = self.undo_history.entry(player).or_default();
        stack.push_back(entry);
        // Evict oldest-first until within the cap (a single push can exceed it by
        // at most one, but the loop is robust if the cap is lowered at runtime).
        while stack.len() > cap {
            stack.pop_front();
        }
    }

    /// Applies up to [`RegionLimits::max_blocks_per_tick`] region cells from the
    /// pending queue this tick, oldest item first, emitting a `BlockChanged` for
    /// every changed cell.
    ///
    /// A completed edit records its captured prior states as an undo entry. An item
    /// that does not finish within the budget resumes from its saved cursor next
    /// tick. Called once per [`run_tick`](Self::run_tick), after the inbox drain,
    /// so a region command enqueued this tick begins applying this tick.
    fn drive_region_work(&mut self, outputs: &mut Vec<GameOutput>) {
        // At least one cell per tick, so progress is always made even if the budget
        // is misconfigured to zero.
        let mut budget = self.region_limits.max_blocks_per_tick.max(1);
        while budget > 0 {
            let Some(mut work) = self.pending_region_work.pop_front() else {
                break;
            };
            if self.advance_region_work(&mut work, &mut budget, outputs) {
                // Completed: an Edit that changed at least one cell is undoable.
                if let RegionWorkKind::Edit { prior, .. } = work.kind {
                    if !prior.is_empty() {
                        self.push_undo(work.player, RegionUndoEntry { blocks: prior });
                    }
                }
            } else {
                // Budget exhausted mid-item: resume it next tick.
                self.pending_region_work.push_front(work);
                break;
            }
        }
    }

    /// Advances one pending region-work item by up to `*budget` cells through the
    /// single block-edit funnel under [`MutationCause::Command`], decrementing
    /// `budget` per cell examined and emitting a `BlockChanged` per changed cell.
    ///
    /// Returns `true` when the item is fully applied, `false` when the budget ran
    /// out first (the item's cursor is left ready to resume). A cell is skipped (no
    /// write, no broadcast, no undo capture) when its chunk is not resident
    /// (cross-shard edits are out of scope), when a [`RegionOp::Replace`] does not
    /// match, or when the new state already equals the current one.
    fn advance_region_work(
        &mut self,
        work: &mut PendingRegionWork,
        budget: &mut usize,
        outputs: &mut Vec<GameOutput>,
    ) -> bool {
        let cause = MutationCause::Command;
        match &mut work.kind {
            RegionWorkKind::Edit {
                region,
                op,
                cursor,
                prior,
            } => {
                while *budget > 0 {
                    let Some(position) = region.block_at_index(*cursor) else {
                        return true; // every cell examined
                    };
                    *cursor += 1;
                    *budget -= 1;
                    let Some(current) = self
                        .chunks
                        .get(position.to_chunk_pos())
                        .and_then(|chunk| chunk.get_block(position))
                    else {
                        continue;
                    };
                    let new_state = match *op {
                        RegionOp::Fill { state } => state,
                        RegionOp::Replace { from, to } => {
                            if current != from {
                                continue;
                            }
                            to
                        }
                    };
                    if new_state == current {
                        continue;
                    }
                    let result = self.apply_block_edit(cause, position, new_state);
                    if matches!(result, MutationResult::Applied { .. }) {
                        prior.push((position, current));
                    }
                    if let Some(output) = block_change_output(cause, 0, position, new_state, result)
                    {
                        outputs.push(output);
                    }
                }
                false
            }
            RegionWorkKind::Undo { restore, cursor } => {
                while *budget > 0 {
                    let Some(&(position, state)) = restore.get(*cursor) else {
                        return true; // every captured cell restored
                    };
                    *cursor += 1;
                    *budget -= 1;
                    let result = self.apply_block_edit(cause, position, state);
                    if let Some(output) = block_change_output(cause, 0, position, state, result) {
                        outputs.push(output);
                    }
                }
                false
            }
        }
    }

    /// Validates and applies a sign-text edit at the tick boundary, returning the
    /// sign's full post-edit state on success or `None` if the edit is refused.
    ///
    /// Refused (silently, mutating nothing and emitting no output) when the actor
    /// is absent, the target chunk is not resident, the target is beyond
    /// [`MAX_REACH`] of the actor, the block at `position` is not a sign, or the
    /// sign is waxed (its text is locked). A sign block whose block-entity is
    /// missing — e.g. one reloaded after a chunk unload dropped the block-entity
    /// while the block itself persisted — gains a fresh blank one here (mirroring
    /// the chest's lazy recreation in [`container_open`](Self::container_open)), so
    /// a reloaded sign stays editable instead of being permanently blank. On
    /// acceptance it replaces the addressed face's four text lines, leaving the
    /// face's color/glow and the other face untouched, and returns a clone of the
    /// updated sign for the [`GameOutput::SignUpdated`] broadcast. The chunk is
    /// marked persist-dirty so the new text is captured into the overlay store and
    /// survives a chunk unload/reload and a server restart.
    fn apply_sign_update(
        &mut self,
        player: PlayerId,
        position: BlockPos,
        is_front: bool,
        lines: [String; SIGN_LINES],
    ) -> Option<Sign> {
        let actor = self.players.get(&player).map(|state| state.position)?;
        if !self.chunks.is_loaded(position.to_chunk_pos()) {
            return None;
        }
        if !within_reach(actor, position) {
            return None;
        }
        let chunk = self.chunks.get_mut(position.to_chunk_pos())?;
        // Only a sign BLOCK can carry sign text; resolve its kind from the block
        // state so a reloaded sign can have its block-entity recreated. A non-sign
        // block (or air) refuses the edit.
        let kind = sign_kind_for_state(chunk.get_block(position)?.as_u32())?;
        // Lazily (re)create a blank sign block-entity if the block carries none:
        // a chunk unload drops the block-entity while the block itself persists, so
        // without this a reloaded sign would be permanently blank and uneditable.
        // Mirrors the chest's lazy recreation in `container_open`.
        if !matches!(chunk.block_entity(position), Some(BlockEntity::Sign(_))) {
            let _ = chunk.set_block_entity(position, BlockEntity::Sign(Sign::new(kind)));
        }
        let Some(BlockEntity::Sign(sign)) = chunk.block_entity_mut(position) else {
            return None;
        };
        if sign.is_waxed() {
            return None;
        }
        sign.set_face_lines(is_front, lines);
        let updated = sign.clone();
        // A sign-text edit changes only the block entity, not the block state, so
        // it does not go through `set_block`; mark the chunk persist-dirty here (as
        // an accepted block edit does) so the new text reaches the overlay store.
        chunk.mark_persist_dirty(position);
        Some(updated)
    }

    /// Opens the chest container at `position` for `player`, returning a snapshot
    /// of its [`CHEST_SLOTS`](ferrumc_world::CHEST_SLOTS) item slots.
    ///
    /// Returns `None` (open refused) unless the actor is present, the target chunk
    /// is resident, the target is within [`MAX_REACH`], and the block at
    /// `position` is a chest. A chest block with no container block-entity yet
    /// (e.g. one placed before this feature, or otherwise missing) gains a fresh
    /// empty one here, so opening is robust to a missing block-entity. The chest
    /// is the world's authoritative copy; the returned snapshot is what the
    /// session layer encodes into the opening `SetContainerContent`.
    ///
    /// Called off-tick by the driver (request/response), like
    /// [`preview_placement`](Self::preview_placement): the only mutation is the
    /// lazy block-entity creation, which is idempotent and order-independent.
    pub fn container_open(
        &mut self,
        player: PlayerId,
        position: BlockPos,
    ) -> Option<Vec<ItemStack>> {
        let actor = self.players.get(&player).map(|state| state.position)?;
        if !self.chunks.is_loaded(position.to_chunk_pos()) {
            return None;
        }
        if !within_reach(actor, position) {
            return None;
        }
        let chunk = self.chunks.get_mut(position.to_chunk_pos())?;
        if !is_chest_state(chunk.get_block(position)?.as_u32()) {
            return None;
        }
        // Lazily create the container if the chest block carries none yet.
        if !matches!(chunk.block_entity(position), Some(BlockEntity::Chest(_))) {
            let _ = chunk.set_block_entity(position, BlockEntity::Chest(ChestInventory::new()));
        }
        let Some(BlockEntity::Chest(chest)) = chunk.block_entity(position) else {
            return None;
        };
        Some(chest.snapshot())
    }

    /// Applies a vanilla left-click on chest slot `slot` with the carried
    /// `cursor`, returning the post-click cursor and the chest's full updated
    /// snapshot.
    ///
    /// The exchange is [`left_click_exchange`], which is item-count conserving in
    /// every branch (pickup / place / merge / swap), so this can never duplicate
    /// or destroy an item. It is applied atomically against the world's
    /// authoritative chest while the driver holds the shard, so two players
    /// clicking the same chest are serialised — there is no read-modify-write race
    /// on the connection's mirror.
    ///
    /// Returns `None` (the caller resyncs) if the actor is absent, the chunk is
    /// not resident, the target is out of reach, the block is no longer a chest,
    /// or `slot` is out of range — in every such case neither the chest nor the
    /// passed cursor is mutated, so the cursor item is never lost.
    pub fn container_left_click(
        &mut self,
        player: PlayerId,
        position: BlockPos,
        slot: usize,
        mut cursor: ItemStack,
    ) -> Option<(ItemStack, Vec<ItemStack>)> {
        let actor = self.players.get(&player).map(|state| state.position)?;
        if !self.chunks.is_loaded(position.to_chunk_pos()) {
            return None;
        }
        if !within_reach(actor, position) {
            return None;
        }
        let chunk = self.chunks.get_mut(position.to_chunk_pos())?;
        if !is_chest_state(chunk.get_block(position)?.as_u32()) {
            return None;
        }
        let Some(BlockEntity::Chest(chest)) = chunk.block_entity_mut(position) else {
            return None;
        };
        let slot_ref = chest.slot_mut(slot)?;
        left_click_exchange(slot_ref, &mut cursor);
        let snapshot = chest.snapshot();
        // The exchange mutated the chest block entity (not the block state), so mark
        // the chunk persist-dirty here (as an accepted block edit does) to write the
        // new contents to the overlay store; an out-of-range slot returned above
        // without mutating, so we only reach this on an applied change.
        chunk.mark_persist_dirty(position);
        Some((cursor, snapshot))
    }

    /// Refines a player placement's held state into the final block-state using
    /// the resident chunks as the neighbour query, or `None` for an
    /// unsupported/unrecognised block.
    ///
    /// This is the single source of placement refinement, shared by the
    /// tick-boundary [`GameInput::BlockPlace`] apply path and the off-tick
    /// read-only [`preview_placement`](Self::preview_placement). Sharing it
    /// guarantees the state previewed to the after-hook equals the state the tick
    /// applies for the common single-edit case (chunks mutate only at tick
    /// boundaries).
    fn refine_placement(
        &self,
        state: BlockStateId,
        clicked_face: Direction,
        cursor_position: Vec3,
        player_yaw: f32,
        position: BlockPos,
    ) -> Option<PlacementResult> {
        let query = ShardNeighborQuery {
            chunks: &self.chunks,
        };
        let ctx = PlacementContext {
            item_block_state: state.as_u32(),
            clicked_face,
            cursor_position,
            player_yaw,
            position,
        };
        compute_placement(&ctx, &query)
    }

    /// Computes the final placed block-state for a player placement *without*
    /// mutating anything — the refinement [`GameInput::BlockPlace`] would apply.
    ///
    /// Read-only and off-tick: the driver calls it to report the final computed
    /// state back to the connection so the `after_block_place` hook fires with the
    /// state the world will hold, not the held item's bare default. An
    /// unsupported/unrecognised block falls back to the held `state` unchanged,
    /// matching the apply path. Because it shares
    /// [`refine_placement`](Self::refine_placement) with the tick and chunks
    /// mutate only at tick boundaries, the preview equals the applied state for
    /// the common single-edit case (neighbour-dependent fences placed in the same
    /// tick are the documented exception).
    #[must_use]
    pub fn preview_placement(
        &self,
        state: BlockStateId,
        clicked_face: Direction,
        cursor_position: Vec3,
        player_yaw: f32,
        position: BlockPos,
    ) -> BlockStateId {
        self.refine_placement(state, clicked_face, cursor_position, player_yaw, position)
            .map_or(state, |r| BlockStateId::new(r.state_id))
    }

    /// Reads the authoritative block state at `position` from the resident chunk,
    /// falling back to [`BlockStateId::AIR`] when the chunk is not resident.
    fn authoritative_state(&self, position: BlockPos) -> BlockStateId {
        self.chunks
            .get(position.to_chunk_pos())
            .and_then(|chunk| chunk.get_block(position))
            .unwrap_or(BlockStateId::AIR)
    }

    /// After a fence is placed at `center`, recomputes each cardinal neighbour that
    /// is the *same* fence so it connects back to the new block, and broadcasts any
    /// that changed.
    ///
    /// Neighbour writes go through the same [`apply_block_edit`](Self::apply_block_edit)
    /// funnel under [`MutationCause::Command`]: they persist and broadcast a viewer
    /// `BlockUpdate` (via [`block_change_output`]) but carry no player, so the router
    /// never acks them (only [`MutationCause::PlayerCreative`] is acked). Cross-shard
    /// neighbours are out of scope this milestone; a neighbour in a non-resident
    /// chunk is simply skipped (stale connectivity at the shard edge).
    fn update_fence_neighbors(
        &mut self,
        outputs: &mut Vec<GameOutput>,
        center: BlockPos,
        fence_name: &str,
    ) {
        for dir in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            let npos = center.offset(dir);
            // Only update a neighbour that is the very same fence.
            let Some(neighbor_state) = self
                .chunks
                .get(npos.to_chunk_pos())
                .and_then(|chunk| chunk.get_block(npos))
            else {
                continue;
            };
            if state_id_to_block_name(neighbor_state.as_u32()) != Some(fence_name) {
                continue;
            }
            // Recompute its connectivity now that the placed fence is visible.
            let recomputed = {
                let query = ShardNeighborQuery {
                    chunks: &self.chunks,
                };
                compute_fence_connection_state(fence_name, npos, &query)
            };
            let Some(new_state) = recomputed else {
                continue;
            };
            if new_state == neighbor_state.as_u32() {
                continue; // already connected; no write, no broadcast
            }
            let cause = MutationCause::Command;
            let result = self.apply_block_edit(cause, npos, BlockStateId::new(new_state));
            // Sequence 0: a Command edit is never acked, so the value is unused.
            if let Some(output) =
                block_change_output(cause, 0, npos, BlockStateId::new(new_state), result)
            {
                outputs.push(output);
            }
        }
    }
}

/// The integer block cell containing world-space `pos` (component-wise floor).
///
/// `f64 as i32` saturates rather than wrapping in Rust, so an out-of-range
/// coordinate maps to `i32::MIN`/`i32::MAX` instead of causing undefined
/// behaviour; callers keep entities within range by other means (void despawn,
/// position validation).
fn block_cell(pos: Vec3) -> BlockPos {
    BlockPos::new(
        pos.x.floor() as i32,
        pos.y.floor() as i32,
        pos.z.floor() as i32,
    )
}

/// Returns `true` if `pos` holds a solid full cube in the resident chunks.
///
/// A free function (not a `&self` method) so it can be called while the shard's
/// entity store is mutably borrowed: it takes only `&LoadedChunkMap`, a field
/// disjoint from `entities`. A non-resident chunk reads as air (not solid), so an
/// entity falling past the shard edge is never grounded by a chunk this shard
/// does not own.
fn is_solid_block(chunks: &LoadedChunkMap, pos: BlockPos) -> bool {
    let Some(state) = block_state_at(chunks, pos) else {
        return false;
    };
    if state.is_air() {
        return false;
    }
    state_id_to_block_name(state.as_u32())
        .and_then(block_metadata)
        .is_some_and(|m| m.is_solid_cube)
}

/// The block-state at `pos` in the resident chunks, or `None` when that chunk is
/// not resident on this shard. A resident air cell reads as `Some(`[`BlockStateId::AIR`]`)`.
///
/// The shared read behind [`is_solid_block`] and [`is_non_replaceable_block`]: a
/// free function taking only `&LoadedChunkMap`, so both can run while `entities` is
/// mutably borrowed (disjoint fields).
fn block_state_at(chunks: &LoadedChunkMap, pos: BlockPos) -> Option<BlockStateId> {
    chunks
        .get(pos.to_chunk_pos())
        .and_then(|c| c.get_block(pos))
}

/// Returns `true` if `pos` holds a non-air, non-replaceable block — the vanilla
/// `!FallingBlock::isFree` test that drives both falling-block rules.
///
/// Unlike [`is_solid_block`] (the *collision* test — only a full cube stops a
/// falling entity), this is broader: any real block, full cube or not (slab, fence,
/// redstone, torch, sapling, sign, …), counts. It answers two symmetric questions:
///
/// - **Support** (called on the cell *below* a gravity block): a placed block over
///   such a cell stays put; over air or a replaceable fluid/plant it falls.
/// - **Break** (called on a falling block's *resting cell*): a faller settling into
///   such a cell breaks; into air or a replaceable cell it settles.
///
/// A non-resident chunk reads as free (a block over the shard edge falls into the
/// void; a faller there does not break).
fn is_non_replaceable_block(chunks: &LoadedChunkMap, pos: BlockPos) -> bool {
    let Some(state) = block_state_at(chunks, pos) else {
        return false;
    };
    if state.is_air() {
        return false;
    }
    !state_id_to_block_name(state.as_u32()).is_some_and(is_replaceable)
}

/// Returns `true` if the block at `pos` is a solid support a falling block lands on
/// but breaks against instead of settling atop — one whose collision top is below a
/// full cube (a slab, soul sand, farmland, chest, dripstone tip, …).
///
/// Vanilla decides this from the support's collision shape: a faller comes to rest at
/// the support's collision top, so a top strictly below `1.0` sinks the faller's feet
/// into the support's own (non-replaceable) cell and breaks it, while a full-height
/// top leaves the faller in the clear cell above, where it settles. The height is read
/// per state from the registry ([`collision_top_y`]), so a bottom slab (`0.5`) breaks
/// and a double slab (`1.0`) settles, with no block-name list to maintain. A support
/// with no collision (`0.0`) never stops a faller here — it fell through to the block
/// below, leaving that thin object in the resting cell for the
/// [`is_non_replaceable_block`] test. A non-resident chunk reads as no such support.
fn breaks_on_support(chunks: &LoadedChunkMap, pos: BlockPos) -> bool {
    let Some(state) = block_state_at(chunks, pos) else {
        return false;
    };
    match collision_top_y(state.as_u32()) {
        Some(top) => top > 0.0 && top < 1.0,
        None => false,
    }
}

/// The collision box of a non-player entity: horizontal `half_width` (the box is
/// centred on the entity's x/z) and vertical `height` (measured up from the
/// entity's y, which is its base/feet), in blocks.
///
/// Vanilla sizes: a falling block is a `0.98`-cube, so a `0.49` half-width and
/// `0.98` height. A [`EntityKind::Simple`] entity has no wire type yet, so it
/// uses a small item-like box (`0.125` half-width, `0.25` height) — a non-zero
/// footprint so it collides like a real entity rather than a degenerate point
/// (a zero-size box centred on a grid line overlaps no cell and would fall
/// through). Typed consumers (arrow `0.5`, …) will set their own dims.
fn entity_dimensions(kind: EntityKind) -> (f64, f64) {
    match kind {
        EntityKind::FallingBlock { .. } => (0.49, 0.98),
        EntityKind::Simple => (0.125, 0.25),
    }
}

/// The outcome of sweeping an entity's box through the world for one tick.
struct Swept {
    /// Clamped position after collision resolution.
    position: Vec3,
    /// Velocity with any blocked axis zeroed.
    velocity: Vec3,
    /// `true` if a downward move was stopped by a block (the entity is resting).
    on_ground: bool,
}

/// Sweeps an entity's axis-aligned box from `pos` by `velocity`, resolving
/// collision against every solid world block along the path one axis at a time
/// (Y, then X, then Z — the vanilla order).
///
/// The box spans `[pos.x ± half_width]`, `[pos.y, pos.y + height]`,
/// `[pos.z ± half_width]`; `pos` is the base centre. Unlike a destination-cell
/// check, the sweep clamps against the whole broadphase region the box crosses,
/// so a fast mover (near terminal velocity) can never tunnel through a one-block
/// floor or wall. A blocked axis zeroes its velocity component; a downward stop
/// reports `on_ground`.
///
/// A free function taking only `&LoadedChunkMap` so it runs while `entities` is
/// mutably borrowed (disjoint fields). `solids` is a caller-owned scratch buffer,
/// cleared and refilled here so a per-tick sweep loop reuses one allocation across
/// every entity instead of allocating per call. Deterministic: the broadphase cells
/// are visited in fixed `x,y,z` order and every step is ordered `f64` arithmetic.
fn sweep_move(
    chunks: &LoadedChunkMap,
    solids: &mut Vec<BlockPos>,
    pos: Vec3,
    half_width: f64,
    height: f64,
    velocity: Vec3,
) -> Swept {
    let min = Vec3::new(pos.x - half_width, pos.y, pos.z - half_width);
    let max = Vec3::new(pos.x + half_width, pos.y + height, pos.z + half_width);

    // Broadphase: every solid cell the box could overlap as it sweeps, padded by
    // one so a block flush against the swept region is still considered. The `as
    // i32` floor saturates at the range ends, and the ±1 pad is saturating too, so
    // an absurd coordinate can never overflow into a wrapped bound.
    let x0 = ((min.x + velocity.x.min(0.0)).floor() as i32).saturating_sub(1);
    let x1 = ((max.x + velocity.x.max(0.0)).floor() as i32).saturating_add(1);
    let y0 = ((min.y + velocity.y.min(0.0)).floor() as i32).saturating_sub(1);
    let y1 = ((max.y + velocity.y.max(0.0)).floor() as i32).saturating_add(1);
    let z0 = ((min.z + velocity.z.min(0.0)).floor() as i32).saturating_sub(1);
    let z1 = ((max.z + velocity.z.max(0.0)).floor() as i32).saturating_add(1);
    solids.clear();
    for bx in x0..=x1 {
        for by in y0..=y1 {
            for bz in z0..=z1 {
                let cell = BlockPos::new(bx, by, bz);
                if is_solid_block(chunks, cell) {
                    solids.push(cell);
                }
            }
        }
    }

    // Clip each axis in turn, advancing the box so later axes test the position
    // already resolved on the earlier ones (prevents clipping a corner through).
    let (mut lo, mut hi) = (min, max);
    let mut dy = velocity.y;
    for &c in solids.iter() {
        dy = clip_y(c, lo, hi, dy);
    }
    lo.y += dy;
    hi.y += dy;

    let mut dx = velocity.x;
    for &c in solids.iter() {
        dx = clip_x(c, lo, hi, dx);
    }
    lo.x += dx;
    hi.x += dx;

    let mut dz = velocity.z;
    for &c in solids.iter() {
        dz = clip_z(c, lo, hi, dz);
    }

    // A clip only ever *reduces* an axis's travel toward zero, so a strictly
    // smaller magnitude means that axis hit a block (avoids an exact `==` float
    // compare). A downward Y hit is a landing.
    let blocked_x = dx.abs() < velocity.x.abs();
    let blocked_y = dy.abs() < velocity.y.abs();
    let blocked_z = dz.abs() < velocity.z.abs();
    Swept {
        position: Vec3::new(pos.x + dx, pos.y + dy, pos.z + dz),
        velocity: Vec3::new(
            if blocked_x { 0.0 } else { velocity.x },
            if blocked_y { 0.0 } else { velocity.y },
            if blocked_z { 0.0 } else { velocity.z },
        ),
        on_ground: velocity.y < 0.0 && blocked_y,
    }
}

/// Clamps a Y move `dy` so the box `[lo, hi]` cannot pass through the full-cube
/// block at `cell`. Only acts when the box overlaps the block on X and Z (strict,
/// so a mere edge-touch never blocks vertical motion).
fn clip_y(cell: BlockPos, lo: Vec3, hi: Vec3, dy: f64) -> f64 {
    let (bx, by, bz) = (
        f64::from(cell.x()),
        f64::from(cell.y()),
        f64::from(cell.z()),
    );
    if hi.x > bx && lo.x < bx + 1.0 && hi.z > bz && lo.z < bz + 1.0 {
        if dy > 0.0 && hi.y <= by {
            let d = by - hi.y;
            if d < dy {
                return d;
            }
        } else if dy < 0.0 && lo.y >= by + 1.0 {
            let d = (by + 1.0) - lo.y;
            if d > dy {
                return d;
            }
        }
    }
    dy
}

/// Clamps an X move `dx` against the full-cube block at `cell`. Acts only on a
/// box overlapping the block on Y and Z.
fn clip_x(cell: BlockPos, lo: Vec3, hi: Vec3, dx: f64) -> f64 {
    let (bx, by, bz) = (
        f64::from(cell.x()),
        f64::from(cell.y()),
        f64::from(cell.z()),
    );
    if hi.y > by && lo.y < by + 1.0 && hi.z > bz && lo.z < bz + 1.0 {
        if dx > 0.0 && hi.x <= bx {
            let d = bx - hi.x;
            if d < dx {
                return d;
            }
        } else if dx < 0.0 && lo.x >= bx + 1.0 {
            let d = (bx + 1.0) - lo.x;
            if d > dx {
                return d;
            }
        }
    }
    dx
}

/// Clamps a Z move `dz` against the full-cube block at `cell`. Acts only on a
/// box overlapping the block on X and Y.
fn clip_z(cell: BlockPos, lo: Vec3, hi: Vec3, dz: f64) -> f64 {
    let (bx, by, bz) = (
        f64::from(cell.x()),
        f64::from(cell.y()),
        f64::from(cell.z()),
    );
    if hi.x > bx && lo.x < bx + 1.0 && hi.y > by && lo.y < by + 1.0 {
        if dz > 0.0 && hi.z <= bz {
            let d = bz - hi.z;
            if d < dz {
                return d;
            }
        } else if dz < 0.0 && lo.z >= bz + 1.0 {
            let d = (bz + 1.0) - lo.z;
            if d > dz {
                return d;
            }
        }
    }
    dz
}

/// A [`NeighborQuery`] backed by a shard's resident chunks.
///
/// Resolves a neighbour's block-state id to a name via the registry and reports it
/// as fence-connectable when it is the same fence or a solid full cube. Air,
/// unknown, and non-resident neighbours are not connectable.
struct ShardNeighborQuery<'a> {
    chunks: &'a LoadedChunkMap,
}

impl NeighborQuery for ShardNeighborQuery<'_> {
    fn is_fence_connectable(&self, position: BlockPos, fence_block_name: &str) -> bool {
        let Some(state) = self
            .chunks
            .get(position.to_chunk_pos())
            .and_then(|chunk| chunk.get_block(position))
        else {
            return false;
        };
        if state.is_air() {
            return false;
        }
        let Some(name) = state_id_to_block_name(state.as_u32()) else {
            return false;
        };
        // A fence connects to the same fence or to any solid full cube.
        name == fence_block_name || block_metadata(name).is_some_and(|m| m.is_solid_cube)
    }

    fn block_state_at(&self, position: BlockPos) -> Option<u32> {
        // Resolve the resident neighbour's state for the placement rules that read
        // it (stair auto-corner `shape`, fence-gate `in_wall`). Air and
        // non-resident cells report `None`, matching the trait contract; a
        // neighbour in another shard's chunk is simply unseen (stale corner at the
        // shard edge, healed when that column streams in).
        self.chunks
            .get(position.to_chunk_pos())
            .and_then(|chunk| chunk.get_block(position))
            .filter(|state| !state.is_air())
            .map(BlockStateId::as_u32)
    }
}

/// Maps an [`apply_block_edit`](SimShard::apply_block_edit) result into the
/// [`GameOutput`] the session layer routes, or `None` when nothing should be
/// sent.
///
/// An [`Applied`](MutationResult::Applied) edit becomes a
/// [`GameOutput::BlockChanged`] (broadcast to viewers, acked to the actor). A
/// [`Rejected`](MutationResult::Rejected) edit by a present player becomes a
/// [`GameOutput::BlockChangeRejected`] (a targeted resync + ack to the actor) —
/// including a [`ChunkNotLoaded`](RejectionReason::ChunkNotLoaded) rejection,
/// which only a *present* actor can reach (residency is checked after the actor),
/// so its prediction must be ended rather than ghosted. The single case that
/// emits nothing is an [`ActorAbsent`](RejectionReason::ActorAbsent) rejection:
/// there is no client session to heal. A non-player cause also emits nothing.
fn block_change_output(
    cause: MutationCause,
    sequence: i32,
    position: BlockPos,
    requested_state: BlockStateId,
    result: MutationResult,
) -> Option<GameOutput> {
    match result {
        MutationResult::Applied { new_state } => Some(GameOutput::BlockChanged {
            position,
            state: new_state,
            sequence,
            cause,
        }),
        MutationResult::Rejected {
            reason,
            authoritative_state,
        } => {
            // Only a player edit has an actor to resync; non-player causes
            // (command/plugin/test) have no client to heal this milestone.
            let MutationCause::PlayerCreative { player } = cause else {
                return None;
            };
            // A genuinely absent actor has no session to ack or resync — stay
            // silent. But ChunkNotLoaded is NOT silenced here: apply_block_edit
            // checks ActorAbsent *before* ChunkNotLoaded, so a ChunkNotLoaded
            // rejection means the actor WAS present. Silencing it stranded that
            // client's optimistic prediction as a ghost block; instead emit a
            // rejection so the actor gets the ack (+ best-known resync) that ends
            // the prediction. The real column corrects it when it streams in.
            if matches!(reason, RejectionReason::ActorAbsent) {
                return None;
            }
            Some(GameOutput::BlockChangeRejected {
                player,
                position,
                sequence,
                requested_state,
                authoritative_state,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use ferrumc_math::ChunkPos;
    use ferrumc_storage::InMemoryStore;
    use ferrumc_world::FlatWorldGenerator;

    use super::*;
    use crate::ownership::ShardId;
    use crate::ticket::{ChunkTicket, TicketReason};

    fn player(name: &str) -> PlayerId {
        PlayerId::offline(name)
    }

    fn shard() -> SimShard {
        SimShard::new(ShardPos::new(0, 0))
    }

    /// A spawn-position helper: the default world spawn used across block-edit
    /// tests, comfortably in reach of the flat surface around it.
    fn spawn() -> Vec3 {
        Vec3::new(8.0, 64.0, 8.0)
    }

    /// Builds a shard with `chunk` generated and resident, with its
    /// freshly-generated dirtiness cleared so later assertions see only the
    /// edits the test makes.
    async fn shard_with_loaded_chunk(chunk: ChunkPos) -> SimShard {
        let mut s = shard();
        let store = InMemoryStore::new();
        let generator = FlatWorldGenerator::new();
        s.loaded_chunks_mut()
            .acquire(
                &store,
                &generator,
                chunk,
                ChunkTicket::of(TicketReason::Player),
            )
            .await
            .expect("acquire chunk");
        // A generated chunk is dirty for its initial save; clear that so a later
        // dirty check reflects only the test's own edit.
        let _ = s.loaded_chunks_mut().take_dirty();
        s
    }

    /// Reads the block at `pos` from the resident chunk owning it.
    fn block_at(s: &SimShard, pos: BlockPos) -> Option<BlockStateId> {
        s.loaded_chunks()
            .get(pos.to_chunk_pos())
            .and_then(|c| c.get_block(pos))
    }

    #[test]
    fn new_uses_default_capacity_and_is_empty() {
        let s = shard();
        assert_eq!(s.shard_pos(), ShardPos::new(0, 0));
        assert_eq!(s.inbox_capacity(), 1024);
        assert_eq!(s.inbox_len(), 0);
        assert_eq!(s.player_count(), 0);
        assert!(!s.is_inbox_full());
    }

    #[test]
    fn cross_shard_outbox_rejects_newest_and_returns_it_intact() {
        let mut source = shard();
        let destination =
            ShardId::try_new(WorldId::new(0), DimensionId::new(0), ShardPos::new(1, 0))
                .expect("valid destination");
        for index in 0..CROSS_SHARD_OUTBOX_CAPACITY {
            let intent = CrossShardIntent::new(
                destination,
                CrossShardPayload::ApplyInput(GameInput::PlayerLeave {
                    player: player(&format!("accepted-{index}")),
                }),
            );
            source.emit_cross_shard(intent).expect("bounded slot");
        }

        let rejected = CrossShardIntent::new(
            destination,
            CrossShardPayload::ApplyInput(GameInput::PlayerLeave {
                player: player("rejected-newest"),
            }),
        );
        assert_eq!(source.emit_cross_shard(rejected.clone()), Err(rejected));
        assert_eq!(
            source.take_cross_shard_outbox().len(),
            CROSS_SHARD_OUTBOX_CAPACITY
        );
        assert!(source.take_cross_shard_outbox().is_empty());
    }

    #[test]
    fn enqueue_does_not_mutate_state_until_tick() {
        let mut s = shard();
        let p = player("alice");
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: Vec3::new(1.0, 64.0, 2.0),
        })
        .expect("room");

        // Queued but not applied yet.
        assert_eq!(s.inbox_len(), 1);
        assert_eq!(s.player_count(), 0);
        assert!(!s.contains_player(p));
        assert_eq!(s.player_position(p), None);

        let outputs = s.run_tick();
        assert_eq!(
            outputs,
            vec![GameOutput::PlayerSpawned {
                player: p,
                position: Vec3::new(1.0, 64.0, 2.0)
            }]
        );
        assert_eq!(s.inbox_len(), 0);
        assert_eq!(s.player_count(), 1);
        assert!(s.contains_player(p));
        assert_eq!(s.player_position(p), Some(Vec3::new(1.0, 64.0, 2.0)));
    }

    #[test]
    fn multiple_moves_in_one_tick_coalesce_to_latest() {
        let mut s = shard();
        let p = player("bob");
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: Vec3::ZERO,
        })
        .expect("room");
        s.enqueue(GameInput::PlayerMove {
            player: p,
            position: Some(Vec3::new(5.0, 0.0, 0.0)),
            yaw: None,
            pitch: None,
        })
        .expect("room");
        s.enqueue(GameInput::PlayerMove {
            player: p,
            position: Some(Vec3::new(9.0, 0.0, 0.0)),
            yaw: None,
            pitch: None,
        })
        .expect("room");

        let outputs = s.run_tick();
        // The two moves coalesce: only the latest position is applied, and a
        // single PlayerMoved (after the spawn) is emitted.
        assert_eq!(
            outputs,
            vec![
                GameOutput::PlayerSpawned {
                    player: p,
                    position: Vec3::ZERO
                },
                GameOutput::PlayerMoved {
                    player: p,
                    position: Vec3::new(9.0, 0.0, 0.0),
                    yaw: 0.0,
                    pitch: 0.0,
                    position_changed: true,
                },
            ]
        );
        assert_eq!(s.player_position(p), Some(Vec3::new(9.0, 0.0, 0.0)));
    }

    #[test]
    fn move_with_rotation_stores_and_emits_yaw_pitch() {
        let mut s = shard();
        let p = player("turner");
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: Vec3::ZERO,
        })
        .expect("room");
        let _ = s.run_tick();

        // A position+rotation move stores both and emits a position-changed
        // PlayerMoved carrying the new yaw/pitch.
        s.enqueue(GameInput::PlayerMove {
            player: p,
            position: Some(Vec3::new(3.0, 0.0, 0.0)),
            yaw: Some(90.0),
            pitch: Some(-30.0),
        })
        .expect("room");
        let outputs = s.run_tick();
        assert_eq!(
            outputs,
            vec![GameOutput::PlayerMoved {
                player: p,
                position: Vec3::new(3.0, 0.0, 0.0),
                yaw: 90.0,
                pitch: -30.0,
                position_changed: true,
            }]
        );
    }

    #[test]
    fn rotation_only_move_keeps_position_and_flags_no_position_change() {
        let mut s = shard();
        let p = player("looker");
        let spawn = Vec3::new(8.0, 64.0, 8.0);
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: spawn,
        })
        .expect("room");
        let _ = s.run_tick();

        // A rotation-only move (no position) updates yaw/pitch but leaves the
        // position untouched and reports position_changed = false.
        s.enqueue(GameInput::PlayerMove {
            player: p,
            position: None,
            yaw: Some(45.0),
            pitch: Some(10.0),
        })
        .expect("room");
        let outputs = s.run_tick();
        assert_eq!(
            outputs,
            vec![GameOutput::PlayerMoved {
                player: p,
                position: spawn,
                yaw: 45.0,
                pitch: 10.0,
                position_changed: false,
            }]
        );
        // The stored position is unchanged by a rotation-only move.
        assert_eq!(s.player_position(p), Some(spawn));
    }

    #[test]
    fn position_only_move_leaves_rotation_unchanged() {
        let mut s = shard();
        let p = player("strafer");
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: Vec3::ZERO,
        })
        .expect("room");
        let _ = s.run_tick();

        // First turn in place, then move position-only: the second move must keep
        // the yaw the first one set (a None component leaves the stored value).
        s.enqueue(GameInput::PlayerMove {
            player: p,
            position: None,
            yaw: Some(120.0),
            pitch: Some(5.0),
        })
        .expect("room");
        let _ = s.run_tick();
        s.enqueue(GameInput::PlayerMove {
            player: p,
            position: Some(Vec3::new(1.0, 0.0, 0.0)),
            yaw: None,
            pitch: None,
        })
        .expect("room");
        let outputs = s.run_tick();
        assert_eq!(
            outputs,
            vec![GameOutput::PlayerMoved {
                player: p,
                position: Vec3::new(1.0, 0.0, 0.0),
                yaw: 120.0,
                pitch: 5.0,
                position_changed: true,
            }]
        );
    }

    #[test]
    fn invalid_move_is_rejected_and_emits_a_correction() {
        let mut s = shard();
        let p = player("nan");
        let spawn = Vec3::new(8.0, 64.0, 8.0);
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: spawn,
        })
        .expect("room");
        let _ = s.run_tick();

        // Every flavour of bad coordinate is rejected.
        for bad in [
            Vec3::new(f64::NAN, 64.0, 8.0),
            Vec3::new(8.0, f64::INFINITY, 8.0),
            Vec3::new(8.0, 64.0, f64::NEG_INFINITY),
            Vec3::new(3.0e7 + 1.0, 64.0, 8.0),
            Vec3::new(8.0, 64.0, -3.0e7 - 1.0),
        ] {
            s.enqueue(GameInput::PlayerMove {
                player: p,
                position: Some(bad),
                yaw: None,
                pitch: None,
            })
            .expect("room");
            let outputs = s.run_tick();
            // The rejected move never changes state and yields a snap-back
            // correction to the last accepted position.
            assert_eq!(
                outputs,
                vec![GameOutput::PlayerPositionCorrected {
                    player: p,
                    position: spawn,
                }]
            );
            assert_eq!(s.player_position(p), Some(spawn));
        }
    }

    #[test]
    fn valid_move_supersedes_a_rejected_one_without_correcting() {
        let mut s = shard();
        let p = player("oscar");
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: Vec3::ZERO,
        })
        .expect("room");
        let _ = s.run_tick();

        // A rejected move followed by a valid one: the valid position wins and no
        // correction is emitted (the PlayerMoved is the authoritative update).
        s.enqueue(GameInput::PlayerMove {
            player: p,
            position: Some(Vec3::new(f64::NAN, 0.0, 0.0)),
            yaw: None,
            pitch: None,
        })
        .expect("room");
        s.enqueue(GameInput::PlayerMove {
            player: p,
            position: Some(Vec3::new(3.0, 4.0, 5.0)),
            yaw: None,
            pitch: None,
        })
        .expect("room");
        let outputs = s.run_tick();
        assert_eq!(
            outputs,
            vec![GameOutput::PlayerMoved {
                player: p,
                position: Vec3::new(3.0, 4.0, 5.0),
                yaw: 0.0,
                pitch: 0.0,
                position_changed: true,
            }]
        );
        assert_eq!(s.player_position(p), Some(Vec3::new(3.0, 4.0, 5.0)));
    }

    #[test]
    fn boundary_coordinates_are_accepted() {
        let mut s = shard();
        let p = player("edge");
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: Vec3::ZERO,
        })
        .expect("room");
        let _ = s.run_tick();

        // Exactly at the magnitude limit is in range (inclusive bound).
        let edge = Vec3::new(3.0e7, -3.0e7, 0.0);
        s.enqueue(GameInput::PlayerMove {
            player: p,
            position: Some(edge),
            yaw: None,
            pitch: None,
        })
        .expect("room");
        let outputs = s.run_tick();
        assert_eq!(
            outputs,
            vec![GameOutput::PlayerMoved {
                player: p,
                position: edge,
                yaw: 0.0,
                pitch: 0.0,
                position_changed: true,
            }]
        );
        assert_eq!(s.player_position(p), Some(edge));
    }

    #[test]
    fn leave_cancels_a_pending_move_in_the_same_tick() {
        let mut s = shard();
        let p = player("quinn");
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: Vec3::ZERO,
        })
        .expect("room");
        let _ = s.run_tick();

        s.enqueue(GameInput::PlayerMove {
            player: p,
            position: Some(Vec3::new(2.0, 0.0, 0.0)),
            yaw: None,
            pitch: None,
        })
        .expect("room");
        s.enqueue(GameInput::PlayerLeave { player: p })
            .expect("room");
        // The leave wins: only a despawn, no stale move for a gone player.
        let outputs = s.run_tick();
        assert_eq!(outputs, vec![GameOutput::PlayerDespawned { player: p }]);
        assert_eq!(s.player_count(), 0);
    }

    #[test]
    fn invalid_move_for_absent_player_is_silent() {
        let mut s = shard();
        let ghost = player("ghost");
        s.enqueue(GameInput::PlayerMove {
            player: ghost,
            position: Some(Vec3::new(f64::NAN, 0.0, 0.0)),
            yaw: None,
            pitch: None,
        })
        .expect("room");
        // No player present: no correction, no output at all.
        assert!(s.run_tick().is_empty());
        assert_eq!(s.player_count(), 0);
    }

    #[test]
    fn duplicate_join_is_ignored() {
        let mut s = shard();
        let p = player("carol");
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: Vec3::ZERO,
        })
        .expect("room");
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: Vec3::new(100.0, 0.0, 0.0),
        })
        .expect("room");

        let outputs = s.run_tick();
        // Only the first join spawns; the second is a no-op and the position is
        // unchanged.
        assert_eq!(
            outputs,
            vec![GameOutput::PlayerSpawned {
                player: p,
                position: Vec3::ZERO
            }]
        );
        assert_eq!(s.player_position(p), Some(Vec3::ZERO));
    }

    #[test]
    fn move_and_leave_for_unknown_player_are_ignored() {
        let mut s = shard();
        let ghost = player("ghost");
        s.enqueue(GameInput::PlayerMove {
            player: ghost,
            position: Some(Vec3::new(1.0, 1.0, 1.0)),
            yaw: None,
            pitch: None,
        })
        .expect("room");
        s.enqueue(GameInput::PlayerLeave { player: ghost })
            .expect("room");

        assert!(s.run_tick().is_empty());
        assert_eq!(s.player_count(), 0);
    }

    #[test]
    fn leave_removes_present_player() {
        let mut s = shard();
        let p = player("dave");
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: Vec3::ZERO,
        })
        .expect("room");
        let _ = s.run_tick();
        assert_eq!(s.player_count(), 1);

        s.enqueue(GameInput::PlayerLeave { player: p })
            .expect("room");
        let outputs = s.run_tick();
        assert_eq!(outputs, vec![GameOutput::PlayerDespawned { player: p }]);
        assert_eq!(s.player_count(), 0);
        assert!(!s.contains_player(p));
    }

    #[test]
    fn inbox_rejects_when_full_then_recovers_after_drain() {
        let cap = NonZeroUsize::new(2).expect("nonzero");
        let mut s = SimShard::with_inbox_capacity(ShardPos::new(3, -1), cap);
        let p = player("erin");

        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: Vec3::ZERO,
        })
        .expect("first");
        s.enqueue(GameInput::PlayerMove {
            player: p,
            position: Some(Vec3::new(1.0, 0.0, 0.0)),
            yaw: None,
            pitch: None,
        })
        .expect("second");
        assert!(s.is_inbox_full());

        // Third is rejected with a classified error; inbox is left untouched.
        let err = s
            .enqueue(GameInput::PlayerMove {
                player: p,
                position: Some(Vec3::new(2.0, 0.0, 0.0)),
                yaw: None,
                pitch: None,
            })
            .expect_err("inbox is full");
        assert_eq!(err, SimError::InboxFull { capacity: 2 });
        assert_eq!(s.inbox_len(), 2);

        // Draining at the tick boundary frees the inbox; enqueue works again.
        let outputs = s.run_tick();
        assert_eq!(outputs.len(), 2);
        assert!(!s.is_inbox_full());
        s.enqueue(GameInput::PlayerLeave { player: p })
            .expect("room after drain");
    }

    #[test]
    fn empty_tick_produces_no_outputs() {
        let mut s = shard();
        assert!(s.run_tick().is_empty());
    }

    #[tokio::test]
    async fn break_replaces_block_with_air_marks_dirty_and_emits_change() {
        let chunk = ChunkPos::new(0, 0);
        let mut s = shard_with_loaded_chunk(chunk).await;
        let p = player("breaker");
        let target = BlockPos::new(8, 63, 8); // flat-world grass surface
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: spawn(),
        })
        .expect("room");
        let _ = s.run_tick();
        // The generated surface block starts non-air, and the chunk is clean.
        assert_ne!(block_at(&s, target), Some(BlockStateId::AIR));
        assert!(!s
            .loaded_chunks()
            .get(chunk)
            .expect("resident")
            .dirty_sections()
            .any());

        s.enqueue(GameInput::BlockBreak {
            player: p,
            position: target,
            sequence: 7,
        })
        .expect("room");
        let outputs = s.run_tick();
        assert_eq!(
            outputs,
            vec![GameOutput::BlockChanged {
                position: target,
                state: BlockStateId::AIR,
                sequence: 7,
                cause: MutationCause::PlayerCreative { player: p },
            }]
        );
        // The chunk reflects the break and the owning section is now dirty for
        // BOTH the network mask and the persistence (persist-dirty) mask, since a
        // PlayerCreative edit is a real gameplay mutation.
        assert_eq!(block_at(&s, target), Some(BlockStateId::AIR));
        let resident = s.loaded_chunks().get(chunk).expect("resident");
        assert!(resident.dirty_sections().any());
        assert!(
            resident.persist_dirty_sections().any(),
            "a player break must mark the chunk persist-dirty"
        );
        // The edit is also journaled for the storage worker.
        assert!(s.has_pending_mutations());
    }

    #[tokio::test]
    async fn place_marks_persist_dirty_and_journals_the_mutation() {
        let chunk = ChunkPos::new(0, 0);
        let mut s = shard_with_loaded_chunk(chunk).await;
        let p = player("placer");
        let target = BlockPos::new(8, 65, 8);
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: spawn(),
        })
        .expect("room");
        let _ = s.run_tick();
        assert!(!s.has_pending_mutations());

        s.enqueue(GameInput::BlockPlace {
            player: p,
            position: target,
            sequence: 1,
            state: DEFAULT_PLACED_STATE,
            clicked_face: Direction::Up,
            cursor_position: Vec3::new(0.5, 0.0, 0.5),
            player_yaw: 0.0,
        })
        .expect("room");
        let _ = s.run_tick();

        assert!(s
            .loaded_chunks()
            .get(chunk)
            .expect("resident")
            .persist_dirty_sections()
            .any());
        let mutations = s.take_mutations();
        assert_eq!(mutations.len(), 1);
        assert_eq!(mutations[0].position(), target);
        assert_eq!(mutations[0].new_state(), DEFAULT_PLACED_STATE);
        assert_eq!(mutations[0].old_state(), BlockStateId::AIR);
        // Draining empties the buffer.
        assert!(!s.has_pending_mutations());
    }

    #[tokio::test]
    async fn place_sets_the_default_block_and_emits_change() {
        let chunk = ChunkPos::new(0, 0);
        let mut s = shard_with_loaded_chunk(chunk).await;
        let p = player("placer");
        let target = BlockPos::new(8, 65, 8); // air just above the surface
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: spawn(),
        })
        .expect("room");
        let _ = s.run_tick();
        assert_eq!(block_at(&s, target), Some(BlockStateId::AIR));

        s.enqueue(GameInput::BlockPlace {
            player: p,
            position: target,
            sequence: 12,
            state: DEFAULT_PLACED_STATE,
            clicked_face: Direction::Up,
            cursor_position: Vec3::new(0.5, 0.0, 0.5),
            player_yaw: 0.0,
        })
        .expect("room");
        let outputs = s.run_tick();
        assert_eq!(
            outputs,
            vec![GameOutput::BlockChanged {
                position: target,
                state: DEFAULT_PLACED_STATE,
                sequence: 12,
                cause: MutationCause::PlayerCreative { player: p },
            }]
        );
        assert_eq!(block_at(&s, target), Some(DEFAULT_PLACED_STATE));
    }

    #[tokio::test]
    async fn place_writes_the_threaded_held_block_state_not_a_default() {
        // The held item's resolved block-state is threaded on the input, so a
        // place must write exactly that state — here glass (562), proving the old
        // hardcoded stone default is gone.
        let chunk = ChunkPos::new(0, 0);
        let mut s = shard_with_loaded_chunk(chunk).await;
        let p = player("builder");
        let target = BlockPos::new(8, 65, 8); // air just above the surface
        let glass = BlockStateId::new(562);
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: spawn(),
        })
        .expect("room");
        let _ = s.run_tick();

        s.enqueue(GameInput::BlockPlace {
            player: p,
            position: target,
            sequence: 8,
            state: glass,
            clicked_face: Direction::Up,
            cursor_position: Vec3::new(0.5, 0.0, 0.5),
            player_yaw: 0.0,
        })
        .expect("room");
        let outputs = s.run_tick();
        assert_eq!(
            outputs,
            vec![GameOutput::BlockChanged {
                position: target,
                state: glass,
                sequence: 8,
                cause: MutationCause::PlayerCreative { player: p },
            }]
        );
        assert_eq!(block_at(&s, target), Some(glass));
        assert_ne!(block_at(&s, target), Some(DEFAULT_PLACED_STATE));
    }

    #[tokio::test]
    async fn break_in_unloaded_chunk_by_present_actor_is_rejected_with_resync() {
        // No chunk resident, but the actor IS present: residency is checked after
        // the actor, so this reaches a ChunkNotLoaded rejection. A present actor
        // optimistically predicted the break, so it must be healed (ack + best-known
        // resync) rather than ghosted; the real column corrects it when it streams
        // in. The authoritative state is air (the chunk is absent).
        let mut s = shard();
        let p = player("homeless");
        let target = BlockPos::new(8, 63, 8);
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: spawn(),
        })
        .expect("room");
        let _ = s.run_tick();
        s.enqueue(GameInput::BlockBreak {
            player: p,
            position: target,
            sequence: 1,
        })
        .expect("room");
        assert_eq!(
            s.run_tick(),
            vec![GameOutput::BlockChangeRejected {
                player: p,
                position: target,
                sequence: 1,
                requested_state: BlockStateId::AIR,
                authoritative_state: BlockStateId::AIR,
            }]
        );
    }

    #[tokio::test]
    async fn reject_block_edit_for_a_place_resyncs_air_without_mutating() {
        // A place refused upstream (plugin Deny): the client predicted the held
        // block at an empty cell. RejectBlockEdit reads the authoritative state
        // (air — the cell is empty) and emits a BlockChangeRejected healing the
        // actor to air, writing nothing to the chunk.
        let chunk = ChunkPos::new(0, 0);
        let mut s = shard_with_loaded_chunk(chunk).await;
        let p = player("denied-placer");
        let target = BlockPos::new(8, 65, 8); // air just above the surface
        let glass = BlockStateId::new(562);
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: spawn(),
        })
        .expect("room");
        let _ = s.run_tick();
        assert_eq!(block_at(&s, target), Some(BlockStateId::AIR));

        s.enqueue(GameInput::RejectBlockEdit {
            player: p,
            position: target,
            sequence: 7,
            requested_state: glass,
        })
        .expect("room");
        assert_eq!(
            s.run_tick(),
            vec![GameOutput::BlockChangeRejected {
                player: p,
                position: target,
                sequence: 7,
                requested_state: glass,
                authoritative_state: BlockStateId::AIR,
            }]
        );
        // The world was never touched.
        assert_eq!(block_at(&s, target), Some(BlockStateId::AIR));
    }

    #[tokio::test]
    async fn reject_block_edit_for_a_break_resyncs_the_surface_without_mutating() {
        // A break refused upstream: the client predicted air at the surface.
        // RejectBlockEdit reads the authoritative surface block and emits a
        // BlockChangeRejected healing the actor back to it, writing nothing.
        let chunk = ChunkPos::new(0, 0);
        let mut s = shard_with_loaded_chunk(chunk).await;
        let p = player("denied-breaker");
        let target = BlockPos::new(8, 63, 8); // flat-world grass surface
        let authoritative = block_at(&s, target).expect("resident surface block");
        assert_ne!(authoritative, BlockStateId::AIR);
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: spawn(),
        })
        .expect("room");
        let _ = s.run_tick();

        s.enqueue(GameInput::RejectBlockEdit {
            player: p,
            position: target,
            sequence: 3,
            requested_state: BlockStateId::AIR,
        })
        .expect("room");
        assert_eq!(
            s.run_tick(),
            vec![GameOutput::BlockChangeRejected {
                player: p,
                position: target,
                sequence: 3,
                requested_state: BlockStateId::AIR,
                authoritative_state: authoritative,
            }]
        );
        // The surface block is untouched.
        assert_eq!(block_at(&s, target), Some(authoritative));
    }

    #[test]
    fn set_game_mode_mutates_present_player_and_ignores_absent() {
        let mut s = shard();
        let p = player("modeswitcher");
        let ghost = player("ghost");
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: Vec3::ZERO,
        })
        .expect("room");
        let _ = s.run_tick();
        // The authoritative mode starts at the default and survives a tick.
        assert_eq!(s.player_game_mode(p), Some(GameMode::default()));

        s.enqueue(GameInput::SetGameMode {
            player: p,
            mode: GameMode::Creative,
        })
        .expect("room");
        // Setting an absent player's mode is a silent no-op (no panic, no spawn).
        s.enqueue(GameInput::SetGameMode {
            player: ghost,
            mode: GameMode::Creative,
        })
        .expect("room");
        // A mode change emits no output.
        assert!(s.run_tick().is_empty());
        assert_eq!(s.player_game_mode(p), Some(GameMode::Creative));
        assert_eq!(s.player_game_mode(ghost), None);
        assert_eq!(s.player_count(), 1);
    }

    #[tokio::test]
    async fn edit_out_of_reach_is_rejected_with_a_resync_when_chunk_is_loaded() {
        // Block (100, 63, 8) lives in chunk (6, 0); load exactly that chunk so
        // the only reason to reject is the ~92-block distance from spawn. Because
        // the chunk is resident the authoritative state is readable, so the actor
        // gets a targeted resync rather than silence.
        let far_chunk = ChunkPos::new(6, 0);
        let mut s = shard_with_loaded_chunk(far_chunk).await;
        let p = player("shortarms");
        let target = BlockPos::new(100, 63, 8);
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: spawn(),
        })
        .expect("room");
        let _ = s.run_tick();

        // The authoritative (untouched) surface state the resync must carry.
        let authoritative = block_at(&s, target).expect("resident surface block");
        assert_ne!(authoritative, BlockStateId::AIR);

        s.enqueue(GameInput::BlockBreak {
            player: p,
            position: target,
            sequence: 3,
        })
        .expect("room");
        assert_eq!(
            s.run_tick(),
            vec![GameOutput::BlockChangeRejected {
                player: p,
                position: target,
                sequence: 3,
                requested_state: BlockStateId::AIR,
                authoritative_state: authoritative,
            }]
        );
        // The block is untouched (still the generated surface, not air).
        assert_eq!(block_at(&s, target), Some(authoritative));
    }

    #[tokio::test]
    async fn block_edit_for_absent_player_is_ignored() {
        let chunk = ChunkPos::new(0, 0);
        let mut s = shard_with_loaded_chunk(chunk).await;
        let ghost = player("ghost");
        s.enqueue(GameInput::BlockBreak {
            player: ghost,
            position: BlockPos::new(8, 63, 8),
            sequence: 1,
        })
        .expect("room");
        s.enqueue(GameInput::BlockPlace {
            player: ghost,
            position: BlockPos::new(8, 65, 8),
            sequence: 2,
            state: DEFAULT_PLACED_STATE,
            clicked_face: Direction::Up,
            cursor_position: Vec3::new(0.5, 0.0, 0.5),
            player_yaw: 0.0,
        })
        .expect("room");
        // An absent actor has no session to ack or resync, so nothing is emitted.
        assert!(s.run_tick().is_empty());
    }

    #[tokio::test]
    async fn block_edit_applies_only_at_tick_boundary() {
        let chunk = ChunkPos::new(0, 0);
        let mut s = shard_with_loaded_chunk(chunk).await;
        let p = player("patient");
        let target = BlockPos::new(8, 63, 8);
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: spawn(),
        })
        .expect("room");
        let _ = s.run_tick();

        s.enqueue(GameInput::BlockBreak {
            player: p,
            position: target,
            sequence: 5,
        })
        .expect("room");
        // Enqueued but not yet applied: the block is unchanged until the tick.
        assert_ne!(block_at(&s, target), Some(BlockStateId::AIR));
        let _ = s.run_tick();
        assert_eq!(block_at(&s, target), Some(BlockStateId::AIR));
    }

    // --- placement integration (compute_placement in the place funnel) ---

    /// `oak_log` default (axis=y); the integration tests derive axis/half/facing
    /// from the placement inputs threaded on the place.
    const OAK_LOG: u32 = 137;
    /// `oak_slab` default (type=bottom).
    const OAK_SLAB: u32 = 12054;
    /// `oak_stairs` default (facing=north, half=bottom).
    const OAK_STAIRS: u32 = 2949;
    /// `torch` (single floor state).
    const TORCH: u32 = 2401;
    /// `oak_fence` default (all sides disconnected).
    const OAK_FENCE: u32 = 6027;

    /// Builds a shard with chunk (0,0) resident and `p` joined at spawn.
    async fn shard_with_player(p: PlayerId) -> SimShard {
        let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
        s.enqueue(GameInput::PlayerJoin {
            player: p,
            position: spawn(),
        })
        .expect("room");
        let _ = s.run_tick();
        s
    }

    /// Enqueues one place of `held` at `target` with the given placement inputs and
    /// returns the tick's outputs.
    #[allow(clippy::too_many_arguments)] // a test helper mirroring the place input's fields
    fn place_block(
        s: &mut SimShard,
        p: PlayerId,
        target: BlockPos,
        held: u32,
        face: Direction,
        cursor_y: f64,
        yaw: f32,
        sequence: i32,
    ) -> Vec<GameOutput> {
        s.enqueue(GameInput::BlockPlace {
            player: p,
            position: target,
            sequence,
            state: BlockStateId::new(held),
            clicked_face: face,
            cursor_position: Vec3::new(0.5, cursor_y, 0.5),
            player_yaw: yaw,
        })
        .expect("room");
        s.run_tick()
    }

    #[tokio::test]
    async fn place_log_on_side_face_sets_axis() {
        let p = player("logger");
        let mut s = shard_with_player(p).await;
        let target = BlockPos::new(8, 65, 8);
        // Clicking an east/west face lays the log along the x axis (136), not the
        // default vertical y (137).
        let _ = place_block(&mut s, p, target, OAK_LOG, Direction::East, 0.5, 0.0, 1);
        assert_eq!(block_at(&s, target), Some(BlockStateId::new(136)));
    }

    #[tokio::test]
    async fn place_slab_bottom_or_top_from_cursor() {
        let p = player("slabber");
        let mut s = shard_with_player(p).await;
        // Top-face click -> bottom slab (default 12054).
        let bottom = BlockPos::new(8, 65, 8);
        let _ = place_block(&mut s, p, bottom, OAK_SLAB, Direction::Up, 0.0, 0.0, 1);
        assert_eq!(block_at(&s, bottom), Some(BlockStateId::new(12054)));
        // Side click in the upper half -> top slab (12052).
        let top = BlockPos::new(9, 65, 8);
        let _ = place_block(&mut s, p, top, OAK_SLAB, Direction::North, 0.8, 0.0, 2);
        assert_eq!(block_at(&s, top), Some(BlockStateId::new(12052)));
    }

    #[tokio::test]
    async fn place_stairs_facing_from_yaw_and_half_from_cursor() {
        let p = player("stairer");
        let mut s = shard_with_player(p).await;
        // yaw 180 -> facing north, top-face click -> bottom half: the default 2949.
        let a = BlockPos::new(8, 65, 8);
        let _ = place_block(&mut s, p, a, OAK_STAIRS, Direction::Up, 0.0, 180.0, 1);
        assert_eq!(block_at(&s, a), Some(BlockStateId::new(2949)));
        // yaw 90 -> facing west, bottom-face click -> top half: 2979.
        let b = BlockPos::new(9, 65, 8);
        let _ = place_block(&mut s, p, b, OAK_STAIRS, Direction::Down, 0.0, 90.0, 2);
        assert_eq!(block_at(&s, b), Some(BlockStateId::new(2979)));
    }

    #[tokio::test]
    async fn place_torch_floor_vs_wall() {
        let p = player("torcher");
        let mut s = shard_with_player(p).await;
        // Top-face click keeps the floor torch (2401).
        let floor = BlockPos::new(8, 65, 8);
        let _ = place_block(&mut s, p, floor, TORCH, Direction::Up, 0.5, 0.0, 1);
        assert_eq!(block_at(&s, floor), Some(BlockStateId::new(2401)));
        // North-face click becomes a wall torch facing north (2402).
        let wall = BlockPos::new(9, 65, 8);
        let _ = place_block(&mut s, p, wall, TORCH, Direction::North, 0.5, 0.0, 2);
        assert_eq!(block_at(&s, wall), Some(BlockStateId::new(2402)));
    }

    #[tokio::test]
    async fn set_block_exact_writes_a_rotated_state_verbatim_bypassing_refinement() {
        // A plugin/command exact write of a rotated state must be stored as-is.
        // The contrast place below proves the player path still refines, so the
        // exact path is genuinely bypassing compute_placement (not a no-op).
        let p = player("plugin-actor");
        let mut s = shard_with_player(p).await;
        let exact_target = BlockPos::new(8, 65, 8);
        // oak_log axis=x (136): a neutral place would re-derive this to axis=y.
        s.enqueue(GameInput::SetBlockExact {
            player: p,
            position: exact_target,
            sequence: 1,
            state: BlockStateId::new(136),
        })
        .expect("room");
        let outputs = s.run_tick();
        assert_eq!(
            outputs,
            vec![GameOutput::BlockChanged {
                position: exact_target,
                state: BlockStateId::new(136),
                sequence: 1,
                cause: MutationCause::PlayerCreative { player: p },
            }]
        );
        assert_eq!(block_at(&s, exact_target), Some(BlockStateId::new(136)));

        // Contrast: the player place path refines the SAME held id 136 with a
        // neutral top-face click back to the default vertical axis=y (137).
        let refined_target = BlockPos::new(9, 65, 8);
        let _ = place_block(&mut s, p, refined_target, 136, Direction::Up, 0.5, 0.0, 2);
        assert_eq!(block_at(&s, refined_target), Some(BlockStateId::new(137)));
    }

    #[tokio::test]
    async fn preview_placement_matches_the_applied_state() {
        // preview_placement (off-tick, read-only) must return the same state the
        // BlockPlace tick applies, so the after-hook reports the final state.
        let p = player("previewer");
        let mut s = shard_with_player(p).await;
        let target = BlockPos::new(8, 65, 8);
        // An east-face log preview -> axis=x (136), the rotated state.
        let preview = s.preview_placement(
            BlockStateId::new(OAK_LOG),
            Direction::East,
            Vec3::new(0.5, 0.5, 0.5),
            0.0,
            target,
        );
        assert_eq!(preview, BlockStateId::new(136));
        // Applying the same place yields exactly the previewed state.
        let _ = place_block(&mut s, p, target, OAK_LOG, Direction::East, 0.5, 0.0, 1);
        assert_eq!(block_at(&s, target), Some(preview));
    }

    #[tokio::test]
    async fn preview_placement_falls_back_to_the_held_state_for_a_simple_cube() {
        // A simple cube (stone, 1) has no placement-derived properties, so the
        // preview is the held state unchanged.
        let p = player("cube-previewer");
        let s = shard_with_player(p).await;
        let target = BlockPos::new(8, 65, 8);
        let preview = s.preview_placement(
            BlockStateId::new(1),
            Direction::North,
            Vec3::new(0.5, 0.9, 0.5),
            200.0,
            target,
        );
        assert_eq!(preview, BlockStateId::new(1));
    }

    #[tokio::test]
    async fn place_simple_cube_is_unchanged() {
        let p = player("mason");
        let mut s = shard_with_player(p).await;
        let target = BlockPos::new(8, 65, 8);
        // Stone (1) is a simple cube: the placement inputs never alter it.
        let _ = place_block(&mut s, p, target, 1, Direction::North, 0.9, 200.0, 1);
        assert_eq!(block_at(&s, target), Some(BlockStateId::new(1)));
    }

    #[tokio::test]
    async fn place_fence_connects_to_neighbor_and_broadcasts_the_update() {
        let p = player("fencer");
        let mut s = shard_with_player(p).await;

        // Place an isolated fence at A: all sides disconnected (6027).
        let a = BlockPos::new(8, 65, 8);
        let _ = place_block(&mut s, p, a, OAK_FENCE, Direction::Up, 0.5, 0.0, 1);
        assert_eq!(block_at(&s, a), Some(BlockStateId::new(6027)));

        // Place a second fence at B, one east of A. B connects west to A (6026),
        // and A is recomputed to connect east to B (6011).
        let b = BlockPos::new(9, 65, 8);
        let outputs = place_block(&mut s, p, b, OAK_FENCE, Direction::Up, 0.5, 0.0, 2);
        assert_eq!(block_at(&s, b), Some(BlockStateId::new(6026)));
        assert_eq!(block_at(&s, a), Some(BlockStateId::new(6011)));

        // The actor's own place is acked (PlayerCreative); the neighbour update at A
        // is broadcast under a non-acking Command cause.
        assert!(outputs.contains(&GameOutput::BlockChanged {
            position: b,
            state: BlockStateId::new(6026),
            sequence: 2,
            cause: MutationCause::PlayerCreative { player: p },
        }));
        assert!(outputs.contains(&GameOutput::BlockChanged {
            position: a,
            state: BlockStateId::new(6011),
            sequence: 0,
            cause: MutationCause::Command,
        }));
    }

    // --- multi-block placement, merges, and waterlogging (Part B apply path) ---

    /// `oak_door` default (facing=north, lower, hinge=left, open/powered=false).
    const OAK_DOOR: u32 = 4697;
    /// `white_bed` default (facing=north, occupied=false, part=foot).
    const WHITE_BED: u32 = 1734;
    /// A still `water` source (level=0), the one fluid a placement may replace.
    const WATER_SOURCE: u32 = 86;

    /// Directly writes `state` at `pos` in the resident chunk — test scaffolding for
    /// a pre-existing world block (a slab to merge onto, a water source, or an
    /// obstruction), bypassing the placement funnel.
    fn set_world_block(s: &mut SimShard, pos: BlockPos, state: u32) {
        s.loaded_chunks_mut()
            .get_mut(pos.to_chunk_pos())
            .expect("resident chunk")
            .set_block(pos, BlockStateId::new(state))
            .expect("in-range y");
    }

    #[tokio::test]
    async fn place_door_writes_lower_and_upper_halves() {
        let p = player("carpenter");
        let mut s = shard_with_player(p).await;
        let lower = BlockPos::new(8, 65, 8);
        let upper = BlockPos::new(8, 66, 8);
        // yaw 0 -> facing south; no neighbours -> hinge right.
        let outputs = place_block(&mut s, p, lower, OAK_DOOR, Direction::Up, 0.5, 0.0, 1);
        assert_eq!(block_at(&s, lower), Some(BlockStateId::new(4717))); // south, lower, hinge=right
        assert_eq!(block_at(&s, upper), Some(BlockStateId::new(4709))); // south, upper, hinge=right
                                                                        // The lower half is acked to the placer; the upper half is broadcast-only.
        assert!(outputs.contains(&GameOutput::BlockChanged {
            position: lower,
            state: BlockStateId::new(4717),
            sequence: 1,
            cause: MutationCause::PlayerCreative { player: p },
        }));
        assert!(outputs.contains(&GameOutput::BlockChanged {
            position: upper,
            state: BlockStateId::new(4709),
            sequence: 1,
            cause: MutationCause::Command,
        }));
    }

    #[tokio::test]
    async fn place_bed_writes_foot_and_head() {
        let p = player("sleeper");
        let mut s = shard_with_player(p).await;
        let foot = BlockPos::new(8, 65, 8);
        let head = BlockPos::new(8, 65, 9); // one cell south (yaw 0 -> facing south)
        let _ = place_block(&mut s, p, foot, WHITE_BED, Direction::Up, 0.5, 0.0, 1);
        assert_eq!(block_at(&s, foot), Some(BlockStateId::new(1738))); // south, foot
        assert_eq!(block_at(&s, head), Some(BlockStateId::new(1737))); // south, head
    }

    #[tokio::test]
    async fn place_door_with_obstructed_upper_rejects_whole_placement() {
        let p = player("blocked-carpenter");
        let mut s = shard_with_player(p).await;
        let lower = BlockPos::new(8, 65, 8);
        let upper = BlockPos::new(8, 66, 8);
        // A ceiling (stone) sits where the upper half would go.
        set_world_block(&mut s, upper, 1);
        let outputs = place_block(&mut s, p, lower, OAK_DOOR, Direction::Up, 0.5, 0.0, 1);
        // Nothing placed: no half-door. The lower cell stays air; the ceiling stands.
        assert_eq!(block_at(&s, lower), Some(BlockStateId::AIR));
        assert_eq!(block_at(&s, upper), Some(BlockStateId::new(1)));
        // The actor is healed: the predicted lower cell resyncs to its real air state.
        assert_eq!(
            outputs,
            vec![GameOutput::BlockChangeRejected {
                player: p,
                position: lower,
                sequence: 1,
                requested_state: BlockStateId::new(OAK_DOOR),
                authoritative_state: BlockStateId::AIR,
            }]
        );
    }

    #[tokio::test]
    async fn place_bed_with_obstructed_head_rejects_whole_placement() {
        let p = player("blocked-sleeper");
        let mut s = shard_with_player(p).await;
        let foot = BlockPos::new(8, 65, 8);
        let head = BlockPos::new(8, 65, 9);
        // A wall (stone) sits where the head would go.
        set_world_block(&mut s, head, 1);
        let outputs = place_block(&mut s, p, foot, WHITE_BED, Direction::Up, 0.5, 0.0, 1);
        assert_eq!(block_at(&s, foot), Some(BlockStateId::AIR));
        assert_eq!(block_at(&s, head), Some(BlockStateId::new(1)));
        assert_eq!(
            outputs,
            vec![GameOutput::BlockChangeRejected {
                player: p,
                position: foot,
                sequence: 1,
                requested_state: BlockStateId::new(WHITE_BED),
                authoritative_state: BlockStateId::AIR,
            }]
        );
    }

    #[tokio::test]
    async fn place_slab_merges_into_a_double_at_the_clicked_cell() {
        let p = player("slab-merger");
        let mut s = shard_with_player(p).await;
        // A bottom slab already sits at the clicked cell.
        let clicked = BlockPos::new(8, 65, 8);
        set_world_block(&mut s, clicked, OAK_SLAB); // 12054 (bottom)
                                                    // The client clicks its top face; the place target steps up to (8,66,8).
        let stepped = BlockPos::new(8, 66, 8);
        let outputs = place_block(&mut s, p, stepped, OAK_SLAB, Direction::Up, 0.5, 0.0, 1);
        // The merge relocates the write onto the clicked cell: it becomes a double.
        assert_eq!(block_at(&s, clicked), Some(BlockStateId::new(12056))); // double
                                                                           // The stepped cell stays empty — no second slab is placed there.
        assert_eq!(block_at(&s, stepped), Some(BlockStateId::AIR));
        // The double is acked to the placer at the clicked cell, not the stepped one.
        assert!(outputs.contains(&GameOutput::BlockChanged {
            position: clicked,
            state: BlockStateId::new(12056),
            sequence: 1,
            cause: MutationCause::PlayerCreative { player: p },
        }));
    }

    #[tokio::test]
    async fn place_slab_into_water_source_applies_waterlogged() {
        let p = player("diver");
        let mut s = shard_with_player(p).await;
        let target = BlockPos::new(8, 65, 8);
        // A still water source occupies the target cell (a replaceable fluid).
        set_world_block(&mut s, target, WATER_SOURCE);
        // A bottom slab placed into it becomes waterlogged (12053), replacing water.
        let _ = place_block(&mut s, p, target, OAK_SLAB, Direction::Up, 0.0, 0.0, 1);
        assert_eq!(block_at(&s, target), Some(BlockStateId::new(12053))); // bottom + waterlogged
    }

    #[tokio::test]
    async fn place_into_a_solid_block_is_rejected() {
        // The replaceability gate keeps the usual "cannot place into a solid" rule:
        // a non-air, non-water target refuses the placement and heals the actor.
        let p = player("overwriter");
        let mut s = shard_with_player(p).await;
        let target = BlockPos::new(8, 65, 8);
        set_world_block(&mut s, target, 1); // stone already there
        let outputs = place_block(&mut s, p, target, OAK_SLAB, Direction::Up, 0.0, 0.0, 1);
        assert_eq!(
            block_at(&s, target),
            Some(BlockStateId::new(1)),
            "solid untouched"
        );
        assert_eq!(
            outputs,
            vec![GameOutput::BlockChangeRejected {
                player: p,
                position: target,
                sequence: 1,
                requested_state: BlockStateId::new(OAK_SLAB),
                authoritative_state: BlockStateId::new(1),
            }]
        );
    }

    /// Enqueues and applies a single region edit, returning the tick's outputs.
    fn region_edit(
        s: &mut SimShard,
        player: PlayerId,
        a: BlockPos,
        b: BlockPos,
        op: RegionOp,
    ) -> Vec<GameOutput> {
        s.enqueue(GameInput::RegionEdit {
            player,
            region: Cuboid::new(a, b),
            op,
        })
        .expect("room");
        s.run_tick()
    }

    /// Enqueues and applies a single region undo, returning the tick's outputs.
    fn region_undo(s: &mut SimShard, player: PlayerId) -> Vec<GameOutput> {
        s.enqueue(GameInput::RegionUndo { player }).expect("room");
        s.run_tick()
    }

    #[tokio::test]
    async fn region_fill_sets_every_cell_and_broadcasts_under_command_cause() {
        let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
        let p = player("builder");
        let stone = BlockStateId::new(1);
        // A 3x2x3 = 18-cell cuboid of air just above the flat surface.
        let (a, b) = (BlockPos::new(2, 64, 2), BlockPos::new(4, 65, 4));
        let outputs = region_edit(&mut s, p, a, b, RegionOp::Fill { state: stone });
        // Every cell changed air -> stone: one broadcast BlockChanged per cell, all
        // under the non-acking Command cause (sequence 0).
        assert_eq!(outputs.len(), 18);
        assert!(outputs.iter().all(|o| matches!(
            o,
            GameOutput::BlockChanged { state, sequence: 0, cause: MutationCause::Command, .. }
                if *state == stone
        )));
        // Spot-check several positions, including both corners.
        assert_eq!(block_at(&s, a), Some(stone));
        assert_eq!(block_at(&s, b), Some(stone));
        assert_eq!(block_at(&s, BlockPos::new(3, 64, 3)), Some(stone));
        // The edit persisted (overlay marked) and journaled.
        assert!(s.has_pending_mutations());
    }

    #[tokio::test]
    async fn region_replace_changes_only_matching_cells() {
        let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
        let p = player("editor");
        let stone = BlockStateId::new(1);
        let dirt = BlockStateId::new(10);
        let glass = BlockStateId::new(562);
        let (a, b) = (BlockPos::new(2, 64, 2), BlockPos::new(4, 65, 4)); // 18 cells
        let _ = region_edit(&mut s, p, a, b, RegionOp::Fill { state: stone });
        // Punch one cell to glass so it is a non-match for the replace below.
        let odd = BlockPos::new(3, 64, 3);
        let _ = region_edit(&mut s, p, odd, odd, RegionOp::Fill { state: glass });
        assert_eq!(block_at(&s, odd), Some(glass));

        // Replace stone -> dirt over the whole region: the 17 stone cells change,
        // the single glass cell is left untouched.
        let outputs = region_edit(
            &mut s,
            p,
            a,
            b,
            RegionOp::Replace {
                from: stone,
                to: dirt,
            },
        );
        assert_eq!(outputs.len(), 17, "only the 17 stone cells change");
        assert_eq!(block_at(&s, odd), Some(glass), "a non-match is untouched");
        assert_eq!(block_at(&s, a), Some(dirt));
        assert_eq!(block_at(&s, b), Some(dirt));
    }

    #[tokio::test]
    async fn region_undo_restores_the_prior_state() {
        let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
        let p = player("undoer");
        let stone = BlockStateId::new(1);
        let (a, b) = (BlockPos::new(2, 64, 2), BlockPos::new(4, 65, 4));
        // Capture originals (all air above the surface).
        let sample = [a, b, BlockPos::new(3, 65, 3)];
        let before: Vec<_> = sample.iter().map(|&pos| block_at(&s, pos)).collect();
        assert!(before.iter().all(|state| *state == Some(BlockStateId::AIR)));

        let _ = region_edit(&mut s, p, a, b, RegionOp::Fill { state: stone });
        assert!(sample.iter().all(|&pos| block_at(&s, pos) == Some(stone)));

        let outputs = region_undo(&mut s, p);
        assert_eq!(
            outputs.len(),
            18,
            "every changed cell is restored and rebroadcast"
        );
        for (&pos, original) in sample.iter().zip(before) {
            assert_eq!(
                block_at(&s, pos),
                original,
                "cell restored to its prior state"
            );
        }
        // History is now empty: a second undo is a silent no-op.
        assert!(region_undo(&mut s, p).is_empty());
    }

    #[tokio::test]
    async fn region_edit_over_the_volume_cap_is_rejected() {
        let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
        s.set_region_limits(RegionLimits {
            max_volume: 8,
            ..RegionLimits::default()
        });
        let p = player("flooder");
        let stone = BlockStateId::new(1);
        // A 3x2x3 = 18-cell cuboid exceeds the cap of 8: nothing changes, nothing
        // is broadcast, and no undo entry is recorded.
        let (a, b) = (BlockPos::new(2, 64, 2), BlockPos::new(4, 65, 4));
        let outputs = region_edit(&mut s, p, a, b, RegionOp::Fill { state: stone });
        assert!(outputs.is_empty());
        assert_eq!(block_at(&s, a), Some(BlockStateId::AIR));
        assert_eq!(block_at(&s, b), Some(BlockStateId::AIR));
        assert!(!s.has_pending_mutations());
        // With nothing recorded, /undo does nothing.
        assert!(region_undo(&mut s, p).is_empty());
    }

    #[tokio::test]
    async fn region_undo_history_is_bounded_and_evicts_oldest() {
        let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
        s.set_region_limits(RegionLimits {
            max_undo_entries: 2,
            ..RegionLimits::default()
        });
        let p = player("historian");
        let stone = BlockStateId::new(1);
        // Three separate single-cell fills -> three undo entries, but only the last
        // two are retained (cap = 2); the first is evicted.
        let cells = [
            BlockPos::new(2, 64, 2),
            BlockPos::new(3, 64, 2),
            BlockPos::new(4, 64, 2),
        ];
        for &c in &cells {
            let _ = region_edit(&mut s, p, c, c, RegionOp::Fill { state: stone });
        }
        assert!(cells.iter().all(|&c| block_at(&s, c) == Some(stone)));

        // Two undos restore the two newest edits; a third finds nothing.
        assert_eq!(region_undo(&mut s, p).len(), 1);
        assert_eq!(region_undo(&mut s, p).len(), 1);
        assert!(region_undo(&mut s, p).is_empty());

        // The oldest edit's entry was evicted, so its cell stays changed; the other
        // two were restored to air.
        assert_eq!(
            block_at(&s, cells[0]),
            Some(stone),
            "evicted edit is not undone"
        );
        assert_eq!(block_at(&s, cells[1]), Some(BlockStateId::AIR));
        assert_eq!(block_at(&s, cells[2]), Some(BlockStateId::AIR));
    }

    /// The default `oak_sign` block-state id from the pinned registry.
    fn oak_sign() -> u32 {
        ferrumc_registry::block_state::block_default_state("oak_sign")
            .expect("oak_sign in registry")
    }

    /// Reads the sign block-entity at `pos`, panicking if there is none.
    fn sign_at(s: &SimShard, pos: BlockPos) -> Sign {
        let chunk = s.loaded_chunks().get(pos.to_chunk_pos()).expect("resident");
        match chunk.block_entity(pos) {
            Some(BlockEntity::Sign(sign)) => sign.clone(),
            other => panic!("expected a sign block-entity, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn placing_a_sign_creates_block_entity_and_opens_the_editor() {
        let p = player("signer");
        let mut s = shard_with_player(p).await;
        let target = BlockPos::new(8, 65, 8);

        let outputs = place_block(&mut s, p, target, oak_sign(), Direction::Up, 1.0, 0.0, 1);

        // The block change is broadcast AND the editor opens for the placer.
        assert!(outputs.iter().any(
            |o| matches!(o, GameOutput::BlockChanged { position, .. } if *position == target)
        ));
        assert!(outputs.contains(&GameOutput::OpenSignEditor {
            player: p,
            position: target,
        }));
        // A blank sign block-entity now exists at the target.
        let sign = sign_at(&s, target);
        assert!(sign.front().lines().iter().all(String::is_empty));
        assert!(sign.back().lines().iter().all(String::is_empty));
    }

    #[tokio::test]
    async fn breaking_a_sign_removes_its_block_entity() {
        let p = player("breaker");
        let mut s = shard_with_player(p).await;
        let target = BlockPos::new(8, 65, 8);
        let _ = place_block(&mut s, p, target, oak_sign(), Direction::Up, 1.0, 0.0, 1);
        assert_eq!(
            s.loaded_chunks()
                .get(target.to_chunk_pos())
                .expect("resident")
                .block_entity_count(),
            1
        );

        // Breaking the sign replaces it with air and clears the block-entity.
        s.enqueue(GameInput::BlockBreak {
            player: p,
            position: target,
            sequence: 2,
        })
        .expect("room");
        let _ = s.run_tick();
        assert!(s
            .loaded_chunks()
            .get(target.to_chunk_pos())
            .expect("resident")
            .block_entity(target)
            .is_none());
    }

    #[tokio::test]
    async fn update_sign_round_trips_text_and_emits_sign_updated() {
        let p = player("editor");
        let mut s = shard_with_player(p).await;
        let target = BlockPos::new(8, 65, 8);
        let _ = place_block(&mut s, p, target, oak_sign(), Direction::Up, 1.0, 0.0, 1);

        let lines = [
            "Welcome".to_owned(),
            "to".to_owned(),
            "FerrumC".to_owned(),
            String::new(),
        ];
        s.enqueue(GameInput::UpdateSign {
            player: p,
            position: target,
            is_front: true,
            lines: lines.clone(),
        })
        .expect("room");
        let outputs = s.run_tick();

        // Exactly one SignUpdated carries the full sign with the new front text.
        let signs: Vec<_> = outputs
            .iter()
            .filter_map(|o| match o {
                GameOutput::SignUpdated { position, sign } => Some((*position, sign.clone())),
                _ => None,
            })
            .collect();
        assert_eq!(signs.len(), 1);
        assert_eq!(signs[0].0, target);
        assert_eq!(signs[0].1.front().lines(), &lines);

        // The stored block-entity reflects the edit; the back face stays blank.
        let stored = sign_at(&s, target);
        assert_eq!(stored.front().lines(), &lines);
        assert!(stored.back().lines().iter().all(String::is_empty));
    }

    #[tokio::test]
    async fn update_sign_marks_chunk_persist_dirty() {
        let p = player("editor");
        let mut s = shard_with_player(p).await;
        let target = BlockPos::new(8, 65, 8);
        let _ = place_block(&mut s, p, target, oak_sign(), Direction::Up, 1.0, 0.0, 1);
        // Placement already marked the chunk persist-dirty; drain it so the test
        // isolates the sign-text edit's own persistence signal.
        let _ = s.loaded_chunks_mut().take_persist_dirty(0);
        assert!(!s
            .loaded_chunks()
            .get(target.to_chunk_pos())
            .expect("resident")
            .persist_dirty_sections()
            .any());

        s.enqueue(GameInput::UpdateSign {
            player: p,
            position: target,
            is_front: true,
            lines: [
                "saved".to_owned(),
                String::new(),
                String::new(),
                String::new(),
            ],
        })
        .expect("room");
        let _ = s.run_tick();

        assert!(
            s.loaded_chunks()
                .get(target.to_chunk_pos())
                .expect("resident")
                .persist_dirty_sections()
                .any(),
            "a sign-text edit must mark the chunk persist-dirty so the text persists",
        );
    }

    #[tokio::test]
    async fn update_sign_recreates_a_missing_block_entity_for_a_reloaded_sign() {
        let p = player("editor");
        let mut s = shard_with_player(p).await;
        let target = BlockPos::new(8, 65, 8);

        // Simulate a reloaded sign: the sign BLOCK is present but its block-entity
        // was dropped on chunk unload. `set_world_block` writes the state directly,
        // bypassing the placement funnel that would otherwise create the entity.
        set_world_block(&mut s, target, oak_sign());
        assert!(s
            .loaded_chunks()
            .get(target.to_chunk_pos())
            .expect("resident")
            .block_entity(target)
            .is_none());

        let lines = [
            "Back".to_owned(),
            "from".to_owned(),
            "reload".to_owned(),
            String::new(),
        ];
        s.enqueue(GameInput::UpdateSign {
            player: p,
            position: target,
            is_front: true,
            lines: lines.clone(),
        })
        .expect("room");
        let outputs = s.run_tick();

        // The edit recreates the block-entity and applies the text rather than
        // being a silent no-op: exactly one SignUpdated carries the new front text.
        let signs: Vec<_> = outputs
            .iter()
            .filter_map(|o| match o {
                GameOutput::SignUpdated { position, sign } => Some((*position, sign.clone())),
                _ => None,
            })
            .collect();
        assert_eq!(signs.len(), 1);
        assert_eq!(signs[0].0, target);
        assert_eq!(signs[0].1.front().lines(), &lines);

        // The recreated block-entity is stored and reflects the edit.
        let stored = sign_at(&s, target);
        assert_eq!(stored.front().lines(), &lines);
        assert!(stored.back().lines().iter().all(String::is_empty));
    }

    #[tokio::test]
    async fn update_sign_is_rejected_when_no_sign_out_of_reach_or_absent() {
        let p = player("validator");
        let mut s = shard_with_player(p).await;
        let target = BlockPos::new(8, 65, 8);
        let lines = ["x".to_owned(), String::new(), String::new(), String::new()];

        // 1. No sign block-entity at the target yet -> silent no-op.
        s.enqueue(GameInput::UpdateSign {
            player: p,
            position: target,
            is_front: true,
            lines: lines.clone(),
        })
        .expect("room");
        assert!(s.run_tick().is_empty());

        // Place a sign so the position now has a sign block-entity.
        let _ = place_block(&mut s, p, target, oak_sign(), Direction::Up, 1.0, 0.0, 1);

        // 2. Move the editor far out of reach, then try to edit -> rejected.
        s.enqueue(GameInput::PlayerMove {
            player: p,
            position: Some(Vec3::new(8.0, 64.0, 100.0)),
            yaw: None,
            pitch: None,
        })
        .expect("room");
        let _ = s.run_tick();
        s.enqueue(GameInput::UpdateSign {
            player: p,
            position: target,
            is_front: true,
            lines: lines.clone(),
        })
        .expect("room");
        assert!(!s
            .run_tick()
            .iter()
            .any(|o| matches!(o, GameOutput::SignUpdated { .. })));

        // 3. An absent player editing the existing sign -> no output.
        let ghost = player("ghost");
        s.enqueue(GameInput::UpdateSign {
            player: ghost,
            position: target,
            is_front: true,
            lines,
        })
        .expect("room");
        assert!(!s
            .run_tick()
            .iter()
            .any(|o| matches!(o, GameOutput::SignUpdated { .. })));

        // The sign's front text was never modified by any rejected edit.
        assert!(sign_at(&s, target)
            .front()
            .lines()
            .iter()
            .all(String::is_empty));
    }

    #[tokio::test]
    async fn region_fill_and_undo_apply_across_ticks_under_the_budget() {
        let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
        // A tiny per-tick budget forces a multi-tick application; the volume cap
        // stays at its default so the 18-cell region is accepted.
        s.set_region_limits(RegionLimits {
            max_blocks_per_tick: 4,
            ..RegionLimits::default()
        });
        let p = player("slowbuilder");
        let stone = BlockStateId::new(1);
        let (a, b) = (BlockPos::new(2, 64, 2), BlockPos::new(4, 65, 4)); // 18 air cells

        // The first tick enqueues the edit and applies only the budget of 4 cells.
        let first = region_edit(&mut s, p, a, b, RegionOp::Fill { state: stone });
        assert_eq!(
            first.len(),
            4,
            "only the per-tick budget applies on tick one"
        );
        assert_ne!(
            block_at(&s, b),
            Some(stone),
            "the far corner is not filled yet"
        );

        // Later ticks drain the rest (4, 4, 4, then the final 2): 5 ticks, 18 cells.
        let mut applied = first.len();
        let mut ticks = 1;
        while applied < 18 {
            let out = s.run_tick();
            assert!(out.len() <= 4, "a tick never exceeds the budget");
            applied += out.len();
            ticks += 1;
        }
        assert_eq!(applied, 18);
        assert_eq!(ticks, 5, "18 cells at 4/tick takes 5 ticks");
        assert_eq!(block_at(&s, a), Some(stone));
        assert_eq!(block_at(&s, b), Some(stone));

        // The whole edit undoes as one logical op, also budgeted across ticks.
        s.enqueue(GameInput::RegionUndo { player: p })
            .expect("room");
        let mut restored = 0;
        loop {
            let out = s.run_tick();
            if out.is_empty() {
                break;
            }
            assert!(out.len() <= 4, "undo also respects the budget");
            restored += out.len();
        }
        assert_eq!(restored, 18, "undo restores all 18 cells across ticks");
        assert_eq!(block_at(&s, a), Some(BlockStateId::AIR));
        assert_eq!(block_at(&s, b), Some(BlockStateId::AIR));
    }

    /// The default `chest` block-state id from the pinned registry.
    fn chest_state() -> u32 {
        ferrumc_registry::block_state::block_default_state("chest").expect("chest in registry")
    }

    /// Builds a present stack of `count` `item_name`s with no components.
    fn stack(item_name: &str, count: u8) -> ItemStack {
        ItemStack::new(
            ferrumc_items::ItemId::from_name(item_name).expect("item in registry"),
            std::num::NonZeroU8::new(count).expect("non-zero count"),
            ferrumc_items::ComponentPatch::empty(),
        )
    }

    #[tokio::test]
    async fn placing_a_chest_creates_an_empty_container() {
        let p = player("chester");
        let mut s = shard_with_player(p).await;
        let target = BlockPos::new(8, 65, 8);
        let _ = place_block(&mut s, p, target, chest_state(), Direction::Up, 1.0, 0.0, 1);

        let chunk = s
            .loaded_chunks()
            .get(target.to_chunk_pos())
            .expect("resident");
        match chunk.block_entity(target) {
            Some(BlockEntity::Chest(chest)) => {
                assert_eq!(chest.slots().len(), ferrumc_world::CHEST_SLOTS);
                assert!(chest.slots().iter().all(|slot| slot.item().is_none()));
            }
            other => panic!("expected an empty chest block-entity, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn container_open_validates_chest_reach_and_residency() {
        let p = player("opener");
        let mut s = shard_with_player(p).await;
        let chest = BlockPos::new(8, 65, 8);
        let _ = place_block(&mut s, p, chest, chest_state(), Direction::Up, 1.0, 0.0, 1);

        // A reachable chest opens with a 27-slot empty snapshot.
        let snapshot = s.container_open(p, chest).expect("chest opens");
        assert_eq!(snapshot.len(), ferrumc_world::CHEST_SLOTS);
        assert!(snapshot.iter().all(|slot| slot.item().is_none()));

        // A non-chest block (air next to the chest) does not open.
        assert!(s.container_open(p, BlockPos::new(9, 65, 8)).is_none());
        // An out-of-reach chest does not open (far beyond MAX_REACH).
        assert!(s.container_open(p, BlockPos::new(8, 65, 80)).is_none());
        // An absent player does not open anything.
        assert!(s.container_open(player("ghost"), chest).is_none());
    }

    #[tokio::test]
    async fn container_left_click_put_then_take_conserves_items() {
        let p = player("trader");
        let mut s = shard_with_player(p).await;
        let chest = BlockPos::new(8, 65, 8);
        let _ = place_block(&mut s, p, chest, chest_state(), Direction::Up, 1.0, 0.0, 1);

        // Place a whole cursor stack into empty chest slot 0.
        let cursor = stack("diamond", 5);
        let (cursor, snapshot) = s
            .container_left_click(p, chest, 0, cursor)
            .expect("place into empty slot");
        assert!(cursor.item().is_none(), "cursor emptied after placing");
        assert_eq!(snapshot[0].count(), 5);
        assert_eq!(
            snapshot[0].item(),
            ferrumc_items::ItemId::from_name("diamond")
        );

        // Take the whole stack back onto an empty cursor.
        let (cursor, snapshot) = s
            .container_left_click(p, chest, 0, ItemStack::empty())
            .expect("pick the stack back up");
        assert_eq!(cursor.count(), 5, "no item duplicated or lost");
        assert_eq!(cursor.item(), ferrumc_items::ItemId::from_name("diamond"));
        assert!(
            snapshot[0].item().is_none(),
            "chest slot emptied after pickup"
        );
    }

    // --- Non-player entity store -------------------------------------------
    //
    // Store tests use `gravity == 0.0` to isolate the store mechanics from the
    // physics step: a zero-gravity, zero-velocity entity never moves, so
    // run_tick produces no follow-up EntityMoved to reason about here. Gravity
    // behaviour is covered by the physics section below.

    #[test]
    fn spawn_entity_stores_fields_and_starts_airborne() {
        let mut s = shard();
        let pos = Vec3::new(1.0, 64.0, -2.0);
        let vel = Vec3::new(0.0, -0.04, 0.0);
        let id = s.spawn_entity(pos, vel, 0.0).expect("id available");

        assert!(s.contains_entity(id));
        assert_eq!(s.entity_count(), 1);
        assert_eq!(s.entity_position(id), Some(pos));
        assert_eq!(s.entity_velocity(id), Some(vel));
        // Entities are airborne on spawn: no collision pass has grounded them yet.
        assert_eq!(s.entity_on_ground(id), Some(false));
    }

    #[test]
    fn entity_ids_are_monotonic_starting_at_one() {
        let mut s = shard();
        let a = s.spawn_entity(spawn(), Vec3::ZERO, 0.0).expect("id");
        let b = s.spawn_entity(spawn(), Vec3::ZERO, 0.0).expect("id");
        let c = s.spawn_entity(spawn(), Vec3::ZERO, 0.0).expect("id");
        assert_eq!(a.get(), 1);
        assert_eq!(b.get(), 2);
        assert_eq!(c.get(), 3);
    }

    #[test]
    fn removed_ids_are_not_reused() {
        let mut s = shard();
        let a = s.spawn_entity(spawn(), Vec3::ZERO, 0.0).expect("id");
        assert!(s.remove_entity(a));
        // A removed id must not be handed out again: the next spawn advances.
        let b = s.spawn_entity(spawn(), Vec3::ZERO, 0.0).expect("id");
        assert_ne!(a, b);
        assert_eq!(b.get(), 2);
    }

    #[test]
    fn remove_entity_reports_presence_and_shrinks_the_store() {
        let mut s = shard();
        let id = s.spawn_entity(spawn(), Vec3::ZERO, 0.0).expect("id");
        assert!(s.remove_entity(id));
        assert!(!s.contains_entity(id));
        assert_eq!(s.entity_count(), 0);
        // Removing an absent id is a no-op that reports false.
        assert!(!s.remove_entity(id));
    }

    #[test]
    fn accessors_return_none_for_absent_entities() {
        let s = shard();
        let ghost = EntityId::new(999);
        assert!(!s.contains_entity(ghost));
        assert_eq!(s.entity_position(ghost), None);
        assert_eq!(s.entity_velocity(ghost), None);
        assert_eq!(s.entity_on_ground(ghost), None);
    }

    #[test]
    fn entity_ids_iterate_in_ascending_order() {
        let mut s = shard();
        for _ in 0..5 {
            s.spawn_entity(spawn(), Vec3::ZERO, 0.0).expect("id");
        }
        let ids: Vec<i32> = s.entity_ids().map(EntityId::get).collect();
        assert_eq!(ids, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn spawn_entity_reports_exhaustion_instead_of_wrapping() {
        let mut s = shard();
        // Reserve the last representable id. This spawn uses i32::MAX and then
        // the range is exhausted.
        s.next_entity_id = Some(i32::MAX);
        let last = s.spawn_entity(spawn(), Vec3::ZERO, 0.0).expect("last id");
        assert_eq!(last.get(), i32::MAX);
        // The range is now exhausted: the next spawn fails and leaves the store
        // untouched (only the one entity remains).
        assert_eq!(
            s.spawn_entity(spawn(), Vec3::ZERO, 0.0),
            Err(SimError::EntityIdExhausted)
        );
        assert_eq!(s.entity_count(), 1);
    }

    // --- Entity spawn/despawn via the GameInput boundary -------------------
    //
    // These use `gravity: 0.0` and zero velocity so no EntityMoved is produced;
    // the physics section covers the falling case.

    /// A zero-gravity, zero-velocity spawn input at the default spawn position.
    fn static_spawn() -> GameInput {
        GameInput::SpawnEntity {
            position: spawn(),
            velocity: Vec3::ZERO,
            gravity: 0.0,
        }
    }

    #[test]
    fn spawn_entity_input_emits_output_with_fresh_id_and_populates_store() {
        let mut s = shard();
        let position = Vec3::new(1.0, 64.0, 2.0);
        let velocity = Vec3::ZERO;
        s.enqueue(GameInput::SpawnEntity {
            position,
            velocity,
            gravity: 0.0,
        })
        .expect("inbox has room");
        let outputs = s.run_tick();

        assert_eq!(outputs.len(), 1);
        let GameOutput::EntitySpawned {
            entity,
            position: out_pos,
            velocity: out_vel,
            ..
        } = outputs[0]
        else {
            panic!("expected EntitySpawned, got {:?}", outputs[0]);
        };
        // First id in a fresh shard is FIRST_ENTITY_ID (see const doc).
        assert_eq!(entity.get(), 1);
        assert_eq!(out_pos, position);
        assert_eq!(out_vel, velocity);
        assert!(s.contains_entity(entity));
        assert_eq!(s.entity_count(), 1);
    }

    #[test]
    fn despawn_entity_input_emits_output_and_removes_from_store() {
        let mut s = shard();
        s.enqueue(static_spawn()).expect("room");
        let outputs = s.run_tick();
        let GameOutput::EntitySpawned { entity, .. } = outputs[0] else {
            panic!("expected spawn");
        };

        s.enqueue(GameInput::DespawnEntity { entity })
            .expect("room");
        let outputs = s.run_tick();

        assert_eq!(outputs, vec![GameOutput::EntityDespawned { entity }]);
        assert!(!s.contains_entity(entity));
        assert_eq!(s.entity_count(), 0);
    }

    #[test]
    fn despawn_of_absent_entity_is_silent_no_op() {
        let mut s = shard();
        // Never spawned; despawn must not emit and must not mutate the store.
        s.enqueue(GameInput::DespawnEntity {
            entity: EntityId::new(42),
        })
        .expect("room");
        let outputs = s.run_tick();
        assert!(outputs.is_empty());
        assert_eq!(s.entity_count(), 0);
    }

    #[test]
    fn entity_spawns_emit_in_inbox_fifo_order_with_monotonic_ids() {
        let mut s = shard();
        for x in 0..3 {
            s.enqueue(GameInput::SpawnEntity {
                position: Vec3::new(f64::from(x), 64.0, 0.0),
                velocity: Vec3::ZERO,
                gravity: 0.0,
            })
            .expect("room");
        }
        let outputs = s.run_tick();

        let ids: Vec<i32> = outputs
            .iter()
            .map(|o| match o {
                GameOutput::EntitySpawned { entity, .. } => entity.get(),
                other => panic!("unexpected output {other:?}"),
            })
            .collect();
        // Ids advance monotonically in the exact order the inbox delivered them.
        assert_eq!(ids, vec![1, 2, 3]);
    }

    #[test]
    fn spawned_then_despawned_in_one_tick_emits_both_in_order() {
        let mut s = shard();
        s.enqueue(static_spawn()).expect("room");
        // The client cannot address the entity yet (id not known upstream), but a
        // driver that composes intra-tick spawn+despawn (e.g. a plugin path)
        // still expects deterministic ordering.
        s.enqueue(GameInput::DespawnEntity {
            entity: EntityId::new(1),
        })
        .expect("room");
        let outputs = s.run_tick();

        assert_eq!(outputs.len(), 2);
        assert!(matches!(outputs[0], GameOutput::EntitySpawned { .. }));
        assert!(matches!(outputs[1], GameOutput::EntityDespawned { .. }));
        assert_eq!(s.entity_count(), 0);
    }

    #[test]
    fn enqueue_of_spawn_input_does_not_mutate_store_before_tick() {
        let mut s = shard();
        s.enqueue(static_spawn()).expect("room");
        // Enforces the tick-boundary invariant: the store must not change until
        // run_tick is called.
        assert_eq!(s.entity_count(), 0);
        s.run_tick();
        assert_eq!(s.entity_count(), 1);
    }

    // --- Per-tick gravity, integration, and air drag -----------------------

    #[test]
    fn gravity_accelerates_a_falling_entity_each_tick() {
        let mut s = shard();
        // An item dropped from rest: no initial velocity, item gravity.
        let start = Vec3::new(0.0, 64.0, 0.0);
        s.enqueue(GameInput::SpawnEntity {
            position: start,
            velocity: Vec3::ZERO,
            gravity: crate::physics::GRAVITY_ITEM,
        })
        .expect("room");
        // Tick 1: velocity gains gravity (-0.04), integrates, then drags.
        s.run_tick();

        let id = EntityId::new(1);
        // After one tick: v.y = 0 + (-0.04) = -0.04; y = 64 + (-0.04) = 63.96.
        assert_eq!(
            s.entity_position(id),
            Some(Vec3::new(0.0, 64.0 + crate::physics::GRAVITY_ITEM, 0.0))
        );
        // Velocity was integrated, then decayed by air drag on the y axis.
        let expected_vy = crate::physics::GRAVITY_ITEM * crate::physics::AIR_DRAG_Y;
        assert_eq!(
            s.entity_velocity(id),
            Some(Vec3::new(0.0, expected_vy, 0.0))
        );
    }

    #[test]
    fn zero_gravity_zero_velocity_entity_never_moves() {
        let mut s = shard();
        s.enqueue(static_spawn()).expect("room");
        let outputs = s.run_tick();
        // Only the spawn: nothing accelerates it and nothing to integrate.
        assert_eq!(outputs.len(), 1);
        assert!(matches!(outputs[0], GameOutput::EntitySpawned { .. }));
        // A second, input-free tick still produces no motion.
        assert!(s.run_tick().is_empty());
    }

    #[test]
    fn terminal_velocity_is_a_fixed_point() {
        // Terminal velocity is the fixed point of v -> (v + g)·d. Seeding an
        // entity already at terminal, one tick must leave it (bit-)unchanged.
        // (A free fall would reach the void despawn height long before converging
        // by drag alone, so the fixed-point property is asserted directly.)
        let terminal = crate::physics::GRAVITY_LIVING * crate::physics::AIR_DRAG_Y
            / (1.0 - crate::physics::AIR_DRAG_Y);
        let mut s = shard();
        s.enqueue(GameInput::SpawnEntity {
            position: Vec3::new(0.0, 320.0, 0.0),
            velocity: Vec3::new(0.0, terminal, 0.0),
            gravity: crate::physics::GRAVITY_LIVING,
        })
        .expect("room");
        s.run_tick();
        let vy = s.entity_velocity(EntityId::new(1)).unwrap().y;
        assert!(
            (vy - terminal).abs() < 1e-12,
            "vy {vy} should stay at terminal {terminal}"
        );
    }

    #[test]
    fn heavier_gravity_falls_faster() {
        // A living entity (−0.08) outpaces an item (−0.04) after equal ticks.
        let mut s = shard();
        s.enqueue(GameInput::SpawnEntity {
            position: Vec3::new(0.0, 256.0, 0.0),
            velocity: Vec3::ZERO,
            gravity: crate::physics::GRAVITY_ITEM,
        })
        .expect("room");
        s.enqueue(GameInput::SpawnEntity {
            position: Vec3::new(0.0, 256.0, 0.0),
            velocity: Vec3::ZERO,
            gravity: crate::physics::GRAVITY_LIVING,
        })
        .expect("room");
        for _ in 0..20 {
            s.run_tick();
        }
        let item_y = s.entity_position(EntityId::new(1)).unwrap().y;
        let living_y = s.entity_position(EntityId::new(2)).unwrap().y;
        assert!(
            living_y < item_y,
            "living entity (y={living_y}) should be below item (y={item_y})"
        );
    }

    #[test]
    fn moved_outputs_are_emitted_in_ascending_entity_id_order() {
        let mut s = shard();
        for x in 0..3 {
            s.enqueue(GameInput::SpawnEntity {
                position: Vec3::new(f64::from(x), 64.0, 0.0),
                velocity: Vec3::ZERO,
                gravity: crate::physics::GRAVITY_ITEM,
            })
            .expect("room");
        }
        let outputs = s.run_tick();
        // Filter to just the move outputs; assert they arrive in id order.
        let move_ids: Vec<i32> = outputs
            .iter()
            .filter_map(|o| match o {
                GameOutput::EntityMoved { entity, .. } => Some(entity.get()),
                _ => None,
            })
            .collect();
        assert_eq!(move_ids, vec![1, 2, 3]);
    }

    #[test]
    fn physics_runs_on_empty_input_tick() {
        let mut s = shard();
        s.enqueue(GameInput::SpawnEntity {
            position: Vec3::new(0.0, 64.0, 0.0),
            velocity: Vec3::ZERO,
            gravity: crate::physics::GRAVITY_ITEM,
        })
        .expect("room");
        // Tick 1: spawn + first fall step.
        s.run_tick();
        // Tick 2: no inputs at all — gravity must still act.
        let outputs = s.run_tick();
        assert_eq!(outputs.len(), 1);
        assert!(matches!(outputs[0], GameOutput::EntityMoved { .. }));
    }

    // --- Ground collision (needs a resident chunk) -------------------------
    //
    // The flat generator puts a solid grass block at y=63, so its top face is
    // world y=64.0 — the height a falling entity lands at. Spawns start low
    // enough that per-tick fall speed stays below one block, so the single-block
    // ground check lands cleanly (see apply_entity_physics scope notes on
    // tunneling at high speed).

    #[tokio::test]
    async fn falling_entity_lands_on_solid_ground() {
        let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
        // (8, _, 8) is inside chunk (0,0); start two blocks up so the fall stays
        // slow enough not to tunnel.
        let id = s
            .spawn_entity(
                Vec3::new(8.0, 66.0, 8.0),
                Vec3::ZERO,
                crate::physics::GRAVITY_ITEM,
            )
            .expect("id");
        // Tick until grounded; the bound prevents an infinite loop on a bug.
        let mut grounded = false;
        for _ in 0..200 {
            s.run_tick();
            if s.entity_on_ground(id) == Some(true) {
                grounded = true;
                break;
            }
        }
        assert!(grounded, "entity should have landed within the tick budget");
        // Rests on the grass block's top face; vertical motion is zeroed.
        assert_eq!(s.entity_position(id).map(|p| p.y), Some(64.0));
        assert_eq!(s.entity_velocity(id).map(|v| v.y), Some(0.0));
    }

    #[tokio::test]
    async fn landed_entity_stays_put_and_emits_nothing() {
        let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
        let id = s
            .spawn_entity(
                Vec3::new(8.0, 66.0, 8.0),
                Vec3::ZERO,
                crate::physics::GRAVITY_ITEM,
            )
            .expect("id");
        for _ in 0..200 {
            s.run_tick();
            if s.entity_on_ground(id) == Some(true) {
                break;
            }
        }
        let resting = s.entity_position(id);
        // Once grounded, further ticks neither move it nor emit output.
        for _ in 0..5 {
            let outputs = s.run_tick();
            assert!(outputs.is_empty(), "a resting entity must emit nothing");
        }
        assert_eq!(s.entity_position(id), resting);
    }

    #[tokio::test]
    async fn a_fast_faller_does_not_tunnel_through_a_thin_floor() {
        // Regression for the AABB sweep: an entity moving faster than one block per
        // tick must still be stopped by a one-block-thick floor, not skip past it
        // (a destination-cell-only check would step straight over the ledge).
        let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
        let sand = sand_state();
        // A single solid block floating in the air — the flat world is all air from
        // y=64 up, so this ledge is exactly one block thick.
        let ledge = BlockPos::new(8, 100, 8);
        set_world_block(&mut s, ledge, sand);

        // Drop a falling block from well above with a per-tick velocity over one
        // block.
        let id = s
            .spawn_falling_block(
                Vec3::new(8.5, 110.0, 8.5),
                Vec3::new(0.0, -5.0, 0.0),
                crate::physics::GRAVITY_ITEM,
                BlockStateId::new(sand),
            )
            .expect("id");
        for _ in 0..50 {
            s.run_tick();
            if !s.contains_entity(id) {
                break;
            }
        }
        // It came to rest on the ledge: its block sits at the ledge's top face
        // (y=101), and nothing punched through to a cell below the ledge.
        assert_eq!(
            block_at(&s, BlockPos::new(8, 101, 8)),
            Some(BlockStateId::new(sand)),
            "the fast faller should rest on top of the ledge"
        );
        assert!(
            block_at(&s, BlockPos::new(8, 99, 8)).is_some_and(BlockStateId::is_air),
            "nothing should have tunnelled below the ledge"
        );
    }

    #[tokio::test]
    async fn a_falling_block_breaks_on_any_object_in_its_resting_cell() {
        // Every non-replaceable object in the resting cell breaks the faller, not
        // just torches: saplings, signs, flowers, plates, rails, redstone, …. Each
        // runs on a fresh shard; the object must survive and no block is placed.
        for obj in [
            "torch",
            "oak_sign",
            "oak_wall_sign",
            "oak_sapling",
            "dandelion",
            "wheat",
            "stone_pressure_plate",
            "rail",
            "redstone_wire",
            "lever",
        ] {
            let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
            let sand = sand_state();
            let obj_state = block_metadata(obj)
                .unwrap_or_else(|| panic!("{obj} in registry"))
                .default_state;
            // The object sits on the grass at y=63, occupying (8,64,8) — the resting
            // cell a block falling down column 8 would settle into.
            let cell = BlockPos::new(8, 64, 8);
            set_world_block(&mut s, cell, obj_state);

            let id = s
                .spawn_falling_block(
                    Vec3::new(8.5, 70.0, 8.5),
                    Vec3::ZERO,
                    crate::physics::GRAVITY_ITEM,
                    BlockStateId::new(sand),
                )
                .expect("id");
            for _ in 0..80 {
                s.run_tick();
                if !s.contains_entity(id) {
                    break;
                }
            }
            // Broke instead of settling: gone, object untouched, no sand placed.
            assert!(!s.contains_entity(id), "the block should break on {obj}");
            assert_eq!(
                block_at(&s, cell),
                Some(BlockStateId::new(obj_state)),
                "{obj} survives; the block did not overwrite it"
            );
        }
    }

    #[tokio::test]
    async fn a_falling_block_replaces_water_in_its_resting_cell() {
        let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
        let sand = sand_state();
        // A water source on the grass at (8,64,8): water is not a solid cube, so a
        // block falls straight through it and settles in its cell, replacing it
        // (water counts as replaceable, so the block does not break).
        set_world_block(&mut s, BlockPos::new(8, 64, 8), WATER_SOURCE);
        let id = s
            .spawn_falling_block(
                Vec3::new(8.5, 70.0, 8.5),
                Vec3::ZERO,
                crate::physics::GRAVITY_ITEM,
                BlockStateId::new(sand),
            )
            .expect("id");
        for _ in 0..80 {
            s.run_tick();
            if !s.contains_entity(id) {
                break;
            }
        }
        assert_eq!(
            block_at(&s, BlockPos::new(8, 64, 8)),
            Some(BlockStateId::new(sand)),
            "the block should replace the water it settled into"
        );
    }

    #[tokio::test]
    async fn a_falling_block_breaks_on_a_non_full_support_but_settles_on_a_double_slab() {
        use ferrumc_registry::block_state::compute_state_id;

        // A faller that lands on a solid but non-full-height support (soul sand,
        // farmland, cake, a bottom slab) breaks instead of settling atop it: the
        // support survives and nothing is placed in the air cell above. Each support is
        // paired with the state to place — a uniform block uses its default, the bottom
        // slab an explicit half-height state.
        let bottom_slab = compute_state_id(
            "oak_slab",
            &BTreeMap::from([("type", "bottom"), ("waterlogged", "false")]),
        )
        .expect("bottom slab state");
        let supports = [
            (
                "soul_sand",
                block_metadata("soul_sand").unwrap().default_state,
            ),
            (
                "farmland",
                block_metadata("farmland").unwrap().default_state,
            ),
            ("cake", block_metadata("cake").unwrap().default_state),
            ("bottom_slab", bottom_slab),
        ];
        for (support, support_state) in supports {
            let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
            let sand = sand_state();
            let cell = BlockPos::new(8, 64, 8);
            set_world_block(&mut s, cell, support_state);
            let id = s
                .spawn_falling_block(
                    Vec3::new(8.5, 70.0, 8.5),
                    Vec3::ZERO,
                    crate::physics::GRAVITY_ITEM,
                    BlockStateId::new(sand),
                )
                .expect("id");
            for _ in 0..80 {
                s.run_tick();
                if !s.contains_entity(id) {
                    break;
                }
            }
            assert!(
                !s.contains_entity(id),
                "the block should break landing on {support}"
            );
            assert_eq!(
                block_at(&s, cell),
                Some(BlockStateId::new(support_state)),
                "{support} survives the break"
            );
            assert_eq!(
                block_at(&s, BlockPos::new(8, 65, 8)),
                Some(BlockStateId::AIR),
                "nothing is placed above {support}"
            );
        }

        // A double slab is a genuine full cube, so the faller settles on top of it.
        let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
        let sand = sand_state();
        let double = compute_state_id("oak_slab", &BTreeMap::from([("type", "double")]))
            .expect("double slab state");
        set_world_block(&mut s, BlockPos::new(8, 64, 8), double);
        let id = s
            .spawn_falling_block(
                Vec3::new(8.5, 70.0, 8.5),
                Vec3::ZERO,
                crate::physics::GRAVITY_ITEM,
                BlockStateId::new(sand),
            )
            .expect("id");
        for _ in 0..80 {
            s.run_tick();
            if !s.contains_entity(id) {
                break;
            }
        }
        assert_eq!(
            block_at(&s, BlockPos::new(8, 65, 8)),
            Some(BlockStateId::new(sand)),
            "the block settles on top of a double slab"
        );
    }

    #[tokio::test]
    async fn a_side_torch_does_not_break_a_block_falling_past_it() {
        let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
        let sand = sand_state();
        let torch = block_metadata("torch")
            .expect("torch in registry")
            .default_state;
        // A torch in the *adjacent* column (9,64,8), not in the faller's resting
        // cell (8,64,8): it must not interfere.
        set_world_block(&mut s, BlockPos::new(9, 64, 8), torch);

        let id = s
            .spawn_falling_block(
                Vec3::new(8.5, 70.0, 8.5),
                Vec3::ZERO,
                crate::physics::GRAVITY_ITEM,
                BlockStateId::new(sand),
            )
            .expect("id");
        for _ in 0..80 {
            s.run_tick();
            if !s.contains_entity(id) {
                break;
            }
        }
        // The block settled normally at (8,64,8); the side torch is untouched.
        assert_eq!(
            block_at(&s, BlockPos::new(8, 64, 8)),
            Some(BlockStateId::new(sand)),
            "the block should settle when only a side-column torch is present"
        );
        assert_eq!(
            block_at(&s, BlockPos::new(9, 64, 8)),
            Some(BlockStateId::new(torch)),
            "the side torch is untouched"
        );
    }

    #[tokio::test]
    async fn entity_over_the_void_never_grounds() {
        // A chunk far from (0,0) is not resident, so an entity above unloaded
        // space reads air below it and keeps falling — it never grounds on a
        // chunk this shard does not own.
        let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
        // x=500 is outside the single resident chunk.
        let id = s
            .spawn_entity(
                Vec3::new(500.0, 64.0, 8.0),
                Vec3::ZERO,
                crate::physics::GRAVITY_ITEM,
            )
            .expect("id");
        let start_y = s.entity_position(id).unwrap().y;
        for _ in 0..20 {
            s.run_tick();
        }
        assert_eq!(s.entity_on_ground(id), Some(false));
        assert!(
            s.entity_position(id).unwrap().y < start_y,
            "entity over unloaded space should keep falling"
        );
    }

    #[test]
    fn entity_that_falls_into_the_void_is_despawned() {
        let mut s = shard();
        // Spawn just above the void threshold with a downward velocity; no chunk
        // is resident, so nothing grounds it — it crosses VOID_DESPAWN_Y and is
        // removed rather than falling forever.
        let id = s
            .spawn_entity(
                // Already below the threshold, so the first tick removes it.
                Vec3::new(0.0, super::VOID_DESPAWN_Y - 1.0, 0.0),
                Vec3::new(0.0, -1.0, 0.0),
                crate::physics::GRAVITY_ITEM,
            )
            .expect("id");
        let outputs = s.run_tick();
        assert!(!s.contains_entity(id), "voided entity should be despawned");
        assert!(
            outputs.contains(&GameOutput::EntityDespawned { entity: id }),
            "a void despawn must be reported"
        );
        assert_eq!(s.entity_count(), 0);
    }

    #[tokio::test]
    async fn breaking_the_floor_un_grounds_a_resting_entity() {
        let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
        // Spawn at the cell centre (x/z = 8.5) so the entity's footprint sits
        // wholly inside column 8: breaking the single block below then removes its
        // only support. An integer-x spawn would straddle columns 7 and 8 and stay
        // held up by the neighbour when just one is broken.
        let id = s
            .spawn_entity(
                Vec3::new(8.5, 66.0, 8.5),
                Vec3::ZERO,
                crate::physics::GRAVITY_ITEM,
            )
            .expect("id");
        // Let it land on the grass at y=63 (top face y=64.0).
        for _ in 0..200 {
            s.run_tick();
            if s.entity_on_ground(id) == Some(true) {
                break;
            }
        }
        assert_eq!(s.entity_on_ground(id), Some(true));
        let landed_y = s.entity_position(id).unwrap().y;

        // Remove the supporting grass block (a Command edit needs no actor/reach).
        s.apply_block_edit(
            MutationCause::Command,
            BlockPos::new(8, 63, 8),
            BlockStateId::AIR,
        );

        // Next physics tick: the floor is gone, so the entity un-grounds and
        // falls again.
        s.run_tick();
        assert_eq!(s.entity_on_ground(id), Some(false));
        assert!(
            s.entity_position(id).unwrap().y < landed_y,
            "entity should fall once its floor is broken"
        );
    }

    // --- Falling blocks ----------------------------------------------------

    #[tokio::test]
    async fn falling_block_becomes_a_block_on_landing_and_despawns() {
        let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
        // The cell above the grass (y=63) is air; a falling block resting on the
        // grass lands there.
        let target = BlockPos::new(8, 64, 8);
        assert!(block_at(&s, target).is_some_and(BlockStateId::is_air));

        // Drop a sand-like falling block (any solid state works; reuse OAK_LOG,
        // a known solid full cube in this crate's test constants).
        let carried = BlockStateId::new(OAK_LOG);
        let id = s
            .spawn_falling_block(
                Vec3::new(8.0, 66.0, 8.0),
                Vec3::ZERO,
                crate::physics::GRAVITY_ITEM,
                carried,
            )
            .expect("id");

        // Tick until it lands and converts (the entity despawns on landing).
        let mut converted = false;
        for _ in 0..200 {
            s.run_tick();
            if !s.contains_entity(id) {
                converted = true;
                break;
            }
        }
        assert!(converted, "falling block should land and despawn");
        // The carried block now occupies the resting cell.
        assert_eq!(block_at(&s, target), Some(carried));
        assert_eq!(s.entity_count(), 0);
    }

    #[tokio::test]
    async fn falling_block_restores_block_and_despawns_the_same_tick() {
        // On landing the shard restores the block and despawns the entity in the
        // same tick: a single `BlockChanged` + `EntityDespawned` pair, the block
        // change ordered first.
        let mut s = shard_with_loaded_chunk(ChunkPos::new(0, 0)).await;
        let carried = BlockStateId::new(OAK_LOG);
        let id = s
            .spawn_falling_block(
                Vec3::new(8.0, 66.0, 8.0),
                Vec3::ZERO,
                crate::physics::GRAVITY_ITEM,
                carried,
            )
            .expect("id");

        // Advance to the landing tick (the one that restores the block).
        let mut landing = Vec::new();
        for _ in 0..200 {
            let outputs = s.run_tick();
            if outputs
                .iter()
                .any(|o| matches!(o, GameOutput::BlockChanged { .. }))
            {
                landing = outputs;
                break;
            }
        }
        // The block restore and the despawn share the landing tick, block first.
        let block_idx = landing
            .iter()
            .position(|o| matches!(o, GameOutput::BlockChanged { .. }))
            .expect("a BlockChanged on landing");
        let despawn_idx = landing
            .iter()
            .position(|o| o == &GameOutput::EntityDespawned { entity: id })
            .expect("the entity despawns on the landing tick");
        assert!(
            block_idx < despawn_idx,
            "the block restore precedes the despawn"
        );
        assert!(!s.contains_entity(id), "the entity is gone after it lands");
    }

    // --- Automatic falling-block detection (place / break triggers) --------

    /// The default state id of `sand`, a gravity-affected block in the registry.
    fn sand_state() -> u32 {
        block_metadata("sand")
            .expect("sand in the pinned registry")
            .default_state
    }

    #[tokio::test]
    async fn placing_a_gravity_block_without_support_makes_it_fall_and_settle() {
        let p = player("sandbuilder");
        let mut s = shard_with_player(p).await;
        let sand = sand_state();
        // (8,66,8) has air at (8,65,8) below it — no support.
        let hang = BlockPos::new(8, 66, 8);
        let outputs = place_block(&mut s, p, hang, sand, Direction::Up, 0.0, 0.0, 1);

        // Placed then immediately converted: the cell is air and an entity falls.
        assert!(block_at(&s, hang).is_some_and(BlockStateId::is_air));
        assert_eq!(s.entity_count(), 1);
        // The spawn carries the falling-block kind + the removed block-state, so
        // the session can render `minecraft:falling_block` with the right block.
        assert!(
            outputs.iter().any(|o| matches!(
                o,
                GameOutput::EntitySpawned {
                    kind: SpawnedEntityKind::FallingBlock { block },
                    ..
                } if *block == BlockStateId::new(sand)
            )),
            "placing an unsupported gravity block should spawn a falling-block entity carrying its state"
        );

        // It settles as sand on the grass surface (top face y=64.0 -> cell y=64).
        for _ in 0..200 {
            s.run_tick();
            if s.entity_count() == 0 {
                break;
            }
        }
        assert_eq!(
            block_at(&s, BlockPos::new(8, 64, 8)),
            Some(BlockStateId::new(sand))
        );
    }

    #[tokio::test]
    async fn a_supported_gravity_block_does_not_fall() {
        let p = player("sandbuilder");
        let mut s = shard_with_player(p).await;
        let sand = sand_state();
        // Placed directly on the grass surface (y=63): supported, so it stays.
        place_block(
            &mut s,
            p,
            BlockPos::new(8, 64, 8),
            sand,
            Direction::Up,
            0.0,
            0.0,
            1,
        );
        assert_eq!(
            block_at(&s, BlockPos::new(8, 64, 8)),
            Some(BlockStateId::new(sand))
        );
        assert_eq!(s.entity_count(), 0);
    }

    #[tokio::test]
    async fn a_gravity_block_placed_on_redstone_stays_put() {
        let p = player("sandbuilder");
        let mut s = shard_with_player(p).await;
        let sand = sand_state();
        let redstone = block_metadata("redstone_wire")
            .expect("redstone_wire in registry")
            .default_state;
        // Redstone dust on the grass at y=64. It is not a full cube, but it still
        // supports a block placed on top — a gravity block placed above it must not
        // fall (matching vanilla; the block only breaks if it *lands* on redstone).
        set_world_block(&mut s, BlockPos::new(8, 64, 8), redstone);
        place_block(
            &mut s,
            p,
            BlockPos::new(8, 65, 8),
            sand,
            Direction::Up,
            0.0,
            0.0,
            1,
        );
        assert_eq!(
            block_at(&s, BlockPos::new(8, 65, 8)),
            Some(BlockStateId::new(sand)),
            "the block should rest on the redstone, not fall"
        );
        assert_eq!(s.entity_count(), 0, "no falling entity should spawn");
    }

    #[tokio::test]
    async fn breaking_the_support_makes_the_gravity_block_above_fall() {
        let p = player("digger");
        let mut s = shard_with_player(p).await;
        let sand = sand_state();
        // Sand resting on the grass at y=64 stays put.
        place_block(
            &mut s,
            p,
            BlockPos::new(8, 64, 8),
            sand,
            Direction::Up,
            0.0,
            0.0,
            1,
        );
        assert_eq!(s.entity_count(), 0);

        // Break the grass beneath it: the sand loses support and falls.
        s.enqueue(GameInput::BlockBreak {
            player: p,
            position: BlockPos::new(8, 63, 8),
            sequence: 2,
        })
        .expect("room");
        s.run_tick();
        assert!(block_at(&s, BlockPos::new(8, 64, 8)).is_some_and(BlockStateId::is_air));
        assert_eq!(s.entity_count(), 1);

        // It lands on the dirt at y=62 (top face y=63.0 -> cell y=63).
        for _ in 0..200 {
            s.run_tick();
            if s.entity_count() == 0 {
                break;
            }
        }
        assert_eq!(
            block_at(&s, BlockPos::new(8, 63, 8)),
            Some(BlockStateId::new(sand))
        );
    }

    #[tokio::test]
    async fn chest_item_move_marks_chunk_persist_dirty_and_a_refused_click_does_not() {
        let p = player("trader");
        let mut s = shard_with_player(p).await;
        let chest = BlockPos::new(8, 65, 8);
        let _ = place_block(&mut s, p, chest, chest_state(), Direction::Up, 1.0, 0.0, 1);
        // Drain the placement's persist-dirty signal so the test isolates the item
        // move's own.
        let _ = s.loaded_chunks_mut().take_persist_dirty(0);
        assert!(!s
            .loaded_chunks()
            .get(chest.to_chunk_pos())
            .expect("resident")
            .persist_dirty_sections()
            .any());

        // An applied item move marks the chunk persist-dirty.
        let _ = s
            .container_left_click(p, chest, 0, stack("diamond", 5))
            .expect("place into slot");
        assert!(
            s.loaded_chunks()
                .get(chest.to_chunk_pos())
                .expect("resident")
                .persist_dirty_sections()
                .any(),
            "a chest item move must mark the chunk persist-dirty so contents persist",
        );

        // A refused click (out-of-range slot) mutates nothing and must NOT mark the
        // chunk persist-dirty.
        let _ = s.loaded_chunks_mut().take_persist_dirty(0);
        assert!(s
            .container_left_click(p, chest, ferrumc_world::CHEST_SLOTS, stack("diamond", 1))
            .is_none());
        assert!(
            !s.loaded_chunks()
                .get(chest.to_chunk_pos())
                .expect("resident")
                .persist_dirty_sections()
                .any(),
            "a refused click must not mark the chunk persist-dirty",
        );
    }

    #[tokio::test]
    async fn container_left_click_rejects_out_of_range_slot_without_losing_cursor() {
        let p = player("fumbler");
        let mut s = shard_with_player(p).await;
        let chest = BlockPos::new(8, 65, 8);
        let _ = place_block(&mut s, p, chest, chest_state(), Direction::Up, 1.0, 0.0, 1);

        // An out-of-range slot is refused; the caller keeps its cursor (no loss).
        assert!(s
            .container_left_click(p, chest, ferrumc_world::CHEST_SLOTS, stack("diamond", 3))
            .is_none());
        // The chest is untouched.
        let snapshot = s.container_open(p, chest).expect("chest opens");
        assert!(snapshot.iter().all(|slot| slot.item().is_none()));
    }

    #[tokio::test]
    async fn breaking_the_base_collapses_a_whole_gravity_column() {
        let p = player("miner");
        let mut s = shard_with_player(p).await;
        let sand = sand_state();
        // Stack three sand on the grass: y=64, 65, 66. Each is supported by the
        // one below (or the grass), so none falls while stacking.
        for y in 64..=66 {
            place_block(
                &mut s,
                p,
                BlockPos::new(8, y, 8),
                sand,
                Direction::Up,
                0.0,
                0.0,
                1,
            );
        }
        assert_eq!(s.entity_count(), 0);

        // Break the grass beneath the stack: the whole column loses support and
        // collapses into three falling entities at once.
        s.enqueue(GameInput::BlockBreak {
            player: p,
            position: BlockPos::new(8, 63, 8),
            sequence: 9,
        })
        .expect("room");
        s.run_tick();
        assert_eq!(s.entity_count(), 3, "the full column should be falling");
        for y in 64..=66 {
            assert!(block_at(&s, BlockPos::new(8, y, 8)).is_some_and(BlockStateId::is_air));
        }

        // They re-stack on the dirt (surface now at y=62, top face 63.0): sand at
        // y=63, 64, 65.
        for _ in 0..300 {
            s.run_tick();
            if s.entity_count() == 0 {
                break;
            }
        }
        for y in 63..=65 {
            assert_eq!(
                block_at(&s, BlockPos::new(8, y, 8)),
                Some(BlockStateId::new(sand)),
                "sand should re-stack at y={y}"
            );
        }
    }
}
