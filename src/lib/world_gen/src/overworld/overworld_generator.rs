use crate::biome::Biome;
use crate::biome_chunk::{BiomeChunk, NoisePoint};
use crate::overworld::carver::OverworldCarver;
use crate::overworld::noise_biome_parameters::overworld_biomes;
use crate::overworld::noise_depth::OverworldBiomeNoise;
use crate::overworld::surface::OverworldSurface;
use crate::pos::{ChunkHeight, ChunkPos};
use crate::random::{Rng, Xoroshiro128PlusPlus};
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::vanilla_chunk_format::BlockData;
use itertools::Itertools;

pub struct OverworldGenerator {
    seed: u64,
    chunk_height: ChunkHeight,
    biome_noise: OverworldBiomeNoise,
    biomes: Vec<(NoisePoint, Biome)>,
    surface: OverworldSurface,
    carver: OverworldCarver,
}

impl OverworldGenerator {
    pub fn new(seed: u64) -> Self {
        let random = Xoroshiro128PlusPlus::from_seed(seed).fork_positional();
        let biome_noise = OverworldBiomeNoise::new(random);
        let chunk_height = ChunkHeight {
            min_y: -64,
            height: 384,
        };
        Self {
            seed,
            chunk_height,
            biome_noise,
            biomes: overworld_biomes(),
            surface: OverworldSurface::new(random, chunk_height),
            carver: OverworldCarver::new(),
        }
    }

    fn generate_biomes(&self, pos: ChunkPos) -> BiomeChunk {
        BiomeChunk::generate(&self.biome_noise, &self.biomes, pos, self.chunk_height)
    }

    pub fn generate_chunk(&self, pos: ChunkPos) -> Chunk {
        let mut chunk = Chunk::new(pos.pos.x, pos.pos.y, "overworld".to_string());
        let stone = BlockData {
            name: "minecraft:stone".to_string(),
            properties: None,
        }
        .to_block_id();
        let air = BlockData::default().to_block_id();
        pos.iter_columns()
            .cartesian_product(self.chunk_height.iter())
            .map(|(c, y)| c.block(y))
            .for_each(|pos| {
                chunk.set_block(
                    pos.x,
                    pos.y,
                    pos.z,
                    if self.biome_noise.final_density(pos) > 0.0 {
                        stone.clone()
                    } else {
                        air.clone()
                    },
                );
            });

        chunk
    }
}
