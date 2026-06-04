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
5. **Place trees** — including the canopies of trees rooted in neighbouring chunks (see
   [Trees](#trees-and-cross-chunk-canopies)).
6. **Carve caves** — 3D ridged-noise caves (retained from the previous generator).
7. **Bedrock floor** — an unbreakable layer at `Y = -64`, laid last so caves cannot punch through.

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
0.00 → 30   deep ocean floor
0.30 → 48   continental shelf
0.42 → 62   coastline (just below sea level)
0.50 → 68   low inland
0.70 → 88   hills
1.00 → 112  high inland
```

### Surface height composition

Per column (`carving/initial_height.rs`, `WorldGenerator::column`):

```
base       = continental_spline(continentalness)        # absolute base height
landness   = clamp(continentalness / 0.42, 0, 1)         # 0 in ocean, 1 fully inland
amplitude  = (MIN + (MAX - MIN) * landness) * (1 - 0.7 * erosion)
detail     = (detail_noise * 2 - 1) * amplitude          # ~80-block hills
surface    = base + detail
```

`detail_noise` (`0.0125`, ~80-block features) is the higher-frequency layer that adds the hills a
player notices while walking. Its amplitude is scaled down in oceans (low `landness`) and in
high-erosion regions, so the sea floor and eroded plateaus stay flat while inland low-erosion ground
gets proper relief. `MIN_DETAIL_AMPLITUDE` / `MAX_DETAIL_AMPLITUDE` bound it (currently `3` / `37`).

## Biomes

`WorldGenerator::get_biome` classifies each column from its climate sample and surface height,
returning a shared reference to one of the pre-built decorators. Because the climate fields are
low-frequency, the result is broad bands rather than per-column noise.

Selection order:

1. **Water / coast**, by surface height relative to `SEA_LEVEL`:
   - `surface < SEA_LEVEL` → ocean; the deepest basins (low continentalness or far below the
     surface) → deep ocean.
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
| ocean | 35 | `ocean.rs` | sand + sandstone sea bed |
| deep_ocean | 13 | `ocean.rs` | same decorator, different ID |
| beach | 3 | `beach.rs` | sand strip over sandstone |
| snowy_beach | 45 | `beach.rs` | same decorator, different ID |
| snowy_plains | 46 | `snowy.rs` | snowy grass + snow layer, treeless |
| snowy_taiga | 48 | `snowy.rs` | snowy grass + snow layer, spruce |
| windswept_hills | 62 | `mountain.rs` | bare stone, snow cap above `Y = 110` |

Registry IDs are the indices of the biome in the dynamic biome registry sent during configuration
(see `assets/data/registry_packets.json`). Mixed-biome sections are not yet encoded on the network
path, so each chunk is filled with a single dominant biome (the most common of the 16 biome-cell
columns).

## Water

Water is **not** tied to a biome. After decoration, a single global pass floods every column whose
surface is below `SEA_LEVEL` (63), filling from the surface up to the water level. This is what
produces natural coastlines and inland basins, and it decouples the water level from biome selection
(an earlier design flooded only inside the ocean biome and to a different level, which left a band of
"dry sea bed" where the two disagreed).

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

## Key constants

| Constant | Location | Value | Meaning |
| --- | --- | --- | --- |
| `SEA_LEVEL` | `climate.rs` | 63 | Global water level. |
| `MAX_GENERATED_HEIGHT` | `lib.rs` | 176 | Stone-fill ceiling; exceeds the tallest possible surface. |
| `MIN_WORLD_Y` | `lib.rs` | -64 | Overworld floor / bedrock layer. |
| `MAX_CANOPY_RADIUS` | `biomes/trees.rs` | 2 | Tree overscan contract (see above). |

## Tuning notes

- **Region scale** is set by the climate frequencies in `Climate::new`. Lower frequency → larger
  oceans/continents and biome bands.
- **Land/ocean ratio and coast shape** are set by the continentalness spline control points.
- **Hilliness** is set by `MIN/MAX_DETAIL_AMPLITUDE` and `EROSION_FLATTENING` in
  `carving/initial_height.rs`.
- **Biome boundaries** are the thresholds in `get_biome`.

## Files

- `lib.rs` — pipeline orchestration, biome selection, water pass.
- `climate.rs` — climate noise fields, continentalness spline, `SEA_LEVEL`.
- `carving/initial_height.rs` — detail noise and surface composition (`column`, `carve_surface`).
- `carving/erosion.rs` — erosion noise field.
- `terrain_noise.rs` — `NoiseGenerator` (fBm) and `Spline` helpers.
- `biomes/` — per-biome decorators, shared tree placement, and tree shapes.
- `caves.rs` — cave carving.
