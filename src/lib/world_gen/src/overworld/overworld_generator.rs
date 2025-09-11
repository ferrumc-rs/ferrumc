use crate::biome::Biome;
use crate::biome_chunk::{BiomeChunk, BiomeNoise, NoisePoint};
use crate::common::surface::Surface;
use crate::overworld::carver::{CanyonCarver, Caver};
use crate::pos::{ChunkHeight, ChunkPos};
use ferrumc_world::chunk_format::Chunk;

pub struct WorldGenerator {
    seed: u64,
    chunk_height: ChunkHeight,
    biome_noise: BiomeNoise,
    biomes: Vec<(NoisePoint, Biome)>,
    surface: Surface,
    cave_carver: Caver,
    extra_cave_carver: Caver,
    canyon_carver: CanyonCarver,
}

impl WorldGenerator {
    pub fn new(seed: u64) -> Self {
        todo!()
        // Self { _seed: seed }
    }

    fn generate_biomes(&self, pos: ChunkPos) -> BiomeChunk {
        BiomeChunk::generate(&self.biome_noise, &self.biomes, pos, self.chunk_height)
    }

    pub fn generate_chunk(&self, pos: ChunkPos) -> Chunk {
        todo!()
    }
}
