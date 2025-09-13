use itertools::Itertools;

use crate::{
    biome::Biome,
    pos::{BlockPos, ChunkHeight, ChunkPos},
};

pub struct BiomeChunk {
    min_y: i32,
    biomes: Vec<Biome>,
}

impl BiomeChunk {
    pub(crate) fn generate(
        noise: &impl BiomeNoise,
        biomes: &[(NoisePoint, Biome)],
        pos: ChunkPos,
        chunk_height: ChunkHeight,
    ) -> Self {
        fn get_best(noise: [i64; 6], biomes: &[(NoisePoint, Biome)]) -> Biome {
            *biomes
                .iter()
                .map(|(noise_point, b)| (noise_point.fitness(noise), b))
                .min_by_key(|(fitness, _)| *fitness)
                .unwrap()
                .1
        }
        let biomes = (0..16)
            .step_by(4)
            .cartesian_product((0..16).step_by(4))
            .cartesian_product(chunk_height.iter().step_by(4))
            .map(|((x, z), y)| pos.block(x, y, z))
            .map(|pos| noise.at(pos))
            .map(|noise| get_best(noise, &biomes))
            .collect();
        let min_y = chunk_height.min_y.div_euclid(4);

        Self { biomes, min_y }
    }

    pub fn at(&self, pos: BlockPos) -> Biome {
        let pos = pos.div_euclid((4, 4, 4).into());
        let i = pos.x & 3 | (pos.z & 3) << 2 | (pos.y - self.min_y) << 4;
        self.biomes[i as usize]
    }
}

fn f32_to_i64(val: f32) -> i64 {
    (val * 10000.0) as i64
}
pub(crate) trait BiomeNoise {
    fn at_inner(&self, pos: BlockPos) -> [f64; 6];
    //TODO: internal at
    fn at(&self, pos: BlockPos) -> [i64; 6] {
        self.at_inner(pos).map(|a| a as f32).map(f32_to_i64)
    }
}

pub(crate) struct NoisePoint {
    data: [(i64, i64); 6],
}

impl NoisePoint {
    pub(crate) fn new(
        temperature: (f32, f32),
        humidity: (f32, f32),
        continentalness: (f32, f32),
        erosion: (f32, f32),
        depth: (f32, f32),
        peaks_and_valleys: (f32, f32),
        biome: Biome,
    ) -> (Self, Biome) {
        (
            Self {
                data: [
                    (f32_to_i64(temperature.0), f32_to_i64(temperature.1)),
                    (f32_to_i64(humidity.0), f32_to_i64(humidity.1)),
                    (f32_to_i64(continentalness.0), f32_to_i64(continentalness.1)),
                    (f32_to_i64(erosion.0), f32_to_i64(erosion.1)),
                    (f32_to_i64(depth.0), f32_to_i64(depth.1)),
                    (
                        f32_to_i64(peaks_and_valleys.0),
                        f32_to_i64(peaks_and_valleys.1),
                    ),
                ],
            },
            biome,
        )
    }

    fn fitness(&self, noise: [i64; 6]) -> u64 {
        fn fitness(val: (i64, i64), noise: i64) -> u64 {
            let l = noise - val.1;
            let l1 = val.0 - noise;
            (l.max(l1).max(0) as u64).pow(2)
        }
        self.data
            .iter()
            .zip(noise)
            .map(|(val, noise)| fitness(*val, noise))
            .fold(0, |a, b| a + b)
    }
}
