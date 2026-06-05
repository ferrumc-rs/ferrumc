# Terrain Generation

The overworld terrain generator lives in the `ferrumc-world-gen` crate (`src/lib/world_gen`). It is
a climate-driven, layered pipeline built on the existing `ferrumc_world::chunk::Chunk` API: large
regions (oceans, continents, biome bands) are laid out by low-frequency *climate* noise rather than
per-column decisions, so generation produces broad contiguous regions instead of uniform noise.

Generation is a pure function of the world seed and global coordinates — no cross-column or
cross-chunk shared state — which is what lets chunks generate in parallel and lets a chunk resolve
features (such as tree canopies) rooted in its neighbours identically to how those neighbours
resolve them.

## Pipeline

`WorldGenerator::generate_chunk` runs these stages per chunk:

1. **Stone fill** — every section up to `MAX_GENERATED_HEIGHT` is filled solid with stone.
2. **Carve surface** (`carving`) — a single pass composes each column's final surface height and
   clears the stone above it.
3. **Decorate** (`biomes`) — each column's biome places its surface cover (grass/dirt, sand,
   snow, …).
4. **Flood water** — one global pass fills every column whose surface is below `SEA_LEVEL`,
   independent of biome.
5. **Carve caves** — 3D ridged-noise caves (`caves.rs`). Carved *before* trees so a cave can never
   hollow out a trunk or canopy that was already placed.
6. **Surface finish** — on each column's *post-cave* top: re-cover exposed dirt with grass, and lay
   snow on snowy biomes and high mountain peaks (see [Surface finish](#surface-finish)).
7. **Place trees** — including the canopies of trees rooted in neighbouring chunks (see
   [Trees](#trees-and-cross-chunk-canopies)). Columns where a cave opens at the surface are skipped.
8. **Vegetation** — scatter ground plants (grass, flowers, desert plants) on the finished surface
   (see [Vegetation](#vegetation)).
9. **Biome data** — the dominant biome ID is written to the chunk sections.
10. **Bedrock floor** — an unbreakable layer at `Y = -64`, laid last so caves cannot punch through.

The per-column surface height and climate sample are passed around as local arrays (`Heightmap`,
`ColumnClimate`) for the duration of one `generate_chunk` call; nothing outside generation needs
them.

## Climate model

`climate.rs` holds the low-frequency noise fields that drive region layout. All are sampled in world
space and normalised to `[0, 1]`:

| Field | Frequency | Role |
| --- | --- | --- |
| `continentalness` | `0.0015` (~660-block features) | Broadest field. Maps through a spline to an absolute base surface height: deep ocean basin → continental shelf → coastline → raised inland. This is what makes oceans and continents *large*. |
| `temperature` | `0.0012` | Biome climate axis: low → snowy, high → desert. |
| `humidity` | `0.0013` | Biome climate axis: low → dry, high → forest. |
| `erosion` (`carving/erosion.rs`) | `0.03`, splined | Surface ruggedness: damps local detail amplitude (flattens plateaus) and feeds biome selection. |

The continentalness → height spline control points:

```
0.00 → 16   deep ocean floor (~47 below sea level)
0.18 → 30   ocean basin
0.32 → 48   continental shelf
0.42 → 60   coastline (just below sea level)
0.50 → 68   low inland
0.70 → 88   hills
1.00 → 112  high inland
```

### Surface height composition

Per column (`carving/initial_height.rs`, `WorldGenerator::column`):

```
base       = continental_spline(continentalness)        # absolute base height
flat       = flatness(erosion)                           # 0 = rugged, 1 = flat (steep mid-section)
rugged     = 1 - flat
target     = min(base, PLAINS_LEVEL)                     # only pull down, never lift ocean floors
land_base  = base + (target - base) * flat               # high land settles toward the plains level
landness   = clamp(continentalness / 0.42, 0, 1)         # 0 in ocean, 1 fully inland
amplitude  = MIN + (MAX - MIN) * landness * rugged       # MIN floor everywhere
detail     = (detail_noise * 2 - 1) * amplitude          # ~80-block hills
surface    = land_base + detail
```

Elevation and ruggedness are both driven by **erosion** through a single `flatness(erosion)` factor:

- **Low erosion → rugged.** `flat ≈ 0`, so the column keeps its full continental base height and full
  detail amplitude. These are the mountains (the biome classifier selects them from the same
  low-erosion band).
- **High erosion → flat.** `flat ≈ 1`, so the column is pulled down toward `PLAINS_LEVEL` (`70`, just
  above sea level) and its detail collapses to the `MIN_DETAIL_AMPLITUDE` floor. These are the plains.

Pulling plains down to their own low level — rather than letting them inherit the continental base —
is what stops them riding high just because they border raised ground. The pull is *downward only*
(`min(base, PLAINS_LEVEL)`), so ocean floors are never lifted.

`flatness` has a deliberately **steep middle section** (the `0.18 → 0.30` erosion band). As the smooth
erosion field crosses it, the surface height changes over just a few blocks, so the boundary between
low plains and rugged high ground reads as an abrupt **terrace edge / fault** rather than a gentle
ramp — adding terrain variety. The riser sits at the same erosion value the classifier uses to pick
mountains, so the height step and the biome change line up.

`detail_noise` (`0.0125`, ~80-block features) is the higher-frequency layer that adds the hills a
player notices while walking; `MIN_DETAIL_AMPLITUDE` / `MAX_DETAIL_AMPLITUDE` bound it (currently
`3` / `37`). The `MIN` floor is always present so even flat plains and the sea bed are never dead flat.

## Biomes

`WorldGenerator::classify` maps each column's climate sample and surface height to a pair: the
surface **decorator** to run and the **registry biome ID** to record. These usually coincide, but
one shared ocean decorator backs many ocean registry variants whose ID is chosen separately (see
below). Because the climate fields are low-frequency, the result is broad bands rather than
per-column noise.

Selection order:

1. **Water / coast**, by surface height relative to `SEA_LEVEL`:
   - `surface < SEA_LEVEL` → an ocean variant (see [Ocean variants](#ocean-variants)).
   - `surface <= SEA_LEVEL + 2` → beach (snowy beach where cold).
2. **Mountains** — rugged, low-erosion high ground (`erosion < 0.30 && surface >= 95`).
3. **Land climate grid** — on temperature × humidity:
   - cold (`temperature < 0.30`): snowy plains, or snowy taiga where humid.
   - temperate (`temperature < 0.62`): plains, or forest where humid.
   - hot: desert where dry, otherwise forest.

Each biome's surface cover is placed by a decorator in `biomes/` implementing `BiomeGenerator`:

| Biome | Registry ID | Decorator | Notes |
| --- | --- | --- | --- |
| plains | 40 | `plains.rs` | grass/dirt, sparse oak |
| forest | 21 | `forest.rs` | grass/dirt, dense oak + birch mix |
| desert | 14 | `desert.rs` | sand over sandstone, treeless |
| ocean variants | see below | `ocean.rs` | sand + sandstone sea bed (one decorator, many IDs) |
| beach | 3 | `beach.rs` | sand strip over sandstone |
| snowy_beach | 45 | `beach.rs` | same decorator, different ID |
| snowy_plains | 46 | `snowy.rs` | snowy grass + dirt; snow laid by the surface-finish pass, treeless |
| snowy_taiga | 48 | `snowy.rs` | snowy grass + dirt; snow laid by the surface-finish pass, spruce |
| windswept_hills | 62 | `mountain.rs` | bare stone; snow cap above `Y = 110` laid by the surface-finish pass |

Registry IDs are the indices of the biome in the dynamic biome registry sent during configuration
(see `assets/data/registry_packets.json`). Mixed-biome sections are not yet encoded on the network
path, so each chunk is filled with a single dominant biome (the most common of the 16 biome-cell
columns).

### Ocean variants

The sea bed is identical for every ocean, so a single `ocean.rs` decorator serves all of them and
`WorldGenerator::ocean_variant_id` picks the registry ID from temperature and depth (only the ID,
and thus the client's water colour, differs). A column counts as *deep* when `continentalness < 0.20`
or `surface <= SEA_LEVEL - 18`.

| Temperature | Shallow | Deep |
| --- | --- | --- |
| `< 0.15` (frozen) | frozen_ocean (22) | deep_frozen_ocean (11) |
| `< 0.35` (cold) | cold_ocean (6) | deep_cold_ocean (9) |
| `< 0.55` | ocean (35) | deep_ocean (13) |
| `< 0.75` (lukewarm) | lukewarm_ocean (29) | deep_lukewarm_ocean (12) |
| else (warm) | warm_ocean (58) | warm_ocean (58) — no deep warm variant exists |

## Water

Water is **not** tied to a biome. After decoration, a single global pass floods every column whose
surface is below `SEA_LEVEL` (63), filling from the surface up to the water level. This is what
produces natural coastlines and inland basins, and it decouples the water level from biome selection
(an earlier design flooded only inside the ocean biome and to a different level, which left a band of
"dry sea bed" where the two disagreed).

## Surface finish

Caves are carved before trees and can open at the surface (cave mouths), stripping the cover off a
biome's surface and leaving bare dirt — and, if cover blocks like snow had already been placed,
leaving them floating where the old surface used to be. To avoid both, surface cover that depends on
the *final* shape of the ground is applied in one per-column pass **after** carving, working on the
column's actual post-cave top block:

- **Grass** — for grassy biomes (`WorldGenerator::grass_cover_for` returns a grass block for plains,
  forest, and the snowy biomes), a bare-dirt top is converted to the biome's grass block. Columns
  whose surface was never carved already have grass on top and are unchanged.
- **Snow** — a snow layer is laid on the real top of snowy biomes (always) and of mountain peaks
  above `MOUNTAIN_SNOW_LINE` (`Y = 110`), only into the air directly above the surface. Because it
  follows the post-cave top, snow never floats over a cave mouth (the bug this replaced: the snowy
  decorator used to place snow at the pre-cave `surface_y + 1`, which caves then undercut).

Everything is keyed on the column's own biome, so the pass is deterministic and needs no cross-chunk
lookup.

## Trees and cross-chunk canopies

Tree placement (`biomes/tree_placement.rs`, `biomes/trees.rs`) is a pure function of the world seed
and the trunk's global column: a deterministic per-position hash gated by a low-frequency density
field decides where trunks grow and how tall they are. Each biome supplies its own spacing, density,
and tree kind through the shared `TreePlacer`; shapes (`Oak`, `Birch`, `Spruce`) live in `trees.rs`.

A canopy can overhang into neighbouring chunks. Rather than writing into other chunks (which would
require shared state and break per-chunk parallelism), every chunk **overscans** its neighbours by
`MAX_CANOPY_RADIUS` columns, resolves every nearby tree, and places only the blocks that fall within
its own `0..16` bounds. Because placement is deterministic, neighbouring chunks resolve the same
trees and their portions tile seamlessly.

The overscan width equals `MAX_CANOPY_RADIUS` exactly, so this constant is a hard contract:

- No tree shape's widest leaf ring may exceed `MAX_CANOPY_RADIUS` from its trunk. A wider ring would
  be clipped at chunk borders.
- Adding a larger tree requires raising `MAX_CANOPY_RADIUS`, which automatically widens the
  overscan.
- The contract is enforced two ways: `place_leaf_ring` asserts the radius in debug builds, and the
  `canopy_is_complete_across_chunks` test verifies that every leaf an unclipped tree would have is
  present once the surrounding chunks are generated.

Leaves are placed only into air, so adjacent solid terrain on a slope can legitimately occlude part
of a low canopy — this is expected and is not a chunk-boundary clip.

### Interaction with caves

Caves are carved *before* trees, so a cave can never hollow out a trunk or canopy that was already
placed. Trees are then kept clear of cave mouths: the placement loop skips any column where
`WorldGenerator::cave_opening_at_surface` reports a cave at the surface, so no tree sprouts over (or
is undercut by) a hole. That gate samples the continuous cave noise directly (not the per-chunk
interpolation grid), so it is a seed-pure function every chunk agrees on — at the cost of a small
approximation against the actual carve.

## Vegetation

Ground plants are scattered in a final pass (`biomes/vegetation.rs`) after trees, so they land on
the real, post-cave surface and never inside a trunk or a cave. Every plant occupies a single column
(cacti stack vertically but stay in one column), so — unlike tree canopies — there is no cross-chunk
overhang and no overscan is needed. Placement is a pure per-column function of the world seed and the
global column (a salted hash, the same mix the tree placer uses), so it is stable and reproducible.

Each column's surface block decides what, if anything, grows:

| Biome | Ground | Plants |
| --- | --- | --- |
| plains (40), forest (21) | grass block | ~3% flowers (dandelion / poppy / cornflower / oxeye daisy), ferns in forest, then ~short grass; rest bare |
| desert (14) | sand | sparse cacti (1–2 tall) and dead bushes |

Other biomes (oceans, beaches, snowy, mountains) grow nothing for now. Snowy biomes are skipped
naturally: their surface block is the snow layer, not a grass block, so the grassland rule does not
match.

## Key constants

| Constant | Location | Value | Meaning |
| --- | --- | --- | --- |
| `SEA_LEVEL` | `climate.rs` | 63 | Global water level. |
| `MAX_GENERATED_HEIGHT` | `lib.rs` | 176 | Stone-fill ceiling; exceeds the tallest possible surface. |
| `MIN_WORLD_Y` | `lib.rs` | -64 | Overworld floor / bedrock layer. |
| `MOUNTAIN_SNOW_LINE` | `lib.rs` | 110 | Surface height above which mountain peaks are snow-capped. |
| `PLAINS_LEVEL` | `carving/initial_height.rs` | 70 | Low elevation that high-erosion (flat) land is pulled down toward. |
| `MAX_CANOPY_RADIUS` | `biomes/trees.rs` | 2 | Tree overscan contract (see above). |

## Tuning notes

- **Region scale** is set by the climate frequencies in `Climate::new`. Lower frequency → larger
  oceans/continents and biome bands.
- **Land/ocean ratio and coast shape** are set by the continentalness spline control points.
- **Plains height and the plains/mountain fault** are set by `PLAINS_LEVEL` and the `flatness` control
  points in `carving/initial_height.rs`: lower `PLAINS_LEVEL` → lower plains; a steeper `flatness`
  riser → a sharper terrace/cliff between plains and high ground.
- **Hilliness** is bounded by `MIN/MAX_DETAIL_AMPLITUDE` in `carving/initial_height.rs`; ruggedness is
  driven by erosion via `flatness` (low erosion → full hills).
- **Biome boundaries** are the thresholds in `classify` (and `ocean_variant_id` for oceans).

## Files

- `lib.rs` — pipeline orchestration, biome classification (`classify`, `ocean_variant_id`), water
  pass, surface finish (grass + snow), cave-mouth tree gate.
- `climate.rs` — climate noise fields, continentalness spline, `SEA_LEVEL`.
- `carving/initial_height.rs` — detail noise and surface composition (`column`, `carve_surface`).
- `carving/erosion.rs` — erosion noise field.
- `terrain_noise.rs` — `NoiseGenerator` (fBm) and `Spline` helpers.
- `biomes/` — per-biome decorators, shared tree placement, tree shapes, and ground vegetation
  (`vegetation.rs`).
- `caves.rs` — cave carving.
