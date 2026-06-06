//! World generation.
//!
//! A layered, climate-driven terrain pipeline built on FerrumC's existing
//! [`ferrumc_world::chunk::Chunk`] API. Large-scale region layout (oceans, continents, biome bands)
//! is driven by low-frequency climate noise rather than per-column choices, so generation produces
//! broad contiguous regions instead of uniform noise (see [`climate`]).
//!
//! 1. Fill the column with stone up to [`MAX_GENERATED_HEIGHT`].
//! 2. Carve the surface in a single pass (`carving`): the climate continentalness field sets an
//!    absolute base height, a local detail field adds hills (its amplitude scaled by continentalness
//!    and erosion), and the stone above the final surface is cleared.
//! 3. Per column, pick a biome from the climate sample + surface height and let it decorate the
//!    surface (`biomes`).
//! 4. Flood every column below [`climate::SEA_LEVEL`] with water in one global pass, independent of
//!    biome — so coastlines and inland basins fill naturally.
//! 5. Carve caves (`caves`), before trees so a cave can never hollow out a placed trunk.
//! 6. Re-cover any dirt a cave exposed at the surface of a grassy biome with grass.
//! 7. Place trees, including the canopies of trees rooted in neighbouring chunks; columns where a
//!    cave opens at the surface are skipped so no tree sprouts over a hole.
//! 8. Scatter ground vegetation (grass, flowers, desert plants) on the finished surface.
//!
//! The per-column surface height and climate sample are passed around as plain local arrays
//! (`Heightmap` / `ColumnClimate`) during a single `generate_chunk` call rather than being stored on
//! the chunk, since nothing outside generation needs them.

mod biomes;
mod carving;
mod caves;
mod climate;
pub mod errors;
mod interp;
mod terrain_noise;

use crate::climate::{Climate, ClimateSample, SEA_LEVEL};
use crate::errors::WorldGenError;
use crate::terrain_noise::NoiseGenerator;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::{ChunkBlockPos, ChunkPos};
use noise::{MultiFractal, NoiseFn, RidgedMulti};

/// The highest Y the generator fills before carving. The whole column is stone up to here, then
/// the carving pass clears it back down to the real surface. Sized to comfortably exceed the
/// tallest surface the climate model can produce (continental base + detail amplitude).
pub const MAX_GENERATED_HEIGHT: i16 = 176;

/// The lowest world Y (overworld floor). The bedrock layer sits here.
const MIN_WORLD_Y: i16 = -64;

/// Surface height at and above which mountain (windswept_hills) peaks are capped with snow. Applied
/// in the post-cave surface-finish pass so the cap sits on the real top, never floating over a cave.
const MOUNTAIN_SNOW_LINE: i16 = 110;

/// A per-column surface height map for one chunk (`[local_x][local_z]`).
pub(crate) type Heightmap = [[i16; 16]; 16];

/// Per-column climate sample captured during carving, reused for biome selection. One entry per
/// column of the chunk being generated.
pub(crate) type ColumnClimate = [[ClimateSample; 16]; 16];

/// Trait implemented by each biome's surface decorator.
///
/// Biomes only *decorate* a single column (`local_x`, `local_z`): the base stone terrain and the
/// surface height are already established by the carving stages, so a biome just places its
/// surface blocks (grass/dirt, sand/sandstone + water, etc.) relative to the column's height.
pub(crate) trait BiomeGenerator {
    /// Returns the Minecraft registry biome ID for this biome (used in chunk data packets).
    fn biome_id(&self) -> u8;
    fn _biome_name(&self) -> String;
    /// Decorates column (`x`, `z`) of `chunk`, where `surface_y` is the established surface height
    /// for that column. Decoration only writes blocks within the column itself (surface cover, water
    /// fill, etc.); cross-chunk features such as trees are handled separately via [`tree_at`].
    ///
    /// [`tree_at`]: BiomeGenerator::tree_at
    fn decorate(
        &self,
        chunk: &mut Chunk,
        x: u8,
        z: u8,
        surface_y: i16,
    ) -> Result<(), WorldGenError>;
    /// Returns the tree to grow at the given global column, or `None` if none grows there.
    ///
    /// This must be a pure function of the world seed and the global position so it yields the same
    /// answer regardless of which chunk asks — that is what lets a chunk resolve trees rooted in its
    /// neighbours and place the canopy blocks that overhang into it (see [`biomes::trees`]). The
    /// default implementation grows no trees.
    fn tree_at(
        &self,
        _global_x: i32,
        _global_z: i32,
        _surface_y: i16,
    ) -> Option<biomes::trees::Tree> {
        None
    }
    fn new(seed: u64) -> Self
    where
        Self: Sized;
}

/// Terrain generator. Holds the noise samplers, the climate model, and the per-biome surface
/// decorators; one instance is shared across all chunk generation for a world.
///
/// The biome decorators only depend on the world seed, so they are built once here rather than per
/// column. Constructing them is not free — each builds its own fractal-noise samplers — so
/// rebuilding them for every column was a significant per-chunk cost.
pub struct WorldGenerator {
    /// Climate model: low-frequency continentalness/temperature/humidity driving region layout.
    pub(crate) climate: Climate,
    /// Erosion field (splined), used to damp local detail amplitude and to select biomes.
    pub(crate) erosion_noise: NoiseGenerator,
    /// Higher-frequency local detail layered on top of the continental base height.
    pub(crate) detail_noise: NoiseGenerator,
    caves_layer: RidgedMulti<noise::OpenSimplex>,
    // Pre-built biome decorators. Some decorators back several registry biomes that differ only in
    // ID (one `ocean` decorator serves every ocean variant; `beach`/`snowy_beach` share a shape);
    // the recorded ID is resolved in `classify`.
    plains: biomes::plains::PlainsBiome,
    forest: biomes::forest::ForestBiome,
    desert: biomes::desert::DesertBiome,
    ocean: biomes::ocean::OceanBiome,
    beach: biomes::beach::BeachBiome,
    snowy_beach: biomes::beach::BeachBiome,
    snowy_plains: biomes::snowy::SnowyBiome,
    snowy_taiga: biomes::snowy::SnowyBiome,
    mountain: biomes::mountain::MountainBiome,
    /// Scatters ground vegetation (grass, flowers, desert plants) in a pass after trees.
    vegetation: biomes::vegetation::Vegetation,
}

impl WorldGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            climate: Climate::new(seed),
            erosion_noise: carving::erosion::erosion_noise(seed.wrapping_add(3)),
            detail_noise: carving::initial_height::detail_noise(seed.wrapping_add(2)),
            // Caves are sampled on a coarse 3D grid and trilinearly interpolated, which already
            // smooths away the finest octaves; three octaves give effectively the same large-scale
            // tunnel/chamber structure as five for roughly 40% fewer noise evaluations (the cave
            // pass is ~half of all chunk generation cost). The lacunarity is raised so the three
            // octaves still span a useful range of feature sizes.
            caves_layer: RidgedMulti::<noise::OpenSimplex>::new((seed.wrapping_add(100)) as u32)
                .set_frequency(0.01)
                .set_lacunarity(2.8)
                .set_octaves(3)
                .set_persistence(0.8)
                .set_attenuation(0.3),
            plains: biomes::plains::PlainsBiome::new(seed),
            forest: biomes::forest::ForestBiome::new(seed),
            desert: biomes::desert::DesertBiome::new(seed),
            ocean: biomes::ocean::OceanBiome::new(seed),
            beach: biomes::beach::BeachBiome::with_id(3),
            snowy_beach: biomes::beach::BeachBiome::with_id(45),
            snowy_plains: biomes::snowy::SnowyBiome::plains(seed),
            snowy_taiga: biomes::snowy::SnowyBiome::taiga(seed),
            mountain: biomes::mountain::MountainBiome::new(seed),
            vegetation: biomes::vegetation::Vegetation::new(seed.wrapping_add(53)),
        }
    }

    /// 3D cave noise sample (retained from FerrumC's previous generator).
    pub(crate) fn cave_noise(&self, x: f64, y: f64, z: f64) -> f64 {
        self.caves_layer.get([x, y, z])
    }

    /// Classifies a column from its climate sample and surface height, returning the surface
    /// decorator to run and the registry biome ID to record. Selection is pure, so the decorator is
    /// borrowed (not allocated) per column and every chunk agrees on every column.
    ///
    /// Decorator and ID usually coincide (`id == decorator.biome_id()`), but ocean uses one shared
    /// sea-bed decorator across many registry variants whose ID is chosen from temperature and depth
    /// by [`WorldGenerator::ocean_variant_id`].
    ///
    /// Water and coast are resolved by surface height relative to [`SEA_LEVEL`] first; the remaining
    /// land is classified on a temperature × humidity grid, with the most rugged low-erosion peaks
    /// becoming mountains. Because the climate fields are low-frequency, the result is broad bands.
    fn classify(&self, c: ClimateSample, surface_y: i16) -> (&dyn BiomeGenerator, u8) {
        let cold = c.temperature < 0.30;

        // Water and coastline. Submerged columns are ocean variants chosen by temperature/depth.
        if surface_y < SEA_LEVEL {
            let deep = c.continentalness < 0.20 || surface_y <= SEA_LEVEL - 18;
            return (&self.ocean, Self::ocean_variant_id(c.temperature, deep));
        }
        if surface_y <= SEA_LEVEL + 2 {
            let beach: &dyn BiomeGenerator = if cold { &self.snowy_beach } else { &self.beach };
            return (beach, beach.biome_id());
        }

        // Land. Rugged, low-erosion high ground becomes mountains.
        let land: &dyn BiomeGenerator = if c.erosion < 0.30 && surface_y >= 95 {
            &self.mountain
        } else if cold {
            if c.humidity < 0.5 {
                &self.snowy_plains
            } else {
                &self.snowy_taiga
            }
        } else if c.temperature < 0.62 {
            if c.humidity < 0.45 {
                &self.plains
            } else {
                &self.forest
            }
        } else if c.humidity < 0.40 {
            // Hot and dry.
            &self.desert
        } else {
            &self.forest
        };
        (land, land.biome_id())
    }

    /// The decorator for a column, ignoring the recorded ID (used by tree placement, which only
    /// needs `tree_at`).
    fn biome_decorator(&self, c: ClimateSample, surface_y: i16) -> &dyn BiomeGenerator {
        self.classify(c, surface_y).0
    }

    /// Picks the registry biome ID for a submerged column from temperature and depth. The sea bed is
    /// identical across variants; only the ID (and thus the client's water colour) differs.
    fn ocean_variant_id(temperature: f32, deep: bool) -> u8 {
        // (shallow, deep) registry IDs per temperature band.
        let (shallow, deep_id) = if temperature < 0.15 {
            (22, 11) // frozen_ocean, deep_frozen_ocean
        } else if temperature < 0.35 {
            (6, 9) // cold_ocean, deep_cold_ocean
        } else if temperature < 0.55 {
            (35, 13) // ocean, deep_ocean
        } else if temperature < 0.75 {
            (29, 12) // lukewarm_ocean, deep_lukewarm_ocean
        } else {
            (58, 58) // warm_ocean (no deep warm variant exists)
        };
        if deep { deep_id } else { shallow }
    }

    /// The grass block a biome covers its surface with, or `None` for biomes without grass. Used by
    /// the grass-fill pass to re-cover dirt that cave carving exposed at the surface.
    fn grass_cover_for(biome_id: u8) -> Option<BlockStateId> {
        match biome_id {
            40 | 21 => Some(ferrumc_macros::block!("grass_block", { snowy: false })), // plains, forest
            46 | 48 => Some(ferrumc_macros::block!("grass_block", { snowy: true })), // snowy_plains, snowy_taiga
            _ => None,
        }
    }

    /// Whether a cave opens at the column's surface, used as a pure gate so trees do not sprout over
    /// (or get undercut by) a cave mouth. This samples the continuous cave noise directly rather than
    /// the per-chunk interpolation grid `generate_caves` uses, so it is a seed-pure function of the
    /// column (every chunk agrees) at the cost of a small approximation against the actual carve.
    fn cave_opening_at_surface(&self, global_x: i32, global_z: i32, surface_y: i16) -> bool {
        // Slightly below the carve threshold (0.6) so trees keep clear of cave-mouth rims, not just
        // the hole itself.
        const GATE: f64 = 0.55;
        self.cave_noise(
            f64::from(global_x) / 2.0,
            f64::from(surface_y) / 2.0,
            f64::from(global_z) / 2.0,
        ) > GATE
    }

    pub fn generate_chunk(&self, pos: ChunkPos) -> Result<Chunk, WorldGenError> {
        let mut chunk = Chunk::new_empty();

        // 1. Fill every section up to the generated ceiling with stone. Section coordinate `s`
        //    covers world Y [s*16, s*16+15]; the overworld floor is at section -4 (Y -64).
        let stone = ferrumc_macros::block!("stone");
        let top_section = (MAX_GENERATED_HEIGHT / 16) as i8; // 176/16 = 11
        for section_y in -4..top_section {
            chunk.fill_section(section_y, stone);
        }

        // 2. Carve the surface in a single pass: compose each column's final height from the climate
        //    continentalness base + erosion-scaled local detail, clear the stone above it, and
        //    capture the surface height and climate sample for biome selection.
        let mut heightmap: Heightmap = [[SEA_LEVEL; 16]; 16];
        let mut col_climate: ColumnClimate = [[ClimateSample::default(); 16]; 16];
        self.carve_surface(&mut chunk, pos, &mut heightmap, &mut col_climate);

        // 3. Decorate each column according to its biome, recording the ID for the biome-data step.
        let mut col_biome_ids = [[0u8; 16]; 16];
        for x in 0..16u8 {
            for z in 0..16u8 {
                let surface_y = heightmap[x as usize][z as usize];
                let (biome, biome_id) =
                    self.classify(col_climate[x as usize][z as usize], surface_y);
                biome.decorate(&mut chunk, x, z, surface_y)?;
                col_biome_ids[x as usize][z as usize] = biome_id;
            }
        }

        // 4. Flood every column whose surface is below the water level, in one biome-independent
        //    pass. This is what gives natural coastlines and inland basins rather than tying water
        //    to a single ocean biome. Decoration of submerged columns has already laid a sea/lake
        //    bed (the ocean decorator), so this only fills the water above it.
        let water = ferrumc_macros::block!("water", { level: 0 });
        for x in 0..16u8 {
            for z in 0..16u8 {
                let surface_y = heightmap[x as usize][z as usize];
                for y in (surface_y + 1)..=SEA_LEVEL {
                    chunk.set_block(ChunkBlockPos::new(x, y, z), water);
                }
            }
        }

        // 5. Carve caves through the solid terrain. Done before trees so a cave can never carve away
        //    an already-placed trunk or canopy; trees are then gated away from cave mouths below.
        self.generate_caves(&mut chunk, pos);

        // 6. Finish the surface after carving: re-grass exposed dirt and lay snow on the real top.
        //    Both are applied to the *post-cave* top of each column, so a cave that opened the
        //    surface gets a grassy rim and snow that follows the carved ground rather than floating
        //    where the original surface used to be. Keyed on each column's own biome, so it is
        //    deterministic and needs no cross-chunk lookup.
        let snow = ferrumc_macros::block!("snow", { layers: 1 });
        for x in 0..16u8 {
            for z in 0..16u8 {
                let id = col_biome_ids[x as usize][z as usize];
                // Find the topmost solid (non-air, non-water) block in the column. Nothing solid is
                // ever placed above the carved surface height (decoration writes at or below it, the
                // water flood is skipped here, and caves only remove), so the scan can start there
                // instead of at the generated ceiling — skipping ~100 air blocks per column.
                let scan_top = heightmap[x as usize][z as usize].min(MAX_GENERATED_HEIGHT);
                let mut top_y = None;
                for y in (MIN_WORLD_Y..=scan_top).rev() {
                    let pos = ChunkBlockPos::new(x, y, z);
                    let b = chunk.get_block(pos);
                    if ferrumc_macros::match_block!("air", b)
                        || ferrumc_macros::match_block!("water", b)
                    {
                        continue;
                    }
                    // Re-cover bare dirt with the biome's grass.
                    if ferrumc_macros::match_block!("dirt", b)
                        && let Some(grass) = Self::grass_cover_for(id)
                    {
                        chunk.set_block(pos, grass);
                    }
                    top_y = Some(y);
                    break;
                }
                // Lay a snow layer on the real top of snowy biomes (always) and mountain peaks
                // (above the snow line), only into the air directly above the surface.
                if let Some(y) = top_y {
                    let wants_snow = matches!(id, 46 | 48) || (id == 62 && y >= MOUNTAIN_SNOW_LINE);
                    if wants_snow {
                        let above = ChunkBlockPos::new(x, y + 1, z);
                        if ferrumc_macros::match_block!("air", chunk.get_block(above)) {
                            chunk.set_block(above, snow);
                        }
                    }
                }
            }
        }

        // 7. Place trees, including the canopies of trees rooted in neighbouring chunks.
        //    A tree's canopy can overhang up to MAX_CANOPY_RADIUS blocks past its trunk, so this
        //    scans that far beyond the chunk edges. Tree placement is a pure function of the world
        //    seed and the trunk's global column, so every chunk resolves the same trees and writes
        //    only the blocks that fall within its own bounds — no cross-chunk writes, no shared
        //    state, no locking. Trunk columns outside this chunk are clipped by `place_tree`.
        //    Columns where a cave opens at the surface are skipped so no tree sprouts over a hole.
        let base_x = pos.x() * 16;
        let base_z = pos.z() * 16;
        let overscan = biomes::trees::MAX_CANOPY_RADIUS;
        for global_x in (base_x - overscan)..(base_x + 16 + overscan) {
            for global_z in (base_z - overscan)..(base_z + 16 + overscan) {
                // Interior columns were already evaluated during carving; reuse the stored height and
                // climate sample instead of recomputing the multi-octave noise fields. Only the
                // overscan ring outside this chunk needs a fresh `column()` call.
                let (surface_y, sample) = if (base_x..base_x + 16).contains(&global_x)
                    && (base_z..base_z + 16).contains(&global_z)
                {
                    let lx = (global_x - base_x) as usize;
                    let lz = (global_z - base_z) as usize;
                    (heightmap[lx][lz], col_climate[lx][lz])
                } else {
                    self.column(global_x, global_z)
                };
                if self.cave_opening_at_surface(global_x, global_z, surface_y) {
                    continue;
                }
                let biome = self.biome_decorator(sample, surface_y);
                if let Some(tree) = biome.tree_at(global_x, global_z, surface_y) {
                    biomes::trees::place_tree(
                        &mut chunk,
                        global_x - base_x,
                        global_z - base_z,
                        &tree,
                    );
                }
            }
        }

        // 8. Scatter ground vegetation (grass, flowers, desert plants) on the finished surface.
        //    Runs after trees so plants land on real ground and never inside a trunk; every plant is
        //    a single column, so no cross-chunk overscan is needed.
        self.vegetation
            .decorate(&mut chunk, base_x, base_z, &col_biome_ids);

        // 9. Write biome IDs into the chunk's section biome data.
        //     Mixed-biome sections require a network encoding path that is not yet implemented,
        //     so all sections are set to a single uniform biome. The dominant biome among the
        //     16 biome-cell columns (4×4 grid, one per 4-block square) is used; for most
        //     chunks this is the only biome present anyway, so the result is exact.
        let mut biome_counts = [0u32; 256];
        for cell_x in 0..4u8 {
            for cell_z in 0..4u8 {
                let id = col_biome_ids[usize::from(cell_x) * 4][usize::from(cell_z) * 4];
                biome_counts[usize::from(id)] += 1;
            }
        }
        let dominant_biome = biome_counts
            .iter()
            .enumerate()
            .max_by_key(|&(_, c)| c)
            .map(|(id, _)| id as u8)
            .unwrap_or(40); // plains as fallback
        chunk.fill_biome(dominant_biome);

        // 10. Lay an unbreakable bedrock floor at the bottom of the world (Y = -64). Done last so
        //     cave carving cannot punch through it.
        let bedrock = ferrumc_macros::block!("bedrock");
        for x in 0..16u8 {
            for z in 0..16u8 {
                chunk.set_block(ChunkBlockPos::new(x, MIN_WORLD_Y, z), bedrock);
            }
        }

        Ok(chunk)
    }

    /// Clears (sets to air) every block in column (`x`, `z`) strictly above `surface_y` up to the
    /// generated ceiling. Shared by the carving stages so a lowered surface actually removes the
    /// stone above it.
    fn clear_above(chunk: &mut Chunk, x: u8, z: u8, surface_y: i16) {
        let air = ferrumc_macros::block!("air");
        for y in (surface_y + 1)..=MAX_GENERATED_HEIGHT {
            chunk.set_block(ChunkBlockPos::new(x, y, z), air);
        }
    }
}

// Carving stage implementations live in the `carving` submodule (as `impl WorldGenerator`).

#[cfg(test)]
mod tests {
    use super::*;
    use ferrumc_macros::{block, match_block};

    #[test]
    fn generates_without_error() {
        let generator = WorldGenerator::new(0);
        assert!(generator.generate_chunk(ChunkPos::new(0, 0)).is_ok());
    }

    #[test]
    fn high_and_low_coordinates_are_ok() {
        let generator = WorldGenerator::new(67890);
        assert!(
            generator
                .generate_chunk(ChunkPos::new((1 << 21) - 1, (1 << 21) - 1))
                .is_ok()
        );
        assert!(
            generator
                .generate_chunk(ChunkPos::new(-((1 << 21) - 1), -((1 << 21) - 1)))
                .is_ok()
        );
    }

    #[test]
    fn generated_chunk_is_not_all_air() {
        let generator = WorldGenerator::new(13579);
        let chunk = generator.generate_chunk(ChunkPos::new(0, 0)).unwrap();
        let mut solid = 0;
        for x in 0..16u8 {
            for z in 0..16u8 {
                for y in -64..192i16 {
                    if !match_block!("air", chunk.get_block(ChunkBlockPos::new(x, y, z))) {
                        solid += 1;
                    }
                }
            }
        }
        assert!(solid > 0, "generated chunk should contain solid blocks");
    }

    #[test]
    fn neighbouring_chunks_differ() {
        let generator = WorldGenerator::new(24680);
        let a = generator.generate_chunk(ChunkPos::new(0, 0)).unwrap();
        let b = generator.generate_chunk(ChunkPos::new(10, 10)).unwrap();
        // Compare the surface column at local (0,0): different terrain noise should usually differ.
        let col = |c: &Chunk| {
            (-64..192i16)
                .map(|y| c.get_block(ChunkBlockPos::new(0, y, 0)).raw())
                .collect::<Vec<_>>()
        };
        assert_ne!(
            col(&a),
            col(&b),
            "distant chunks should generate differently"
        );
    }

    /// Computes the highest non-air Y in a column (the surface). Returns `None` if all air.
    fn surface_height(chunk: &Chunk, x: u8, z: u8) -> Option<i16> {
        (-64..256i16)
            .rev()
            .find(|&y| !match_block!("air", chunk.get_block(ChunkBlockPos::new(x, y, z))))
    }

    /// Regression guard against "superflat" terrain: the surface must vary by a meaningful amount
    /// across a region. Before the height-noise rescale the effective frequency was so low and the
    /// amplitude so small that every column landed at the same Y and the world looked flat.
    #[test]
    fn terrain_has_meaningful_height_variation() {
        let generator = WorldGenerator::new(0);
        let mut min_h = i16::MAX;
        let mut max_h = i16::MIN;
        for cx in 0..6 {
            for cz in 0..6 {
                let chunk = generator.generate_chunk(ChunkPos::new(cx, cz)).unwrap();
                for &(x, z) in &[(0u8, 0u8), (8, 8), (15, 15)] {
                    if let Some(h) = surface_height(&chunk, x, z) {
                        min_h = min_h.min(h);
                        max_h = max_h.max(h);
                    }
                }
            }
        }
        let spread = max_h - min_h;
        assert!(
            spread >= 12,
            "terrain is too flat: surface spread over the sampled region was only {spread} \
             blocks (min {min_h}, max {max_h}); expected hills of at least ~12 blocks"
        );
    }

    /// The bottom world layer (Y = -64) must be solid bedrock in every column, and caves must not
    /// have punched through it.
    #[test]
    fn world_floor_is_bedrock() {
        let generator = WorldGenerator::new(99);
        let chunk = generator.generate_chunk(ChunkPos::new(3, 7)).unwrap();
        for x in 0..16u8 {
            for z in 0..16u8 {
                let floor = chunk.get_block(ChunkBlockPos::new(x, -64, z));
                assert_eq!(
                    floor,
                    block!("bedrock"),
                    "world floor at ({x},-64,{z}) should be bedrock, was {floor:?}"
                );
            }
        }
    }

    /// Cross-chunk tree continuity: a tree rooted one column inside a chunk edge must drop its
    /// canopy into the neighbouring chunk. Regression guard against the old behaviour that clipped
    /// (discarded) boundary leaves instead of placing them in the neighbour, leaving half-trees.
    #[test]
    fn tree_canopy_crosses_chunk_boundary() {
        let generator = WorldGenerator::new(0);

        // Find a tree whose trunk sits on the western edge of its chunk (global x divisible by 16),
        // so its canopy overhangs the chunk to the west. Searching tree placement is cheap (no chunk
        // generation), so a wide scan reliably finds an edge-aligned tree.
        // Restrict to the plains biome so the tree is a known oak (forest mixes in birch, taiga uses
        // spruce), keeping the leaf-block assertion below unambiguous.
        let mut found = None;
        'search: for gx in (0..8192i32).step_by(16) {
            for gz in 0..512i32 {
                let (surface_y, sample) = generator.column(gx, gz);
                let (biome, id) = generator.classify(sample, surface_y);
                if id != 40 {
                    continue;
                }
                if let Some(tree) = biome.tree_at(gx, gz, surface_y) {
                    found = Some((gx, gz, tree.surface_y, tree.trunk_height));
                    break 'search;
                }
            }
        }
        let (gx, gz, surface_y, trunk_height) =
            found.expect("expected to find an edge-aligned plains tree within the scan region");

        // One column west of the trunk (dx = -1) is part of the wide leaf ring and lands at local
        // x = 15 of the western neighbour chunk. That leaf only exists if the canopy was placed
        // across the boundary rather than clipped.
        let neighbour = ChunkPos::new((gx >> 4) - 1, gz >> 4);
        let chunk = generator.generate_chunk(neighbour).unwrap();

        let leaf_y = surface_y + i16::from(trunk_height) - 1; // top of trunk, minus one (wide ring)
        let local_x = (gx - 1).rem_euclid(16) as u8; // == 15
        let local_z = gz.rem_euclid(16) as u8;
        let block = chunk.get_block(ChunkBlockPos::new(local_x, leaf_y, local_z));
        assert!(
            match_block!("oak_leaves", block),
            "expected a canopy leaf from the neighbouring tree at local \
             ({local_x},{leaf_y},{local_z}) of chunk {neighbour:?}, found {block:?}"
        );
    }

    /// Returns true if the 5×5 columns centred on (gx,gz) all share the same surface height, so a
    /// tree there has no terrain occluding its lower canopy (which would confound a leaf-presence
    /// check).
    fn flat_around(generator: &WorldGenerator, gx: i32, gz: i32) -> Option<i16> {
        let (h0, _) = generator.column(gx, gz);
        for dx in -2..=2 {
            for dz in -2..=2 {
                if generator.column(gx + dx, gz + dz).0 != h0 {
                    return None;
                }
            }
        }
        Some(h0)
    }

    /// Returns the set of leaf positions (absolute world coords) a tree produces with no clipping,
    /// by placing it at the centre of a scratch chunk (far from any edge) and reading back the
    /// canopy. Used as the ground-truth "unbounded" canopy to compare cross-chunk placement against.
    fn unbounded_canopy(
        surface_y: i16,
        kind: biomes::trees::TreeKind,
        trunk_height: u8,
    ) -> std::collections::HashSet<(i32, i32, i16)> {
        let mut scratch = Chunk::new_empty();
        for y in -4..(MAX_GENERATED_HEIGHT / 16) as i8 {
            scratch.fill_section(y, block!("stone"));
        }
        for x in 0..16u8 {
            for z in 0..16u8 {
                WorldGenerator::clear_above(&mut scratch, x, z, surface_y);
            }
        }
        biomes::trees::place_tree(
            &mut scratch,
            8,
            8,
            &biomes::trees::Tree {
                kind,
                surface_y,
                trunk_height,
            },
        );
        let mut leaves = std::collections::HashSet::new();
        for x in 0..16i32 {
            for z in 0..16i32 {
                for y in (surface_y - 1)..=(surface_y + i16::from(trunk_height) + 2) {
                    let b = scratch.get_block(ChunkBlockPos::new(x as u8, y, z as u8));
                    if match_block!("oak_leaves", b)
                        || match_block!("spruce_leaves", b)
                        || match_block!("birch_leaves", b)
                    {
                        leaves.insert((x - 8, z - 8, y)); // offset relative to the centred trunk
                    }
                }
            }
        }
        leaves
    }

    /// Cross-chunk canopy completeness: every leaf an unclipped tree would have must be present once
    /// the surrounding chunks are generated. This is the regression guard that keeps the
    /// `overscan == MAX_CANOPY_RADIUS` gate honest — if a new (larger) tree shape exceeds the gate,
    /// its overhang would be clipped and this fails. Trees are probed on flat ground so the
    /// air-only placement rule (which legitimately drops leaves into solid terrain on slopes) does
    /// not confound the check.
    #[test]
    fn canopy_is_complete_across_chunks() {
        let generator = WorldGenerator::new(0);
        let mut probed = 0;
        let mut failures: Vec<String> = vec![];

        'outer: for gx in -512..512i32 {
            // Only trunks within MAX_CANOPY_RADIUS of a chunk edge produce a cross-chunk canopy.
            let lx = gx.rem_euclid(16);
            let r = biomes::trees::MAX_CANOPY_RADIUS;
            if lx > r && lx < 16 - r {
                continue;
            }
            for gz in -512..512i32 {
                if probed >= 24 {
                    break 'outer;
                }
                let (surface_y, sample) = generator.column(gx, gz);
                // Match generation: a cave mouth at the surface suppresses the tree, so do not probe
                // those columns (their canopy is intentionally absent).
                if generator.cave_opening_at_surface(gx, gz, surface_y) {
                    continue;
                }
                let (biome, id) = generator.classify(sample, surface_y);
                let Some(tree) = biome.tree_at(gx, gz, surface_y) else {
                    continue;
                };
                if flat_around(&generator, gx, gz).is_none() {
                    continue;
                }

                let kind = match id {
                    48 => biomes::trees::TreeKind::Spruce,
                    _ => biomes::trees::TreeKind::Oak, // oak/birch share a shape
                };
                let truth = unbounded_canopy(surface_y, kind, tree.trunk_height);

                let cx0 = gx >> 4;
                let cz0 = gz >> 4;
                for &(odx, odz, y) in &truth {
                    let wx = gx + odx;
                    let wz = gz + odz;
                    let chunk = generator
                        .generate_chunk(ChunkPos::new(wx >> 4, wz >> 4))
                        .unwrap();
                    let b = chunk.get_block(ChunkBlockPos::new(
                        wx.rem_euclid(16) as u8,
                        y,
                        wz.rem_euclid(16) as u8,
                    ));
                    let present = match_block!("oak_leaves", b)
                        || match_block!("spruce_leaves", b)
                        || match_block!("birch_leaves", b);
                    if !present {
                        failures.push(format!(
                            "tree@({gx},{gz}) biome={id} chunk=({cx0},{cz0}) leaf off \
                             ({odx},{odz},dy={}) missing, found {b:?}",
                            y - surface_y
                        ));
                    }
                }
                probed += 1;
            }
        }
        assert!(
            probed > 0,
            "found no edge-aligned trees on flat ground to probe"
        );
        assert!(
            failures.is_empty(),
            "cross-chunk canopy gaps across {probed} trees:\n{}",
            failures.join("\n")
        );
    }

    /// The climate model must produce both ocean basins (surface well below sea level) and raised
    /// continents (surface well above it) across a region — not the uniform mid-height terrain the
    /// previous single height field produced.
    #[test]
    fn oceans_and_continents_exist() {
        let generator = WorldGenerator::new(0);
        let mut saw_ocean = false;
        let mut saw_continent = false;
        // The pure column function is cheap, so a wide sparse sweep is enough to span several
        // continentalness regions at the ~600-block region scale.
        for gx in (-4096..4096i32).step_by(64) {
            for gz in (-4096..4096i32).step_by(64) {
                let (surface_y, _) = generator.column(gx, gz);
                if surface_y < SEA_LEVEL - 5 {
                    saw_ocean = true;
                }
                if surface_y > SEA_LEVEL + 15 {
                    saw_continent = true;
                }
            }
        }
        assert!(
            saw_ocean && saw_continent,
            "expected both ocean basins and raised continents \
             (saw_ocean={saw_ocean}, saw_continent={saw_continent})"
        );
    }

    /// Water must never appear above the world water level: the global flood pass fills only up to
    /// [`SEA_LEVEL`]. Regression guard against the old per-biome flood that could leave water
    /// stranded or fill to an inconsistent level.
    #[test]
    fn water_never_above_sea_level() {
        let generator = WorldGenerator::new(7);
        // Generate a spread of chunks likely to include coastline/ocean.
        for cx in -4..4 {
            for cz in -4..4 {
                let chunk = generator.generate_chunk(ChunkPos::new(cx, cz)).unwrap();
                for x in 0..16u8 {
                    for z in 0..16u8 {
                        for y in (SEA_LEVEL + 1)..MAX_GENERATED_HEIGHT {
                            let b = chunk.get_block(ChunkBlockPos::new(x, y, z));
                            assert!(
                                !match_block!("water", b),
                                "water found above sea level at ({x},{y},{z}) in chunk \
                                 ({cx},{cz})"
                            );
                        }
                    }
                }
            }
        }
    }

    /// Biomes must form a varied layout over a large region: at least a few distinct biomes should
    /// appear, confirming the climate grid is actually partitioning the world rather than collapsing
    /// to one biome.
    #[test]
    fn biome_variety_over_region() {
        let generator = WorldGenerator::new(0);
        let mut seen = std::collections::HashSet::new();
        for gx in (-8192..8192i32).step_by(128) {
            for gz in (-8192..8192i32).step_by(128) {
                let (surface_y, sample) = generator.column(gx, gz);
                seen.insert(generator.classify(sample, surface_y).1);
            }
        }
        assert!(
            seen.len() >= 3,
            "expected at least 3 distinct biomes over the region, saw {}: {:?}",
            seen.len(),
            seen
        );
    }

    /// Coastlines must be sandy: a column just above sea level should classify as a beach (or its
    /// snowy variant), so land never meets water with a grass cliff.
    #[test]
    fn coast_classifies_as_beach() {
        let generator = WorldGenerator::new(0);
        let mut saw_beach = false;
        'scan: for gx in (-8192..8192i32).step_by(8) {
            for gz in (-8192..8192i32).step_by(8) {
                let (surface_y, sample) = generator.column(gx, gz);
                let id = generator.classify(sample, surface_y).1;
                if id == 3 || id == 45 {
                    saw_beach = true;
                    break 'scan;
                }
            }
        }
        assert!(
            saw_beach,
            "expected at least one beach column along a coastline"
        );
    }

    /// Caves must never undercut a tree: trees are placed after caves and gated away from cave
    /// mouths, so every tree trunk sits on solid ground. Regression guard against the old ordering
    /// that carved caves last and could hollow out a trunk or the block beneath it.
    #[test]
    fn trees_not_undercut_by_caves() {
        let generator = WorldGenerator::new(0);
        for cx in -3..3 {
            for cz in -3..3 {
                let chunk = generator.generate_chunk(ChunkPos::new(cx, cz)).unwrap();
                for x in 0..16u8 {
                    for z in 0..16u8 {
                        // Find the lowest log in the column, if any.
                        let mut lowest_log = None;
                        for y in MIN_WORLD_Y..MAX_GENERATED_HEIGHT {
                            let b = chunk.get_block(ChunkBlockPos::new(x, y, z));
                            if match_block!("oak_log", b)
                                || match_block!("birch_log", b)
                                || match_block!("spruce_log", b)
                            {
                                lowest_log = Some(y);
                                break;
                            }
                        }
                        if let Some(y) = lowest_log {
                            let below = chunk.get_block(ChunkBlockPos::new(x, y - 1, z));
                            assert!(
                                !match_block!("air", below) && !match_block!("cave_air", below),
                                "tree trunk at ({x},{y},{z}) in chunk ({cx},{cz}) is undercut by \
                                 air below it ({below:?})"
                            );
                        }
                    }
                }
            }
        }
    }

    /// Snow must never float: it is laid in the post-cave surface-finish pass on the real top of
    /// each column, so every snow block must rest on a solid block. Regression guard against the old
    /// behaviour that placed snow at the pre-cave surface, which caves then undercut.
    #[test]
    fn snow_never_floats() {
        let generator = WorldGenerator::new(0);
        for cx in -3..3 {
            for cz in -3..3 {
                let chunk = generator.generate_chunk(ChunkPos::new(cx, cz)).unwrap();
                for x in 0..16u8 {
                    for z in 0..16u8 {
                        for y in (MIN_WORLD_Y + 1)..MAX_GENERATED_HEIGHT {
                            if match_block!("snow", chunk.get_block(ChunkBlockPos::new(x, y, z))) {
                                let below = chunk.get_block(ChunkBlockPos::new(x, y - 1, z));
                                assert!(
                                    !match_block!("air", below)
                                        && !match_block!("cave_air", below)
                                        && !match_block!("water", below),
                                    "floating snow at ({x},{y},{z}) in chunk ({cx},{cz}); \
                                     block below is {below:?}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    /// Ground vegetation must rest on valid ground: grass/fern/flowers on a grass block, cactus on
    /// sand or another cactus, dead bush on sand. Also a sanity check that some short grass is
    /// actually placed, so the pass is not silently doing nothing.
    #[test]
    fn vegetation_sits_on_valid_ground() {
        let generator = WorldGenerator::new(0);
        let mut short_grass_seen = 0u32;
        for cx in -4..4 {
            for cz in -4..4 {
                let chunk = generator.generate_chunk(ChunkPos::new(cx, cz)).unwrap();
                for x in 0..16u8 {
                    for z in 0..16u8 {
                        for y in (MIN_WORLD_Y + 1)..MAX_GENERATED_HEIGHT {
                            let b = chunk.get_block(ChunkBlockPos::new(x, y, z));
                            let below = chunk.get_block(ChunkBlockPos::new(x, y - 1, z));
                            if match_block!("short_grass", b) {
                                short_grass_seen += 1;
                            }
                            if match_block!("short_grass", b)
                                || match_block!("fern", b)
                                || match_block!("dandelion", b)
                                || match_block!("poppy", b)
                                || match_block!("cornflower", b)
                                || match_block!("oxeye_daisy", b)
                            {
                                assert!(
                                    match_block!("grass_block", below),
                                    "plant at ({x},{y},{z}) chunk ({cx},{cz}) not on grass: {below:?}"
                                );
                            }
                            if match_block!("dead_bush", b) {
                                assert!(
                                    match_block!("sand", below),
                                    "dead bush at ({x},{y},{z}) chunk ({cx},{cz}) not on sand: {below:?}"
                                );
                            }
                            if match_block!("cactus", b) {
                                assert!(
                                    match_block!("sand", below) || match_block!("cactus", below),
                                    "cactus at ({x},{y},{z}) chunk ({cx},{cz}) not on sand/cactus: {below:?}"
                                );
                            }
                        }
                    }
                }
            }
        }
        assert!(
            short_grass_seen > 0,
            "expected some short grass to be placed across the sampled chunks"
        );
    }

    /// Cacti must never have a horizontally adjacent block (the vanilla rule). Driven directly on a
    /// flat sand surface (natural deserts are far from the origin and slow to generate): the
    /// vegetation pass is run over many synthetic all-desert chunks, and every placed cactus must
    /// have only air at its four horizontal neighbours.
    #[test]
    fn cactus_has_no_horizontal_neighbours() {
        let vegetation = biomes::vegetation::Vegetation::new(7);
        let desert_ids = [[14u8; 16]; 16];
        let sand = block!("sand");
        let mut cactus_seen = 0u32;

        for cx in 0..16i32 {
            for cz in 0..16i32 {
                // A flat sand slab at Y=64, air above.
                let mut chunk = Chunk::new_empty();
                for x in 0..16u8 {
                    for z in 0..16u8 {
                        chunk.set_block(ChunkBlockPos::new(x, 64, z), sand);
                    }
                }
                vegetation.decorate(&mut chunk, cx * 16, cz * 16, &desert_ids);

                for x in 0..16u8 {
                    for z in 0..16u8 {
                        for y in 65..70i16 {
                            if !match_block!("cactus", chunk.get_block(ChunkBlockPos::new(x, y, z)))
                            {
                                continue;
                            }
                            cactus_seen += 1;
                            for (dx, dz) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                                let nx = i32::from(x) + dx;
                                let nz = i32::from(z) + dz;
                                assert!(
                                    (0..16).contains(&nx) && (0..16).contains(&nz),
                                    "cactus at edge column ({x},{y},{z})"
                                );
                                let n = chunk.get_block(ChunkBlockPos::new(nx as u8, y, nz as u8));
                                assert!(
                                    match_block!("air", n),
                                    "cactus at ({x},{y},{z}) has neighbour {n:?}"
                                );
                            }
                        }
                    }
                }
            }
        }
        assert!(
            cactus_seen > 0,
            "expected at least one cactus across the synthetic desert chunks"
        );
    }

    /// In grassy biomes, no column may end in bare dirt exposed to the sky: the grass-fill pass must
    /// re-cover dirt that cave carving stripped. The topmost solid (non-air, non-water) block of
    /// every column in a grassy chunk must not be plain dirt.
    #[test]
    fn no_exposed_dirt_top_in_grass_biomes() {
        let generator = WorldGenerator::new(0);
        for cx in -3..3 {
            for cz in -3..3 {
                let chunk = generator.generate_chunk(ChunkPos::new(cx, cz)).unwrap();
                for x in 0..16u8 {
                    for z in 0..16u8 {
                        let (surface_y, sample) =
                            generator.column(i32::from(x) + cx * 16, i32::from(z) + cz * 16);
                        let id = generator.classify(sample, surface_y).1;
                        if WorldGenerator::grass_cover_for(id).is_none() {
                            continue; // not a grassy column
                        }
                        for y in (MIN_WORLD_Y..=MAX_GENERATED_HEIGHT).rev() {
                            let b = chunk.get_block(ChunkBlockPos::new(x, y, z));
                            if match_block!("air", b) || match_block!("water", b) {
                                continue;
                            }
                            assert!(
                                !match_block!("dirt", b),
                                "bare dirt exposed at top of grassy column ({x},{y},{z}) in chunk \
                                 ({cx},{cz}); grass-fill should have covered it"
                            );
                            break;
                        }
                    }
                }
            }
        }
    }

    /// Oceans must come in more than one climate flavour across a large region (frozen/cold/ocean/
    /// lukewarm/warm and their deep forms), driven by the temperature field.
    #[test]
    fn ocean_biomes_have_variety() {
        let generator = WorldGenerator::new(0);
        // The set of ocean registry IDs the variant mapping can emit.
        let ocean_ids: std::collections::HashSet<u8> =
            [22u8, 11, 6, 9, 35, 13, 29, 12, 58].into_iter().collect();
        let mut seen = std::collections::HashSet::new();
        for gx in (-8192..8192i32).step_by(64) {
            for gz in (-8192..8192i32).step_by(64) {
                let (surface_y, sample) = generator.column(gx, gz);
                let id = generator.classify(sample, surface_y).1;
                if ocean_ids.contains(&id) {
                    seen.insert(id);
                }
            }
        }
        assert!(
            seen.len() >= 3,
            "expected at least 3 distinct ocean biomes over the region, saw {}: {:?}",
            seen.len(),
            seen
        );
    }
}
