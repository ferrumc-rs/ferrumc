# TODO: Gravity & Entity Physics

> Scope and prerequisites for simulated gravity in FerrumC v2. Physics belongs to
> `ferrumc-sim` — the shard exclusively owns entities and applies changes at tick
> boundaries (see the Crate Map in [`../../../CLAUDE.md`](../../../CLAUDE.md) and
> the shard-ownership rule). Every item below must uphold the crate
> [`INVARIANTS.md`](./INVARIANTS.md): deterministic stepping, no wall clock,
> tick-boundary application, bounded inbox, ordered containers.

## Status

**Progress (branch `feat/entity-physics`):** steps 1–5 and the falling-block
consumer (step 6, including client animation 6c) are done. The shard owns a
deterministic non-player entity store, spawns/despawns entities across the
`GameInput`/`GameOutput` boundary, runs a per-tick gravity + integration +
air-drag step with swept AABB collision (no tunneling, all three axes) and void
despawn, and converts unsupported gravity blocks into falling-block entities
that the session layer draws on real clients. Remaining: the other vanilla
consumers (dropped items, projectiles, mobs, player) — each needs a spawner
plus a typed `SpawnedEntityKind`; the physics step and the session render
pipeline already exist.

- [x] 1. Entity store on `SimShard` (`entities` `BTreeMap`, monotonic `EntityId`).
- [x] 2. Spawn/despawn via `GameInput::{SpawnEntity,DespawnEntity}` →
  `GameOutput::{EntitySpawned,EntityDespawned}`.
- [x] 3. Per-tick physics scaffold (`apply_entity_physics`, `EntityMoved`).
- [x] 4. Per-category gravity + air drag (`src/physics.rs`); terminal velocity
  emerges from drag, not a clamp.
- [x] 5. Swept AABB collision (`sweep_move` + `clip_x/y/z`). Each entity has a
  box (`entity_dimensions`) swept the full tick and clamped against every solid
  block along its path, one axis at a time (Y, X, Z), so a fast mover never
  tunnels through a one-block floor/wall and horizontal collision resolves.
  Landing sets `on_ground`; a grounded entity re-checks its floor each tick and
  falls again when it is removed. An entity past the world bottom
  (`VOID_DESPAWN_Y`) is despawned. **Remaining in this area:** per-shape support
  AABBs (slabs/stairs collide as full cubes today, via `is_solid_cube`).
- [~] 6. Vanilla consumers. **Falling blocks done (6a + 6b + 6c):** a placed
  gravity-affected block (sand, gravel, concrete powder, anvil, …) with no
  support, or one whose support is broken, converts to a falling-block entity
  (`is_gravity_affected` predicate + `settle_falling_block` trigger on
  place/break); the entity falls, lands on a solid block, restores its block at
  the resting cell, and despawns. Full player-driven loop works, including
  **column collapse** (breaking the base of a stack drops the whole gravity
  column at once, and the entities re-stack on landing). **6c — session wiring
  (client animation) done:** `EntitySpawned` carries a `SpawnedEntityKind`
  (`FallingBlock { block }`), and `ferrumc-session`'s router translates the
  entity outputs to clientbound `SpawnEntity` (`minecraft:falling_block`, type
  49, block-state in the data field), per-tick move carriers, and
  `RemoveEntities` — scoped to viewers in range, with a network entity id
  allocated from the shared player/entity counter (no wire-id collision).
  **6d — break-on-object done (partial):** a falling block whose resting cell (the
  cell directly above the support) already holds a non-full object — a torch,
  pressure plate, button, lever, rail, or redstone component
  (`breaks_falling_block`) — breaks instead of settling. The check is on the
  resting cell only, so a wall torch on a side block never breaks a block falling
  down an adjacent column. A falling block also settles *through* water/lava,
  replacing the fluid in its resting cell (fluids are non-solid and replaceable).
  **Gaps:** "break" currently despawns the entity without dropping an item (item
  entities pending); the object set is a focused name-keyed predicate, not a full
  `replaceable` classification (the vendored `blocks.json` carries no replaceable
  flag, so flowers/dead bushes are not yet covered); non-block entities in the
  resting cell (item frames, paintings) are **not** destroyed on landing — those
  entities do not exist in the server yet, and this logic only reads the block
  grid, so the vanilla "falling block destroys the entity it lands on" rule waits
  on the non-block entity system; replaced fluids do not re-flow, and concrete
  powder does not convert to concrete in water (both need fluid simulation).
  **Remaining refinements:** `scaffolding` lateral rules, landing damage
  (anvil/dripstone), item drop on break (needs item entities). **Still to do:**
  dropped items, projectiles, mobs (each needs a spawner + a `SpawnedEntityKind`
  variant with its entity type; the render pipeline already exists).
- [ ] 7. Server-side player gravity (gated on fall damage / movement validation).

Original scoping notes below are kept for context; one decision changed during
implementation — see the **Gravity itself** section.

The simulation is currently a skeleton. `SimShard` (see [`./src/shard.rs`](./src/shard.rs))
owns chunks, chunk tickets, block mutation, and per-player state. Player movement
is **client-authoritative**: `GameInput::PlayerMove` carries the client's
position/rotation and the shard only validates it (`GameOutput::PlayerMoved` /
`PlayerPositionCorrected` in [`./src/message.rs`](./src/message.rs)). There is:

- **no entity store** for non-player entities,
- **no `Velocity`** (the only `velocity` in the tree is the wire type
  `EntityVelocity` in `ferrumc-proto`, not a simulated value),
- **no per-tick entity step**, and no gravity, drag, or ground detection.

Gravity is not a standalone roadmap item. It is a prerequisite hidden inside
three unstarted [`ROADMAP.md`](../../../docs/ROADMAP.md) entries — *Mobs + AI*,
*Projectiles*, and *Item entities (dropped items, pickup)* — none of which can
land without it.

The **wire layer is already ahead of the simulation**: `ferrumc-proto` generates
`SpawnEntity` and `EntityVelocity` packets, so the network can describe entities
the sim cannot yet produce. This confirms the bottleneck is the entity store
here, not networking.

## Prerequisites (the foundation, before any gravity)

### 1. Entity store on the shard

Give `SimShard` an entity store for non-player entities. Per the simulation model
in `CLAUDE.md`, use a `SlotMap` + component vecs, held in an **ordered container**
so iteration and output ordering stay deterministic (INVARIANTS: "ordered
containers so output ordering is deterministic"). Minimum components:

- `Position(Vec3)` — reuse [`ferrumc-math`](../ferrumc-math/src/vec3.rs)`::Vec3`,
  never a raw tuple (coordinate-typing rule).
- `Velocity(Vec3)`
- `OnGround(bool)`

### 2. Entity spawn / despawn

- A `GameInput`/internal spawn path for server-spawned entities (not just
  players), and matching `GameOutput` variants that the session layer maps to the
  already-generated `SpawnEntity` / `EntityVelocity` / `RemoveEntities` packets.
- Deterministic entity ids allocated from an ordered source.

### 3. Per-tick entity step

A physics step inside `SimShard::run_tick` (never on enqueue — INVARIANTS:
"Inputs are applied only at tick boundaries"). Ordering within the tick:

1. apply gravity to `Velocity`,
2. apply drag to `Velocity`,
3. integrate `Velocity` into `Position`,
4. resolve collision against world blocks and set `OnGround`.

## Gravity itself

Once the foundation exists, gravity is small. Split by the One-Crate Rule:

### Pure values → `ferrumc-sim` (`src/physics.rs`)

> **Decision changed during implementation.** Original plan put the constants in
> `ferrumc-math`. They landed in `ferrumc-sim::physics` instead, for two reasons:
> `ferrumc-math` is pure geometry (coordinates, vectors, AABBs) and has no notion
> of game physics — a gravity value is a *simulation rule*, not a math primitive;
> and the One-Crate Rule means a PR touching both `ferrumc-math` and `ferrumc-sim`
> would be two crates. Constants live beside the shard that applies them.

Per-category gravity constants and drag are pure data. Vanilla uses distinct
per-category magnitudes (blocks per tick²) — a single global constant is **not**
vanilla-correct:

| Category | Gravity |
| --- | --- |
| Living entities (mobs, players) | `-0.08` |
| Items, falling blocks, primed TNT, boats, armor stands | `-0.04` |
| Arrows, tridents | `-0.05` |
| Thrown items (snowball, egg, ender pearl, potion), XP orbs | `-0.03` |
| Llama spit | `-0.06` |

- **Air drag** (`AIR_DRAG_Y = 0.98`): multiply the vertical velocity by `0.98`
  per tick. This produces the terminal velocity (~`-3.92` for living entities)
  implicitly — no hardcoded clamp. Done.
- Gravity is stored per entity (an `f64` field on the shard's `EntityState`,
  seeded at spawn from a category constant), so items/projectiles/blocks differ
  without branching on entity type in the step. Done.

### Application → `ferrumc-sim`

The step that reads `Gravity`/`Velocity`/`OnGround`, applies the constants, and
integrates. Ground detection queries world blocks through the chunk data the
shard already owns ([`ferrumc-world`](../ferrumc-world/) is already a dependency)
— never a database handle (INVARIANTS: "owns chunk data but never a database
handle").

## Vanilla coverage (in dependency order)

1. **Item entities** (dropped items, gravity `-0.04`) — the simplest simulated
   entity, no AI, and already a roadmap item. The first real consumer of the
   entity store + gravity step.
2. **Falling blocks** — sand, red sand, gravel, suspicious sand/gravel, all 16
   concrete powders, anvils (all damage states), dragon egg, pointed dripstone,
   scaffolding. Needs a block-support/neighbour-update trigger that converts an
   unsupported block into a falling-block entity (gravity `-0.04`) and re-places
   or drops it on landing (anvils and pointed dripstone deal landing damage).
3. **Projectiles** — arrows/tridents (`-0.05`), thrown items and XP orbs
   (`-0.03`). Gravity + collision; combat wiring is separate.
4. **Mobs** — inherit gravity; flight/swim are AI counter-forces layered on top,
   not "gravity disabled". Gated on *Mobs + AI*.
5. **Server-side player gravity** — gravity for the player entity, gated on fall
   damage and movement validation (anti-cheat). Deferred: player movement is
   client-authoritative today; this replaces trust with server authority and is
   scoped with those features, not before.

## Invariants any implementation must uphold

- **Deterministic**: identical input sequences produce identical entity state and
  output ordering. No wall clock in the step; use `Tick`.
- **Tick-boundary only**: no state mutation on enqueue.
- **Bounded**: any new queue/collection is bounded; no unbounded growth from
  entity spawns.
- **No unwrap/expect** outside tests; classified error variants.
- **Typed coordinates**: `Vec3`/`BlockPos`, never raw tuples, across any public
  API.

## Suggested order of work

1. Entity store on `SimShard` (ordered, deterministic) — one-crate PR, testable
   with the existing tick harness.
2. Entity spawn/despawn + `GameOutput` → `SpawnEntity`/`RemoveEntities` mapping.
3. Per-tick entity step scaffold (integration only, gravity zero).
4. Gravity constants + drag in `ferrumc-math`; wire them into the step.
5. Item entities as the first end-to-end consumer.
6. Falling blocks (support trigger + landing).
7. Projectiles, then mobs (post-AI), then server-side player gravity.
