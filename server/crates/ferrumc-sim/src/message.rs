//! Messages crossing the simulation boundary.
//!
//! [`GameInput`] flows *into* a shard — upstream the session router produces it
//! from validated network events — and is applied only at tick boundaries.
//! [`GameOutput`] flows *out* of a shard after a tick and is routed back to
//! sessions/network. The simulation never touches sockets; it only exchanges
//! these typed messages.

use ferrumc_core::{EntityId, GameMode, PlayerId};
use ferrumc_math::{BlockPos, Cuboid, Direction, Vec3};
use ferrumc_world::{BlockStateId, Sign, SIGN_LINES};

use crate::mutation::MutationCause;
use crate::region::RegionOp;

/// An input applied to the simulation at the next tick boundary.
///
/// Inputs are intentionally minimal for this milestone: player presence and
/// movement. The enum is `#[non_exhaustive]` because new variants will be added
/// as the simulation grows, so downstream `match`es must include a wildcard arm.
///
/// `PartialEq` (but not `Eq`) is derived because positions are floating-point
/// [`Vec3`]s.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum GameInput {
    /// A player entered the shard.
    PlayerJoin {
        /// Identity of the joining player.
        player: PlayerId,
        /// World-space position the player joins at.
        position: Vec3,
    },
    /// A player moved, rotated, or both.
    ///
    /// Carries each component independently so one variant covers all three
    /// serverbound move packets: a position-only `SetPlayerPosition`
    /// (`position` set, `yaw`/`pitch` `None`), a `SetPlayerPositionAndRotation`
    /// (all set), and a rotation-only `SetPlayerRotation` (`position` `None`,
    /// `yaw`/`pitch` set). A `None` field leaves that component unchanged; an
    /// input with no component set has no effect. `yaw`/`pitch` are degrees.
    PlayerMove {
        /// Identity of the moving player.
        player: PlayerId,
        /// New world-space position, if this move changed it.
        position: Option<Vec3>,
        /// New body yaw in degrees, if this move changed it.
        yaw: Option<f32>,
        /// New pitch in degrees, if this move changed it.
        pitch: Option<f32>,
    },
    /// A player left the shard.
    PlayerLeave {
        /// Identity of the leaving player.
        player: PlayerId,
    },
    /// A player broke (destroyed) the block at `position`.
    ///
    /// Decoded upstream from a serverbound `PlayerAction` "start destroying
    /// block". The simulation validates the edit at the tick boundary
    /// (actor present, target chunk resident, target within reach) and, on
    /// acceptance, replaces the block with [`BlockStateId::AIR`]. Held-tool and
    /// drop rules are out of scope this milestone.
    BlockBreak {
        /// Identity of the player breaking the block.
        player: PlayerId,
        /// Absolute position of the block to break.
        position: BlockPos,
        /// The block-action sequence the client stamped on the originating
        /// `PlayerAction`, echoed back in an `AcknowledgeBlockChange` on accept.
        sequence: i32,
    },
    /// A player placed a block at `position`.
    ///
    /// Decoded upstream from a serverbound `UseItemOn`; `position` is the block
    /// adjacent to the clicked face. The simulation validates the edit at the
    /// tick boundary (the same checks as [`BlockBreak`](GameInput::BlockBreak))
    /// and, on acceptance, computes the *correct* placed block-state from
    /// `state` (the held item's default), `clicked_face`, `cursor_position`, and
    /// `player_yaw` via `ferrumc_placement::compute_placement` (rotation/facing/
    /// half/fence connectivity) before writing it through the single block-edit
    /// funnel. An unsupported or unrecognised block falls back to writing `state`
    /// unchanged. `state` itself is resolved by the app from the player's held
    /// hotbar stack, so the simulation stays inventory-free.
    BlockPlace {
        /// Identity of the player placing the block.
        player: PlayerId,
        /// Absolute position the block is placed at.
        position: BlockPos,
        /// The block-action sequence the client stamped on the originating
        /// `UseItemOn`, echoed back in an `AcknowledgeBlockChange` on accept.
        sequence: i32,
        /// The held item's default block-state (the placement input); the
        /// simulation refines it into the final placed state.
        state: BlockStateId,
        /// The face of the targeted block the player clicked, used to derive axis
        /// (logs), half (slabs/stairs), and wall-torch facing.
        clicked_face: Direction,
        /// The cursor hit point inside the targeted block (`0.0..=1.0` per axis);
        /// its `y` selects a slab/stair half.
        cursor_position: Vec3,
        /// The player's yaw in degrees at place time, used to derive horizontal
        /// facing (stairs face the player; furnaces face away).
        player_yaw: f32,
    },
    /// Write an authoritative, *exact* block-state at `position`, applied
    /// verbatim — **not** refined by `compute_placement`.
    ///
    /// Produced by a plugin/command exact-state write (a `before_block_*`
    /// `Replace` decision or a `WorldIntent::SetBlock`): the state is already the
    /// final one the plugin chose, so the simulation must store it byte-for-byte
    /// rather than re-deriving axis/half/facing from neutral placement inputs
    /// (which would, for example, rewrite a plugin-set `oak_log axis=x` to the
    /// default vertical `axis=y`). Only a player [`UseItemOn`](GameInput::BlockPlace)
    /// placement is refined by `compute_placement`.
    ///
    /// It is still validated by the same block-edit funnel as every other edit
    /// (actor present, target chunk resident, target within reach); on acceptance
    /// it emits a [`GameOutput::BlockChanged`] carrying the acting player so the
    /// actor still gets the ack/resync. No fence-neighbour pass runs — an exact
    /// write is authoritative, not a placement.
    SetBlockExact {
        /// Identity of the player on whose behalf the exact write is applied (the
        /// actor that receives the ack/resync).
        player: PlayerId,
        /// Absolute position the block is written at.
        position: BlockPos,
        /// The block-action sequence to echo back in an `AcknowledgeBlockChange`
        /// on accept.
        sequence: i32,
        /// The exact block-state to write, applied verbatim.
        state: BlockStateId,
    },
    /// Request a resync + acknowledgement for a block edit that was refused
    /// *upstream* of the simulation (a plugin `Deny`, spawn-protection veto, etc.)
    /// without ever mutating the world.
    ///
    /// The upstream layers (net/app) have no world access, so they cannot read the
    /// authoritative block state to heal the actor's optimistic prediction. They
    /// route the refusal here instead: at the tick boundary the shard reads the
    /// authoritative state at `position` from the resident chunk and emits a
    /// [`GameOutput::BlockChangeRejected`] — the same ack + mandatory resync funnel
    /// an in-sim rejection (out of reach, unloaded chunk) already uses — so every
    /// rejection site heals the client identically. No chunk write and no journal
    /// entry are produced; the handler only *reads* chunk state, keeping the tick
    /// deterministic.
    RejectBlockEdit {
        /// Identity of the player whose predicted edit must be undone.
        player: PlayerId,
        /// Absolute position of the refused edit (where the authoritative state is
        /// read to heal the client).
        position: BlockPos,
        /// The block-action sequence the client stamped on the originating edit,
        /// echoed back in an `AcknowledgeBlockChange` so its pending prediction
        /// ends.
        sequence: i32,
        /// The state the client optimistically predicted (air for a refused break,
        /// the held block for a refused place); used only to classify the metric,
        /// never applied.
        requested_state: BlockStateId,
    },
    /// Spawn a non-player entity at `position` with an initial `velocity`.
    ///
    /// Produced by any authority that can introduce an entity into the world
    /// (a plugin/command spawn, a future block-support failure that turns a
    /// gravity block into a falling-block entity, a mob death that drops an
    /// item). The simulation assigns a fresh [`EntityId`] at the tick boundary
    /// and emits a matching [`GameOutput::EntitySpawned`] carrying that id so
    /// the session layer can map it to a clientbound `SpawnEntity` packet. The
    /// input carries no entity-type field yet: the first typed consumer (item
    /// entities) will add one when it lands, together with the network-side
    /// packet mapping — this milestone is entity-store plumbing only.
    SpawnEntity {
        /// World-space position the entity is created at.
        position: Vec3,
        /// Initial velocity in blocks per tick. Zero for a stationary spawn;
        /// non-zero to seed a projectile or a falling block.
        velocity: Vec3,
        /// Per-tick downward acceleration in blocks per tick² (negative),
        /// chosen from a [`crate::physics`] category constant — `GRAVITY_ITEM`
        /// for an item/falling block, `GRAVITY_LIVING` for a mob, etc. The
        /// spawner picks the category; the simulation stays type-agnostic.
        gravity: f64,
    },
    /// Despawn the non-player entity with `entity`.
    ///
    /// Produced by any authority that removes an entity (an item picked up,
    /// a projectile that landed, a falling block that finished falling, a mob
    /// killed). Despawn for an absent entity is a silent no-op at the tick
    /// boundary: retries after a natural removal never emit a spurious output.
    /// On acceptance the shard emits a [`GameOutput::EntityDespawned`] the
    /// session layer maps to a clientbound `RemoveEntities` packet.
    DespawnEntity {
        /// Identity of the entity to remove.
        entity: EntityId,
    },
    /// Set the authoritative server-side game mode of a player.
    ///
    /// Produced by the app's `/gamemode` command path (in addition to the
    /// clientbound `GameEvent` that switches the client visually) so the
    /// simulation owns the mode that later enforcement reads (creative
    /// no-decrement, block-break speed, flight). Applied at the tick boundary;
    /// a `SetGameMode` for an absent player is a silent no-op. Emits no output.
    SetGameMode {
        /// Identity of the player whose mode changes.
        player: PlayerId,
        /// The new authoritative game mode.
        mode: GameMode,
    },
    /// Apply a region (cuboid) block edit on behalf of `player`.
    ///
    /// Produced by the app's `/fill` and `/replace` commands. The whole cuboid is
    /// applied at one tick boundary through the same single block-edit funnel a
    /// single edit uses (under [`MutationCause::Command`]): every changed cell
    /// persists via the chunk overlay and broadcasts a `BlockUpdate` to viewers,
    /// but carries no acked sequence (a command edit is authoritative, not a
    /// client prediction). The shard captures the prior block-state of every
    /// changed cell into a bounded per-player undo history before applying, so a
    /// later [`RegionUndo`](GameInput::RegionUndo) can restore it.
    ///
    /// The edit is bounded: a cuboid whose volume exceeds the shard's configured
    /// region cap is rejected wholesale (the command layer already rejected it
    /// with a user-facing error; this is defense in depth). Cells in chunks not
    /// resident in this shard are skipped (cross-shard region edits are out of
    /// scope this milestone).
    RegionEdit {
        /// Identity of the player on whose behalf the edit is applied (keys the
        /// undo history; the player need not be present, since a command edit is
        /// authoritative and not reach-checked).
        player: PlayerId,
        /// The cuboid of blocks the edit addresses.
        region: Cuboid,
        /// How every block in the cuboid changes.
        op: RegionOp,
    },
    /// Undo `player`'s most recent region edit, restoring the prior block-states
    /// it captured.
    ///
    /// Produced by the app's `/undo` command. Pops the player's newest undo entry
    /// and re-applies each captured prior state through the same funnel (so the
    /// restoration also persists and broadcasts). An undo is *not* itself pushed
    /// onto the history, so repeated `/undo` walks back through successive edits.
    /// A `RegionUndo` for a player with no history is a silent no-op.
    RegionUndo {
        /// Identity of the player whose most recent region edit is undone.
        player: PlayerId,
    },
    /// A player submitted new text for one face of a sign they were editing.
    ///
    /// Decoded upstream from a serverbound `UpdateSign`. The simulation validates
    /// the edit at the tick boundary (the actor is present, the target chunk is
    /// resident, the target is within [`MAX_REACH`](crate::SimShard) reach, and a
    /// sign block-entity exists there and is not waxed) and, on acceptance,
    /// replaces the addressed face's four text lines and emits a
    /// [`GameOutput::SignUpdated`] for the session layer to broadcast as a
    /// `BlockEntityData`. A failed validation is a silent no-op (no output, no
    /// mutation): net never writes the world directly, so a stale or out-of-reach
    /// edit is simply dropped.
    UpdateSign {
        /// Identity of the editing player.
        player: PlayerId,
        /// Absolute position of the sign being edited.
        position: BlockPos,
        /// `true` to edit the front face, `false` the back.
        is_front: bool,
        /// The four new text lines, top to bottom (each a plain string the
        /// network layer wraps as a text component).
        lines: [String; SIGN_LINES],
    },
}

/// An output produced by the simulation during a tick.
///
/// Outputs are deterministic given the inbox contents: identical input
/// sequences yield identical output sequences. The enum is `#[non_exhaustive]`
/// for the same forward-compatibility reasons as [`GameInput`].
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum GameOutput {
    /// A player became present in the shard.
    PlayerSpawned {
        /// Identity of the spawned player.
        player: PlayerId,
        /// World-space position the player spawned at.
        position: Vec3,
    },
    /// A player's position and/or rotation changed.
    ///
    /// Always carries the player's current absolute `position`, `yaw`, and
    /// `pitch` (degrees) as of this tick, plus `position_changed`: `true` when
    /// this output applied a new position (the session layer broadcasts a
    /// relative/absolute move carrier), `false` for a rotation-only change (the
    /// session layer broadcasts a rotation-only carrier). The head yaw is sent
    /// alongside either way so a viewer's head turns.
    PlayerMoved {
        /// Identity of the moved player.
        player: PlayerId,
        /// Current absolute world-space position.
        position: Vec3,
        /// Current body yaw in degrees.
        yaw: f32,
        /// Current pitch in degrees.
        pitch: f32,
        /// `true` if this tick applied a new position; `false` if it was a
        /// rotation-only change.
        position_changed: bool,
    },
    /// A player's movement was rejected; the client should snap back to this
    /// authoritative position.
    ///
    /// Emitted at the tick boundary when a [`GameInput::PlayerMove`] carried
    /// non-finite or out-of-range coordinates and no valid move superseded it in
    /// the same tick. Carries the player's last accepted position so the session
    /// layer can send a correction to the desynced client.
    PlayerPositionCorrected {
        /// Identity of the player to correct.
        player: PlayerId,
        /// Authoritative position the client must return to.
        position: Vec3,
    },
    /// A player was removed from the shard.
    PlayerDespawned {
        /// Identity of the despawned player.
        player: PlayerId,
    },
    /// A non-player entity became present in the shard.
    ///
    /// Emitted at the tick boundary after a [`GameInput::SpawnEntity`] is
    /// accepted, carrying the freshly allocated [`EntityId`] and the
    /// authoritative spawn state so the session layer can build a clientbound
    /// `SpawnEntity` packet (id, position, velocity) without a follow-up query
    /// into the shard.
    EntitySpawned {
        /// Identity assigned to the new entity.
        entity: EntityId,
        /// World-space position the entity spawned at.
        position: Vec3,
        /// Initial velocity in blocks per tick.
        velocity: Vec3,
    },
    /// A non-player entity's position changed after per-tick physics
    /// integration.
    ///
    /// Emitted at the tick boundary for every resident entity whose velocity is
    /// non-zero: the shard integrates `position += velocity` and reports the new
    /// absolute position so the session layer can broadcast a position update to
    /// viewers. Zero-velocity entities emit nothing — the invariant that
    /// outputs describe real state transitions holds. The current velocity is
    /// not carried because clients extrapolate from prior velocity packets;
    /// wire-side velocity is a separate packet the session layer sends when the
    /// velocity itself changes (not implemented this milestone).
    EntityMoved {
        /// Identity of the moved entity.
        entity: EntityId,
        /// Current absolute world-space position after this tick's integration.
        position: Vec3,
    },
    /// A non-player entity was removed from the shard.
    ///
    /// Emitted at the tick boundary after a [`GameInput::DespawnEntity`] for a
    /// resident entity. A despawn for an already-absent entity emits nothing:
    /// the invariant that outputs describe real state transitions is preserved
    /// so a retry never doubles the client-visible removal.
    EntityDespawned {
        /// Identity of the removed entity.
        entity: EntityId,
    },
    /// The block at `position` changed to `state`.
    ///
    /// Emitted at the tick boundary after the simulation applies an accepted
    /// block break (`state` is [`BlockStateId::AIR`]) or place. The session
    /// layer broadcasts it to viewers in range as a clientbound `BlockUpdate`
    /// and, for a [`MutationCause::PlayerCreative`] edit, echoes `sequence` back
    /// to the acting player as an `AcknowledgeBlockChange`.
    BlockChanged {
        /// Absolute position of the changed block.
        position: BlockPos,
        /// The block's new state.
        state: BlockStateId,
        /// The originating block-action sequence to acknowledge to the actor.
        sequence: i32,
        /// What caused the mutation (carries the acting player for the ack).
        cause: MutationCause,
    },
    /// A player's block edit was rejected; the actor's predicted change must heal
    /// to the authoritative state.
    ///
    /// Emitted at the tick boundary when an edit that has a client to heal is
    /// refused — the actor is present and the target chunk is resident, so the
    /// authoritative state is readable (e.g. the target is out of reach). The
    /// session layer sends only the actor a `BlockUpdate` carrying
    /// `authoritative_state` at `position` *followed by* an
    /// `AcknowledgeBlockChange` echoing `sequence`: the `BlockUpdate` sets the
    /// client's known server state and the ack ends its pending prediction so
    /// that state is what it displays. The ack is what actually reverts the ghost
    /// block on a real 1.21.8 client (a `BlockUpdate` alone is swallowed while a
    /// prediction is pending), so it is mandatory on reject just as on accept.
    /// Nothing is broadcast (viewers never saw the predicted change). Rejections
    /// with no client to heal (absent actor, unloaded chunk) emit nothing.
    BlockChangeRejected {
        /// The player whose predicted edit must be undone.
        player: PlayerId,
        /// Absolute position of the rejected edit.
        position: BlockPos,
        /// The originating block-action sequence to acknowledge to the actor, so
        /// its pending client-side prediction ends and reverts to the
        /// authoritative state.
        sequence: i32,
        /// The state the client optimistically predicted (air for a break, the
        /// placed block for a place); used by the app to classify the metric.
        requested_state: BlockStateId,
        /// The authoritative state to resync the actor to.
        authoritative_state: BlockStateId,
    },
    /// A sign block-entity's text was created or changed and must be (re)rendered
    /// for everyone who can see it.
    ///
    /// Emitted at the tick boundary after an accepted [`GameInput::UpdateSign`].
    /// The session layer encodes `sign` into the network NBT and broadcasts a
    /// `BlockEntityData` (droppable) to every viewer within view distance of
    /// `position`, so the sign renders its new text. The full sign snapshot is
    /// carried (not just the edited face) so the carrier is self-contained.
    SignUpdated {
        /// Absolute position of the sign.
        position: BlockPos,
        /// The sign's full post-edit state (both faces). Boxed to keep
        /// [`GameOutput`] small (a [`Sign`] holds eight text lines).
        sign: Box<Sign>,
    },
    /// A player placed a sign and should be shown its editing screen.
    ///
    /// Emitted at the tick boundary after an accepted [`GameInput::BlockPlace`]
    /// (or exact write) whose resulting block is a sign and whose block-entity was
    /// just created. The session layer sends only `player` an `OpenSignEditor`
    /// targeting `position`'s front face, opening the client's sign-edit screen;
    /// the client replies with an `UpdateSign` when the player confirms.
    OpenSignEditor {
        /// The player to open the editor for (the placer).
        player: PlayerId,
        /// Absolute position of the just-placed sign.
        position: BlockPos,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inputs_compare_by_value() {
        let p = PlayerId::offline("notch");
        let a = GameInput::PlayerJoin {
            player: p,
            position: Vec3::new(1.0, 2.0, 3.0),
        };
        let b = a.clone();
        assert_eq!(a, b);
        assert_ne!(
            a,
            GameInput::PlayerMove {
                player: p,
                position: Some(Vec3::new(1.0, 2.0, 3.0)),
                yaw: None,
                pitch: None,
            }
        );
    }

    #[test]
    fn outputs_compare_by_value() {
        let p = PlayerId::offline("jeb_");
        let spawned = GameOutput::PlayerSpawned {
            player: p,
            position: Vec3::ZERO,
        };
        assert_eq!(spawned.clone(), spawned);
        assert_ne!(spawned, GameOutput::PlayerDespawned { player: p });
    }
}
