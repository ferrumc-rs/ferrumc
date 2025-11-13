use crate::biome::Biome;
use crate::biome_chunk::{BiomeChunk, NoisePoint};
use crate::errors::WorldGenError;
use crate::overworld::carver::OverworldCarver;
use crate::overworld::noise_biome_parameters::overworld_biomes;
use crate::overworld::noise_depth::{OverworldBiomeNoise, generate_interpolation_data};
use crate::overworld::surface::OverworldSurface;
use crate::pos::{ChunkHeight, ChunkPos};
use crate::random::Xoroshiro128PlusPlus;
use bevy_math::IVec2;
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::vanilla_chunk_format::BlockData;
use itertools::Itertools;

pub struct OverworldGenerator {
    seed: u64,
    biome_seed: u64,
    chunk_height: ChunkHeight,
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
        let chunk_height = ChunkHeight {
            min_y: -64,
            height: 384,
        };
        Self {
            seed,
            biome_seed: u64::from_be_bytes(
                cthash::sha2_256(&seed.to_be_bytes())[0..8]
                    .try_into()
                    .unwrap(),
            ),
            chunk_height,
            biome_noise,
            biomes: overworld_biomes(),
            surface: OverworldSurface::new(random, chunk_height),
            carver: OverworldCarver::new(chunk_height),
        }
    }

    fn generate_biomes(&self, pos: ChunkPos) -> BiomeChunk {
        BiomeChunk::generate(
            &self.biome_noise,
            self.seed,
            &self.biomes,
            pos,
            self.chunk_height,
        )
    }

    pub fn generate_chunk(&self, x: i32, z: i32) -> Result<Chunk, WorldGenError> {
        let mut chunk = Chunk::new(x, z, "overworld".to_string());
        let stone = BlockData {
            name: "minecraft:stone".to_string(),
            properties: None,
        }
        .to_block_id();
        let air = BlockData::default().to_block_id();
        if x.abs() < 4 && z.abs() < 4 {
            // generate_interpolation_data(
            //     &self.biome_noise,
            //     ChunkPos::from(IVec2::new(x * 16, z * 16)),
            //     &mut chunk,
            // );
            ChunkPos::from(IVec2::new(x * 16, z * 16))
                .iter_columns()
                .cartesian_product(self.chunk_height.iter())
                .map(|(c, y)| c.block(y))
                .map(|pos| {
                    let final_density = self
                        .biome_noise
                        .post_process(pos, self.biome_noise.pre_baked_final_density(pos));
                    chunk.set_block(
                        pos.x,
                        pos.y,
                        pos.z,
                        if final_density > 0.0 { stone } else { air },
                    )
                })
                .find(Result::is_err)
                .unwrap_or(Ok(()))?;
        }
        Ok(chunk)
    }
}
