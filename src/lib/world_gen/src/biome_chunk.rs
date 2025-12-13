use bevy_math::{DVec3, IVec3};
use itertools::Itertools;

use crate::biome::Biome;
use ferrumc_world::pos::{BlockPos, ChunkHeight, ChunkPos};

pub struct BiomeChunk {
    min_y: i16,
    biomes: Vec<Biome>,
    seed: u64,
}

impl BiomeChunk {
    pub fn generate(
        biome_noise: impl Fn(BlockPos) -> [f64; 6],
        seed: u64,
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
            .map(|((x, z), y)| pos.chunk_block((x as u8, y as i16, z as u8).into()))
            .map(|pos| biome_noise(pos).map(|a| a as f32).map(f32_to_i64))
            .map(|noise| get_best(noise, biomes))
            .collect();
        let min_y = chunk_height.min_y;

        Self {
            biomes,
            min_y,
            seed,
        }
    }

    fn intern_at(&self, pos: IVec3) -> Biome {
        let pos = pos.abs();
        let i = pos.x & 3 | (pos.z & 3) << 2 | (pos.y - self.min_y as i32) << 4;
        self.biomes[i as usize]
    }
    pub fn at(&self, pos: BlockPos) -> Biome {
        fn next(left: i64, right: i64) -> i64 {
            const MULTIPLIER: i64 = 6364136223846793005;
            const INCREMENT: i64 = 1442695040888963407;
            left.wrapping_mul(left.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT))
                .wrapping_add(right)
        }

        fn get_fiddle(seed: i64) -> f64 {
            let d = (((seed >> 24) % 1024 + 1024) % 1024) as f64 / 1024.0;
            (d - 0.5) * 0.9
        }

        fn get_fiddled_distance(seed: i64, pos: IVec3, noise: DVec3) -> f64 {
            let mut l = next(seed, pos.x.into());
            l = next(l, pos.y.into());
            l = next(l, pos.z.into());
            l = next(l, pos.x.into());
            l = next(l, pos.y.into());
            l = next(l, pos.z.into());

            let x_fiddle = get_fiddle(l);

            l = next(l, seed);
            let y_fiddle = get_fiddle(l);

            l = next(l, seed);
            let z_fiddle = get_fiddle(l);

            (noise + DVec3::new(x_fiddle, y_fiddle, z_fiddle)).length_squared()
        }

        let i = pos.pos - 2;

        let pos = i >> 2;

        let delta = (i & 3).as_dvec3() / 4.0;

        let mut offset_pos = IVec3::splat(0);
        let mut dist = f64::INFINITY;

        for i7 in 0..8 {
            let curr_offset = IVec3::new((i7 & 4) >> 2, (i7 & 2) >> 1, i7 & 1);
            let curr_offset_pos = pos + curr_offset;

            let curr_noise = delta - curr_offset.as_dvec3();

            let curr_dist = get_fiddled_distance(self.seed as i64, curr_offset_pos, curr_noise);

            if dist > curr_dist {
                offset_pos = curr_offset_pos;
                dist = curr_dist;
            }
        }

        self.intern_at(offset_pos)
    }
}

fn f32_to_i64(val: f32) -> i64 {
    (val * 10000.0) as i64
}
pub(crate) trait BiomeNoise {
    fn at_inner(&self, pos: BlockPos) -> [f64; 6];
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
            .sum()
    }
}
