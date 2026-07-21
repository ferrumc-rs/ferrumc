//! [`SessionRouter`] and [`PlayerSessionHandle`]: the player<->shard mapping and
//! the message-based bridge between connections and simulation shards.

use std::collections::{BTreeMap, BTreeSet};

use tokio::sync::mpsc::{self, error::TrySendError};

use ferrumc_core::{DimensionId, EntityId, PlayerId, TextComponent, WorldId};
use ferrumc_math::{BlockPos, ChunkPos, ShardPos, Vec3};
use ferrumc_net::{Criticality, OutboundPriority};
use ferrumc_proto::generated::play::ClientboundPlayPacket;
use ferrumc_sim::{
    BlockStateId, GameInput, GameOutput, MutationCause, ShardId, ShardPartitioner,
    SpawnedEntityKind,
};

use crate::delivery::{DeliveryLane, InputDeliveryError, InputDeliveryPolicy};
use crate::directory::{ShardCoverage, ShardDirectory, ShardLease, ShardRegistrationId};
use crate::error::SessionError;
use crate::event::NetEvent;
use crate::outbound::OutboundMessage;
use crate::translate::{
    ack_shell, block_update_shell, chunk_for_position, entity_spawn_shell, entity_teleport_shell,
    falling_block_spawn_shell, move_shell, play_packet_to_input, player_info_add,
    player_info_remove, remove_entities_shell, set_equipment_shell, set_head_rotation_shell,
    update_entity_position_and_rotation_shell, update_entity_rotation_shell,
};

/// Default capacity of each shard's input channel.
///
/// Sized to mirror the simulation shard inbox (1024 queued inputs): far above
/// the per-tick volume a well-behaved router produces, so hitting it signals a
/// stall or a flood — exactly when [reject backpressure](SessionError::ShardInboxFull)
/// should engage.
pub const DEFAULT_SHARD_INPUT_CAPACITY: usize = 1024;

/// Default number of shard-input slots reserved for lifecycle/control traffic.
///
/// Sixteen slots are 1.6% of the default 1,024-entry queue: enough headroom for
/// a bounded burst of joins, leaves, and rejected-edit healing while preserving
/// almost the entire queue for ordinary state traffic. Control traffic is still
/// bounded; once these slots are consumed it receives the same explicit full
/// outcome as every other input.
pub const DEFAULT_SHARD_CONTROL_RESERVE: usize = 16;

/// Default capacity of each player's outbound channel.
///
/// A connection's writer task drains this; a moderate bound absorbs a normal
/// burst of clientbound shells while still capping a backlog. A persistently
/// full queue is the caller's cue to disconnect a client that cannot keep up.
pub const DEFAULT_OUTBOUND_CAPACITY: usize = 256;

/// Default view distance, in chunks, scoping which viewers a player is broadcast
/// to.
///
/// Matches the app's default play view distance. Visibility is a square
/// (Chebyshev) chunk-distance test: a subject is shown to a viewer when their
/// chunks differ by at most this many on both axes.
pub const DEFAULT_VIEW_DISTANCE: i32 = 10;

/// First network entity id handed to a joining player.
///
/// Starts at `2` to stay clear of `0` and of the id `1` the app assigns a client
/// to *itself* in `JoinGame`, so a remote player's entity can never collide with
/// a viewer's own.
const FIRST_ENTITY_ID: i32 = 2;

/// The current single-world app's typed routing scope.
const DEFAULT_WORLD: WorldId = WorldId::new(0);

/// The current single-dimension app's typed routing scope.
const DEFAULT_DIMENSION: DimensionId = DimensionId::new(0);

/// One current runtime endpoint registered in the shard directory.
///
/// `home` identifies the simulation owner for compatibility and diagnostics;
/// it is not the set of logical targets the endpoint covers.
#[derive(Debug, Clone)]
struct ShardEndpoint {
    home: ShardId,
    sender: mpsc::Sender<GameInput>,
}

/// A stable data-plane binding to one directory coverage slot.
///
/// Generations authorize control-plane replacement/removal. Player bindings
/// intentionally retain the stable slot and home instead, so an authorized
/// sender rotation keeps existing sessions routable without silently moving
/// them to newly introduced, more-specific coverage.
#[derive(Debug, Clone, Copy)]
struct ShardBinding {
    coverage: ShardCoverage,
    registration_id: ShardRegistrationId,
    home: ShardId,
}

/// The router's private per-player record.
///
/// Holds the routing target (`shard` + `outbound` channel) plus the lightweight
/// view state the router needs to scope and address visibility broadcasts: the
/// display `name` shown on the tab list and nameplate, the network `entity_id`
/// other clients see this player as, the last-known `position` (seeded at join,
/// refreshed from every movement output) used to scope visibility, and the
/// per-viewer `delivered` baselines used to encode movement deltas. This is
/// routing metadata mirrored from simulation outputs, not authoritative world
/// state — the simulation still owns the real positions.
#[derive(Debug)]
struct SessionEntry {
    shard: ShardBinding,
    outbound: mpsc::Sender<OutboundMessage>,
    name: String,
    entity_id: i32,
    position: Vec3,
    /// The player's last-known body yaw in degrees (seeded at join, refreshed from
    /// every movement output). Carried on the spawn sent to a viewer entering view
    /// so the remote player appears facing the right way.
    yaw: f32,
    /// The player's last-known pitch in degrees (seeded at join, refreshed from
    /// every movement output).
    pitch: f32,
    /// The pre-encoded `SetEquipment` body (the player's full equipment set — main
    /// hand, off hand, and the four armor pieces — as continuation-terminated
    /// slot+Slot entries) for this player, cached at join and refreshed on any
    /// held-item or worn-equipment change, sent to viewers as they enter view.
    /// Empty means "no equipment to show" (the send is skipped). The body is opaque
    /// here: the app owns the trusted Slot encoder and the equipment-slot layout.
    equipment: Vec<u8>,
    /// The last position of each *subject* (keyed by the subject's network
    /// `entity_id`) actually **delivered** to this viewer.
    ///
    /// A relative movement delta is computed against what this viewer received,
    /// not the global cached position, so a viewer that missed an update never
    /// drifts. Seeded when a spawn is successfully enqueued to this viewer,
    /// advanced *only* on a successful move enqueue (a move dropped under
    /// backpressure leaves the baseline stale, so the next delta grows and
    /// naturally promotes to an absolute teleport that re-syncs), and pruned when
    /// the subject despawns. Bounded by the number of connected players.
    delivered: BTreeMap<i32, Vec3>,
}

/// The router's view state for one renderable non-player entity (a falling
/// block, later items/projectiles), mirrored from simulation
/// [`GameOutput`](GameOutput)s.
///
/// The simulation identifies entities by [`EntityId`]; the client needs a
/// network entity id, so the router allocates one from the same monotonic source
/// as players ([`allocate_entity_id`](SessionRouter::allocate_entity_id)) — the
/// shared counter keeps the two id spaces from colliding on the wire. The stored
/// `position` and `kind` let the router re-spawn the entity for a viewer that
/// walks into range mid-flight (rebuilding the same `SpawnEntity` shell), and the
/// `uuid` is a stable per-entity id minted from the network id (a falling block
/// has no `PlayerId` to source one from). Per-viewer movement baselines live in
/// each [`SessionEntry::delivered`] map, keyed by `network_id`, exactly as for
/// players.
#[derive(Debug)]
struct EntityView {
    /// The network entity id the client sees this entity as.
    network_id: i32,
    /// A stable UUID for the `SpawnEntity` packet, derived from `network_id`.
    uuid: uuid::Uuid,
    /// Last position broadcast, used to re-spawn a late viewer at the right place.
    position: Vec3,
    /// What the entity is, so a re-spawn rebuilds the correct typed packet.
    kind: SpawnedEntityKind,
}

/// A snapshot of an in-range peer captured for a join-visibility exchange.
///
/// Copied out of the player map before the mutable per-recipient sends so the map
/// can be borrowed mutably inside the loop (mirrors the name/equipment clones the
/// move broadcast makes). Carries everything a spawn + equipment send needs.
struct PeerSnapshot {
    player: PlayerId,
    entity_id: i32,
    name: String,
    position: Vec3,
    yaw: f32,
    pitch: f32,
    equipment: Vec<u8>,
}

/// A newly installed shard endpoint and its bounded input receiver.
///
/// The caller gives the receiver to the shard worker and retains
/// [`lease`](Self::lease) for an authorized future sender rotation or removal.
/// Dropping this value without extracting the receiver closes the endpoint.
#[derive(Debug)]
pub struct ShardRegistration {
    home: ShardId,
    lease: ShardLease,
    receiver: mpsc::Receiver<GameInput>,
}

impl ShardRegistration {
    /// Returns the canonical home shard used for compatibility and diagnostics.
    pub const fn shard_id(&self) -> ShardId {
        self.home
    }

    /// Returns the coverage and generation authorizing this registration.
    pub fn lease(&self) -> ShardLease {
        self.lease.clone()
    }

    /// Consumes the registration and returns its bounded input receiver.
    pub fn into_receiver(self) -> mpsc::Receiver<GameInput> {
        self.receiver
    }

    /// Consumes the registration into its home, lease, and bounded receiver.
    pub fn into_parts(self) -> (ShardId, ShardLease, mpsc::Receiver<GameInput>) {
        (self.home, self.lease, self.receiver)
    }
}

/// A handle to one player's session, returned by
/// [`SessionRouter::join_player`].
///
/// The connection's writer task owns this handle and drains the outbound channel
/// — with [`recv`](Self::recv) (await) or [`try_recv`](Self::try_recv)
/// (non-blocking) — to deliver clientbound packets to the client. The handle
/// also records which [`shard`](Self::shard) the player joined.
///
/// Dropping the handle closes the outbound channel; the router observes this as a
/// [`SessionError::OutboundClosed`] the next time it tries to route an output to
/// the player, and the player should then be disconnected.
#[derive(Debug)]
pub struct PlayerSessionHandle {
    player: PlayerId,
    shard: ShardId,
    outbound: mpsc::Receiver<OutboundMessage>,
}

impl PlayerSessionHandle {
    /// The player this session belongs to.
    pub fn player(&self) -> PlayerId {
        self.player
    }

    /// The shard the player was routed to on join.
    pub fn shard(&self) -> ShardPos {
        self.shard.position()
    }

    /// The canonical home shard of the endpoint selected on join.
    ///
    /// This is the registered runtime endpoint's home, not necessarily the
    /// logical 8x8 target containing the player's position when world coverage
    /// selected one endpoint for the whole scope.
    pub fn shard_id(&self) -> ShardId {
        self.shard
    }

    /// Awaits the next outbound message, or `None` once the router has dropped the
    /// session and the channel is drained.
    ///
    /// The [`OutboundMessage`] carries the packet together with the
    /// [`Criticality`](ferrumc_net::Criticality) and
    /// [`OutboundPriority`](ferrumc_net::OutboundPriority) the router assigned it,
    /// so the writer enqueues at the carried priority and escalates a dropped
    /// mandatory packet without re-inferring either from packet type.
    pub async fn recv(&mut self) -> Option<OutboundMessage> {
        self.outbound.recv().await
    }

    /// Returns the next queued outbound message without waiting, or `None` if none
    /// is ready (the queue is empty or the router has dropped the session).
    pub fn try_recv(&mut self) -> Option<OutboundMessage> {
        self.outbound.try_recv().ok()
    }
}

/// Routes network events to simulation shards and simulation outputs back to
/// connections, owning the player<->shard mapping.
///
/// The router is the only component that knows where each player lives. It never
/// holds a [`SimShard`](ferrumc_sim::SimShard), a chunk, a socket, or a database
/// handle: it communicates purely by [`GameInput`]/[`GameOutput`] messages over
/// bounded [`mpsc`] channels.
///
/// # Wiring
///
/// - [`register_shard`](Self::register_shard) creates a shard's bounded input
///   channel and hands back the receiving half for the shard worker to drain.
/// - [`join_player`](Self::join_player) places a player on the shard owning their
///   spawn position, sends a [`GameInput::PlayerJoin`], and returns a
///   [`PlayerSessionHandle`] for the connection.
/// - [`route_event`](Self::route_event) translates a [`NetEvent`] and forwards it
///   to the player's shard.
/// - [`route_output`](Self::route_output) turns a [`GameOutput`] into the
///   clientbound packets it implies and delivers them to the relevant
///   connection(s).
/// - [`disconnect_player`](Self::disconnect_player) notifies the shard, then
///   drops the mapping and broadcasts the departure.
///
/// # Backpressure
///
/// Every channel is bounded and routing uses non-blocking sends, so the router
/// never blocks the tick loop. Each input has an [`InputDeliveryPolicy`]; normal
/// data stops before consuming the fixed control reserve, while join, leave, and
/// rejected-edit healing may use that reserved tail. The ownership-preserving
/// APIs return an [`InputDeliveryError`] carrying the exact rejected input.
///
/// Outbound traffic carries an explicit criticality (see
/// [`Criticality`](ferrumc_net::Criticality)) tagged onto each packet in an
/// [`OutboundMessage`] envelope **at the send site** — so the connection writer
/// (Layer B) honors the router's intent instead of re-inferring criticality from
/// packet type, which is wrong for context-dependent packets (a `BlockUpdate` is a
/// droppable viewer broadcast in one send, a mandatory actor resync in another).
/// *Droppable* sends — movement broadcasts and viewer block updates — are lossy: a
/// full recipient misses the update, which the next one supersedes. *Mandatory*
/// sends — block-change acks, rejected-edit resyncs, position corrections, join
/// spawns, visibility-enter spawns, and despawns — are never silently dropped: a
/// recipient that cannot accept one (full *or* closed) is returned by
/// [`route_output`](Self::route_output) / [`disconnect_player`](Self::disconnect_player)
/// for disconnection, since losing one would strand a prediction, leave an
/// invisible body, or ghost an entity. The router is the *sole sender* of each
/// per-player channel, so a
/// `capacity()` check before a multi-packet atomic group (a resync paired with
/// its ack) is race-free.
///
/// # Single-shard binding (this milestone)
///
/// A player stays bound to the shard they joined for the lifetime of the
/// session: movement routes to that shard even if the new position lies in
/// another shard's region. Cross-shard handoff is a later milestone.
///
/// # Visibility broadcasts (this milestone)
///
/// Beyond echoing a player's own outputs, the router gives players sight of one
/// another. On [`join_player`](Self::join_player) it exchanges a player-list add
/// plus an entity spawn between the newcomer and every existing player within
/// view distance; on [`disconnect_player`](Self::disconnect_player) it sends a
/// player-list remove *and* an entity despawn for the departing player; and
/// [`route_output`](Self::route_output) broadcasts a mover's new position to the
/// viewers in range (relative move or teleport by distance). Visibility is scoped
/// to [`view_distance`](Self::view_distance) chunks. The router keeps a per-player
/// last-known position to make this chunk-distance test, refreshed from
/// simulation outputs — never authored here.
#[derive(Debug)]
pub struct SessionRouter {
    shards: ShardDirectory<ShardEndpoint>,
    players: BTreeMap<PlayerId, SessionEntry>,
    pending_disconnects: BTreeSet<PlayerId>,
    /// Renderable non-player entities (falling blocks) the router currently
    /// broadcasts, keyed by the simulation [`EntityId`]. Bounded by the number of
    /// live entities: every entry is inserted on an `EntitySpawned` and removed on
    /// the matching `EntityDespawned` (which the shard emits on landing or void
    /// despawn), so it never grows without bound.
    entities: BTreeMap<EntityId, EntityView>,
    shard_input_capacity: usize,
    shard_control_reserve: usize,
    outbound_capacity: usize,
    view_distance: i32,
    next_entity_id: i32,
}

impl SessionRouter {
    /// Creates an empty router with the default channel capacities
    /// ([`DEFAULT_SHARD_INPUT_CAPACITY`] and [`DEFAULT_OUTBOUND_CAPACITY`]) and
    /// the default [`DEFAULT_VIEW_DISTANCE`].
    pub fn new() -> Self {
        Self::with_capacities(DEFAULT_SHARD_INPUT_CAPACITY, DEFAULT_OUTBOUND_CAPACITY)
    }

    /// Creates an empty router with explicit channel capacities, the default
    /// [`DEFAULT_SHARD_CONTROL_RESERVE`], and the default
    /// [`DEFAULT_VIEW_DISTANCE`].
    ///
    /// Each capacity is clamped to at least `1`, since a bounded
    /// [`mpsc`] channel cannot have zero capacity.
    pub fn with_capacities(shard_input_capacity: usize, outbound_capacity: usize) -> Self {
        Self::with_capacities_and_control_reserve(
            shard_input_capacity,
            outbound_capacity,
            DEFAULT_SHARD_CONTROL_RESERVE,
        )
    }

    /// Creates an empty router with explicit channel capacities and shard-input
    /// control reserve.
    ///
    /// Channel capacities are clamped to at least `1`. The control reserve is
    /// clamped below the shard capacity so even the smallest queue retains one
    /// data slot; a capacity-one queue therefore has no physically reservable
    /// tail but remains fully bounded.
    pub fn with_capacities_and_control_reserve(
        shard_input_capacity: usize,
        outbound_capacity: usize,
        shard_control_reserve: usize,
    ) -> Self {
        let shard_input_capacity = shard_input_capacity.max(1);
        Self {
            shards: ShardDirectory::new(),
            players: BTreeMap::new(),
            pending_disconnects: BTreeSet::new(),
            entities: BTreeMap::new(),
            shard_input_capacity,
            shard_control_reserve: shard_control_reserve
                .min(shard_input_capacity.saturating_sub(1)),
            outbound_capacity: outbound_capacity.max(1),
            view_distance: DEFAULT_VIEW_DISTANCE,
            next_entity_id: FIRST_ENTITY_ID,
        }
    }

    /// The configured per-shard input channel capacity.
    pub fn shard_input_capacity(&self) -> usize {
        self.shard_input_capacity
    }

    /// The number of tail slots ordinary data traffic leaves available for
    /// lifecycle/control inputs in every shard queue.
    pub fn shard_control_reserve(&self) -> usize {
        self.shard_control_reserve
    }

    /// The configured per-player outbound channel capacity.
    pub fn outbound_capacity(&self) -> usize {
        self.outbound_capacity
    }

    /// The view distance, in chunks, used to scope visibility broadcasts.
    pub fn view_distance(&self) -> i32 {
        self.view_distance
    }

    /// Sets the view distance, in chunks, used to scope visibility broadcasts.
    ///
    /// Clamped to `0` at minimum (a negative distance would hide every player,
    /// including those sharing a chunk). The app sets this from its configured
    /// play view distance at startup.
    pub fn set_view_distance(&mut self, chunks: i32) {
        self.view_distance = chunks.max(0);
    }

    /// The number of shards currently registered.
    pub fn shard_count(&self) -> usize {
        self.shards.len()
    }

    /// `true` if the default world/dimension can route logical `shard`.
    ///
    /// Exact coverage or a world-covering fallback may satisfy the lookup.
    pub fn is_shard_registered(&self, shard: ShardPos) -> bool {
        ShardPartitioner::for_shard(DEFAULT_WORLD, DEFAULT_DIMENSION, shard)
            .ok()
            .is_some_and(|target| self.shards.resolve(target).is_some())
    }

    /// The number of players with an active session.
    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    /// The number of overloaded sessions whose control-lane leave is retained
    /// for an explicit retry.
    ///
    /// The set is bounded by [`player_count`](Self::player_count): it stores at
    /// most one retry marker per connected player and never grows from repeated
    /// failures for the same session.
    pub fn pending_disconnect_count(&self) -> usize {
        self.pending_disconnects.len()
    }

    /// `true` if `player` has an active session.
    pub fn is_player_connected(&self, player: PlayerId) -> bool {
        self.players.contains_key(&player)
    }

    /// The shard `player` is bound to, or `None` if they have no session.
    pub fn player_shard(&self, player: PlayerId) -> Option<ShardPos> {
        self.player_shard_id(player).map(ShardId::position)
    }

    /// The canonical home shard of `player`'s registered runtime endpoint.
    ///
    /// A world-covering endpoint can own a player whose position belongs to a
    /// different logical target, so this returns the endpoint home selected at
    /// join rather than recomputing from the player's position.
    pub fn player_shard_id(&self, player: PlayerId) -> Option<ShardId> {
        self.players.get(&player).map(|entry| entry.shard.home)
    }

    /// Registers exact default-world coverage at `shard`, returning its bounded
    /// input receiver.
    ///
    /// This source-compatible adapter preserves the current app's exact spatial
    /// registration until its explicit world-covering adoption. New supervisors
    /// should use [`register_exact_shard`](Self::register_exact_shard) or
    /// [`register_world_shard`](Self::register_world_shard), which return the
    /// validation result and lease.
    ///
    /// Duplicate registration is refused without replacing the active endpoint;
    /// the returned receiver is already closed because this legacy signature
    /// cannot carry a typed error. An invalid out-of-domain position or exhausted
    /// generation fails closed in the same way. The fallible typed APIs expose
    /// those failures directly, and sender rotation always requires
    /// [`replace_shard_registration`](Self::replace_shard_registration) with the
    /// current lease.
    pub fn register_shard(&mut self, shard: ShardPos) -> mpsc::Receiver<GameInput> {
        let Ok(home) = ShardPartitioner::for_shard(DEFAULT_WORLD, DEFAULT_DIMENSION, shard) else {
            return self.closed_shard_receiver();
        };
        match self.register_exact_shard(home) {
            Ok(registration) => registration.into_receiver(),
            Err(_) => self.closed_shard_receiver(),
        }
    }

    /// Registers one exact logical shard endpoint.
    ///
    /// # Errors
    ///
    /// Returns [`SessionError::ShardDirectory`] if exact coverage is occupied
    /// or a fresh generation cannot be allocated. The existing endpoint remains
    /// unchanged.
    pub fn register_exact_shard(
        &mut self,
        home: ShardId,
    ) -> Result<ShardRegistration, SessionError> {
        self.register_shard_coverage(ShardCoverage::exact(home), home)
    }

    /// Registers `home` as the endpoint covering its entire world/dimension.
    ///
    /// Resolution keeps the target logical [`ShardId`] distinct from this
    /// endpoint home. Exact registrations in the same scope take precedence for
    /// new resolutions; otherwise positions arbitrarily far from `home` route
    /// through this one bounded endpoint without enumerating regions.
    ///
    /// # Errors
    ///
    /// Returns [`SessionError::ShardDirectory`] if world coverage is occupied
    /// or a fresh generation cannot be allocated.
    pub fn register_world_shard(
        &mut self,
        home: ShardId,
    ) -> Result<ShardRegistration, SessionError> {
        self.register_shard_coverage(ShardCoverage::world(home.world(), home.dimension()), home)
    }

    /// Rotates the sender for the registration authorized by `lease`.
    ///
    /// Coverage and endpoint home remain unchanged. Existing player bindings to
    /// that stable coverage slot immediately use the new bounded sender, while
    /// the old receiver closes after draining.
    ///
    /// # Errors
    ///
    /// Returns [`SessionError::ShardDirectory`] when `lease` belongs to another
    /// directory, is stale, or the generation space is exhausted. A failure
    /// leaves the active endpoint unchanged.
    pub fn replace_shard_registration(
        &mut self,
        lease: &ShardLease,
    ) -> Result<ShardRegistration, SessionError> {
        let home = self.shards.get(lease)?.endpoint().home;
        let (sender, receiver) = mpsc::channel(self.shard_input_capacity);
        let next = self.shards.replace(lease, ShardEndpoint { home, sender })?;
        Ok(ShardRegistration {
            home,
            lease: next,
            receiver,
        })
    }

    /// Removes the endpoint authorized by `lease` and closes its sender.
    ///
    /// # Errors
    ///
    /// Returns [`SessionError::ShardDirectory`] if `lease` belongs to another
    /// directory, is absent, or is stale; the current registration then remains
    /// unchanged.
    pub fn unregister_shard_registration(
        &mut self,
        lease: &ShardLease,
    ) -> Result<ShardId, SessionError> {
        Ok(self.shards.remove(lease)?.home)
    }

    fn register_shard_coverage(
        &mut self,
        coverage: ShardCoverage,
        home: ShardId,
    ) -> Result<ShardRegistration, SessionError> {
        let (sender, receiver) = mpsc::channel(self.shard_input_capacity);
        let lease = self
            .shards
            .register(coverage, ShardEndpoint { home, sender })?;
        Ok(ShardRegistration {
            home,
            lease,
            receiver,
        })
    }

    fn closed_shard_receiver(&self) -> mpsc::Receiver<GameInput> {
        let (sender, receiver) = mpsc::channel(self.shard_input_capacity);
        drop(sender);
        receiver
    }

    /// The network entity id a `player` is broadcast to other clients as, or
    /// `None` if they have no session.
    pub fn player_entity_id(&self, player: PlayerId) -> Option<i32> {
        self.players.get(&player).map(|entry| entry.entity_id)
    }

    /// The last-known position the router holds for `player`, or `None` if they
    /// have no session.
    ///
    /// This is routing metadata (seeded at join, refreshed from movement
    /// outputs), not the simulation's authoritative position.
    pub fn player_position(&self, player: PlayerId) -> Option<Vec3> {
        self.players.get(&player).map(|entry| entry.position)
    }

    /// Joins `player` (displayed as `name`) at `position`, routing them to the
    /// owning shard and making them mutually visible to nearby players.
    ///
    /// Determines the shard from `position`, sends a [`GameInput::PlayerJoin`] to
    /// it, records the mapping (storing the display `name`, allocating the
    /// player's network entity id, and seeding their position), exchanges
    /// player-list + spawn packets with every existing player within view
    /// distance, and returns a [`PlayerSessionHandle`] carrying the player's
    /// outbound channel. The `name` is what other clients show on the tab list
    /// and the nameplate above the spawned entity.
    ///
    /// # Errors
    ///
    /// - [`SessionError::UnknownShard`] if no shard owns `position`.
    /// - [`SessionError::DuplicatePlayer`] if `player` already has a session.
    /// - [`SessionError::OutboundFull`] if the joiner's fresh outbound channel is
    ///   too small to hold its mandatory existing-player visibility — a tab-list
    ///   add + entity spawn for every in-range player. The join is *staged* against
    ///   guaranteed capacity, so this is rejected up front (the app maps it to a
    ///   disconnect) rather than leaving the joiner connected in a broken
    ///   half-world after a mid-burst overflow.
    /// - [`SessionError::ShardInboxFull`] / [`SessionError::ShardClosed`] if the
    ///   join could not be delivered to the shard.
    ///
    /// On any error the joining player is not registered, so its join can be
    /// retried cleanly. An already-connected slow viewer may have been explicitly
    /// disconnected during the preflight that makes room for mutual visibility.
    ///
    /// The joiner's tab-list add and entity spawn are delivered to existing
    /// viewers as *mandatory* packets. A viewer already lacking capacity for the
    /// pair is disconnected before the join is accepted; a viewer that closes in
    /// the remaining race is disconnected afterward, with a rejected control-lane
    /// leave retained for explicit retry. The joiner's *own* mandatory visibility
    /// is guaranteed by capacity staging — its fresh channel provably holds every
    /// existing-player add + spawn — so a joiner is never left half-loaded.
    pub fn join_player(
        &mut self,
        player: PlayerId,
        name: &str,
        position: Vec3,
    ) -> Result<PlayerSessionHandle, SessionError> {
        self.join_player_with_equipment(player, name, position, Vec::new())
    }

    /// Joins `player` exactly like [`join_player`](Self::join_player), but also
    /// caches their pre-encoded `equipment` (the full `SetEquipment` body) so it is
    /// sent to every viewer the join makes the player visible to.
    ///
    /// Caching the equipment *at join* (rather than via a follow-up call) closes the
    /// enter-view race: viewers that become visible to the joiner during this call
    /// receive the full equipment set immediately with the spawn, not only after the
    /// next change. An empty `equipment` skips the cosmetic send. All the error and
    /// backpressure behaviour of [`join_player`](Self::join_player) applies here.
    pub fn join_player_with_equipment(
        &mut self,
        player: PlayerId,
        name: &str,
        position: Vec3,
        equipment: Vec<u8>,
    ) -> Result<PlayerSessionHandle, SessionError> {
        let target = ShardPartitioner::for_chunk(
            DEFAULT_WORLD,
            DEFAULT_DIMENSION,
            chunk_for_position(position),
        );
        let shard = self
            .shards
            .resolve(target)
            .map(|resolved| ShardBinding {
                coverage: resolved.coverage(),
                registration_id: resolved.registration_id(),
                home: resolved.endpoint().home,
            })
            .ok_or(SessionError::UnknownShard {
                shard: target.position(),
            })?;
        if self.players.contains_key(&player) {
            return Err(SessionError::DuplicatePlayer { player });
        }

        let joiner_chunk = chunk_for_position(position);
        // Stage the joiner's mandatory existing-player visibility against
        // guaranteed channel capacity. Each in-range existing player costs the
        // joiner two mandatory packets (a tab-list add + an entity spawn) on its
        // own fresh outbound channel, so the burst fits only when `2 * in_range <=
        // outbound_capacity`. If it would not fit, the joiner could not receive its
        // full visibility (finding #9: a silently dropped mandatory spawn leaves an
        // invisible body), so reject the join up front with ZERO side effects — no
        // shard `PlayerJoin`, no map insert — rather than half-load the world.
        let in_range = self
            .players
            .values()
            .filter(|entry| {
                within_view(
                    joiner_chunk,
                    chunk_for_position(entry.position),
                    self.view_distance,
                )
            })
            .count();
        if in_range.saturating_mul(2) > self.outbound_capacity {
            return Err(SessionError::OutboundFull { player });
        }

        // Remove an in-range viewer that cannot accept the joiner's mandatory
        // tab-list add + spawn before admitting the joiner. Otherwise the joiner
        // would first receive that viewer's spawn and immediately need a
        // mandatory despawn, doubling the fresh-channel burst and risking a
        // ghost. The slow viewer's control-lane leave is itself explicit: a full
        // lane aborts this join before PlayerJoin is sent, with the viewer mapping
        // retained for retry.
        let slow_viewers: Vec<_> = self
            .players
            .iter()
            .filter(|(_, entry)| {
                within_view(
                    joiner_chunk,
                    chunk_for_position(entry.position),
                    self.view_distance,
                ) && (entry.outbound.is_closed() || entry.outbound.capacity() < 2)
            })
            .map(|(viewer, _)| *viewer)
            .collect();
        for viewer in slow_viewers {
            if self.players.contains_key(&viewer) {
                self.disconnect_player(viewer)?;
            }
        }

        // Notify the shard before recording anything: if the join cannot be
        // delivered, the player must not be left half-registered.
        self.send_to_shard(shard, GameInput::PlayerJoin { player, position })?;

        let (tx, rx) = mpsc::channel(self.outbound_capacity);
        let entity_id = self.allocate_entity_id();
        self.players.insert(
            player,
            SessionEntry {
                shard,
                outbound: tx,
                name: name.to_owned(),
                entity_id,
                position,
                yaw: 0.0,
                pitch: 0.0,
                equipment,
                delivered: BTreeMap::new(),
            },
        );
        let mut to_disconnect = Vec::new();
        self.broadcast_join_visibility(player, position, &mut to_disconnect);
        // Disconnect existing viewers whose mandatory tab-list add overflowed,
        // draining the resulting leave cascade iteratively (bounded by player
        // count). Capacity staging above guarantees the joiner's own channel held
        // its full visibility, so it can no longer surface here; the `== player`
        // skip remains as a defensive guard.
        to_disconnect.retain(|viewer| *viewer != player);
        // A full control tail cannot roll back the already accepted join. Every
        // failed leave is retained in `pending_disconnects`, where the app can
        // retry it after the shard drains instead of losing it here.
        let _retained_failure = self.drain_disconnect_worklist(to_disconnect);
        Ok(PlayerSessionHandle {
            player,
            shard: shard.home,
            outbound: rx,
        })
    }

    /// Allocates the next network entity id, advancing the counter.
    fn allocate_entity_id(&mut self) -> i32 {
        let id = self.next_entity_id;
        // Wrap rather than overflow-panic; a process churning through 2^31 joins
        // is not a real concern, but a panic in the router would be.
        self.next_entity_id = self.next_entity_id.wrapping_add(1);
        id
    }

    /// Exchanges player-list-add + spawn packets between the joiner and every
    /// existing player within view distance.
    ///
    /// Each in-range existing player is told about the joiner, and the joiner is
    /// told about each in-range existing player, so visibility is symmetric after
    /// a single join. Both the tab-list add and the entity spawn are **mandatory**
    /// (a viewer that cannot accept either is pushed to `to_disconnect`): a dropped
    /// spawn is unrecoverable, leaving the viewer a tab-list entry with no visible
    /// body. Each successful spawn also seeds the recipient's `delivered` baseline so
    /// the very next movement delta is measured against the position they actually
    /// received. The joiner's own visibility cannot overflow here: [`join_player`](Self::join_player)
    /// stages it against guaranteed channel capacity before calling this.
    ///
    /// After each mandatory spawn, a **droppable** [`SetEquipment`](set_equipment_shell)
    /// for the spawned subject is sent to the recipient (both directions) from the
    /// subject's cached equipment, so the joiner and existing players immediately see
    /// what each other is holding. The equipment send is skipped when the cached body
    /// is empty (nothing to show); being droppable, it never forces a disconnect.
    fn broadcast_join_visibility(
        &mut self,
        joiner: PlayerId,
        joiner_position: Vec3,
        to_disconnect: &mut Vec<PlayerId>,
    ) {
        let Some(joiner_entry) = self.players.get(&joiner) else {
            return;
        };
        let joiner_eid = joiner_entry.entity_id;
        let joiner_name = joiner_entry.name.clone();
        let joiner_yaw = joiner_entry.yaw;
        let joiner_pitch = joiner_entry.pitch;
        let joiner_equipment = joiner_entry.equipment.clone();
        let joiner_chunk = chunk_for_position(joiner_position);
        // Snapshot the in-range existing players first so the player map can be
        // mutated (seeding delivered baselines) while iterating.
        let others: Vec<PeerSnapshot> = self
            .players
            .iter()
            .filter(|(&other, entry)| {
                other != joiner
                    && within_view(
                        joiner_chunk,
                        chunk_for_position(entry.position),
                        self.view_distance,
                    )
            })
            .map(|(&other, entry)| PeerSnapshot {
                player: other,
                entity_id: entry.entity_id,
                name: entry.name.clone(),
                position: entry.position,
                yaw: entry.yaw,
                pitch: entry.pitch,
                equipment: entry.equipment.clone(),
            })
            .collect();
        for other in others {
            // Show the joiner to the existing player: both the tab-list add and the
            // entity spawn are mandatory (a dropped spawn leaves an invisible body).
            if let Some(entry) = self.players.get_mut(&other.player) {
                Self::send_mandatory(
                    entry,
                    player_info_add(joiner, &joiner_name),
                    other.player,
                    to_disconnect,
                );
                Self::send_mandatory_spawn(
                    entry,
                    entity_spawn_shell(
                        joiner_eid,
                        joiner,
                        joiner_position,
                        joiner_yaw,
                        joiner_pitch,
                    ),
                    joiner_eid,
                    joiner_position,
                    other.player,
                    to_disconnect,
                );
                if !joiner_equipment.is_empty() {
                    Self::send_droppable(
                        entry,
                        set_equipment_shell(joiner_eid, joiner_equipment.clone()),
                        other.player,
                        to_disconnect,
                    );
                }
            }
            // Show the existing player to the joiner.
            if let Some(entry) = self.players.get_mut(&joiner) {
                Self::send_mandatory(
                    entry,
                    player_info_add(other.player, &other.name),
                    joiner,
                    to_disconnect,
                );
                Self::send_mandatory_spawn(
                    entry,
                    entity_spawn_shell(
                        other.entity_id,
                        other.player,
                        other.position,
                        other.yaw,
                        other.pitch,
                    ),
                    other.entity_id,
                    other.position,
                    joiner,
                    to_disconnect,
                );
                if !other.equipment.is_empty() {
                    Self::send_droppable(
                        entry,
                        set_equipment_shell(other.entity_id, other.equipment),
                        joiner,
                        to_disconnect,
                    );
                }
            }
        }
    }

    /// Translates and routes a [`NetEvent`] to the player's shard.
    ///
    /// A [`NetEvent::Disconnected`] runs the full [disconnect
    /// cleanup](Self::disconnect_player). A [`NetEvent::Play`] is translated with
    /// [`net_event_to_input`](crate::net_event_to_input); events with no
    /// simulation effect (keep-alives and the like) are a no-op `Ok(())`.
    ///
    /// # Errors
    ///
    /// - [`SessionError::UnknownPlayer`] if the event targets a player with no
    ///   session.
    /// - [`SessionError::ShardInboxFull`] / [`SessionError::ShardClosed`] if the
    ///   input could not be delivered to the shard.
    /// - [`SessionError::StaleShardBinding`] if the player's endpoint was removed
    ///   and re-registered without an explicit session handoff.
    pub fn route_event(&mut self, event: &NetEvent) -> Result<(), SessionError> {
        self.route_event_owned(event)
            .map_err(InputDeliveryError::into_error)
    }

    /// Translates and routes a [`NetEvent`] while preserving ownership of any
    /// rejected simulation input.
    ///
    /// This is the backpressure-aware counterpart to [`route_event`](Self::route_event).
    /// Events with no simulation effect still return `Ok(())`.
    ///
    /// # Errors
    ///
    /// Returns an [`InputDeliveryError`] containing the exact undelivered input,
    /// its typed policy, and the classified [`SessionError`].
    pub fn route_event_owned(&mut self, event: &NetEvent) -> Result<(), InputDeliveryError> {
        match event {
            NetEvent::Disconnected { player, .. } => {
                self.disconnect_player_owned(*player).map(|_| ())
            }
            NetEvent::Play { player, packet } => match play_packet_to_input(*player, packet) {
                Some(input) => self.route_input_owned(*player, input),
                None => Ok(()),
            },
        }
    }

    /// Routes an already-translated [`GameInput`] to `player`'s shard.
    ///
    /// Used by the driver for inputs that originate from a command rather than a
    /// decoded network event (for example a `/gamemode`-driven
    /// [`GameInput::SetGameMode`]). Routing matches [`route_event`](Self::route_event):
    /// a block edit goes to the block's owning shard, everything else to the
    /// player's bound shard.
    ///
    /// # Errors
    ///
    /// - [`SessionError::UnknownPlayer`] if `player` has no session.
    /// - [`SessionError::ShardInboxFull`] / [`SessionError::ShardClosed`] if the
    ///   input could not be delivered to the shard.
    /// - [`SessionError::StaleShardBinding`] if the player's endpoint lineage is
    ///   no longer current.
    pub fn route_game_input(&self, player: PlayerId, input: GameInput) -> Result<(), SessionError> {
        self.route_game_input_owned(player, input)
            .map_err(InputDeliveryError::into_error)
    }

    /// Routes an already-translated input while preserving it on rejection.
    ///
    /// # Errors
    ///
    /// Returns an [`InputDeliveryError`] containing the exact input and its
    /// policy if the player/shard lookup or bounded send fails.
    pub fn route_game_input_owned(
        &self,
        player: PlayerId,
        input: GameInput,
    ) -> Result<(), InputDeliveryError> {
        self.route_input_owned(player, input)
    }

    /// Routes a simulation [`GameOutput`] to its network recipient(s), returning
    /// the players that must be disconnected — those whose channel is *closed* and
    /// those that overflowed a **mandatory** packet (the slow-client policy).
    ///
    /// The mapping per output:
    /// - [`GameOutput::PlayerSpawned`] refreshes the player's cached position;
    ///   the spawn itself was already broadcast to viewers at
    ///   [`join_player`](Self::join_player) time, so nothing else is sent.
    /// - [`GameOutput::PlayerMoved`] broadcasts the move/rotation (droppable) to
    ///   every *other* in-range player — a relative
    ///   `UpdateEntityPositionAndRotation` when every axis fits the i16 fixed-point
    ///   range (else an absolute `EntityTeleport`) plus a `SetHeadRotation` for a
    ///   position change, or an `UpdateEntityRotation` + `SetHeadRotation` for a
    ///   rotation-only turn — measured per viewer against its delivered baseline,
    ///   then refreshes the cached position/rotation used for visibility scoping.
    /// - [`GameOutput::PlayerPositionCorrected`] snaps the player itself back to
    ///   the authoritative position; it is **mandatory** (not a broadcast).
    /// - [`GameOutput::BlockChanged`] broadcasts a `BlockUpdate` (droppable) to
    ///   every in-range player (the actor included) and, for a
    ///   [`MutationCause::PlayerCreative`] edit, sends the acting player alone a
    ///   **mandatory** `AcknowledgeBlockChange` echoing the sequence.
    /// - [`GameOutput::BlockChangeRejected`] sends only the acting player a
    ///   `BlockUpdate` carrying the authoritative state followed by an
    ///   `AcknowledgeBlockChange` echoing the rejected sequence. Both are mandatory
    ///   and enqueued **atomically** (capacity reserved for the pair, else the
    ///   client is disconnected) so the resync and the ack that ends the client's
    ///   prediction never separate. Never a broadcast, since viewers never saw the
    ///   predicted change.
    /// - [`GameOutput::PlayerDespawned`] (and any future variant) sends nothing:
    ///   leave notifications are issued by
    ///   [`disconnect_player`](Self::disconnect_player), where the departing
    ///   player's identity is still known.
    ///
    /// # Backpressure
    ///
    /// Droppable traffic (movement broadcasts, viewer `BlockUpdate`s) is lossy
    /// under backpressure: a recipient whose channel is *full* misses the update,
    /// which a later one supersedes, so the router never blocks the tick loop.
    /// Mandatory traffic (corrections, acks, resyncs) is never silently dropped: a
    /// recipient that cannot accept it — full *or* closed — is returned for the
    /// caller to disconnect.
    #[allow(clippy::too_many_lines)] // one match over every routed GameOutput variant
    pub fn route_output(&mut self, output: &GameOutput) -> Vec<PlayerId> {
        let mut to_disconnect = Vec::new();
        match output {
            GameOutput::PlayerSpawned { player, position } => {
                self.update_position(*player, *position);
            }
            GameOutput::PlayerMoved {
                player,
                position,
                yaw,
                pitch,
                position_changed,
            } => {
                // Broadcast BEFORE refreshing the cached position: visibility is
                // scoped against the new position, but each viewer's delta is
                // measured against its own delivered baseline, not this cache.
                self.broadcast_move(
                    *player,
                    *position,
                    *yaw,
                    *pitch,
                    *position_changed,
                    &mut to_disconnect,
                );
                self.update_position_and_rotation(*player, *position, *yaw, *pitch);
            }
            GameOutput::PlayerPositionCorrected { player, position } => {
                // A correction snaps a desynced client back to its authoritative
                // position; it is mandatory (a dropped correction strands the
                // client off-position), so a full/closed channel disconnects it.
                if let Some(entry) = self.players.get(player) {
                    Self::send_mandatory(entry, move_shell(*position), *player, &mut to_disconnect);
                }
            }
            GameOutput::BlockChanged {
                position,
                state,
                sequence,
                cause,
            } => {
                // The viewer broadcast is droppable (the world stays correct); the
                // actor's ack is mandatory — it confirms the client's optimistic
                // prediction, and dropping it strands that prediction forever.
                self.broadcast_block_update(*position, *state, &mut to_disconnect);
                if let MutationCause::PlayerCreative { player } = cause {
                    if let Some(entry) = self.players.get(player) {
                        Self::send_mandatory(
                            entry,
                            ack_shell(*sequence),
                            *player,
                            &mut to_disconnect,
                        );
                    }
                }
            }
            GameOutput::BlockChangeRejected {
                player,
                position,
                sequence,
                authoritative_state,
                ..
            } => {
                // Heal only the actor; viewers never saw the predicted change. The
                // resync `BlockUpdate` and the `AcknowledgeBlockChange` are
                // mandatory AND must arrive together: an ack without the resync
                // heals to the wrong state, and a resync without the ack leaves the
                // ghost block (the ack is what ends the client's prediction via
                // endPredictionsUpTo). Both are forced onto the (Mandatory, State)
                // class — the resync overrides a `BlockUpdate`'s droppable/`World`
                // default so it shares the ack's queue. The channel is FIFO and the
                // writer drains one message at a time (flushing after each), so the
                // resync is always processed before the ack: the ack can never
                // survive a dropped resync, and a dropped mandatory resync escalates
                // to a disconnect at Layer B. Reserve both slots up front — the
                // router is the sole sender of this channel, so `capacity()` cannot
                // shrink under us — and if the pair will not fit, disconnect rather
                // than enqueue a partial group.
                if let Some(entry) = self.players.get(player) {
                    // The resync is mandatory and cannot be skipped. If the
                    // authoritative state has no wire encoding (an unrepresentable
                    // id), there is no faithful packet to heal with, so fail closed
                    // and disconnect — the same as a channel that cannot fit the
                    // pair. This is unreachable for legitimately stored states (the
                    // entry boundary rejects unrepresentable ids), so it is a
                    // defensive fail-safe, not a normal path.
                    if let Some(resync_packet) = block_update_shell(*position, *authoritative_state)
                    {
                        if entry.outbound.capacity() >= 2 {
                            let resync = entry.outbound.try_send(OutboundMessage::new(
                                resync_packet,
                                Criticality::Mandatory,
                                OutboundPriority::State,
                            ));
                            let ack = entry
                                .outbound
                                .try_send(OutboundMessage::mandatory(ack_shell(*sequence)));
                            // With capacity >= 2 and a sole sender, only a closed
                            // channel can fail either send.
                            if resync.is_err() || ack.is_err() {
                                to_disconnect.push(*player);
                            }
                        } else {
                            to_disconnect.push(*player);
                        }
                    } else {
                        to_disconnect.push(*player);
                    }
                }
            }
            GameOutput::SignUpdated { position, sign } => {
                // Broadcast the sign's new text to every viewer within view
                // distance, exactly like a block update (the world is correct
                // regardless, so the carrier is droppable). The acting editor is
                // included so the server-authoritative text confirms their edit.
                let packet = crate::sign_block_entity_data(*position, sign);
                self.broadcast_block_entity_data(*position, &packet, &mut to_disconnect);
            }
            GameOutput::OpenSignEditor { player, position } => {
                // Open the editor for the placer alone. Droppable: a dropped editor
                // just means the sign stays blank (the player can re-place), so it
                // never escalates to a disconnect.
                if let Some(entry) = self.players.get(player) {
                    Self::send_droppable(
                        entry,
                        crate::open_sign_editor(*position),
                        *player,
                        &mut to_disconnect,
                    );
                }
            }
            GameOutput::EntitySpawned {
                entity,
                position,
                kind,
                ..
            } => {
                // Velocity is ignored: the client's position is driven by the
                // per-tick move broadcasts, not spawn-time velocity.
                self.spawn_entity_view(*entity, *position, *kind, &mut to_disconnect);
            }
            GameOutput::EntityMoved { entity, position } => {
                self.broadcast_entity_move(*entity, *position, &mut to_disconnect);
            }
            GameOutput::EntityDespawned { entity } => {
                self.despawn_entity_view(*entity, &mut to_disconnect);
            }
            // A player despawn carries no wire packet here; leave handling lives in
            // disconnect_player. Any other future variant is a no-op until wired.
            _ => {}
        }
        to_disconnect
    }

    /// Sends a **mandatory** clientbound packet to `entry`'s player, recording the
    /// player in `to_disconnect` if it cannot be delivered.
    ///
    /// Mandatory delivery is defined as "delivered, or the slow client is
    /// disconnected" — never silently dropped. Both a *full* and a *closed*
    /// channel push the player for disconnect; the driver then tears the session
    /// down (and the connection observes its channel close).
    fn send_mandatory(
        entry: &SessionEntry,
        packet: ClientboundPlayPacket,
        player: PlayerId,
        to_disconnect: &mut Vec<PlayerId>,
    ) {
        if entry
            .outbound
            .try_send(OutboundMessage::mandatory(packet))
            .is_err()
        {
            to_disconnect.push(player);
        }
    }

    /// Sends a **mandatory** entity `spawn` to `entry`'s player, seeding that
    /// viewer's [`delivered`](SessionEntry::delivered) baseline for `subject_eid`
    /// (at `subject_position`) on success and pushing `player` to `to_disconnect`
    /// on a full *or* closed channel.
    ///
    /// A spawn is mandatory, not droppable: a dropped one is unrecoverable. The
    /// viewer would keep the subject's tab-list entry (delivered separately) yet
    /// never see its body, because a later relative move — or even an absolute
    /// teleport — for an entity id the client never spawned is ignored client-side.
    /// That is the inverse of the ghost a dropped despawn leaves, so the slow-client
    /// policy applies: deliver the spawn or disconnect the viewer. Seeding the
    /// baseline only on success keeps the next movement delta measured against the
    /// position the viewer actually received.
    fn send_mandatory_spawn(
        entry: &mut SessionEntry,
        spawn: ClientboundPlayPacket,
        subject_eid: i32,
        subject_position: Vec3,
        player: PlayerId,
        to_disconnect: &mut Vec<PlayerId>,
    ) {
        if entry
            .outbound
            .try_send(OutboundMessage::mandatory(spawn))
            .is_ok()
        {
            entry.delivered.insert(subject_eid, subject_position);
        } else {
            to_disconnect.push(player);
        }
    }

    /// Sends a **droppable** clientbound packet to `entry`'s player: a *full*
    /// channel silently drops it (a later update supersedes it), while a *closed*
    /// channel pushes the player to `to_disconnect`.
    fn send_droppable(
        entry: &SessionEntry,
        packet: ClientboundPlayPacket,
        player: PlayerId,
        to_disconnect: &mut Vec<PlayerId>,
    ) {
        match entry.outbound.try_send(OutboundMessage::droppable(packet)) {
            Ok(()) | Err(TrySendError::Full(_)) => {}
            Err(TrySendError::Closed(_)) => to_disconnect.push(player),
        }
    }

    /// Refreshes only the cached position the router routes visibility against
    /// (used on spawn, where rotation stays at its join seed).
    fn update_position(&mut self, player: PlayerId, position: Vec3) {
        if let Some(entry) = self.players.get_mut(&player) {
            entry.position = position;
        }
    }

    /// Refreshes the cached position and rotation the router routes visibility
    /// against and carries on the next spawn a viewer entering view receives.
    fn update_position_and_rotation(
        &mut self,
        player: PlayerId,
        position: Vec3,
        yaw: f32,
        pitch: f32,
    ) {
        if let Some(entry) = self.players.get_mut(&player) {
            entry.position = position;
            entry.yaw = yaw;
            entry.pitch = pitch;
        }
    }

    /// Broadcasts `mover`'s new `position` and rotation (`yaw`/`pitch`, degrees) to
    /// every other player within view distance, recording any closed recipients in
    /// `to_disconnect`. `position_changed` distinguishes a real move from a
    /// rotation-only turn in place.
    ///
    /// A viewer that *already* has a [`delivered`](SessionEntry::delivered)
    /// baseline for the mover gets **droppable** cosmetic carriers:
    /// - **position changed**: a relative
    ///   [`update_entity_position_and_rotation_shell`] when every axis fits the i16
    ///   fixed-point range, otherwise an absolute [`entity_teleport_shell`], plus a
    ///   [`set_head_rotation_shell`] so the head turns. The baseline advances
    ///   **only** on a successful move enqueue, so a move dropped under backpressure
    ///   self-corrects: its next delta is measured from the stale baseline, grows,
    ///   and promotes to a teleport — the viewer never drifts.
    /// - **rotation only**: an [`update_entity_rotation_shell`] plus a
    ///   [`set_head_rotation_shell`]; the baseline is left unchanged (no move).
    ///
    /// All of these are droppable, so a *full* channel simply skips the update; a
    /// *closed* channel is recorded for disconnect.
    ///
    /// A viewer with **no** baseline is one the mover is *entering view of* (it
    /// joined out of range, or its earlier spawn was lost). Such a viewer must
    /// never receive a bare movement packet for an entity it never spawned — a real
    /// client ignores it, leaving an invisible body. So the mover is *spawned* into
    /// view instead: a **mandatory** [`player_info_add`] then a **mandatory**
    /// [`entity_spawn_shell`] carrying the facing (which seeds the baseline at this
    /// position), then a **droppable** [`set_equipment_shell`] (skipped when empty)
    /// so the held item shows immediately — and no movement packet this tick. A
    /// viewer that cannot accept either mandatory packet is pushed to
    /// `to_disconnect` (the slow-client policy).
    #[allow(clippy::too_many_arguments)] // one broadcast: mover identity + pose + delivery sink
    fn broadcast_move(
        &mut self,
        mover: PlayerId,
        position: Vec3,
        yaw: f32,
        pitch: f32,
        position_changed: bool,
        to_disconnect: &mut Vec<PlayerId>,
    ) {
        let Some(mover_entry) = self.players.get(&mover) else {
            return;
        };
        let mover_eid = mover_entry.entity_id;
        // Clone the mover's name + equipment before the loop: the per-viewer entries
        // are mutably borrowed inside it, so the immutable borrow of the mover entry
        // cannot survive (mirrors `broadcast_join_visibility`).
        let mover_name = mover_entry.name.clone();
        let mover_equipment = mover_entry.equipment.clone();
        let mover_chunk = chunk_for_position(position);
        let view_distance = self.view_distance;
        // Snapshot the in-range viewers so each entry can be mutated (advancing its
        // delivered baseline) inside the loop.
        let viewers: Vec<PlayerId> = self
            .players
            .iter()
            .filter(|(&other, entry)| {
                other != mover
                    && within_view(
                        mover_chunk,
                        chunk_for_position(entry.position),
                        view_distance,
                    )
            })
            .map(|(&other, _)| other)
            .collect();
        for viewer in viewers {
            let Some(entry) = self.players.get_mut(&viewer) else {
                continue;
            };
            if let Some(last) = entry.delivered.get(&mover_eid).copied() {
                if position_changed {
                    // Already in view, position changed: a relative/absolute move
                    // carrier plus a head turn, all droppable.
                    let packet =
                        relative_or_teleport_with_rotation(mover_eid, last, position, yaw, pitch);
                    match entry.outbound.try_send(OutboundMessage::droppable(packet)) {
                        // Advance the delivered baseline only on success (the
                        // self-correcting invariant; see the method and field docs).
                        Ok(()) => {
                            entry.delivered.insert(mover_eid, position);
                            Self::send_droppable(
                                entry,
                                set_head_rotation_shell(mover_eid, yaw),
                                viewer,
                                to_disconnect,
                            );
                        }
                        Err(TrySendError::Full(_)) => {}
                        Err(TrySendError::Closed(_)) => to_disconnect.push(viewer),
                    }
                } else {
                    // Rotation only: turn the body and head in place, baseline
                    // unchanged (no position moved).
                    Self::send_droppable(
                        entry,
                        update_entity_rotation_shell(mover_eid, yaw, pitch),
                        viewer,
                        to_disconnect,
                    );
                    Self::send_droppable(
                        entry,
                        set_head_rotation_shell(mover_eid, yaw),
                        viewer,
                        to_disconnect,
                    );
                }
            } else {
                // Entering view: spawn the mover (mandatory add + spawn) carrying the
                // facing, seed the baseline, then a droppable equipment send; never a
                // bare movement packet for an unspawned entity.
                Self::send_mandatory(
                    entry,
                    player_info_add(mover, &mover_name),
                    viewer,
                    to_disconnect,
                );
                Self::send_mandatory_spawn(
                    entry,
                    entity_spawn_shell(mover_eid, mover, position, yaw, pitch),
                    mover_eid,
                    position,
                    viewer,
                    to_disconnect,
                );
                if !mover_equipment.is_empty() {
                    Self::send_droppable(
                        entry,
                        set_equipment_shell(mover_eid, mover_equipment.clone()),
                        viewer,
                        to_disconnect,
                    );
                }
            }
        }
    }

    /// Broadcasts `player`'s changed equipment (the pre-encoded `equipment`
    /// [`SetEquipment`](set_equipment_shell) body — the full set: main hand, off
    /// hand, and armor) to every viewer that currently has `player` spawned, and
    /// refreshes the cached body so later viewers entering view receive it too.
    ///
    /// The body is built app-side (it owns the trusted Slot encoder) and passed
    /// opaque. Only viewers holding a [`delivered`](SessionEntry::delivered) baseline
    /// for the subject are sent the update — exactly those that have the entity
    /// spawned. The send is **droppable** and best-effort (cosmetic): a *full* or
    /// *closed* recipient simply misses it, matching the chat broadcasts; a closed
    /// channel is cleaned up when that connection's own loop ends. An unknown player
    /// is a silent no-op.
    pub fn set_equipment(&mut self, player: PlayerId, equipment: Vec<u8>) {
        let subject_eid = match self.players.get_mut(&player) {
            Some(entry) => {
                entry.equipment.clone_from(&equipment);
                entry.entity_id
            }
            None => return,
        };
        let packet = set_equipment_shell(subject_eid, equipment);
        for (&viewer, entry) in &self.players {
            if viewer == player {
                continue;
            }
            if entry.delivered.contains_key(&subject_eid) {
                let _ = entry
                    .outbound
                    .try_send(OutboundMessage::droppable(packet.clone()));
            }
        }
    }

    /// Broadcasts a block change at `position` to every player whose chunk is
    /// within view distance of the block's chunk, recording closed recipients.
    ///
    /// Unlike a movement broadcast there is no actor to exclude: the change is
    /// authoritative for everyone who can see the block, including the player who
    /// caused it. Sends are non-blocking; a recipient whose channel is *full*
    /// simply misses this update (the simulation still holds the correct world
    /// state) while a *closed* recipient is recorded in `closed` for the caller
    /// to disconnect.
    fn broadcast_block_update(
        &self,
        position: BlockPos,
        state: BlockStateId,
        to_disconnect: &mut Vec<PlayerId>,
    ) {
        let block_chunk = position.to_chunk_pos();
        for (&player, entry) in &self.players {
            if !within_view(
                block_chunk,
                chunk_for_position(entry.position),
                self.view_distance,
            ) {
                continue;
            }
            // An unrepresentable state id has no wire encoding; skip the broadcast
            // (it is droppable and the world stays correct) rather than diverge the
            // viewer. The entry boundary rejects such ids before they are stored, so
            // this is a defensive fail-safe.
            if let Some(packet) = block_update_shell(position, state) {
                Self::send_droppable(entry, packet, player, to_disconnect);
            }
        }
    }

    /// Broadcasts a pre-built `BlockEntityData` packet (a sign's text) to every
    /// viewer within view distance of `position`, cloning it per recipient.
    ///
    /// Droppable, mirroring [`broadcast_block_update`](Self::broadcast_block_update):
    /// a *full* recipient simply misses this render (the simulation still holds the
    /// authoritative sign, re-sent when the chunk next streams in) while a *closed*
    /// recipient is recorded in `to_disconnect`.
    fn broadcast_block_entity_data(
        &self,
        position: BlockPos,
        packet: &ClientboundPlayPacket,
        to_disconnect: &mut Vec<PlayerId>,
    ) {
        let block_chunk = position.to_chunk_pos();
        for (&player, entry) in &self.players {
            if !within_view(
                block_chunk,
                chunk_for_position(entry.position),
                self.view_distance,
            ) {
                continue;
            }
            Self::send_droppable(entry, packet.clone(), player, to_disconnect);
        }
    }

    /// Registers a renderable non-player entity and spawns it into view for every
    /// viewer in range, seeding each viewer's [`delivered`](SessionEntry::delivered)
    /// baseline.
    ///
    /// A network entity id is allocated from the shared player/entity counter (so
    /// the wire id spaces never collide) and a stable UUID is derived from it. The
    /// spawn is **mandatory**: a dropped one leaves an entity the viewer can never
    /// see move (a later position packet for an unspawned id is ignored), so a
    /// viewer that cannot accept it is disconnected — the same slow-client policy
    /// as a player spawn.
    ///
    /// A [`SpawnedEntityKind`] with no wire entity type yet (`Simple`) is ignored:
    /// nothing is registered or drawn, so a type-agnostic spawn stays invisible
    /// until its typed consumer lands. Only viewers within
    /// [`view_distance`](Self::view_distance) chunks of the entity are notified; a
    /// viewer that enters range later is spawned lazily by
    /// [`broadcast_entity_move`](Self::broadcast_entity_move).
    /// Snapshots the ids of every connected player within
    /// [`view_distance`](Self::view_distance) chunks of `chunk`.
    ///
    /// Returned as an owned `Vec` so the caller can then borrow `self.players`
    /// mutably per recipient: the in-range filter is an immutable borrow that must
    /// finish first. Shared by the entity spawn and move broadcasts, which scope to
    /// the same square view as [`broadcast_move`](Self::broadcast_move).
    fn viewers_in_range(&self, chunk: ChunkPos) -> Vec<PlayerId> {
        let view_distance = self.view_distance;
        self.players
            .iter()
            .filter(|(_, entry)| {
                within_view(chunk, chunk_for_position(entry.position), view_distance)
            })
            .map(|(&viewer, _)| viewer)
            .collect()
    }

    fn spawn_entity_view(
        &mut self,
        entity: EntityId,
        position: Vec3,
        kind: SpawnedEntityKind,
        to_disconnect: &mut Vec<PlayerId>,
    ) {
        // Only kinds with a wire entity type get a network id and packet; a Simple
        // entity has no renderer yet, so registering it would leak an id and a map
        // entry for something that never draws. Extend as typed consumers land.
        let network_id = self.allocate_entity_id();
        let view = EntityView {
            network_id,
            uuid: entity_uuid(network_id),
            position,
            kind,
        };
        let Some(spawn) = entity_spawn_packet(&view) else {
            return;
        };
        for viewer in self.viewers_in_range(chunk_for_position(position)) {
            if let Some(entry) = self.players.get_mut(&viewer) {
                Self::send_mandatory_spawn(
                    entry,
                    spawn.clone(),
                    network_id,
                    position,
                    viewer,
                    to_disconnect,
                );
            }
        }
        self.entities.insert(entity, view);
    }

    /// Broadcasts a non-player entity's new `position` to every viewer in range,
    /// mirroring [`broadcast_move`](Self::broadcast_move) without the tab-list add
    /// (entities are not player-list entries).
    ///
    /// A viewer that already has the entity (holds a
    /// [`delivered`](SessionEntry::delivered) baseline for it) gets a **droppable**
    /// relative-or-teleport carrier and advances its baseline only on a successful
    /// send (the self-correcting invariant: a dropped move leaves the baseline
    /// stale so the next delta grows and promotes to an absolute teleport). A
    /// viewer with no baseline is *entering view* and is spawned into it
    /// (**mandatory**), seeding the baseline. Unknown entities (never registered,
    /// or a `Simple` kind) are a silent no-op.
    fn broadcast_entity_move(
        &mut self,
        entity: EntityId,
        position: Vec3,
        to_disconnect: &mut Vec<PlayerId>,
    ) {
        let Some(view) = self.entities.get_mut(&entity) else {
            return;
        };
        view.position = position;
        let network_id = view.network_id;
        // Rebuild the spawn once for any viewers entering view this tick.
        let spawn = entity_spawn_packet(view);
        for viewer in self.viewers_in_range(chunk_for_position(position)) {
            let Some(entry) = self.players.get_mut(&viewer) else {
                continue;
            };
            if let Some(last) = entry.delivered.get(&network_id).copied() {
                // A falling block does not rotate, so the yaw/pitch are zero; the
                // carrier still conveys the position delta (or teleports when it
                // overflows the relative range).
                let packet =
                    relative_or_teleport_with_rotation(network_id, last, position, 0.0, 0.0);
                match entry.outbound.try_send(OutboundMessage::droppable(packet)) {
                    Ok(()) => {
                        entry.delivered.insert(network_id, position);
                    }
                    Err(TrySendError::Full(_)) => {}
                    Err(TrySendError::Closed(_)) => to_disconnect.push(viewer),
                }
            } else if let Some(spawn) = spawn.clone() {
                Self::send_mandatory_spawn(
                    entry,
                    spawn,
                    network_id,
                    position,
                    viewer,
                    to_disconnect,
                );
            }
        }
    }

    /// Despawns a non-player entity: removes its registration and sends a
    /// **mandatory** [`remove_entities_shell`] to every viewer that ever received
    /// it, pruning that viewer's [`delivered`](SessionEntry::delivered) baseline.
    ///
    /// Only viewers holding a baseline for the entity are notified — exactly those
    /// that saw its spawn — so a viewer that never had it is not sent a remove for
    /// an id it never spawned. The remove is mandatory: a dropped one leaves a
    /// ghost entity frozen at its last position, so a viewer that cannot accept it
    /// is disconnected. An unknown entity is a silent no-op (a despawn with no
    /// matching spawn).
    fn despawn_entity_view(&mut self, entity: EntityId, to_disconnect: &mut Vec<PlayerId>) {
        let Some(view) = self.entities.remove(&entity) else {
            return;
        };
        let network_id = view.network_id;
        let viewers: Vec<PlayerId> = self.players.keys().copied().collect();
        for viewer in viewers {
            if let Some(entry) = self.players.get_mut(&viewer) {
                if entry.delivered.remove(&network_id).is_some() {
                    Self::send_mandatory(
                        entry,
                        remove_entities_shell(&[network_id]),
                        viewer,
                        to_disconnect,
                    );
                }
            }
        }
    }

    /// Disconnects `player`: removes them from the player list of every remaining
    /// player, drops the player<->shard mapping, and notifies the shard to
    /// despawn them. Returns the shard `player` was on.
    ///
    /// The departure despawn is now **mandatory** (a dropped despawn ghosts the
    /// entity), so a viewer whose outbound channel is full during the leave
    /// broadcast must itself be disconnected — which broadcasts *another* leave.
    /// That cascade is drained here with an iterative worklist (never recursion):
    /// it is bounded because each player is attempted at most once per pass. A
    /// rejected cascade leave keeps that player's mapping and is deduplicated into
    /// the player-bounded pending set; the rest of the worklist is still drained.
    ///
    /// # Errors
    ///
    /// - [`SessionError::UnknownPlayer`] if `player` had no session.
    /// - [`SessionError::ShardInboxFull`] / [`SessionError::ShardClosed`] if any
    ///   required leave could not be delivered. That player's mapping remains so
    ///   the caller can retry or terminate explicitly rather than orphaning a sim
    ///   entity.
    /// - [`SessionError::StaleShardBinding`] if a required player's endpoint
    ///   lineage was removed or replaced by a new registration.
    pub fn disconnect_player(&mut self, player: PlayerId) -> Result<ShardPos, SessionError> {
        self.disconnect_player_owned(player)
            .map_err(InputDeliveryError::into_error)
    }

    /// Disconnects `player` while preserving an undelivered leave input.
    ///
    /// # Errors
    ///
    /// Returns an [`InputDeliveryError`] for the original player or a
    /// cascade-disconnected slow viewer whose control-lane leave could not be
    /// accepted. A failed player's mapping remains registered.
    pub fn disconnect_player_owned(
        &mut self,
        player: PlayerId,
    ) -> Result<ShardPos, InputDeliveryError> {
        let mut worklist = Vec::new();
        let shard = match self.disconnect_one_owned(player, &mut worklist) {
            Ok(shard) => {
                self.pending_disconnects.remove(&player);
                shard
            }
            Err(err) => {
                self.pending_disconnects.insert(player);
                return Err(err);
            }
        };
        if let Some(err) = self.drain_disconnect_worklist(worklist) {
            return Err(err);
        }
        Ok(shard)
    }

    /// Retries every retained slow-session leave against the bounded control
    /// lane.
    ///
    /// Successful leaves are removed from the pending set. A leave rejected by
    /// a still-full or closed shard remains pending, and the first exact rejected
    /// [`GameInput::PlayerLeave`] is returned to the caller. Any additional
    /// failures from the same bounded pass remain represented by their player id
    /// for a later retry.
    ///
    /// # Errors
    ///
    /// Returns an [`InputDeliveryError`] if at least one retained leave is still
    /// rejected by capacity, closure, or a stale endpoint lineage. The
    /// corresponding player mapping remains registered.
    pub fn retry_pending_disconnects(&mut self) -> Result<(), InputDeliveryError> {
        let worklist = self.pending_disconnects.iter().copied().collect();
        if let Some(err) = self.drain_disconnect_worklist(worklist) {
            return Err(err);
        }
        Ok(())
    }

    /// Drains a slow-client leave cascade without ever discarding a rejected
    /// control input.
    ///
    /// Every failure is deduplicated into `pending_disconnects`; the first exact
    /// rejection is returned for callers that can surface it immediately. The
    /// worklist and `attempted` set are both bounded by the connected player map.
    fn drain_disconnect_worklist(
        &mut self,
        mut worklist: Vec<PlayerId>,
    ) -> Option<InputDeliveryError> {
        let mut attempted = BTreeSet::new();
        let mut first_error = None;
        while let Some(next) = worklist.pop() {
            if !attempted.insert(next) {
                continue;
            }
            if !self.players.contains_key(&next) {
                self.pending_disconnects.remove(&next);
                continue;
            }
            match self.disconnect_one_owned(next, &mut worklist) {
                Ok(_) => {
                    self.pending_disconnects.remove(&next);
                }
                Err(err) => {
                    self.pending_disconnects.insert(next);
                    if first_error.is_none() {
                        first_error = Some(err);
                    }
                }
            }
        }
        first_error
    }

    /// Delivers one player's leave, removes their session, and broadcasts their
    /// departure. Viewers whose mandatory despawn overflows are pushed onto the
    /// cascade worklist [`disconnect_player_owned`](Self::disconnect_player_owned)
    /// drains.
    fn disconnect_one_owned(
        &mut self,
        player: PlayerId,
        also_disconnect: &mut Vec<PlayerId>,
    ) -> Result<ShardPos, InputDeliveryError> {
        let leave = GameInput::PlayerLeave { player };
        let policy = InputDeliveryPolicy::for_input(&leave);
        let entry = self.players.get(&player).ok_or_else(|| {
            InputDeliveryError::new(
                leave.clone(),
                policy,
                SessionError::UnknownPlayer { player },
            )
        })?;
        let shard = entry.shard;
        self.send_to_shard_owned(shard, leave, policy)?;
        let Some(entry) = self.players.remove(&player) else {
            return Err(InputDeliveryError::new(
                GameInput::PlayerLeave { player },
                policy,
                SessionError::UnknownPlayer { player },
            ));
        };
        self.broadcast_leave_visibility(player, entry.entity_id, also_disconnect);
        Ok(shard.home.position())
    }

    /// Tells every remaining player that *had `departed` in view* to drop it
    /// (whose network id is `entity_id`) from both their tab list and their world,
    /// and prunes the departed subject from each notified viewer's delivered
    /// baseline.
    ///
    /// Only viewers holding a [`delivered`](SessionEntry::delivered) baseline for
    /// `entity_id` are notified. That baseline is seeded together with the tab-list
    /// add and spawn (in [`broadcast_join_visibility`](Self::broadcast_join_visibility)
    /// and the enter branch of [`broadcast_move`](Self::broadcast_move)), so it is
    /// exactly the set of viewers that ever received the entity. A viewer that never
    /// saw it has neither a tab-list entry nor a spawned body to clear, so sending it
    /// a *mandatory* remove would clean up nothing while risking a spurious
    /// disconnect of a slow, far-away client (a full channel forces the cascade) for
    /// an entity it never had.
    ///
    /// Each notified viewer gets two **mandatory** packets: a [`player_info_remove`]
    /// (Player Info Remove, `0x3E`) to clear the tab-list entry, and a
    /// [`remove_entities_shell`] (Remove Entities, `0x46`) to despawn the entity so
    /// it does not linger as a ghost. A viewer that cannot accept either is pushed to
    /// `to_disconnect`: a dropped despawn is exactly the ghost-entity bug this fixes,
    /// so it is never silently dropped.
    fn broadcast_leave_visibility(
        &mut self,
        departed: PlayerId,
        entity_id: i32,
        to_disconnect: &mut Vec<PlayerId>,
    ) {
        let viewers: Vec<PlayerId> = self.players.keys().copied().collect();
        for viewer in viewers {
            if let Some(entry) = self.players.get_mut(&viewer) {
                // Pruning the baseline both keeps the map bounded by live players and
                // gates the cleanup: a present entry means this viewer actually saw
                // the entity, so it (and only it) is sent the mandatory removes.
                if entry.delivered.remove(&entity_id).is_some() {
                    Self::send_mandatory(
                        entry,
                        player_info_remove(departed),
                        viewer,
                        to_disconnect,
                    );
                    Self::send_mandatory(
                        entry,
                        remove_entities_shell(&[entity_id]),
                        viewer,
                        to_disconnect,
                    );
                }
            }
        }
    }

    /// Broadcasts a System Chat Message carrying `component` to **every**
    /// connected player (including the sender, matching vanilla chat relay).
    ///
    /// `overlay = true` renders the message above the hotbar (action bar);
    /// `overlay = false` renders it in the chat box. The packet is built once via
    /// [`crate::system_chat`] and a clone is delivered to each player's outbound
    /// channel.
    ///
    /// # Backpressure
    ///
    /// Best-effort and lossy, like the visibility broadcasts: a recipient whose
    /// outbound channel is *full* or *closed* simply misses this message (a closed
    /// channel is cleaned up when that connection's own loop ends). A dropped chat
    /// line is never worth stalling the driver, so this never blocks and never
    /// fails.
    pub fn broadcast_system_chat(&self, component: &TextComponent, overlay: bool) {
        let packet = crate::system_chat(component, overlay);
        for entry in self.players.values() {
            let _ = entry
                .outbound
                .try_send(OutboundMessage::droppable(packet.clone()));
        }
    }

    /// Sends a System Chat Message carrying `component` to a single `player`.
    ///
    /// The targeted counterpart to
    /// [`broadcast_system_chat`](Self::broadcast_system_chat): used to deliver a
    /// plugin's `Message` intent to its named recipient (rather than every player)
    /// when that recipient is not the acting connection. An unknown player is a
    /// silent no-op. `overlay = true` renders the message above the hotbar (action
    /// bar); `overlay = false` renders it in the chat box.
    ///
    /// # Backpressure
    ///
    /// Best-effort and lossy, like
    /// [`broadcast_system_chat`](Self::broadcast_system_chat): a recipient whose
    /// outbound channel is *full* or *closed* simply misses the message. A dropped
    /// chat line never stalls the driver, so this never blocks and never fails.
    pub fn send_system_chat_to(&self, player: PlayerId, component: &TextComponent, overlay: bool) {
        if let Some(entry) = self.players.get(&player) {
            let packet = crate::system_chat(component, overlay);
            let _ = entry.outbound.try_send(OutboundMessage::droppable(packet));
        }
    }

    /// Broadcasts a clientbound play `packet` to every connected player.
    ///
    /// The generic counterpart to
    /// [`broadcast_system_chat`](Self::broadcast_system_chat) for a server-wide
    /// packet the driver builds at the app layer — e.g. the `GameEvent` a
    /// `/weather` command toggles. Only the router holds every player's outbound
    /// channel, so a server-wide send must route through here.
    ///
    /// # Backpressure
    ///
    /// Best-effort and lossy, exactly like the visibility/chat broadcasts: a
    /// recipient whose outbound channel is *full* or *closed* simply misses the
    /// packet. This never blocks and never fails.
    pub fn broadcast_play_packet(&self, packet: &ClientboundPlayPacket) {
        for entry in self.players.values() {
            let _ = entry
                .outbound
                .try_send(OutboundMessage::droppable(packet.clone()));
        }
    }

    /// Sends a clientbound play `packet` to a single `player`. An unknown player is
    /// a silent no-op.
    ///
    /// The targeted counterpart to [`broadcast_play_packet`](Self::broadcast_play_packet),
    /// for a packet the driver builds at the app layer and aims at one player the
    /// connection cannot reach directly — e.g. the `change_game_mode` `GameEvent`
    /// sent to a `/gamemode <mode> <player>` target.
    ///
    /// # Backpressure
    ///
    /// Best-effort and lossy: a recipient whose outbound channel is *full* or
    /// *closed* simply misses the packet. This never blocks and never fails.
    pub fn send_play_packet_to(&self, player: PlayerId, packet: ClientboundPlayPacket) {
        if let Some(entry) = self.players.get(&player) {
            let _ = entry.outbound.try_send(OutboundMessage::droppable(packet));
        }
    }

    /// Teleports `player` to `position`: snaps the target's own client to the new
    /// position and updates authoritative simulation state.
    ///
    /// Two effects, both required. A **mandatory** `SynchronizePlayerPosition` is
    /// sent to the target's own channel — the sim's `PlayerMoved` only broadcasts to
    /// *other* viewers, so without this the target itself would never move — and a
    /// [`GameInput::PlayerMove`] is routed to the target's shard so the
    /// authoritative position updates and in-range viewers see the move at the next
    /// tick. Used to fulfil a plugin's `Teleport` intent, which the connection task
    /// cannot satisfy directly (it cannot reach another player's outbound channel).
    ///
    /// The position-sync is mandatory: a target that cannot accept it — *full* or
    /// *closed* — is disconnected (the slow-client policy), exactly like a
    /// [`GameOutput::PlayerPositionCorrected`].
    ///
    /// The authoritative [`GameInput::PlayerMove`] routes to the player's *current*
    /// owning shard, not the destination chunk's shard, so a teleport that crosses a
    /// shard boundary leaves the player simulated by the origin shard while standing
    /// in another shard's region (no cross-shard entity transfer is performed — that
    /// is out of scope for this milestone). Viewer visibility is unaffected because
    /// moves broadcast off the router's global position cache, not per-shard; this is
    /// flagged for the eventual cross-shard-transfer work.
    ///
    /// # Errors
    ///
    /// - [`SessionError::UnknownPlayer`] if `player` has no session.
    /// - [`SessionError::ShardInboxFull`] / [`SessionError::ShardClosed`] if the
    ///   authoritative move could not be delivered to the shard.
    /// - [`SessionError::StaleShardBinding`] if the player's endpoint lineage is
    ///   no longer current.
    pub fn teleport_player(
        &mut self,
        player: PlayerId,
        position: Vec3,
    ) -> Result<(), SessionError> {
        self.teleport_player_owned(player, position)
            .map_err(InputDeliveryError::into_error)
    }

    /// Teleports `player` while preserving the authoritative move if its bounded
    /// shard delivery is rejected.
    ///
    /// The shard move is accepted before the clientbound position sync is sent,
    /// so a full shard queue can never preview a teleport that authoritative
    /// simulation did not accept.
    ///
    /// # Errors
    ///
    /// Returns an [`InputDeliveryError`] for the teleport move or for a required
    /// slow-client leave that could not enter the control reserve.
    pub fn teleport_player_owned(
        &mut self,
        player: PlayerId,
        position: Vec3,
    ) -> Result<(), InputDeliveryError> {
        let input = GameInput::PlayerMove {
            player,
            position: Some(position),
            yaw: None,
            pitch: None,
        };
        let policy = InputDeliveryPolicy::authoritative(DeliveryLane::Data);
        let Some(outbound) = self
            .players
            .get(&player)
            .map(|entry| entry.outbound.clone())
        else {
            return Err(InputDeliveryError::new(
                input,
                policy,
                SessionError::UnknownPlayer { player },
            ));
        };
        self.route_input_with_policy(player, input, policy)?;

        // The target's authoritative move is now accepted. If its mandatory sync
        // cannot be queued, tear the session down through the control-lane leave.
        if outbound
            .try_send(OutboundMessage::mandatory(move_shell(position)))
            .is_err()
        {
            self.disconnect_player_owned(player)?;
        }
        Ok(())
    }

    /// Routes an already-translated input to the shard that should apply it.
    ///
    /// A block edit is routed by the *block's* chunk (see
    /// [`binding_for_block`](Self::binding_for_block)); every other input routes to
    /// the player's bound shard.
    fn route_input_owned(
        &self,
        player: PlayerId,
        input: GameInput,
    ) -> Result<(), InputDeliveryError> {
        let policy = InputDeliveryPolicy::for_input(&input);
        self.route_input_with_policy(player, input, policy)
    }

    /// Routes `input` with an explicit policy override.
    fn route_input_with_policy(
        &self,
        player: PlayerId,
        input: GameInput,
        policy: InputDeliveryPolicy,
    ) -> Result<(), InputDeliveryError> {
        let Some(entry) = self.players.get(&player) else {
            return Err(InputDeliveryError::new(
                input,
                policy,
                SessionError::UnknownPlayer { player },
            ));
        };
        let shard = match &input {
            GameInput::BlockBreak { position, .. }
            | GameInput::BlockPlace { position, .. }
            | GameInput::SetBlockExact { position, .. }
            | GameInput::RejectBlockEdit { position, .. } => {
                self.binding_for_block(*position, entry.shard)
            }
            _ => entry.shard,
        };
        self.send_to_shard_owned(shard, input, policy)
    }

    /// Resolves the shard that should apply a block edit at `position`.
    ///
    /// The minimal multi-shard seam: a block edit belongs to the shard owning the
    /// block's *chunk*, not to the acting player's bound shard — the two diverge
    /// once view distance exceeds a shard's 8-chunk span and a player edits a
    /// block another shard owns. When that owning shard is registered it wins;
    /// otherwise the edit falls back to the player's bound shard (`fallback`). In
    /// this single-shard milestone both resolve to the same shard, so routing is
    /// unchanged — the seam only positions the router for real multi-shard later.
    fn binding_for_block(&self, position: BlockPos, fallback: ShardBinding) -> ShardBinding {
        let Some(current_fallback) = self.shards.current(fallback.coverage) else {
            return fallback;
        };
        if current_fallback.registration_id() != fallback.registration_id
            || current_fallback.endpoint().home != fallback.home
        {
            return fallback;
        }
        let target = ShardPartitioner::for_block(DEFAULT_WORLD, DEFAULT_DIMENSION, position);
        self.shards.resolve(target).map_or(fallback, |resolved| {
            if resolved.coverage() == fallback.coverage {
                fallback
            } else {
                ShardBinding {
                    coverage: resolved.coverage(),
                    registration_id: resolved.registration_id(),
                    home: resolved.endpoint().home,
                }
            }
        })
    }

    /// Non-blocking send of `input` to `shard`'s input channel.
    fn send_to_shard(&self, shard: ShardBinding, input: GameInput) -> Result<(), SessionError> {
        let policy = InputDeliveryPolicy::for_input(&input);
        self.send_to_shard_owned(shard, input, policy)
            .map_err(InputDeliveryError::into_error)
    }

    /// Non-blocking shard send that returns the exact input on rejection.
    fn send_to_shard_owned(
        &self,
        binding: ShardBinding,
        input: GameInput,
        policy: InputDeliveryPolicy,
    ) -> Result<(), InputDeliveryError> {
        let current = self.shards.current(binding.coverage);
        let Some(current) = current else {
            return Err(InputDeliveryError::new(
                input,
                policy,
                SessionError::StaleShardBinding {
                    home: binding.home,
                    registration_id: binding.registration_id,
                    current_registration_id: None,
                },
            ));
        };
        if current.registration_id() != binding.registration_id
            || current.endpoint().home != binding.home
        {
            return Err(InputDeliveryError::new(
                input,
                policy,
                SessionError::StaleShardBinding {
                    home: binding.home,
                    registration_id: binding.registration_id,
                    current_registration_id: Some(current.registration_id()),
                },
            ));
        }
        let endpoint = current.endpoint();
        let sender = &endpoint.sender;
        let shard = endpoint.home.position();

        // The router is the sole sender. Refusing ordinary data once only the
        // reserved tail remains cannot race another producer; the receiver may
        // only free more capacity between this check and `try_send`.
        if policy.lane() == DeliveryLane::Data && sender.capacity() <= self.shard_control_reserve {
            return Err(InputDeliveryError::new(
                input,
                policy,
                SessionError::ShardInboxFull { shard },
            ));
        }

        sender.try_send(input).map_err(|err| match err {
            TrySendError::Full(input) => {
                InputDeliveryError::new(input, policy, SessionError::ShardInboxFull { shard })
            }
            TrySendError::Closed(input) => {
                InputDeliveryError::new(input, policy, SessionError::ShardClosed { shard })
            }
        })
    }
}

/// Builds the movement carrier for a move from `last` to `position` facing
/// `yaw`/`pitch` (degrees): a relative
/// [`update_entity_position_and_rotation_shell`] when every axis fits the i16
/// fixed-point range, otherwise an absolute [`entity_teleport_shell`].
///
/// Selecting per-axis (rather than by a single squared-distance threshold) makes
/// the choice exactly match the wire encoding's limit, so a delta that would
/// overflow `i16` always teleports instead of silently saturating. Both carriers
/// convey the rotation: the relative one as angle bytes, the teleport as f32
/// degrees (its native form).
fn relative_or_teleport_with_rotation(
    entity_id: i32,
    last: Vec3,
    position: Vec3,
    yaw: f32,
    pitch: f32,
) -> ClientboundPlayPacket {
    match update_entity_position_and_rotation_shell(
        entity_id,
        position.x - last.x,
        position.y - last.y,
        position.z - last.z,
        yaw,
        pitch,
    ) {
        Some(relative) => relative,
        None => entity_teleport_shell(entity_id, position, yaw, pitch),
    }
}

/// Returns `true` if chunks `a` and `b` are within `view_distance` chunks on
/// both axes (a square/Chebyshev view, as Minecraft scopes view distance).
fn within_view(a: ChunkPos, b: ChunkPos, view_distance: i32) -> bool {
    let dx = (a.x() - b.x()).abs();
    let dz = (a.z() - b.z()).abs();
    dx.max(dz) <= view_distance
}

/// Builds the clientbound spawn shell for `view`, or `None` for a kind with no
/// wire entity type yet.
///
/// The one bridge from a stored [`EntityView`] to its `SpawnEntity` packet, used
/// both on first spawn and when re-spawning the entity for a viewer that enters
/// view mid-flight, so both paths render it identically.
fn entity_spawn_packet(view: &EntityView) -> Option<ClientboundPlayPacket> {
    match view.kind {
        SpawnedEntityKind::FallingBlock { block } => {
            falling_block_spawn_shell(view.network_id, view.uuid, view.position, block)
        }
        // No renderer for a generic entity yet; extend as typed consumers land.
        _ => None,
    }
}

/// Derives a stable UUID for a non-player entity from its network id.
///
/// A falling block has no [`PlayerId`] to source a UUID from, but a
/// `SpawnEntity` packet still carries one. The value is cosmetic for a
/// non-player entity (the client keys rendering on the numeric entity id, not the
/// UUID), so a deterministic id built from the four network-id bytes is enough:
/// it is distinct per entity and needs no random source.
fn entity_uuid(network_id: i32) -> uuid::Uuid {
    let mut bytes = [0u8; 16];
    bytes[..4].copy_from_slice(&network_id.to_be_bytes());
    uuid::Uuid::from_bytes(bytes)
}

impl Default for SessionRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    // Decoded shell coordinates are exact, representable values, so exact float
    // comparison is intentional in these assertions.
    #![allow(clippy::float_cmp)]

    use ferrumc_core::GameMode;
    use ferrumc_math::{Cuboid, Direction};
    use ferrumc_net::DisconnectReason;
    use ferrumc_proto::generated::play::{GameEvent, ServerboundKeepAlive, ServerboundPlayPacket};
    use ferrumc_sim::{RegionOp, Sign, SignKind};

    use super::*;
    use crate::translate::PLAYER_INFO_ADD;
    use crate::DeliveryPolicy;

    fn player(name: &str) -> PlayerId {
        PlayerId::offline(name)
    }

    /// Spawn position inside shard (0, 0): chunk 0, well within blocks 0..128.
    fn spawn_pos() -> Vec3 {
        Vec3::new(8.0, 64.0, 8.0)
    }

    #[test]
    #[allow(clippy::too_many_lines)] // The table intentionally enumerates every current input variant.
    fn input_delivery_policy_classifies_every_current_variant() {
        let p = player("policy");
        let block = BlockPos::new(8, 64, 8);
        let cases = [
            (
                GameInput::PlayerJoin {
                    player: p,
                    position: spawn_pos(),
                },
                DeliveryPolicy::Authoritative,
                DeliveryLane::Control,
            ),
            (
                GameInput::PlayerMove {
                    player: p,
                    position: Some(spawn_pos()),
                    yaw: None,
                    pitch: None,
                },
                DeliveryPolicy::Coalescible,
                DeliveryLane::Data,
            ),
            (
                GameInput::PlayerLeave { player: p },
                DeliveryPolicy::Authoritative,
                DeliveryLane::Control,
            ),
            (
                GameInput::BlockBreak {
                    player: p,
                    position: block,
                    sequence: 1,
                },
                DeliveryPolicy::Authoritative,
                DeliveryLane::Data,
            ),
            (
                GameInput::BlockPlace {
                    player: p,
                    position: block,
                    sequence: 2,
                    state: BlockStateId::new(1),
                    clicked_face: Direction::Up,
                    cursor_position: Vec3::new(0.5, 1.0, 0.5),
                    player_yaw: 0.0,
                },
                DeliveryPolicy::Authoritative,
                DeliveryLane::Data,
            ),
            (
                GameInput::SetBlockExact {
                    player: p,
                    position: block,
                    sequence: 3,
                    state: BlockStateId::new(2),
                },
                DeliveryPolicy::Authoritative,
                DeliveryLane::Data,
            ),
            (
                GameInput::RejectBlockEdit {
                    player: p,
                    position: block,
                    sequence: 4,
                    requested_state: BlockStateId::AIR,
                },
                DeliveryPolicy::Authoritative,
                DeliveryLane::Control,
            ),
            (
                GameInput::SetGameMode {
                    player: p,
                    mode: GameMode::Creative,
                },
                DeliveryPolicy::Authoritative,
                DeliveryLane::Data,
            ),
            (
                GameInput::RegionEdit {
                    player: p,
                    region: Cuboid::new(block, block),
                    op: RegionOp::Fill {
                        state: BlockStateId::new(3),
                    },
                },
                DeliveryPolicy::Authoritative,
                DeliveryLane::Data,
            ),
            (
                GameInput::RegionUndo { player: p },
                DeliveryPolicy::Authoritative,
                DeliveryLane::Data,
            ),
            (
                GameInput::UpdateSign {
                    player: p,
                    position: block,
                    is_front: true,
                    lines: std::array::from_fn(|index| format!("line {index}")),
                },
                DeliveryPolicy::Authoritative,
                DeliveryLane::Data,
            ),
        ];

        for (input, expected_policy, expected_lane) in cases {
            let policy = InputDeliveryPolicy::for_input(&input);
            assert_eq!(policy.policy(), expected_policy, "input: {input:?}");
            assert_eq!(policy.lane(), expected_lane, "input: {input:?}");
        }
        assert_ne!(DeliveryPolicy::BestEffort, DeliveryPolicy::Authoritative,);
    }

    #[test]
    fn control_lane_remains_available_after_data_capacity_is_reserved() {
        let mut router = SessionRouter::with_capacities_and_control_reserve(3, 16, 1);
        let mut inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("reserve");
        let _handle = router.join_player(p, "reserve", spawn_pos()).expect("join");
        assert!(matches!(
            inbox.try_recv(),
            Ok(GameInput::PlayerJoin { player, .. }) if player == p
        ));

        for mode in [GameMode::Survival, GameMode::Creative] {
            router
                .route_game_input_owned(p, GameInput::SetGameMode { player: p, mode })
                .expect("data slot remains");
        }
        let rejected = GameInput::SetGameMode {
            player: p,
            mode: GameMode::Adventure,
        };
        let err = router
            .route_game_input_owned(p, rejected.clone())
            .expect_err("data traffic stops at the control reserve");
        assert_eq!(err.input(), &rejected);
        assert_eq!(err.policy().policy(), DeliveryPolicy::Authoritative);
        assert_eq!(err.policy().lane(), DeliveryLane::Data);
        assert!(matches!(
            err.error(),
            SessionError::ShardInboxFull { shard } if *shard == ShardPos::new(0, 0)
        ));

        let heal = GameInput::RejectBlockEdit {
            player: p,
            position: BlockPos::new(8, 64, 8),
            sequence: 9,
            requested_state: BlockStateId::AIR,
        };
        router
            .route_game_input_owned(p, heal.clone())
            .expect("control input consumes the reserved slot");

        assert!(matches!(
            inbox.try_recv(),
            Ok(GameInput::SetGameMode {
                mode: GameMode::Survival,
                ..
            })
        ));
        assert!(matches!(
            inbox.try_recv(),
            Ok(GameInput::SetGameMode {
                mode: GameMode::Creative,
                ..
            })
        ));
        assert_eq!(inbox.try_recv(), Ok(heal));
    }

    #[test]
    fn join_uses_control_reserve_after_data_lane_stops() {
        let mut router = SessionRouter::with_capacities_and_control_reserve(2, 16, 1);
        let mut inbox = router.register_shard(ShardPos::new(0, 0));
        let first = player("first");
        let second = player("second");
        let _first_handle = router
            .join_player(first, "first", spawn_pos())
            .expect("first join");
        assert!(matches!(
            inbox.try_recv(),
            Ok(GameInput::PlayerJoin { player, .. }) if player == first
        ));
        router
            .route_game_input_owned(
                first,
                GameInput::SetGameMode {
                    player: first,
                    mode: GameMode::Creative,
                },
            )
            .expect("one data slot");

        let rejected = router
            .route_game_input_owned(first, GameInput::RegionUndo { player: first })
            .expect_err("ordinary data cannot consume the reserve");
        assert_eq!(rejected.policy().lane(), DeliveryLane::Data);

        let _second_handle = router
            .join_player(second, "second", spawn_pos())
            .expect("join consumes the control reserve");
        assert!(matches!(
            inbox.try_recv(),
            Ok(GameInput::SetGameMode { player, .. }) if player == first
        ));
        assert!(matches!(
            inbox.try_recv(),
            Ok(GameInput::PlayerJoin { player, .. }) if player == second
        ));
    }

    #[test]
    fn route_event_rejection_returns_the_translated_coalescible_input() {
        let mut router = SessionRouter::with_capacities(1, 16);
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("event-overload");
        let _handle = router
            .join_player(p, "event-overload", spawn_pos())
            .expect("join");
        let destination = Vec3::new(12.0, 65.0, 14.0);
        let event = NetEvent::play(
            p,
            ServerboundPlayPacket::SetPlayerPosition(
                ferrumc_proto::generated::play::SetPlayerPosition::new(
                    destination.x,
                    destination.y,
                    destination.z,
                    0,
                ),
            ),
        );

        let err = router
            .route_event_owned(&event)
            .expect_err("join still occupies the only shard slot");
        assert_eq!(err.policy().policy(), DeliveryPolicy::Coalescible);
        assert_eq!(err.policy().lane(), DeliveryLane::Data);
        assert_eq!(
            err.input(),
            &GameInput::PlayerMove {
                player: p,
                position: Some(destination),
                yaw: None,
                pitch: None,
            }
        );
    }

    #[test]
    fn closed_shard_returns_the_owned_authoritative_input() {
        let mut router = SessionRouter::new();
        let mut inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("closed-shard");
        let _handle = router
            .join_player(p, "closed-shard", spawn_pos())
            .expect("join");
        let _ = inbox.try_recv();
        drop(inbox);
        let input = GameInput::RegionUndo { player: p };

        let err = router
            .route_game_input_owned(p, input.clone())
            .expect_err("closed shard rejects the input");
        let (returned, policy, error) = err.into_parts();
        assert_eq!(returned, input);
        assert_eq!(policy.policy(), DeliveryPolicy::Authoritative);
        assert_eq!(policy.lane(), DeliveryLane::Data);
        assert_eq!(
            error,
            SessionError::ShardClosed {
                shard: ShardPos::new(0, 0)
            }
        );
    }

    #[test]
    fn disconnect_keeps_mapping_until_control_leave_is_accepted() {
        let mut router = SessionRouter::with_capacities_and_control_reserve(2, 16, 1);
        let mut inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("leave-reserve");
        let _handle = router
            .join_player(p, "leave-reserve", spawn_pos())
            .expect("join");
        let heal = GameInput::RejectBlockEdit {
            player: p,
            position: BlockPos::new(8, 64, 8),
            sequence: 1,
            requested_state: BlockStateId::AIR,
        };
        router
            .route_game_input_owned(p, heal.clone())
            .expect("fill final control slot");

        let err = router
            .disconnect_player_owned(p)
            .expect_err("full control lane rejects leave explicitly");
        assert_eq!(err.input(), &GameInput::PlayerLeave { player: p });
        assert_eq!(err.policy().lane(), DeliveryLane::Control);
        assert!(router.is_player_connected(p));
        assert_eq!(router.pending_disconnect_count(), 1);

        assert!(matches!(
            inbox.try_recv(),
            Ok(GameInput::PlayerJoin { player, .. }) if player == p
        ));
        router
            .retry_pending_disconnects()
            .expect("explicit retry uses the freed control slot");
        assert!(!router.is_player_connected(p));
        assert_eq!(router.pending_disconnect_count(), 0);
        assert_eq!(inbox.try_recv(), Ok(heal));
        assert_eq!(inbox.try_recv(), Ok(GameInput::PlayerLeave { player: p }));
    }

    #[test]
    fn teleport_preview_waits_for_authoritative_delivery() {
        let mut router = SessionRouter::with_capacities(1, 16);
        let mut inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("teleport-overload");
        let mut handle = router
            .join_player(p, "teleport-overload", spawn_pos())
            .expect("join");
        let destination = Vec3::new(40.0, 70.0, 24.0);

        let err = router
            .teleport_player_owned(p, destination)
            .expect_err("join still occupies the shard queue");
        assert_eq!(err.policy().policy(), DeliveryPolicy::Authoritative);
        assert!(matches!(err.input(), GameInput::PlayerMove { .. }));
        assert!(
            handle.try_recv().is_none(),
            "no position sync may precede authoritative acceptance",
        );

        assert!(matches!(
            inbox.try_recv(),
            Ok(GameInput::PlayerJoin { player, .. }) if player == p
        ));
        router
            .teleport_player_owned(p, destination)
            .expect("retry after capacity is accepted");
        assert!(matches!(
            inbox.try_recv(),
            Ok(GameInput::PlayerMove { position: Some(position), .. }) if position == destination
        ));
        let ClientboundPlayPacket::SynchronizePlayerPosition(sync) = handle
            .try_recv()
            .expect("sync follows acceptance")
            .into_packet()
        else {
            panic!("expected a SynchronizePlayerPosition packet");
        };
        assert_eq!((sync.x(), sync.y(), sync.z()), (40.0, 70.0, 24.0));
    }

    #[test]
    fn join_routes_player_to_a_shard() {
        let mut router = SessionRouter::new();
        let mut inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("alice");

        let handle = router.join_player(p, "alice", spawn_pos()).expect("join");
        assert_eq!(handle.player(), p);
        assert_eq!(handle.shard(), ShardPos::new(0, 0));

        // The mapping records the player on the shard.
        assert!(router.is_player_connected(p));
        assert_eq!(router.player_shard(p), Some(ShardPos::new(0, 0)));
        assert_eq!(router.player_count(), 1);

        // The shard's inbox received exactly the PlayerJoin input.
        assert_eq!(
            inbox.try_recv(),
            Ok(GameInput::PlayerJoin {
                player: p,
                position: spawn_pos(),
            })
        );
    }

    /// Extracts the `GameEvent` `(reason, value)` from the next outbound packet on
    /// `handle`, panicking if it is not a `GameEvent`.
    fn next_game_event(handle: &mut PlayerSessionHandle) -> (u8, f32) {
        let ClientboundPlayPacket::GameEvent(event) =
            handle.try_recv().expect("a queued packet").into_packet()
        else {
            panic!("expected a GameEvent");
        };
        (event.reason(), event.value())
    }

    #[test]
    fn broadcast_play_packet_reaches_every_player() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let mut a = router
            .join_player(player("a"), "a", spawn_pos())
            .expect("join a");
        let mut b = router
            .join_player(player("b"), "b", spawn_pos())
            .expect("join b");
        // Drain the join-visibility traffic so only the broadcast remains.
        while a.try_recv().is_some() {}
        while b.try_recv().is_some() {}

        router.broadcast_play_packet(&ClientboundPlayPacket::GameEvent(GameEvent::new(1, 0.0)));

        // Both players receive the same start_raining (reason 1) GameEvent.
        assert_eq!(next_game_event(&mut a), (1, 0.0));
        assert_eq!(next_game_event(&mut b), (1, 0.0));
    }

    #[test]
    fn send_play_packet_to_reaches_only_the_target() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let mut a = router
            .join_player(player("a"), "a", spawn_pos())
            .expect("join a");
        let mut b = router
            .join_player(player("b"), "b", spawn_pos())
            .expect("join b");
        while a.try_recv().is_some() {}
        while b.try_recv().is_some() {}

        // change_game_mode (reason 3) carrying creative (1.0), aimed at b only.
        router.send_play_packet_to(
            player("b"),
            ClientboundPlayPacket::GameEvent(GameEvent::new(3, 1.0)),
        );

        assert_eq!(next_game_event(&mut b), (3, 1.0));
        assert!(a.try_recv().is_none(), "a is not the target");
    }

    #[test]
    fn send_play_packet_to_unknown_player_is_a_no_op() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        // No panic, no effect: there is simply no such session.
        router.send_play_packet_to(
            player("ghost"),
            ClientboundPlayPacket::GameEvent(GameEvent::new(3, 0.0)),
        );
    }

    #[test]
    fn join_without_a_shard_is_rejected() {
        let mut router = SessionRouter::new();
        let err = router
            .join_player(player("bob"), "bob", spawn_pos())
            .expect_err("no shard registered");
        assert_eq!(
            err,
            SessionError::UnknownShard {
                shard: ShardPos::new(0, 0)
            }
        );
        assert_eq!(router.player_count(), 0);
    }

    #[test]
    fn duplicate_join_is_rejected_without_touching_state() {
        let mut router = SessionRouter::new();
        let mut inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("carol");

        let _handle = router
            .join_player(p, "carol", spawn_pos())
            .expect("first join");
        let _ = inbox.try_recv();

        let err = router
            .join_player(p, "carol", spawn_pos())
            .expect_err("already joined");
        assert_eq!(err, SessionError::DuplicatePlayer { player: p });
        // No second PlayerJoin was sent.
        assert!(inbox.try_recv().is_err());
    }

    #[test]
    fn movement_event_routes_to_the_single_shard_as_player_move() {
        let mut router = SessionRouter::new();
        let mut inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("dave");
        let _handle = router.join_player(p, "dave", spawn_pos()).expect("join");
        assert_eq!(
            inbox.try_recv(),
            Ok(GameInput::PlayerJoin {
                player: p,
                position: spawn_pos(),
            })
        );

        // A movement packet far inside the same shard's region.
        let move_event = NetEvent::play(
            p,
            ServerboundPlayPacket::SetPlayerPosition(
                ferrumc_proto::generated::play::SetPlayerPosition::new(20.0, 64.0, 20.0, 0),
            ),
        );
        router.route_event(&move_event).expect("route move");
        assert_eq!(
            inbox.try_recv(),
            Ok(GameInput::PlayerMove {
                player: p,
                position: Some(Vec3::new(20.0, 64.0, 20.0)),
                yaw: None,
                pitch: None,
            })
        );
    }

    #[test]
    fn keep_alive_event_is_a_no_op() {
        let mut router = SessionRouter::new();
        let mut inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("erin");
        let _handle = router.join_player(p, "erin", spawn_pos()).expect("join");
        let _ = inbox.try_recv();

        router
            .route_event(&NetEvent::play(
                p,
                ServerboundPlayPacket::ServerboundKeepAlive(ServerboundKeepAlive::new(1)),
            ))
            .expect("route keep-alive");
        // Nothing reached the shard.
        assert!(inbox.try_recv().is_err());
    }

    #[test]
    fn play_event_for_unknown_player_is_rejected() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("ghost");
        let err = router
            .route_event(&NetEvent::play(
                p,
                ServerboundPlayPacket::SetPlayerPosition(
                    ferrumc_proto::generated::play::SetPlayerPosition::new(0.0, 0.0, 0.0, 0),
                ),
            ))
            .expect_err("no session");
        assert_eq!(err, SessionError::UnknownPlayer { player: p });
    }

    #[test]
    fn own_spawn_and_move_are_not_echoed_to_self() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("frank");
        let mut handle = router.join_player(p, "frank", spawn_pos()).expect("join");

        // A lone player is never shown their own entity, and an accepted move is
        // authoritative client-side: neither is echoed back.
        assert!(router
            .route_output(&GameOutput::PlayerSpawned {
                player: p,
                position: spawn_pos(),
            })
            .is_empty());
        assert!(router
            .route_output(&GameOutput::PlayerMoved {
                player: p,
                position: Vec3::new(4.0, 5.0, 6.0),
                yaw: 0.0,
                pitch: 0.0,
                position_changed: true,
            })
            .is_empty());
        assert!(handle.try_recv().is_none());
        // The move still refreshed the cached position used for routing.
        assert_eq!(router.player_position(p), Some(Vec3::new(4.0, 5.0, 6.0)));
    }

    #[test]
    fn correction_output_sends_a_synchronize_position_packet() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("nora");
        let mut handle = router.join_player(p, "nora", spawn_pos()).expect("join");

        let closed = router.route_output(&GameOutput::PlayerPositionCorrected {
            player: p,
            position: Vec3::new(8.0, 64.0, 8.0),
        });
        assert!(closed.is_empty());
        let ClientboundPlayPacket::SynchronizePlayerPosition(sync) =
            handle.try_recv().expect("a sync packet").into_packet()
        else {
            panic!("expected a SynchronizePlayerPosition packet");
        };
        assert_eq!((sync.x(), sync.y(), sync.z()), (8.0, 64.0, 8.0));
    }

    #[test]
    fn despawn_output_sends_nothing() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("grace");
        let mut handle = router.join_player(p, "grace", spawn_pos()).expect("join");

        let closed = router.route_output(&GameOutput::PlayerDespawned { player: p });
        assert!(closed.is_empty());
        assert!(handle.try_recv().is_none());
    }

    #[test]
    fn output_for_unknown_player_is_a_no_op() {
        let mut router = SessionRouter::new();
        let closed = router.route_output(&GameOutput::PlayerMoved {
            player: player("nobody"),
            position: Vec3::ZERO,
            yaw: 0.0,
            pitch: 0.0,
            position_changed: true,
        });
        assert!(closed.is_empty());
    }

    #[test]
    fn disconnect_cleans_up_the_mapping_and_despawns() {
        let mut router = SessionRouter::new();
        let mut inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("heidi");
        let _handle = router.join_player(p, "heidi", spawn_pos()).expect("join");
        assert_eq!(
            inbox.try_recv(),
            Ok(GameInput::PlayerJoin {
                player: p,
                position: spawn_pos(),
            })
        );
        assert!(router.is_player_connected(p));

        let shard = router.disconnect_player(p).expect("disconnect");
        assert_eq!(shard, ShardPos::new(0, 0));

        // Mapping is gone.
        assert!(!router.is_player_connected(p));
        assert_eq!(router.player_shard(p), None);
        assert_eq!(router.player_count(), 0);

        // The shard was told to despawn the player.
        assert_eq!(inbox.try_recv(), Ok(GameInput::PlayerLeave { player: p }));
    }

    #[test]
    fn disconnect_event_routes_through_cleanup() {
        let mut router = SessionRouter::new();
        let mut inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("ivan");
        let _handle = router.join_player(p, "ivan", spawn_pos()).expect("join");
        let _ = inbox.try_recv();

        router
            .route_event(&NetEvent::disconnected(p, DisconnectReason::ServerShutdown))
            .expect("route disconnect");
        assert!(!router.is_player_connected(p));
        assert_eq!(inbox.try_recv(), Ok(GameInput::PlayerLeave { player: p }));
    }

    #[test]
    fn disconnect_unknown_player_is_rejected() {
        let mut router = SessionRouter::new();
        let err = router
            .disconnect_player(player("nobody"))
            .expect_err("no session");
        assert_eq!(
            err,
            SessionError::UnknownPlayer {
                player: player("nobody")
            }
        );
    }

    #[test]
    fn shard_inbox_full_is_classified_reject_backpressure() {
        // Capacity 1: the join fills the channel, the next input is rejected.
        let mut router = SessionRouter::with_capacities(1, 16);
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("judy");
        let _handle = router.join_player(p, "judy", spawn_pos()).expect("join");

        // The channel now holds the join; a movement input overflows it.
        let err = router
            .route_event(&NetEvent::play(
                p,
                ServerboundPlayPacket::SetPlayerPosition(
                    ferrumc_proto::generated::play::SetPlayerPosition::new(1.0, 1.0, 1.0, 0),
                ),
            ))
            .expect_err("inbox full");
        assert_eq!(
            err,
            SessionError::ShardInboxFull {
                shard: ShardPos::new(0, 0)
            }
        );
    }

    #[test]
    fn broadcast_move_is_lossy_when_a_viewer_is_full() {
        // A capacity-2 outbound channel: the viewer's mandatory join visibility
        // (tab-list add + entity spawn) exactly fills it, so a later move broadcast
        // is dropped (lossy backpressure) without reporting the viewer as closed.
        let mut router = SessionRouter::with_capacities(16, 2);
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("kate");
        let mover = player("kurt");
        let _viewer_handle = router
            .join_player(viewer, "kate", spawn_pos())
            .expect("viewer join");
        let _mover_handle = router
            .join_player(mover, "kurt", spawn_pos())
            .expect("mover join");

        let closed = router.route_output(&GameOutput::PlayerMoved {
            player: mover,
            position: Vec3::new(9.0, 64.0, 9.0),
            yaw: 0.0,
            pitch: 0.0,
            position_changed: true,
        });
        // The viewer is full, not closed: nothing to disconnect.
        assert!(closed.is_empty());
    }

    #[test]
    fn closed_viewer_is_reported_for_disconnect() {
        // Two players in range. If a viewer drops its handle, a broadcast move
        // reports it as closed so the caller can disconnect it.
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("leo");
        let mover = player("mona");
        let viewer_handle = router
            .join_player(viewer, "leo", spawn_pos())
            .expect("viewer join");
        let _mover_handle = router
            .join_player(mover, "mona", spawn_pos())
            .expect("mover join");
        drop(viewer_handle);

        let closed = router.route_output(&GameOutput::PlayerMoved {
            player: mover,
            position: Vec3::new(9.0, 64.0, 9.0),
            yaw: 0.0,
            pitch: 0.0,
            position_changed: true,
        });
        assert_eq!(closed, vec![viewer]);
    }

    /// A representable falling-block state id for the entity-broadcast tests. The
    /// concrete value is irrelevant to the routing; only that it round-trips into
    /// the `SpawnEntity` data field.
    const FALLING_SAND: u32 = 10;

    fn falling_block_spawn(entity: EntityId, position: Vec3) -> GameOutput {
        GameOutput::EntitySpawned {
            entity,
            position,
            velocity: Vec3::ZERO,
            kind: SpawnedEntityKind::FallingBlock {
                block: BlockStateId::new(FALLING_SAND),
            },
        }
    }

    #[test]
    fn falling_block_spawn_reaches_an_in_range_viewer() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("nina");
        let mut handle = router
            .join_player(viewer, "nina", spawn_pos())
            .expect("join");

        let closed = router.route_output(&falling_block_spawn(EntityId::new(1), spawn_pos()));
        assert!(closed.is_empty());

        let ClientboundPlayPacket::SpawnEntity(spawn) =
            handle.try_recv().expect("a spawn packet").into_packet()
        else {
            panic!("expected a SpawnEntity for the falling block");
        };
        // Rendered as minecraft:falling_block (type 49) carrying its state.
        assert_eq!(spawn.entity_type(), 49);
        // FALLING_SAND (10) round-trips into the type-specific data field.
        assert_eq!(spawn.data(), 10);
        assert_eq!((spawn.x(), spawn.y(), spawn.z()), (8.0, 64.0, 8.0));
    }

    #[test]
    fn falling_block_spawn_skips_an_out_of_range_viewer() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("otto");
        let mut handle = router
            .join_player(viewer, "otto", spawn_pos())
            .expect("join");

        // Spawn far beyond the default 10-chunk view distance (x=2000 -> chunk 125).
        let closed = router.route_output(&falling_block_spawn(
            EntityId::new(1),
            Vec3::new(2000.0, 64.0, 8.0),
        ));
        assert!(closed.is_empty());
        assert!(
            handle.try_recv().is_none(),
            "an out-of-range viewer must not receive the spawn"
        );
    }

    #[test]
    fn falling_block_spawn_move_despawn_reaches_a_viewer_in_order() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("pia");
        let mut handle = router
            .join_player(viewer, "pia", spawn_pos())
            .expect("join");
        let entity = EntityId::new(1);

        // Spawn, then a small downward move (within the relative-move range), then
        // despawn. Each step is routed independently, as the driver does per tick.
        assert!(router
            .route_output(&falling_block_spawn(entity, spawn_pos()))
            .is_empty());
        assert!(router
            .route_output(&GameOutput::EntityMoved {
                entity,
                position: Vec3::new(8.0, 62.0, 8.0),
            })
            .is_empty());
        assert!(router
            .route_output(&GameOutput::EntityDespawned { entity })
            .is_empty());

        // First the spawn; capture the allocated network id.
        let ClientboundPlayPacket::SpawnEntity(spawn) =
            handle.try_recv().expect("spawn").into_packet()
        else {
            panic!("expected SpawnEntity first");
        };
        let network_id = spawn.entity_id();

        // Then a relative position+rotation carrier for the 2-block drop.
        let ClientboundPlayPacket::UpdateEntityPositionAndRotation(mv) =
            handle.try_recv().expect("move").into_packet()
        else {
            panic!("expected a relative move for the fall");
        };
        assert_eq!(mv.entity_id(), network_id);

        // Finally a RemoveEntities naming exactly that id.
        let ClientboundPlayPacket::RemoveEntities(remove) =
            handle.try_recv().expect("despawn").into_packet()
        else {
            panic!("expected RemoveEntities on despawn");
        };
        assert_eq!(remove.entity_ids(), [network_id].as_slice());

        // Nothing more, and the entity is no longer tracked.
        assert!(handle.try_recv().is_none());
    }

    #[test]
    fn defaults_match_documented_capacities() {
        let router = SessionRouter::default();
        assert_eq!(router.shard_input_capacity(), DEFAULT_SHARD_INPUT_CAPACITY);
        assert_eq!(
            router.shard_control_reserve(),
            DEFAULT_SHARD_CONTROL_RESERVE
        );
        assert_eq!(router.outbound_capacity(), DEFAULT_OUTBOUND_CAPACITY);
        assert_eq!(router.shard_count(), 0);
        assert_eq!(router.player_count(), 0);
    }

    #[test]
    fn zero_capacity_is_clamped_to_one() {
        let router = SessionRouter::with_capacities(0, 0);
        assert_eq!(router.shard_input_capacity(), 1);
        assert_eq!(router.shard_control_reserve(), 0);
        assert_eq!(router.outbound_capacity(), 1);
    }

    #[tokio::test]
    async fn handle_recv_awaits_a_routed_correction() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("mallory");
        let mut handle = router.join_player(p, "mallory", spawn_pos()).expect("join");

        let closed = router.route_output(&GameOutput::PlayerPositionCorrected {
            player: p,
            position: Vec3::new(7.0, 8.0, 9.0),
        });
        assert!(closed.is_empty());

        let packet = handle.recv().await.expect("a packet").into_packet();
        let ClientboundPlayPacket::SynchronizePlayerPosition(sync) = packet else {
            panic!("expected a SynchronizePlayerPosition packet");
        };
        assert_eq!((sync.x(), sync.y(), sync.z()), (7.0, 8.0, 9.0));
    }

    #[test]
    fn two_players_see_each_other_on_join() {
        let mut router = SessionRouter::new();
        let mut inbox = router.register_shard(ShardPos::new(0, 0));
        let a = player("aaa");
        let b = player("bbb");

        let mut a_handle = router.join_player(a, "aaa", spawn_pos()).expect("a join");
        // `a` is alone: no visibility packets yet.
        assert!(a_handle.try_recv().is_none());

        let mut b_handle = router.join_player(b, "bbb", spawn_pos()).expect("b join");

        let a_eid = router.player_entity_id(a).expect("a entity id");
        let b_eid = router.player_entity_id(b).expect("b entity id");
        // Distinct ids per player, both clear of the local-player id (1).
        assert_ne!(a_eid, b_eid);
        assert!(a_eid >= FIRST_ENTITY_ID && b_eid >= FIRST_ENTITY_ID);

        // `a` learns about `b`; `b` learns about `a` (list add + entity spawn).
        assert_player_info_add(&mut a_handle, b);
        assert_entity_spawn(&mut a_handle, b, b_eid, spawn_pos());
        assert_player_info_add(&mut b_handle, a);
        assert_entity_spawn(&mut b_handle, a, a_eid, spawn_pos());

        // Both joins reached the shard.
        assert!(
            matches!(inbox.try_recv(), Ok(GameInput::PlayerJoin { player, .. }) if player == a)
        );
        assert!(
            matches!(inbox.try_recv(), Ok(GameInput::PlayerJoin { player, .. }) if player == b)
        );
    }

    #[test]
    fn system_chat_reaches_every_connected_player() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let a = player("aaa");
        let b = player("bbb");
        let mut a_handle = router.join_player(a, "aaa", spawn_pos()).expect("a join");
        let mut b_handle = router.join_player(b, "bbb", spawn_pos()).expect("b join");
        // Drain the mutual join-visibility packets so only the chat remains.
        let a_eid = router.player_entity_id(a).expect("a entity id");
        let b_eid = router.player_entity_id(b).expect("b entity id");
        assert_player_info_add(&mut a_handle, b);
        assert_entity_spawn(&mut a_handle, b, b_eid, spawn_pos());
        assert_player_info_add(&mut b_handle, a);
        assert_entity_spawn(&mut b_handle, a, a_eid, spawn_pos());

        let message = TextComponent::text("<aaa> hi everyone");
        router.broadcast_system_chat(&message, false);

        // Both players (the sender included) receive the same chat-box SystemChat.
        for handle in [&mut a_handle, &mut b_handle] {
            let ClientboundPlayPacket::SystemChat(chat) = handle
                .try_recv()
                .expect("a system chat packet")
                .into_packet()
            else {
                panic!("expected a SystemChat");
            };
            assert!(
                !chat.overlay(),
                "chat relay targets the chat box, not the action bar"
            );
        }
    }

    #[test]
    fn send_system_chat_to_targets_only_the_named_player() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let a = player("aaa");
        let b = player("bbb");
        let mut a_handle = router.join_player(a, "aaa", spawn_pos()).expect("a join");
        let mut b_handle = router.join_player(b, "bbb", spawn_pos()).expect("b join");
        // Drain the mutual join-visibility packets so only the chat remains.
        let a_eid = router.player_entity_id(a).expect("a entity id");
        let b_eid = router.player_entity_id(b).expect("b entity id");
        assert_player_info_add(&mut a_handle, b);
        assert_entity_spawn(&mut a_handle, b, b_eid, spawn_pos());
        assert_player_info_add(&mut b_handle, a);
        assert_entity_spawn(&mut b_handle, a, a_eid, spawn_pos());

        let message = TextComponent::text("psst, just for you");
        router.send_system_chat_to(b, &message, false);

        // Only b receives the chat; a's queue stays empty.
        let ClientboundPlayPacket::SystemChat(chat) = b_handle
            .try_recv()
            .expect("a system chat packet")
            .into_packet()
        else {
            panic!("expected a SystemChat");
        };
        assert!(!chat.overlay());
        assert!(
            a_handle.try_recv().is_none(),
            "a targeted message must not reach a non-target"
        );
    }

    #[test]
    fn send_system_chat_to_unknown_player_is_a_no_op() {
        let router = SessionRouter::new();
        // No panic, nothing to deliver: an unknown recipient is silently ignored.
        router.send_system_chat_to(player("ghost"), &TextComponent::text("hi"), false);
    }

    #[test]
    fn teleport_player_syncs_the_target_and_routes_an_authoritative_move() {
        let mut router = SessionRouter::new();
        let mut inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("tptarget");
        let mut handle = router
            .join_player(p, "tptarget", spawn_pos())
            .expect("join");
        // Drain the join input the shard received so only the teleport remains.
        assert_eq!(
            inbox.try_recv(),
            Ok(GameInput::PlayerJoin {
                player: p,
                position: spawn_pos(),
            })
        );

        let dest = Vec3::new(40.0, 70.0, 24.0);
        router.teleport_player(p, dest).expect("teleport");

        // The target's own client is snapped to the destination.
        let ClientboundPlayPacket::SynchronizePlayerPosition(sync) =
            handle.try_recv().expect("a sync packet").into_packet()
        else {
            panic!("expected a SynchronizePlayerPosition packet");
        };
        assert_eq!((sync.x(), sync.y(), sync.z()), (40.0, 70.0, 24.0));

        // The shard receives an authoritative move so state and viewers follow.
        assert_eq!(
            inbox.try_recv(),
            Ok(GameInput::PlayerMove {
                player: p,
                position: Some(dest),
                yaw: None,
                pitch: None,
            })
        );
    }

    #[test]
    fn teleport_player_for_unknown_player_is_rejected() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let p = player("nobody");
        let err = router
            .teleport_player(p, spawn_pos())
            .expect_err("no session");
        assert_eq!(err, SessionError::UnknownPlayer { player: p });
    }

    #[test]
    fn small_move_broadcasts_as_relative_update() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("viewer");
        let mover = player("mover");
        let mut viewer_handle = router
            .join_player(viewer, "viewer", spawn_pos())
            .expect("viewer join");
        let _mover_handle = router
            .join_player(mover, "mover", spawn_pos())
            .expect("mover join");
        let mover_eid = router.player_entity_id(mover).expect("mover entity id");
        // Drain the join-visibility packets the viewer already received.
        assert_player_info_add(&mut viewer_handle, mover);
        assert_entity_spawn(&mut viewer_handle, mover, mover_eid, spawn_pos());

        // A step of (2, 0, 1) from spawn — within 8 blocks — is a relative move.
        let closed = router.route_output(&GameOutput::PlayerMoved {
            player: mover,
            position: Vec3::new(10.0, 64.0, 9.0),
            yaw: 0.0,
            pitch: 0.0,
            position_changed: true,
        });
        assert!(closed.is_empty());
        let ClientboundPlayPacket::UpdateEntityPositionAndRotation(rel) = viewer_handle
            .try_recv()
            .expect("a relative move")
            .into_packet()
        else {
            panic!("expected an UpdateEntityPositionAndRotation for a small move");
        };
        assert_eq!(rel.entity_id(), mover_eid);
        // Deltas are `(new - last) * 4096`: dx = 2, dy = 0, dz = 1.
        assert_eq!(
            (rel.delta_x(), rel.delta_y(), rel.delta_z()),
            (8192, 0, 4096)
        );
    }

    #[test]
    fn large_move_broadcasts_as_teleport() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("viewer");
        let mover = player("mover");
        let mut viewer_handle = router
            .join_player(viewer, "viewer", spawn_pos())
            .expect("viewer join");
        let _mover_handle = router
            .join_player(mover, "mover", spawn_pos())
            .expect("mover join");
        let mover_eid = router.player_entity_id(mover).expect("mover entity id");
        // Drain the join-visibility packets the viewer already received.
        assert_player_info_add(&mut viewer_handle, mover);
        assert_entity_spawn(&mut viewer_handle, mover, mover_eid, spawn_pos());

        // A jump of (12, 0, 12) from spawn — over 8 blocks — teleports absolutely.
        let new_pos = Vec3::new(20.0, 64.0, 20.0);
        let closed = router.route_output(&GameOutput::PlayerMoved {
            player: mover,
            position: new_pos,
            yaw: 0.0,
            pitch: 0.0,
            position_changed: true,
        });
        assert!(closed.is_empty());
        let ClientboundPlayPacket::EntityTeleport(tp) =
            viewer_handle.try_recv().expect("a teleport").into_packet()
        else {
            panic!("expected an EntityTeleport for a large jump");
        };
        assert_eq!(tp.entity_id(), mover_eid);
        assert_eq!((tp.x(), tp.y(), tp.z()), (new_pos.x, new_pos.y, new_pos.z));
    }

    #[test]
    fn far_viewer_is_excluded_by_view_distance() {
        let mut router = SessionRouter::new();
        router.set_view_distance(1);
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let near = player("near");
        let far = player("far");
        let mut near_handle = router
            .join_player(near, "near", spawn_pos())
            .expect("near join");
        // Block 80 -> chunk 5, still inside shard (0, 0) (blocks 0..128) but five
        // chunks from the spawn chunk: out of a view distance of one.
        let far_pos = Vec3::new(80.0, 64.0, 80.0);
        let mut far_handle = router.join_player(far, "far", far_pos).expect("far join");

        // Out of range on join: neither learns about the other.
        assert!(near_handle.try_recv().is_none());
        assert!(far_handle.try_recv().is_none());

        // A far move stays out of range and is not broadcast to the near player.
        let closed = router.route_output(&GameOutput::PlayerMoved {
            player: far,
            position: Vec3::new(81.0, 64.0, 81.0),
            yaw: 0.0,
            pitch: 0.0,
            position_changed: true,
        });
        assert!(closed.is_empty());
        assert!(near_handle.try_recv().is_none());
    }

    #[test]
    fn block_change_broadcasts_block_update_to_in_range_viewer() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("viewer");
        let mut viewer_handle = router
            .join_player(viewer, "viewer", spawn_pos())
            .expect("viewer join");

        // A change in the viewer's chunk reaches them as a BlockUpdate (the lone
        // player got no join-visibility packets, so this is the first one). A
        // command cause has no actor, so no ack competes with the broadcast.
        let closed = router.route_output(&GameOutput::BlockChanged {
            position: BlockPos::new(8, 63, 8),
            state: BlockStateId::AIR,
            sequence: 0,
            cause: MutationCause::Command,
        });
        assert!(closed.is_empty());
        let ClientboundPlayPacket::BlockUpdate(update) = viewer_handle
            .try_recv()
            .expect("a block update")
            .into_packet()
        else {
            panic!("expected a BlockUpdate");
        };
        let loc = update.location();
        assert_eq!((loc.x(), loc.y(), loc.z()), (8, 63, 8));
        assert_eq!(update.block_state(), 0);
    }

    #[test]
    fn block_change_excludes_a_far_viewer() {
        let mut router = SessionRouter::new();
        router.set_view_distance(1);
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("viewer");
        let mut viewer_handle = router
            .join_player(viewer, "viewer", spawn_pos())
            .expect("viewer join");

        // Block (88, 63, 8) is in chunk x = 5, five chunks from the spawn chunk:
        // out of a view distance of one, so nothing is broadcast.
        let closed = router.route_output(&GameOutput::BlockChanged {
            position: BlockPos::new(88, 63, 8),
            state: BlockStateId::AIR,
            sequence: 0,
            cause: MutationCause::Command,
        });
        assert!(closed.is_empty());
        assert!(viewer_handle.try_recv().is_none());
    }

    #[test]
    fn block_change_reports_a_closed_viewer() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("viewer");
        let handle = router
            .join_player(viewer, "viewer", spawn_pos())
            .expect("viewer join");
        // Dropping the handle closes the outbound channel.
        drop(handle);

        let closed = router.route_output(&GameOutput::BlockChanged {
            position: BlockPos::new(8, 63, 8),
            state: BlockStateId::AIR,
            sequence: 0,
            cause: MutationCause::Command,
        });
        assert_eq!(closed, vec![viewer]);
    }

    #[test]
    fn sign_update_broadcasts_block_entity_data_to_in_range_viewer() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("viewer");
        let mut viewer_handle = router
            .join_player(viewer, "viewer", spawn_pos())
            .expect("viewer join");

        let mut sign = Sign::new(SignKind::Sign);
        sign.set_face_lines(
            true,
            ["hi".to_owned(), String::new(), String::new(), String::new()],
        );
        let position = BlockPos::new(8, 64, 8);
        let closed = router.route_output(&GameOutput::SignUpdated {
            position,
            sign: Box::new(sign),
        });
        assert!(closed.is_empty());

        let ClientboundPlayPacket::BlockEntityData(packet) = viewer_handle
            .try_recv()
            .expect("a block-entity data")
            .into_packet()
        else {
            panic!("expected a BlockEntityData");
        };
        let loc = packet.location();
        assert_eq!((loc.x(), loc.y(), loc.z()), (8, 64, 8));
        assert_eq!(packet.block_entity_type(), 7);
    }

    #[test]
    fn sign_update_excludes_a_far_viewer() {
        let mut router = SessionRouter::new();
        router.set_view_distance(1);
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("viewer");
        let mut viewer_handle = router
            .join_player(viewer, "viewer", spawn_pos())
            .expect("viewer join");

        // A sign five chunks away is out of a view distance of one.
        let closed = router.route_output(&GameOutput::SignUpdated {
            position: BlockPos::new(88, 64, 8),
            sign: Box::new(Sign::new(SignKind::Sign)),
        });
        assert!(closed.is_empty());
        assert!(viewer_handle.try_recv().is_none());
    }

    #[test]
    fn open_sign_editor_reaches_only_the_placer() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let placer = player("placer");
        let mut placer_handle = router
            .join_player(placer, "placer", spawn_pos())
            .expect("placer join");

        let position = BlockPos::new(8, 64, 8);
        let closed = router.route_output(&GameOutput::OpenSignEditor {
            player: placer,
            position,
        });
        assert!(closed.is_empty());

        let ClientboundPlayPacket::OpenSignEditor(packet) = placer_handle
            .try_recv()
            .expect("an open-sign-editor")
            .into_packet()
        else {
            panic!("expected an OpenSignEditor");
        };
        let loc = packet.location();
        assert_eq!((loc.x(), loc.y(), loc.z()), (8, 64, 8));
        assert!(packet.is_front_text());
        // No further packet was sent to the placer.
        assert!(placer_handle.try_recv().is_none());
    }

    #[test]
    fn open_sign_editor_for_unknown_player_is_a_no_op() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        // No player joined: routing the editor is a silent no-op (nothing to send,
        // no one to disconnect).
        let closed = router.route_output(&GameOutput::OpenSignEditor {
            player: player("ghost"),
            position: BlockPos::new(0, 64, 0),
        });
        assert!(closed.is_empty());
    }

    #[test]
    fn accepted_player_edit_acks_only_the_actor() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let actor = player("actor");
        let viewer = player("viewer");
        let mut actor_handle = router
            .join_player(actor, "actor", spawn_pos())
            .expect("actor join");
        let mut viewer_handle = router
            .join_player(viewer, "viewer", spawn_pos())
            .expect("viewer join");
        // Drain the mutual join-visibility packets so only the edit remains.
        let viewer_eid = router.player_entity_id(viewer).expect("viewer entity id");
        let actor_eid = router.player_entity_id(actor).expect("actor entity id");
        assert_player_info_add(&mut actor_handle, viewer);
        assert_entity_spawn(&mut actor_handle, viewer, viewer_eid, spawn_pos());
        assert_player_info_add(&mut viewer_handle, actor);
        assert_entity_spawn(&mut viewer_handle, actor, actor_eid, spawn_pos());

        let closed = router.route_output(&GameOutput::BlockChanged {
            position: BlockPos::new(8, 63, 8),
            state: BlockStateId::AIR,
            sequence: 55,
            cause: MutationCause::PlayerCreative { player: actor },
        });
        assert!(closed.is_empty());

        // The actor sees the authoritative BlockUpdate (it is in range too) and
        // then the AcknowledgeBlockChange echoing its sequence.
        let ClientboundPlayPacket::BlockUpdate(_) = actor_handle
            .try_recv()
            .expect("actor block update")
            .into_packet()
        else {
            panic!("expected a BlockUpdate for the actor first");
        };
        let ClientboundPlayPacket::AcknowledgeBlockChange(ack) =
            actor_handle.try_recv().expect("actor ack").into_packet()
        else {
            panic!("expected an AcknowledgeBlockChange for the actor");
        };
        assert_eq!(ack.sequence(), 55);

        // The viewer sees the broadcast BlockUpdate but never an ack.
        let ClientboundPlayPacket::BlockUpdate(_) = viewer_handle
            .try_recv()
            .expect("viewer block update")
            .into_packet()
        else {
            panic!("expected a BlockUpdate for the viewer");
        };
        assert!(viewer_handle.try_recv().is_none());
    }

    #[test]
    fn rejected_player_edit_resyncs_only_the_actor() {
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let actor = player("actor");
        let viewer = player("viewer");
        let mut actor_handle = router
            .join_player(actor, "actor", spawn_pos())
            .expect("actor join");
        let mut viewer_handle = router
            .join_player(viewer, "viewer", spawn_pos())
            .expect("viewer join");
        // Drain the mutual join-visibility packets so only the resync remains.
        let viewer_eid = router.player_entity_id(viewer).expect("viewer entity id");
        let actor_eid = router.player_entity_id(actor).expect("actor entity id");
        assert_player_info_add(&mut actor_handle, viewer);
        assert_entity_spawn(&mut actor_handle, viewer, viewer_eid, spawn_pos());
        assert_player_info_add(&mut viewer_handle, actor);
        assert_entity_spawn(&mut viewer_handle, actor, actor_eid, spawn_pos());

        // A rejected break: only the actor is healed — first the authoritative
        // state, then the ack that ends its pending prediction (the ack is what
        // actually reverts the ghost block on a real client).
        let closed = router.route_output(&GameOutput::BlockChangeRejected {
            player: actor,
            position: BlockPos::new(8, 63, 8),
            sequence: 77,
            requested_state: BlockStateId::AIR,
            authoritative_state: BlockStateId::new(1),
        });
        assert!(closed.is_empty());

        let ClientboundPlayPacket::BlockUpdate(update) =
            actor_handle.try_recv().expect("actor resync").into_packet()
        else {
            panic!("expected a BlockUpdate resync for the actor");
        };
        let loc = update.location();
        assert_eq!((loc.x(), loc.y(), loc.z()), (8, 63, 8));
        assert_eq!(update.block_state(), 1);
        // The ack follows the resync, echoing the rejected sequence so the client
        // ends its prediction and displays the authoritative state.
        let ClientboundPlayPacket::AcknowledgeBlockChange(ack) =
            actor_handle.try_recv().expect("actor ack").into_packet()
        else {
            panic!("expected an AcknowledgeBlockChange after the resync");
        };
        assert_eq!(ack.sequence(), 77);
        assert!(actor_handle.try_recv().is_none());
        // The viewer never saw the predicted change, so it gets nothing.
        assert!(viewer_handle.try_recv().is_none());
    }

    #[test]
    fn disconnect_broadcasts_player_remove_and_entity_despawn() {
        let mut router = SessionRouter::new();
        let mut inbox = router.register_shard(ShardPos::new(0, 0));
        let stay = player("stay");
        let leave = player("leave");
        let mut stay_handle = router
            .join_player(stay, "stay", spawn_pos())
            .expect("stay join");
        let _leave_handle = router
            .join_player(leave, "leave", spawn_pos())
            .expect("leave join");
        let leave_eid = router.player_entity_id(leave).expect("leave entity id");
        // Drain the staying player's join visibility and the two shard joins.
        assert_player_info_add(&mut stay_handle, leave);
        assert_entity_spawn(&mut stay_handle, leave, leave_eid, spawn_pos());
        let _ = inbox.try_recv();
        let _ = inbox.try_recv();

        router.disconnect_player(leave).expect("disconnect");

        // The staying player is told to drop the leaver from the tab list, via the
        // dedicated Player Info Remove (0x3E) packet carrying the leaver's UUID.
        let ClientboundPlayPacket::RemovePlayerInfo(remove) = stay_handle
            .try_recv()
            .expect("a player-remove packet")
            .into_packet()
        else {
            panic!("expected a RemovePlayerInfo");
        };
        assert_eq!(remove.players(), [leave.as_uuid()].as_slice());
        // ...and to despawn the leaver's entity so it does not linger as a ghost.
        let ClientboundPlayPacket::RemoveEntities(despawn) = stay_handle
            .try_recv()
            .expect("a remove-entities packet")
            .into_packet()
        else {
            panic!("expected a RemoveEntities");
        };
        assert_eq!(despawn.entity_ids(), [leave_eid].as_slice());
        // The shard was told to despawn the leaver.
        assert_eq!(
            inbox.try_recv(),
            Ok(GameInput::PlayerLeave { player: leave })
        );
    }

    #[test]
    fn leave_does_not_force_disconnect_a_viewer_that_never_saw_the_departed() {
        // Outbound capacity 1: any mandatory send to a full channel overflows and
        // cascade-disconnects the recipient. The old all-viewers leave broadcast
        // sent a mandatory remove to *every* player, so a far client that never saw
        // the leaver would be force-disconnected for an entity it never had.
        let mut router = SessionRouter::with_capacities(16, 1);
        router.set_view_distance(1);
        let _inbox = router.register_shard(ShardPos::new(0, 0));

        let leaver = player("leaver");
        let far = player("far");
        // Block 80 -> chunk 5, five chunks from spawn: out of a view distance of
        // one, so the two never exchange visibility and `far` holds no `delivered`
        // baseline for the leaver.
        let far_pos = Vec3::new(80.0, 64.0, 80.0);
        let _leaver_handle = router
            .join_player(leaver, "leaver", spawn_pos())
            .expect("leaver join");
        let mut far_handle = router.join_player(far, "far", far_pos).expect("far join");
        // No join visibility crossed the range gap.
        assert!(far_handle.try_recv().is_none());

        // Occupy `far`'s single outbound slot so any further mandatory send would
        // overflow (and, under the old code, trip the disconnect cascade).
        router.broadcast_system_chat(&TextComponent::text("hi"), false);

        // The leaver disconnects. `far` never saw it, so it must be left untouched.
        router.disconnect_player(leaver).expect("disconnect");

        // `far` is still connected and was sent no leave packets: only the chat that
        // was buffered before the leave remains.
        assert!(router.is_player_connected(far));
        let ClientboundPlayPacket::SystemChat(_) = far_handle
            .try_recv()
            .expect("the buffered chat")
            .into_packet()
        else {
            panic!("expected the buffered SystemChat");
        };
        assert!(far_handle.try_recv().is_none());
    }

    #[test]
    fn view_distance_default_and_setter() {
        let mut router = SessionRouter::new();
        assert_eq!(router.view_distance(), DEFAULT_VIEW_DISTANCE);
        router.set_view_distance(4);
        assert_eq!(router.view_distance(), 4);
        // A negative distance clamps to zero (chunk-mates still see each other).
        router.set_view_distance(-3);
        assert_eq!(router.view_distance(), 0);
    }

    #[test]
    fn full_outbound_disconnects_on_a_mandatory_ack_not_silent_drop() {
        // Outbound capacity 1. A lone actor's single slot is filled by a mandatory
        // correction, so the next accepted edit's mandatory ack cannot be
        // delivered: the actor is returned for disconnect rather than the ack being
        // silently dropped (the bug this fix closes).
        let mut router = SessionRouter::with_capacities(16, 1);
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let actor = player("actor");
        let _handle = router
            .join_player(actor, "actor", spawn_pos())
            .expect("join");

        // Fill the one outbound slot with a mandatory correction (delivered).
        let closed = router.route_output(&GameOutput::PlayerPositionCorrected {
            player: actor,
            position: spawn_pos(),
        });
        assert!(closed.is_empty());

        // The viewer-broadcast BlockUpdate is droppable (full -> dropped), but the
        // actor's ack is mandatory and the channel is full, so the actor is
        // returned for disconnect.
        let closed = router.route_output(&GameOutput::BlockChanged {
            position: BlockPos::new(8, 63, 8),
            state: BlockStateId::AIR,
            sequence: 9,
            cause: MutationCause::PlayerCreative { player: actor },
        });
        assert_eq!(closed, vec![actor]);
    }

    #[test]
    fn full_outbound_disconnects_on_a_mandatory_rejected_resync() {
        // A rejected edit's resync+ack pair needs two free slots. With capacity 1
        // the pair cannot be enqueued atomically, so the actor is disconnected
        // rather than receiving a partial (ack-without-resync) group.
        let mut router = SessionRouter::with_capacities(16, 1);
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let actor = player("actor");
        let _handle = router
            .join_player(actor, "actor", spawn_pos())
            .expect("join");
        // Use the single slot so capacity() is 0 (< 2 required for the pair).
        let _ = router.route_output(&GameOutput::PlayerPositionCorrected {
            player: actor,
            position: spawn_pos(),
        });

        let closed = router.route_output(&GameOutput::BlockChangeRejected {
            player: actor,
            position: BlockPos::new(8, 63, 8),
            sequence: 3,
            requested_state: BlockStateId::AIR,
            authoritative_state: BlockStateId::new(1),
        });
        assert_eq!(closed, vec![actor]);
    }

    #[test]
    fn full_outbound_disconnects_a_viewer_on_a_mandatory_spawn() {
        // An *existing* viewer whose channel is full when a new player arrives has
        // no room for the joiner's mandatory tab-list add + entity spawn. Rather
        // than ghosting an invisible body (a tab entry whose entity was never
        // spawned), the viewer is disconnected. The joiner itself is fine: its own
        // fresh channel is staged with guaranteed capacity for the visibility it
        // receives (one in-range player needs 2 slots, and capacity is 2).
        let mut router = SessionRouter::with_capacities(16, 2);
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("viewer");
        let joiner = player("joiner");
        let _viewer_handle = router
            .join_player(viewer, "viewer", spawn_pos())
            .expect("viewer join");
        assert!(router.is_player_connected(viewer));

        // Pre-fill the viewer's 2-slot channel with two mandatory corrections so it
        // has no room left for the joiner's incoming add + spawn.
        for _ in 0..2 {
            assert!(router
                .route_output(&GameOutput::PlayerPositionCorrected {
                    player: viewer,
                    position: spawn_pos(),
                })
                .is_empty());
        }

        // The joiner's arrival tries to send the (full) viewer a mandatory add then
        // spawn -> the viewer is disconnected. The join itself succeeds (staging:
        // in_range=1 needs 2 <= capacity 2) and the joiner stays connected.
        let _joiner_handle = router
            .join_player(joiner, "joiner", spawn_pos())
            .expect("joiner join");
        assert!(!router.is_player_connected(viewer));
        assert!(router.is_player_connected(joiner));
    }

    #[test]
    fn rejected_resync_envelope_is_mandatory_on_the_state_queue() {
        // The rejected-edit resync `BlockUpdate` is forced onto the (Mandatory,
        // State) class so it co-queues with the ack and cannot be silently dropped
        // at Layer B — even though a `BlockUpdate`'s type default is (Droppable,
        // World). The ack rides (Mandatory, State) by type, so the pair shares the
        // State queue and FIFO keeps the resync ahead of the ack.
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let actor = player("actor");
        let mut handle = router
            .join_player(actor, "actor", spawn_pos())
            .expect("join");

        let closed = router.route_output(&GameOutput::BlockChangeRejected {
            player: actor,
            position: BlockPos::new(8, 63, 8),
            sequence: 12,
            requested_state: BlockStateId::AIR,
            authoritative_state: BlockStateId::new(1),
        });
        assert!(closed.is_empty());

        let resync = handle.try_recv().expect("resync envelope");
        assert!(matches!(
            resync.packet(),
            ClientboundPlayPacket::BlockUpdate(_)
        ));
        assert_eq!(resync.criticality(), Criticality::Mandatory);
        assert_eq!(resync.priority(), OutboundPriority::State);
        // The carried classification diverges from the packet type's default — the
        // whole reason the envelope exists.
        assert_eq!(
            Criticality::for_packet(resync.packet()),
            Criticality::Droppable
        );
        assert_eq!(
            OutboundPriority::for_packet(resync.packet()),
            OutboundPriority::World
        );

        let ack = handle.try_recv().expect("ack envelope");
        assert!(matches!(
            ack.packet(),
            ClientboundPlayPacket::AcknowledgeBlockChange(_)
        ));
        assert_eq!(ack.criticality(), Criticality::Mandatory);
        assert_eq!(ack.priority(), OutboundPriority::State);
    }

    #[test]
    fn mover_entering_view_range_spawns_instead_of_teleporting() {
        // A player that moves INTO a viewer's range has no delivered baseline there,
        // so it must be SPAWNED (mandatory player_info add + entity spawn) before any
        // movement — never a bare teleport for an entity the client never spawned.
        let mut router = SessionRouter::new();
        router.set_view_distance(1);
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("viewer");
        let mover = player("mover");
        let mut viewer_handle = router
            .join_player(viewer, "viewer", spawn_pos())
            .expect("viewer join");
        // The mover joins far out of view (chunk 5 vs the viewer's chunk 0).
        let far_pos = Vec3::new(80.0, 64.0, 80.0);
        let _mover_handle = router
            .join_player(mover, "mover", far_pos)
            .expect("mover join");
        let mover_eid = router.player_entity_id(mover).expect("mover entity id");
        // Out of range on join: the viewer learned nothing about the mover.
        assert!(viewer_handle.try_recv().is_none());

        // The mover steps into the viewer's chunk: a mandatory add + spawn, no move.
        let near_pos = Vec3::new(10.0, 64.0, 9.0);
        let closed = router.route_output(&GameOutput::PlayerMoved {
            player: mover,
            position: near_pos,
            yaw: 0.0,
            pitch: 0.0,
            position_changed: true,
        });
        assert!(closed.is_empty());
        assert_player_info_add(&mut viewer_handle, mover);
        assert_entity_spawn(&mut viewer_handle, mover, mover_eid, near_pos);
        // No movement packet this tick — the spawn already carries the position.
        assert!(viewer_handle.try_recv().is_none());

        // The baseline is now seeded at the spawn position, so the next small step
        // is a relative move measured from there (not another spawn).
        let step = Vec3::new(11.0, 64.0, 9.0);
        let closed = router.route_output(&GameOutput::PlayerMoved {
            player: mover,
            position: step,
            yaw: 0.0,
            pitch: 0.0,
            position_changed: true,
        });
        assert!(closed.is_empty());
        let ClientboundPlayPacket::UpdateEntityPositionAndRotation(rel) = viewer_handle
            .try_recv()
            .expect("a relative move after the spawn seeded the baseline")
            .into_packet()
        else {
            panic!("expected an UpdateEntityPositionAndRotation once a baseline exists");
        };
        assert_eq!(rel.entity_id(), mover_eid);
        // dx = (11 - 10) * 4096 = 4096.
        assert_eq!((rel.delta_x(), rel.delta_y(), rel.delta_z()), (4096, 0, 0));
    }

    #[test]
    fn join_rejected_when_outbound_too_small_for_existing_visibility() {
        // Capacity 2 holds exactly one existing player's mandatory add + spawn. A
        // third joiner sees two in-range players -> needs 4 slots, so it is rejected
        // up front (finding #9) rather than left half-loaded with a dropped spawn.
        let mut router = SessionRouter::with_capacities(16, 2);
        let mut inbox = router.register_shard(ShardPos::new(0, 0));
        let a = player("aaa");
        let b = player("bbb");
        let _a = router.join_player(a, "aaa", spawn_pos()).expect("a join");
        let _b = router.join_player(b, "bbb", spawn_pos()).expect("b join");
        // Drain the two existing shard joins.
        let _ = inbox.try_recv();
        let _ = inbox.try_recv();
        assert_eq!(router.player_count(), 2);

        let c = player("ccc");
        let err = router
            .join_player(c, "ccc", spawn_pos())
            .expect_err("staging rejects an oversized visibility burst");
        assert_eq!(err, SessionError::OutboundFull { player: c });
        // Nothing half-registered, and no PlayerJoin reached the shard.
        assert_eq!(router.player_count(), 2);
        assert!(!router.is_player_connected(c));
        assert!(inbox.try_recv().is_err());
    }

    #[test]
    fn full_outbound_disconnects_a_viewer_on_a_mandatory_despawn() {
        // A viewer whose channel is full when another player leaves must itself be
        // disconnected (the despawn is mandatory) rather than ghosting the entity.
        // Capacity 2 so stay survives leave's join (the mandatory add + spawn fill
        // both slots) and is full — not disconnected — when the despawn arrives.
        let mut router = SessionRouter::with_capacities(16, 2);
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let stay = player("stay");
        let leave = player("leave");
        let _stay_handle = router
            .join_player(stay, "stay", spawn_pos())
            .expect("stay join");
        let _leave_handle = router
            .join_player(leave, "leave", spawn_pos())
            .expect("leave join");
        // At leave's join, stay received the mandatory player_info_add(leave) and the
        // mandatory entity spawn, filling both outbound slots. Both stay connected.
        assert!(router.is_player_connected(stay));
        assert_eq!(router.player_count(), 2);

        // Disconnecting leave broadcasts a mandatory despawn to stay, whose channel
        // is full -> stay is cascaded into a disconnect too (the despawn is never
        // silently dropped). The cascade is bounded and both end up gone.
        router.disconnect_player(leave).expect("disconnect leave");
        assert!(!router.is_player_connected(stay));
        assert_eq!(router.player_count(), 0);
    }

    #[test]
    fn exact_eight_block_move_teleports_instead_of_saturating() {
        // 8 blocks * 4096 = 32768 overflows i16; the move must teleport, not
        // saturate to 32767 (a 1/4096-block error that accumulates each tick).
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("viewer");
        let mover = player("mover");
        let mut viewer_handle = router
            .join_player(viewer, "viewer", spawn_pos())
            .expect("viewer join");
        let _mover_handle = router
            .join_player(mover, "mover", spawn_pos())
            .expect("mover join");
        let mover_eid = router.player_entity_id(mover).expect("mover entity id");
        assert_player_info_add(&mut viewer_handle, mover);
        assert_entity_spawn(&mut viewer_handle, mover, mover_eid, spawn_pos());

        let new_pos = Vec3::new(spawn_pos().x + 8.0, spawn_pos().y, spawn_pos().z);
        let closed = router.route_output(&GameOutput::PlayerMoved {
            player: mover,
            position: new_pos,
            yaw: 0.0,
            pitch: 0.0,
            position_changed: true,
        });
        assert!(closed.is_empty());
        let ClientboundPlayPacket::EntityTeleport(tp) =
            viewer_handle.try_recv().expect("a teleport").into_packet()
        else {
            panic!("expected an EntityTeleport for an exact-8-block move");
        };
        assert_eq!(tp.entity_id(), mover_eid);
        assert_eq!((tp.x(), tp.y(), tp.z()), (new_pos.x, new_pos.y, new_pos.z));
    }

    #[test]
    fn dropped_relative_move_does_not_drift() {
        // A viewer whose channel is full misses a relative move; once it drains, the
        // next delta is measured against the last DELIVERED position (still spawn),
        // not the global one, so the missed step is recovered rather than lost.
        let mut router = SessionRouter::with_capacities(16, 2);
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("viewer");
        let mover = player("mover");
        let mut viewer_handle = router
            .join_player(viewer, "viewer", spawn_pos())
            .expect("viewer join");
        let _mover_handle = router
            .join_player(mover, "mover", spawn_pos())
            .expect("mover join");
        let mover_eid = router.player_entity_id(mover).expect("mover entity id");

        // The viewer's 2-slot channel is now full (player_info_add + entity_spawn),
        // and its delivered baseline for the mover is the spawn position.
        // Move 1 to x=10 cannot enqueue (full) and is dropped; the baseline stays at
        // spawn rather than advancing to 10.
        let closed = router.route_output(&GameOutput::PlayerMoved {
            player: mover,
            position: Vec3::new(10.0, 64.0, 8.0),
            yaw: 0.0,
            pitch: 0.0,
            position_changed: true,
        });
        assert!(closed.is_empty());

        // Drain the two join packets; the channel is now empty.
        assert_player_info_add(&mut viewer_handle, mover);
        assert_entity_spawn(&mut viewer_handle, mover, mover_eid, spawn_pos());

        // Move 2 to x=11: the delta is measured from the delivered spawn (x=8), so it
        // is +3 blocks (12288), NOT +1 (which would mean the baseline had wrongly
        // advanced to 10 and the viewer drifted two blocks behind).
        let closed = router.route_output(&GameOutput::PlayerMoved {
            player: mover,
            position: Vec3::new(11.0, 64.0, 8.0),
            yaw: 0.0,
            pitch: 0.0,
            position_changed: true,
        });
        assert!(closed.is_empty());
        let ClientboundPlayPacket::UpdateEntityPositionAndRotation(rel) = viewer_handle
            .try_recv()
            .expect("a relative move")
            .into_packet()
        else {
            panic!("expected an UpdateEntityPositionAndRotation");
        };
        assert_eq!(rel.entity_id(), mover_eid);
        assert_eq!((rel.delta_x(), rel.delta_y(), rel.delta_z()), (12288, 0, 0));
    }

    /// A non-empty pre-encoded equipment body (a single slot 0 entry + a fake Slot).
    /// The router treats it opaquely; only its non-emptiness and round-trip matter.
    fn equipment_body() -> Vec<u8> {
        vec![0x00, 0x01, 0x01]
    }

    /// Reads the next `SetEquipment` packet off `handle`, asserting its entity id
    /// and opaque body.
    fn assert_set_equipment(handle: &mut PlayerSessionHandle, eid: i32, body: &[u8]) {
        let ClientboundPlayPacket::SetEquipment(equip) = handle
            .try_recv()
            .expect("a set-equipment packet")
            .into_packet()
        else {
            panic!("expected a SetEquipment packet");
        };
        assert_eq!(equip.entity_id(), eid);
        assert_eq!(equip.equipments(), body);
    }

    #[test]
    fn viewer_receives_equipment_on_enter_view_at_join() {
        // A player joining in view of an existing viewer is shown — beyond the
        // mandatory add + spawn — a droppable SetEquipment carrying their held item.
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("viewer");
        let held = player("held");
        let mut viewer_handle = router
            .join_player(viewer, "viewer", spawn_pos())
            .expect("viewer join");
        // The held-item player joins with a non-empty cached equipment body.
        let _held_handle = router
            .join_player_with_equipment(held, "held", spawn_pos(), equipment_body())
            .expect("held join");
        let held_eid = router.player_entity_id(held).expect("held entity id");

        // The viewer learns of the new player: add, spawn, then the equipment.
        assert_player_info_add(&mut viewer_handle, held);
        assert_entity_spawn(&mut viewer_handle, held, held_eid, spawn_pos());
        assert_set_equipment(&mut viewer_handle, held_eid, &equipment_body());
    }

    #[test]
    fn hotbar_change_broadcasts_equipment_to_in_view_viewer() {
        // Both players already in view of each other; the subject switches its held
        // item, and the viewer (which has the subject spawned) gets a SetEquipment.
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("viewer");
        let subject = player("subject");
        let mut viewer_handle = router
            .join_player(viewer, "viewer", spawn_pos())
            .expect("viewer join");
        let _subject_handle = router
            .join_player(subject, "subject", spawn_pos())
            .expect("subject join");
        let subject_eid = router.player_entity_id(subject).expect("subject entity id");
        // Drain the join visibility (both joined with empty equipment -> no
        // equipment packets at join).
        assert_player_info_add(&mut viewer_handle, subject);
        assert_entity_spawn(&mut viewer_handle, subject, subject_eid, spawn_pos());

        router.set_equipment(subject, equipment_body());
        assert_set_equipment(&mut viewer_handle, subject_eid, &equipment_body());
    }

    #[test]
    fn set_equipment_skips_a_viewer_that_never_saw_the_subject() {
        // A far viewer that never had the subject spawned (no delivered baseline)
        // is not sent the hotbar-change equipment.
        let mut router = SessionRouter::new();
        router.set_view_distance(1);
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let far = player("far");
        let subject = player("subject");
        let mut far_handle = router
            .join_player(far, "far", Vec3::new(80.0, 64.0, 80.0))
            .expect("far join");
        let _subject_handle = router
            .join_player(subject, "subject", spawn_pos())
            .expect("subject join");
        // Out of range: no visibility exchanged.
        assert!(far_handle.try_recv().is_none());

        router.set_equipment(subject, equipment_body());
        // The far viewer never saw the subject, so it gets nothing.
        assert!(far_handle.try_recv().is_none());
    }

    #[test]
    fn moving_and_rotating_broadcasts_position_and_rotation_with_nonzero_yaw() {
        // A position+rotation move to an in-view viewer yields an
        // UpdateEntityPositionAndRotation carrying the quantized yaw (90 deg -> 64)
        // followed by a SetHeadRotation.
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("viewer");
        let mover = player("mover");
        let mut viewer_handle = router
            .join_player(viewer, "viewer", spawn_pos())
            .expect("viewer join");
        let _mover_handle = router
            .join_player(mover, "mover", spawn_pos())
            .expect("mover join");
        let mover_eid = router.player_entity_id(mover).expect("mover entity id");
        assert_player_info_add(&mut viewer_handle, mover);
        assert_entity_spawn(&mut viewer_handle, mover, mover_eid, spawn_pos());

        let closed = router.route_output(&GameOutput::PlayerMoved {
            player: mover,
            position: Vec3::new(10.0, 64.0, 9.0),
            yaw: 90.0,
            pitch: 0.0,
            position_changed: true,
        });
        assert!(closed.is_empty());

        let ClientboundPlayPacket::UpdateEntityPositionAndRotation(rel) = viewer_handle
            .try_recv()
            .expect("a position+rotation move")
            .into_packet()
        else {
            panic!("expected an UpdateEntityPositionAndRotation");
        };
        assert_eq!(rel.entity_id(), mover_eid);
        // yaw 90 deg quantizes to angle byte 64 (non-zero: the player no longer
        // faces north).
        assert_eq!(rel.yaw(), 64);
        assert_ne!(rel.yaw(), 0);
        // The head rotation follows.
        let ClientboundPlayPacket::SetHeadRotation(head) = viewer_handle
            .try_recv()
            .expect("a head rotation")
            .into_packet()
        else {
            panic!("expected a SetHeadRotation after the move");
        };
        assert_eq!(head.entity_id(), mover_eid);
        assert_eq!(head.head_yaw(), 64);
    }

    #[test]
    fn rotation_only_move_broadcasts_rotation_and_head_rotation() {
        // A rotation-only move (position unchanged) to an in-view viewer yields an
        // UpdateEntityRotation + SetHeadRotation and no position carrier.
        let mut router = SessionRouter::new();
        let _inbox = router.register_shard(ShardPos::new(0, 0));
        let viewer = player("viewer");
        let mover = player("mover");
        let mut viewer_handle = router
            .join_player(viewer, "viewer", spawn_pos())
            .expect("viewer join");
        let _mover_handle = router
            .join_player(mover, "mover", spawn_pos())
            .expect("mover join");
        let mover_eid = router.player_entity_id(mover).expect("mover entity id");
        assert_player_info_add(&mut viewer_handle, mover);
        assert_entity_spawn(&mut viewer_handle, mover, mover_eid, spawn_pos());

        let closed = router.route_output(&GameOutput::PlayerMoved {
            player: mover,
            position: spawn_pos(),
            yaw: 90.0,
            pitch: 45.0,
            position_changed: false,
        });
        assert!(closed.is_empty());

        let ClientboundPlayPacket::UpdateEntityRotation(rot) = viewer_handle
            .try_recv()
            .expect("a rotation-only update")
            .into_packet()
        else {
            panic!("expected an UpdateEntityRotation for a rotation-only move");
        };
        assert_eq!(rot.entity_id(), mover_eid);
        // yaw 90 -> 64, pitch 45 -> 32; both non-zero (no longer facing north/level).
        assert_eq!((rot.yaw(), rot.pitch()), (64, 32));
        let ClientboundPlayPacket::SetHeadRotation(head) = viewer_handle
            .try_recv()
            .expect("a head rotation")
            .into_packet()
        else {
            panic!("expected a SetHeadRotation after the rotation");
        };
        assert_eq!(head.entity_id(), mover_eid);
        assert_eq!(head.head_yaw(), 64);
    }

    /// Asserts the next packet on `handle` is a player-list add for `expected`.
    ///
    /// The Add Player body leads with a count byte (`1`) then the 16-byte UUID;
    /// the name / properties / listed fields follow (asserted in `translate.rs`).
    fn assert_player_info_add(handle: &mut PlayerSessionHandle, expected: PlayerId) {
        let ClientboundPlayPacket::PlayerInfoUpdate(info) = handle
            .try_recv()
            .expect("a player-info packet")
            .into_packet()
        else {
            panic!("expected a PlayerInfoUpdate");
        };
        assert_eq!(info.action(), PLAYER_INFO_ADD);
        assert_eq!(info.entries()[0], 1);
        assert_eq!(&info.entries()[1..17], expected.as_uuid().as_bytes());
    }

    /// Asserts the next packet on `handle` is a spawn of `expected` with the
    /// given entity id at `pos`, rendered as the player entity type.
    fn assert_entity_spawn(
        handle: &mut PlayerSessionHandle,
        expected: PlayerId,
        eid: i32,
        pos: Vec3,
    ) {
        let ClientboundPlayPacket::SpawnEntity(spawn) =
            handle.try_recv().expect("a spawn packet").into_packet()
        else {
            panic!("expected a SpawnEntity packet");
        };
        assert_eq!(spawn.entity_uuid(), expected.as_uuid());
        assert_eq!(spawn.entity_id(), eid);
        assert_eq!(
            spawn.entity_type(),
            149,
            "remote players spawn as minecraft:player"
        );
        assert_eq!((spawn.x(), spawn.y(), spawn.z()), (pos.x, pos.y, pos.z));
    }
}
