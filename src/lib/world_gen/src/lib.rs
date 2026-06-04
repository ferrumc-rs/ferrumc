//! World generation.
//!
//! This is a port of the layered terrain pipeline from the upstream `feature/nicer-terrain`
//! branch, adapted to FerrumC's current chunk model. The upstream branch rewrote the entire
//! chunk/section model (`chunk_format`, `EditBatch`, per-section `block_counts`, etc.); rather
//! than pull that in (which would conflict with the fluid system, palette, and network
//! serialization), this port keeps the existing [`ferrumc_world::chunk::Chunk`] API and reproduces
//! only the *generation algorithm*:
//!
//! 1. Fill the column with stone up to [`MAX_GENERATED_HEIGHT`].
//! 2. Carve an initial height field from broad height noise (`carving::initial_height`).
//! 3. Apply erosion to lower the surface further (`carving::erosion`).
//! 4. Per column, pick a biome from the noise/height and let it decorate the surface
//!    (`biomes::{plains, ocean, mountain}`).
//! 5. Carve caves (retained from FerrumC's existing generator; the upstream branch dropped them).
//!
//! The per-column surface height and noise values are passed around as plain local arrays
//! (`Heightmap` / `ColumnNoise`) during a single `generate_chunk` call rather than being stored on
//! the chunk, since nothing outside generation needs them.

mod biomes;
mod carving;
mod caves;
pub mod errors;
mod interp;
mod terrain_noise;

use crate::errors::WorldGenError;
use crate::terrain_noise::NoiseGenerator;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::{ChunkBlockPos, ChunkPos};
use noise::{MultiFractal, NoiseFn, RidgedMulti};

/// The highest Y the generator fills before carving. The whole column is stone up to here, then
/// height + erosion carve it back down to the real surface.
pub const MAX_GENERATED_HEIGHT: i16 = 192;

/// The nominal sea-level-ish baseline the height field is centred on.
pub const BASELINE_HEIGHT: i16 = 82;

/// The lowest world Y (overworld floor). The bedrock layer sits here.
const MIN_WORLD_Y: i16 = -64;

/// A per-column surface height map for one chunk (`[local_x][local_z]`).
pub(crate) type Heightmap = [[i16; 16]; 16];

/// Per-column noise values captured during carving, reused for biome selection.
#[derive(Default, Clone, Copy)]
pub(crate) struct ColumnNoise {
    pub erosion: f32,
    pub height: f32,
}

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
    /// for that column. `chunk_x` / `chunk_z` are the chunk's grid coordinates; combined with
    /// the local `x` / `z` they give the global block position needed for cross-chunk-consistent
    /// feature placement (e.g. trees must sample noise at the same global position regardless of
    /// which chunk the tree trunk falls in).
    fn decorate(
        &self,
        chunk: &mut Chunk,
        x: u8,
        z: u8,
        surface_y: i16,
        chunk_x: i32,
        chunk_z: i32,
    ) -> Result<(), WorldGenError>;
    fn new(seed: u64) -> Self
    where
        Self: Sized;
}

/// Terrain generator. Holds the noise samplers and the per-biome surface decorators; one instance
/// is shared across all chunk generation for a world.
///
/// The biome decorators (`plains`, `ocean`, `mountain`) only depend on the world seed, so they are
/// built once here rather than per column. Constructing them is not free — each builds its own
/// fractal-noise samplers — so rebuilding them for every column was a significant per-chunk cost.
pub struct WorldGenerator {
    height_noise: NoiseGenerator,
    erosion_noise: NoiseGenerator,
    caves_layer: RidgedMulti<noise::OpenSimplex>,
    plains: biomes::plains::PlainsBiome,
    ocean: biomes::ocean::OceanBiome,
    mountain: biomes::mountain::MountainBiome,
}

impl WorldGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            height_noise: carving::initial_height::height_noise(seed.wrapping_add(2)),
            erosion_noise: carving::erosion::erosion_noise(seed.wrapping_add(3)),
            caves_layer: RidgedMulti::<noise::OpenSimplex>::new((seed.wrapping_add(100)) as u32)
                .set_frequency(0.01)
                .set_lacunarity(2.5)
                .set_octaves(5)
                .set_persistence(0.8)
                .set_attenuation(0.3),
            plains: biomes::plains::PlainsBiome::new(seed),
            ocean: biomes::ocean::OceanBiome::new(seed),
            mountain: biomes::mountain::MountainBiome::new(seed),
        }
    }

    /// 3D cave noise sample (retained from FerrumC's previous generator).
    pub(crate) fn cave_noise(&self, x: f64, y: f64, z: f64) -> f64 {
        self.caves_layer.get([x, y, z])
    }
    /// Selects the biome for a column from its captured noise/height. Returns a shared reference to
    /// one of the pre-built decorators (see [`WorldGenerator`]); selection is pure, so the decorator
    /// is borrowed rather than allocated per column.
    fn get_biome(&self, noise: ColumnNoise, surface_y: i16) -> &dyn BiomeGenerator {
        if surface_y < 50 {
            return &self.ocean;
        }
        if noise.erosion <= 0.3 {
            return &self.mountain;
        }
        &self.plains
    }

    pub fn generate_chunk(&self, pos: ChunkPos) -> Result<Chunk, WorldGenError> {
        let mut chunk = Chunk::new_empty();

        // 1. Fill every section whose top is at or below MAX_GENERATED_HEIGHT with stone.
        //    Section index i covers world Y [i*16-64, i*16-64+15]; fill while the section bottom is
        //    below the generated ceiling.
        let stone = ferrumc_macros::block!("stone");
        let top_section = (MAX_GENERATED_HEIGHT / 16) as i8; // 192/16 = 12
        for section_y in -4..top_section {
            chunk.fill_section(section_y, stone);
        }

        // 2 + 3. Carve the surface height field (initial height, then erosion). These return the
        //         per-column surface height and the noise values used, captured for biome
        //         selection.
        let mut heightmap: Heightmap = [[BASELINE_HEIGHT; 16]; 16];
        let mut col_noise = [[ColumnNoise::default(); 16]; 16];
        self.apply_initial_height(&mut chunk, pos, &mut heightmap, &mut col_noise);
        self.apply_erosion(&mut chunk, pos, &mut heightmap, &mut col_noise);

        // 4. Decorate each column according to its biome, recording the ID for step 4b.
        let mut col_biome_ids = [[0u8; 16]; 16];
        for x in 0..16u8 {
            for z in 0..16u8 {
                let surface_y = heightmap[x as usize][z as usize];
                let biome = self.get_biome(col_noise[x as usize][z as usize], surface_y);
                biome.decorate(&mut chunk, x, z, surface_y, pos.x(), pos.z())?;
                col_biome_ids[x as usize][z as usize] = biome.biome_id();
            }
        }

        // 4b. Write biome IDs into the chunk's section biome data.
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

        // 5. Carve caves through the solid terrain (retained feature).
        self.generate_caves(&mut chunk, pos);

        // 6. Lay an unbreakable bedrock floor at the bottom of the world (Y = -64). Done last so
        //    cave carving cannot punch through it.
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
}
