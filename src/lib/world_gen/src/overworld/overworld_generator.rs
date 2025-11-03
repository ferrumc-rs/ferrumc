use crate::biome::Biome;
use crate::biome_chunk::{BiomeChunk, NoisePoint};
use crate::errors::WorldGenError;
use crate::overworld::carver::OverworldCarver;
use crate::overworld::noise_biome_parameters::overworld_biomes;
use crate::overworld::noise_depth::OverworldBiomeNoise;
use crate::overworld::surface::OverworldSurface;
use crate::pos::{ChunkHeight, ChunkPos};
use crate::random::Xoroshiro128PlusPlus;
use bevy_math::IVec2;
use ferrumc_macros::block;
use ferrumc_world::block_id::BlockId;
use ferrumc_world::chunk_format::Chunk;
use itertools::Itertools;

pub(super) const CHUNK_HEIGHT: ChunkHeight = ChunkHeight::new(-64, 380);

pub struct OverworldGenerator {
    seed: u64,
    biome_seed: u64,
    biome_noise: OverworldBiomeNoise,
    biomes: Vec<(NoisePoint, Biome)>,
    surface: OverworldSurface,
    carver: OverworldCarver,
}

impl OverworldGenerator {
    pub fn new(_seed: u64) -> Self {
        let seed = 1;
        let random = Xoroshiro128PlusPlus::from_seed(seed).fork();
        let biome_noise = OverworldBiomeNoise::new(random);
        Self {
            seed,
            biome_seed: u64::from_be_bytes(
                cthash::sha2_256(&seed.to_be_bytes())[0..8]
                    .try_into()
                    .unwrap(),
            ),
            biome_noise,
            biomes: overworld_biomes(),
            surface: OverworldSurface::new(random),
            carver: OverworldCarver::new(),
        }
    }

    fn generate_biomes(&self, pos: ChunkPos) -> BiomeChunk {
        BiomeChunk::generate(
            &self.biome_noise,
            self.seed,
            &self.biomes,
            pos,
            CHUNK_HEIGHT,
        )
    }

    pub fn generate_chunk(&self, x: i32, z: i32) -> Result<Chunk, WorldGenError> {
        let mut chunk = Chunk::new(x, z, "overworld".to_string());
        if x.abs() < 4 && z.abs() < 4 {
            // generate_interpolation_data(
            //     &self.biome_noise,
            //     ChunkPos::from(IVec2::new(x * 16, z * 16)),
            //     &mut chunk,
            // );
            ChunkPos::from(IVec2::new(x * 16, z * 16))
                .iter_columns()
                .cartesian_product(CHUNK_HEIGHT.iter())
                .map(|(c, y)| c.block(y))
                .map(|pos| {
                    let final_density = self
                        .biome_noise
                        .post_process(pos, self.biome_noise.pre_baked_final_density(pos));
                    chunk.set_block(
                        pos.x,
                        pos.y,
                        pos.z,
                        if final_density > 0.0 {
                            block!("stone")
                        } else {
                            block!("air")
                        },
                    )
                })
                .find(Result::is_err)
                .unwrap_or(Ok(()))?;
        }
        Ok(chunk)
    }
}
