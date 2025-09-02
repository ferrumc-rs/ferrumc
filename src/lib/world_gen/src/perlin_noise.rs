use crate::random::{Random, RandomFactory, Rng, RngFactory};
use std::mem::MaybeUninit;

use bevy_math::{DVec3, FloatExt};

//reference net.minecraft.world.level.levelgen.Noises
//only shift and swamp have a different name in the resourcelocation, but we could rename them and
//do some macro magic (maybe)
pub const RIDGE: ConstNormalNoise<6> =
    ConstNormalNoise::new("minecraft:ridge", 7, [1.0, 2.0, 1.0, 0.0, 0.0, 0.0]);
pub const SHIFT: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:offset", 3, [1.0, 1.0, 1.0, 0.0]);
pub const TEMPERATURE: ConstNormalNoise<6> =
    ConstNormalNoise::new("minecraft:temperature", 10, [1.5, 0.0, 1.0, 0.0, 0.0, 0.0]);
pub const TEMPERATURE_LARGE: ConstNormalNoise<6> = ConstNormalNoise::new(
    "minecraft:temperature_large",
    8,
    [1.5, 0.0, 1.0, 0.0, 0.0, 0.0],
);
pub const VEGETATION: ConstNormalNoise<6> =
    ConstNormalNoise::new("minecraft:vegetation", 8, [1.0, 1.0, 0.0, 0.0, 0.0, 0.0]);
pub const VEGETATION_LARGE: ConstNormalNoise<6> = ConstNormalNoise::new(
    "minecraft:vegetation_large",
    6,
    [1.0, 1.0, 0.0, 0.0, 0.0, 0.0],
);
pub const CONTINENTALNESS: ConstNormalNoise<9> = ConstNormalNoise::new(
    "minecraft:continentalness",
    9,
    [1.0, 1.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0],
);
pub const CONTINENTALNESS_LARGE: ConstNormalNoise<9> = ConstNormalNoise::new(
    "minecraft:continentalness_large",
    7,
    [1.0, 1.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0],
);
pub const EROSION: ConstNormalNoise<5> =
    ConstNormalNoise::new("minecraft:erosion", 9, [1.0, 1.0, 0.0, 1.0, 1.0]);
pub const EROSION_LARGE: ConstNormalNoise<5> =
    ConstNormalNoise::new("minecraft:erosion_large", 7, [1.0, 1.0, 0.0, 1.0, 1.0]);
pub const AQUIFER_BARRIER: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:aquifer_barrier", 3, [1.0]);
pub const AQUIFER_FLUID_LEVEL_FLOODEDNESS: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:aquifer_fluid_level_floodedness", 7, [1.0]);
pub const AQUIFER_LAVA: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:aquifer_lava", 1, [1.0]);
pub const AQUIFER_FLUID_LEVEL_SPREAD: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:aquifer_fluid_level_spread", 5, [1.0]);
pub const PILLAR: ConstNormalNoise<2> = ConstNormalNoise::new("minecraft:pillar", 7, [1.0, 1.0]);
pub const PILLAR_RARENESS: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:pillar_rareness", 8, [1.0]);
pub const PILLAR_THICKNESS: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:pillar_thickness", 8, [1.0]);
pub const SPAGHETTI_2D: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_2d", 7, [1.0]);
pub const SPAGHETTI_2D_ELEVATION: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_2d_elevation", 8, [1.0]);
pub const SPAGHETTI_2D_MODULATOR: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_2d_modulator", 11, [1.0]);
pub const SPAGHETTI_2D_THICKNESS: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_2d_thickness", 11, [1.0]);
pub const SPAGHETTI_3D_1: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_3d_1", 7, [1.0]);
pub const SPAGHETTI_3D_2: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_3d_2", 7, [1.0]);
pub const SPAGHETTI_3D_RARITY: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_3d_rarity", 11, [1.0]);
pub const SPAGHETTI_3D_THICKNESS: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_3d_thickness", 8, [1.0]);
pub const SPAGHETTI_ROUGHNESS: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_roughness", 5, [1.0]);
pub const SPAGHETTI_ROUGHNESS_MODULATOR: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:spaghetti_roughness_modulator", 8, [1.0]);
pub const CAVE_ENTRANCE: ConstNormalNoise<3> =
    ConstNormalNoise::new("minecraft:cave_entrance", 7, [0.4, 0.5, 1.0]);
pub const CAVE_LAYER: ConstNormalNoise<1> = ConstNormalNoise::new("minecraft:cave_layer", 8, [1.0]);
pub const CAVE_CHEESE: ConstNormalNoise<9> = ConstNormalNoise::new(
    "minecraft:cave_cheese",
    8,
    [0.5, 1.0, 2.0, 1.0, 2.0, 1.0, 0.0, 2.0, 0.0],
);
pub const ORE_VEININESS: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:ore_veininess", 8, [1.0]);
pub const ORE_VEIN_A: ConstNormalNoise<1> = ConstNormalNoise::new("minecraft:ore_vein_a", 7, [1.0]);
pub const ORE_VEIN_B: ConstNormalNoise<1> = ConstNormalNoise::new("minecraft:ore_vein_b", 7, [1.0]);
pub const ORE_GAP: ConstNormalNoise<1> = ConstNormalNoise::new("minecraft:ore_gap", 5, [1.0]);
pub const NOODLE: ConstNormalNoise<1> = ConstNormalNoise::new("minecraft:noodle", 8, [1.0]);
pub const NOODLE_THICKNESS: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:noodle_thickness", 8, [1.0]);
pub const NOODLE_RIDGE_A: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:noodle_ridge_a", 7, [1.0]);
pub const NOODLE_RIDGE_B: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:noodle_ridge_b", 7, [1.0]);
pub const JAGGED: ConstNormalNoise<16> = ConstNormalNoise::new(
    "minecraft:jagged",
    16,
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    ],
);
pub const SURFACE: ConstNormalNoise<3> =
    ConstNormalNoise::new("minecraft:surface", 6, [1.0, 1.0, 1.0]);
pub const SURFACE_SECONDARY: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:surface_secondary", 6, [1.0, 1.0, 0.0, 1.0]);
pub const CLAY_BANDS_OFFSET: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:clay_bands_offset", 8, [1.0]);
pub const BADLANDS_PILLAR: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:badlands_pillar", 2, [1.0, 1.0, 1.0, 1.0]);
pub const BADLANDS_PILLAR_ROOF: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:badlands_pillar_roof", 8, [1.0]);
pub const BADLANDS_SURFACE: ConstNormalNoise<3> =
    ConstNormalNoise::new("minecraft:badlands_surface", 6, [1.0, 1.0, 1.0]);
pub const ICEBERG_PILLAR: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:iceberg_pillar", 6, [1.0, 1.0, 1.0, 1.0]);
pub const ICEBERG_PILLAR_ROOF: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:iceberg_pillar_roof", 3, [1.0]);
pub const ICEBERG_SURFACE: ConstNormalNoise<3> =
    ConstNormalNoise::new("minecraft:iceberg_surface", 6, [1.0, 1.0, 1.0]);
pub const SWAMP: ConstNormalNoise<1> = ConstNormalNoise::new("minecraft:surface_swamp", 2, [1.0]);
pub const CALCITE: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:calcite", 9, [1.0, 1.0, 1.0, 1.0]);
pub const GRAVEL: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:gravel", 8, [1.0, 1.0, 1.0, 1.0]);
pub const POWDER_SNOW: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:powder_snow", 6, [1.0, 1.0, 1.0, 1.0]);
pub const PACKED_ICE: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:packed_ice", 7, [1.0, 1.0, 1.0, 1.0]);
pub const ICE: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:ice", 4, [1.0, 1.0, 1.0, 1.0]);
pub const SOUL_SAND_LAYER: ConstNormalNoise<9> = ConstNormalNoise::new(
    "minecraft:soul_sand_layer",
    8,
    [1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.013333333333333334],
);
pub const GRAVEL_LAYER: ConstNormalNoise<9> = ConstNormalNoise::new(
    "minecraft:gravel_layer",
    8,
    [1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.013333333333333334],
);
pub const PATCH: ConstNormalNoise<6> = ConstNormalNoise::new(
    "minecraft:patch",
    5,
    [1.0, 0.0, 0.0, 0.0, 0.0, 0.013333333333333334],
);
pub const NETHERRACK: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:netherrack", 3, [1.0, 0.0, 0.0, 0.35]);
pub const NETHER_WART: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:nether_wart", 3, [1.0, 0.0, 0.0, 0.9]);
pub const NETHER_STATE_SELECTOR: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:nether_state_selector", 4, [1.0]);

pub struct ConstNormalNoise<const N: usize> {
    first: ConstPerlinNoise<N>,
    second: ConstPerlinNoise<N>,
    factor: f64,
    name: &'static str,
}

impl<const N: usize> ConstNormalNoise<N> {
    pub const fn new(name: &'static str, first_octave: u32, amplitudes: [f64; N]) -> Self {
        const fn expected_deviation(octaves: usize) -> f64 {
            0.16666666666666666 / (0.1 * (1.0 + 1.0 / (octaves as f64 + 1.0)))
        }
        let mut first_idx = 0;
        let mut i = 0;
        while i < N {
            if amplitudes[i] != 0.0 {
                first_idx = i;
                break;
            }
            i += 1;
        }

        let mut last_idx = 0;
        let mut i = N;
        while i > 0 {
            i -= 1;
            if amplitudes[i] != 0.0 {
                last_idx = i;
                break;
            }
        }
        Self {
            name,
            factor: expected_deviation(last_idx - first_idx),
            first: ConstPerlinNoise::new(first_octave, amplitudes),
            second: ConstPerlinNoise::new(first_octave, amplitudes),
        }
    }

    pub fn init(&self, random: RandomFactory) -> NormalNoise<N> {
        let mut rng = random.with_hash(self.name);
        NormalNoise {
            first: self.first.init(rng.fork_positional()),
            second: self.second.init(rng.fork_positional()),
            factor: self.factor,
        }
    }
}

pub struct ConstPerlinNoise<const N: usize> {
    first_octave: u32,
    amplitudes: [f64; N],
    lowest_freq_input_factor: f64,
    lowest_freq_value_factor: f64,
    max: f64,
}

impl<const N: usize> ConstPerlinNoise<N> {
    pub const fn new(first_octave: u32, amplitudes: [f64; N]) -> Self {
        assert!(!amplitudes.is_empty());
        let lowest_freq_input_factor = 2u32.pow(first_octave) as f64;
        let lowest_freq_value_factor = 2u32.pow((amplitudes.len() - 1) as u32) as f64
            / (2u32.pow(amplitudes.len() as u32) as f64 - 1.0);

        let mut max = 0.0;
        let mut d1 = lowest_freq_value_factor;
        let mut i = 0;
        while i < N {
            max += amplitudes[i] * 2.0 * d1;
            d1 /= 2.0;
            i += 1;
        }
        Self {
            first_octave,
            amplitudes,
            lowest_freq_input_factor,
            lowest_freq_value_factor,
            max,
        }
    }

    pub fn init(&self, factory: RandomFactory) -> PerlinNoise<N> {
        let mut noise_levels: [MaybeUninit<ImprovedNoise>; N] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for (i, noise_level) in noise_levels.iter_mut().enumerate() {
            noise_level.write(ImprovedNoise::new(
                factory.with_hash(&format!("octave_{}", self.first_octave + i as u32)),
            ));
        }
        PerlinNoise {
            noise_levels: unsafe { MaybeUninit::array_assume_init(noise_levels) },
            amplitudes: self.amplitudes,
            lowest_freq_input_factor: self.lowest_freq_input_factor,
            lowest_freq_value_factor: self.lowest_freq_value_factor,
            max: self.max,
        }
    }
}

///reference: net.minecraft.world.level.levelgen.synth.NormalNoise
pub struct NormalNoise<const N: usize> {
    first: PerlinNoise<N>,
    second: PerlinNoise<N>,
    factor: f64,
}

impl<const N: usize> NormalNoise<N> {
    pub fn get_value(&self, pos: DVec3) -> f64 {
        (self.first.get_value(pos) + self.second.get_value(pos * 1.0181268882175227)) * self.factor
    }
}

///reference: net.minecraft.world.level.levelgen.synth.PerlinNoise
pub struct PerlinNoise<const N: usize> {
    amplitudes: [f64; N],
    noise_levels: [ImprovedNoise; N],
    lowest_freq_input_factor: f64,
    lowest_freq_value_factor: f64,
    max: f64,
}

impl<const N: usize> PerlinNoise<N> {
    pub fn get_value(&self, point: DVec3) -> f64 {
        let mut res = 0.0;
        let mut freq_input_factor = self.lowest_freq_input_factor;
        let mut freq_value_factor = self.lowest_freq_value_factor;

        for (noise, amp) in self.noise_levels.iter().zip(self.amplitudes.iter()) {
            res += amp * noise.noise((point * freq_input_factor).map(wrap)) * freq_value_factor;

            freq_input_factor *= 2.0;
            freq_value_factor /= 2.0;
        }

        res
    }
}

fn smoothstep(input: f64) -> f64 {
    input * input * input * (input * (input * 6.0 - 15.0) + 10.0)
}

fn wrap(input: f64) -> f64 {
    input - ((input / 2f64.powi(25) + 0.5).floor() * 2f64.powi(25))
}

/// reference: net.minecraft.world.level.levelgen.synth.ImprovedNoise
pub struct ImprovedNoise {
    p: [u8; 256],
    offset: DVec3,
}

impl ImprovedNoise {
    pub fn new(mut random: Random) -> Self {
        let offset = DVec3::new(random.next_f64(), random.next_f64(), random.next_f64()) * 256.0;

        let mut p = [0u8; 256];
        for (i, p) in p.iter_mut().enumerate() {
            *p = i as u8;
        }

        for i in 0..256 {
            p.swap(i, i + random.next_bounded(256 - i as u32) as usize);
        }

        Self { p, offset }
    }

    pub fn noise(&self, at: DVec3) -> f64 {
        let actual = at + self.offset;
        let grid = actual.floor();
        let delta = actual - grid;

        let grid = grid.as_ivec3();
        let i = self.p(grid.x);
        let i1 = self.p(grid.x + 1);
        let i2 = self.p(i + grid.y);
        let i3 = self.p(i + grid.y + 1);
        let i4 = self.p(i1 + grid.y);
        let i5 = self.p(i1 + grid.y + 1);

        let d = grad_dot(self.p(i2 + grid.z), delta);
        let d1 = grad_dot(self.p(i4 + grid.z), delta - DVec3::new(1.0, 0.0, 0.0));
        let d2 = grad_dot(self.p(i3 + grid.z), delta - DVec3::new(0.0, 1.0, 0.0));
        let d3 = grad_dot(self.p(i5 + grid.z), delta - DVec3::new(1.0, 1.0, 0.0));
        let d4 = grad_dot(self.p(i2 + grid.z + 1), delta - DVec3::new(0.0, 0.0, 1.0));
        let d5 = grad_dot(self.p(i4 + grid.z + 1), delta - DVec3::new(1.0, 0.0, 1.0));
        let d6 = grad_dot(self.p(i3 + grid.z + 1), delta - DVec3::new(0.0, 1.0, 1.0));
        let d7 = grad_dot(self.p(i5 + grid.z + 1), delta - DVec3::new(1.0, 1.0, 1.0));

        let smooth = delta.map(smoothstep);

        lerp3(smooth, d, d1, d2, d3, d4, d5, d6, d7)
    }

    fn p(&self, index: i32) -> i32 {
        self.p[(index & 0xFF) as usize].into()
    }
}

fn grad_dot(grad_index: i32, p: DVec3) -> f64 {
    const SIMPLEX_GRADIENT: [DVec3; 16] = [
        DVec3::new(1.0, 1.0, 0.0),
        DVec3::new(-1.0, 1.0, 0.0),
        DVec3::new(1.0, -1.0, 0.0),
        DVec3::new(-1.0, -1.0, 0.0),
        DVec3::new(1.0, 0.0, 1.0),
        DVec3::new(-1.0, 0.0, 1.0),
        DVec3::new(1.0, 0.0, -1.0),
        DVec3::new(-1.0, 0.0, -1.0),
        DVec3::new(0.0, 1.0, 1.0),
        DVec3::new(0.0, -1.0, 1.0),
        DVec3::new(0.0, 1.0, -1.0),
        DVec3::new(0.0, -1.0, -1.0),
        DVec3::new(1.0, 1.0, 0.0),
        DVec3::new(0.0, -1.0, 1.0),
        DVec3::new(-1.0, 1.0, 0.0),
        DVec3::new(0.0, -1.0, -1.0),
    ];
    p.dot(SIMPLEX_GRADIENT[grad_index as usize & 15])
}

pub fn lerp2(delta1: f64, delta2: f64, start1: f64, end1: f64, start2: f64, end2: f64) -> f64 {
    start1
        .lerp(end1, delta1)
        .lerp(start2.lerp(end2, delta1), delta2)
}

#[allow(clippy::too_many_arguments)]
pub fn lerp3(
    delta: DVec3,
    start1: f64,
    end1: f64,
    start2: f64,
    end2: f64,
    start3: f64,
    end3: f64,
    start4: f64,
    end4: f64,
) -> f64 {
    lerp2(delta.x, delta.y, start1, end1, start2, end2)
        .lerp(lerp2(delta.x, delta.y, start3, end3, start4, end4), delta.z)
}

#[test]
fn test_normal_noise() {
    let rng = Random::Xoroshiro128PlusPlus(crate::random::Xoroshiro128PlusPlus::new(0, 0))
        .fork_positional();
    let noise = ConstNormalNoise::new("test", 5, [0.0, 2.0, 1.5, 0.1, -1.0, 0.0, 0.0]).init(rng);

    assert_eq!(noise.factor, 1.3333333333333333, "Mismatch in noise factor");
    assert_eq!(
        noise.get_value(DVec3::new(0.0, 0.0, 0.0)),
        -0.12641617678290623,
        "Mismatch in noise at zero"
    );
    assert_eq!(
        noise.get_value(DVec3::new(1000.0, -10.0, -232.0)),
        0.09024440133389773,
        "Mismatch in noise"
    );
    assert_eq!(
        noise.get_value(DVec3::new(10000.123, 203.5, -20031.78)),
        -0.29161635943682196,
        "Mismatch in noise"
    );
}

#[test]
fn test_improved_noise() {
    let rng = Random::Xoroshiro128PlusPlus(crate::random::Xoroshiro128PlusPlus::new(0, 0));
    let noise = ImprovedNoise::new(rng);

    assert_eq!(
        noise.noise(DVec3::new(0.0, 0.0, 0.0)),
        -0.045044799854318,
        "Mismatch in noise at zero"
    );
    assert_eq!(
        noise.noise(DVec3::new(10000.123, 203.5, -20031.78)),
        -0.18708168179464396,
        "Mismatch in noise"
    );
}

#[test]
fn test_perlin_noise() {
    let rng = Random::Xoroshiro128PlusPlus(crate::random::Xoroshiro128PlusPlus::new(0, 0))
        .fork_positional();

    let perlin_noise = ConstPerlinNoise::new(3, [0.0, 1.0, -1.0, 0.0, 0.5, 0.0]).init(rng);

    assert_eq!(
        perlin_noise.lowest_freq_input_factor, 8.0,
        "Mismatch in lowest_freq_input_factor"
    );
    assert_eq!(
        perlin_noise.lowest_freq_value_factor, 0.5079365079365079,
        "Mismatch in lowest_freq_value_factor"
    );
    assert_eq!(perlin_noise.max, 0.2857142857142857, "Mismatch in max");
    assert_eq!(
        perlin_noise.get_value(DVec3::new(0.0, 0.0, 0.0)),
        0.11030635847227427,
        "Mismatch in get_value at zero"
    );
    assert_eq!(
        perlin_noise.get_value(DVec3::new(10000.123, 203.5, -20031.78)),
        -0.005210683092268373,
        "Mismatch in get_value"
    );
}
