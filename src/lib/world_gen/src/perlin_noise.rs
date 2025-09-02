use bevy_math::{DVec3, FloatExt};

use crate::random::Xoroshiro128PlusPlus;

///reference: net.minecraft.world.level.levelgen.synth.NormalNoise
#[allow(dead_code)]
pub struct NormalNoise {
    first: PerlinNoise,
    second: PerlinNoise,
    factor: f64,
}

#[allow(dead_code)]
impl NormalNoise {
    pub fn new(mut random: Xoroshiro128PlusPlus, first_octave: i32, amplitudes: Vec<f64>) -> Self {
        fn expected_deviation(octaves: usize) -> f64 {
            0.16666666666666666 / (0.1 * (1.0 + 1.0 / (octaves as f64 + 1.0)))
        }
        Self {
            factor: expected_deviation(
                amplitudes
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, x)| **x != 0.0)
                    .unwrap()
                    .0
                    - amplitudes
                        .iter()
                        .enumerate()
                        .find(|(_, x)| **x != 0.0)
                        .unwrap()
                        .0,
            ),
            first: PerlinNoise::new(&mut random, first_octave, amplitudes.clone()),
            second: PerlinNoise::new(&mut random, first_octave, amplitudes),
        }
    }

    pub fn get_value(&self, pos: DVec3) -> f64 {
        (self.first.get_value(pos) + self.second.get_value(pos * 1.0181268882175227)) * self.factor
    }
}

#[allow(dead_code)]
///reference: net.minecraft.world.level.levelgen.synth.PerlinNoise
pub struct PerlinNoise {
    first_octave: i32,
    amplitudes: Vec<f64>,
    noise_levels: Vec<ImprovedNoise>,
    lowest_freq_input_factor: f64,
    lowest_freq_value_factor: f64,
    max: f64,
}

#[allow(dead_code)]
impl PerlinNoise {
    pub fn new(random: &mut Xoroshiro128PlusPlus, first_octave: i32, amplitudes: Vec<f64>) -> Self {
        let factory = random.fork_positional();

        let noise_levels: Vec<ImprovedNoise> = (0..amplitudes.len())
            .map(|i| {
                // optional: skip if amp == 0
                ImprovedNoise::new(
                    &mut factory.with_hash(&format!("octave_{}", first_octave + i as i32)),
                )
            })
            .collect();

        let lowest_freq_input_factor = 2.0f64.powi(first_octave);
        let lowest_freq_value_factor = 2.0f64.powi((amplitudes.len() - 1) as i32)
            / (2.0f64.powi(amplitudes.len() as i32) - 1.0);

        let mut max = 0.0;
        let mut d1 = lowest_freq_value_factor;

        for &amp in amplitudes.iter() {
            max += amp * 2.0 * d1;
            d1 /= 2.0;
        }
        PerlinNoise {
            first_octave,
            amplitudes,
            noise_levels,
            lowest_freq_input_factor,
            lowest_freq_value_factor,
            max,
        }
    }

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
#[allow(dead_code)]
pub struct ImprovedNoise {
    p: [u8; 256],
    offset: DVec3,
}

#[allow(dead_code)]
impl ImprovedNoise {
    pub fn new(random: &mut Xoroshiro128PlusPlus) -> Self {
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
    let rng = Xoroshiro128PlusPlus::new(0, 0);
    let noise = NormalNoise::new(rng, 5, vec![0.0, 2.0, 1.5, 0.1, -1.0, 0.0, 0.0]);

    assert_eq!(noise.factor, 1.3333333333333333, "Mismatch in noise factor");
    assert_eq!(
        noise.get_value(DVec3::new(0.0, 0.0, 0.0)),
        0.3070105188501303,
        "Mismatch in noise at zero"
    );
    assert_eq!(
        noise.get_value(DVec3::new(1000.0, -10.0, -232.0)),
        -0.3120632840894116,
        "Mismatch in noise"
    );
    assert_eq!(
        noise.get_value(DVec3::new(10000.123, 203.5, -20031.78)),
        -0.14363335564194124,
        "Mismatch in noise"
    );
}

#[test]
fn test_improved_noise() {
    let mut rng = Xoroshiro128PlusPlus::new(0, 0);
    let noise = ImprovedNoise::new(&mut rng);

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
    let mut rng = Xoroshiro128PlusPlus::new(0, 0);

    let perlin_noise = PerlinNoise::new(&mut rng, 3, vec![0.0, 1.0, -1.0, 0.0, 0.5, 0.0]);

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
