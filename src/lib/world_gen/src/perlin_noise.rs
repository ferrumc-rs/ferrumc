use std::ops::{Add, Mul, Sub};

use crate::random::Xoroshiro128PlusPlus;

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

    pub fn get_value(&self, point: Point<f64>) -> f64 {
        let mut res = 0.0;
        let mut freq_input_factor = self.lowest_freq_input_factor;
        let mut freq_value_factor = self.lowest_freq_value_factor;

        for (noise, amp) in self.noise_levels.iter().zip(self.amplitudes.iter()) {
            res += amp * noise.noise(point.scale(freq_input_factor).wrap()) * freq_value_factor;

            freq_input_factor *= 2.0;
            freq_value_factor /= 2.0;
        }

        res
    }
}

//TODO: this probably should be moved somewhere else
#[derive(Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Mul<Output = T> + Add<Output = T>> Mul for Point<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Point<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl Point<f64> {
    fn random(rng: &mut Xoroshiro128PlusPlus) -> Self {
        Self {
            x: rng.next_f64(),
            y: rng.next_f64(),
            z: rng.next_f64(),
        }
    }

    fn scale(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }

    fn smoothstep(self) -> Self {
        fn smoothstep(input: f64) -> f64 {
            input * input * input * (input * (input * 6.0 - 15.0) + 10.0)
        }
        Self {
            x: smoothstep(self.x),
            y: smoothstep(self.y),
            z: smoothstep(self.z),
        }
    }

    fn wrap(self) -> Self {
        Self {
            x: self.x - ((self.x / 33_554_432.0 + 0.5).floor() * 33_554_432.0),
            y: self.y - ((self.y / 33_554_432.0 + 0.5).floor() * 33_554_432.0),
            z: self.z - ((self.z / 33_554_432.0 + 0.5).floor() * 33_554_432.0),
        }
    }
}

/// reference: net.minecraft.world.level.levelgen.synth.ImprovedNoise
#[allow(dead_code)]
pub struct ImprovedNoise {
    p: [u8; 256],
    offset: Point<f64>,
}

#[allow(dead_code)]
impl ImprovedNoise {
    pub fn new(random: &mut Xoroshiro128PlusPlus) -> Self {
        let offset = Point::random(random).scale(256.0);

        let mut p = [0u8; 256];
        for (i, p) in p.iter_mut().enumerate() {
            *p = i as u8;
        }

        for i in 0..256 {
            p.swap(i, i + random.next_bounded(256 - i as u32) as usize);
        }

        Self { p, offset }
    }

    pub fn noise(&self, at: Point<f64>) -> f64 {
        let actual = at + self.offset;
        let grid = actual.floor();
        let delta = actual - grid;
        let grid = Point {
            x: grid.x as i32,
            y: grid.y as i32,
            z: grid.z as i32,
        };

        let i = self.p(grid.x);
        let i1 = self.p(grid.x + 1);
        let i2 = self.p(i + grid.y);
        let i3 = self.p(i + grid.y + 1);
        let i4 = self.p(i1 + grid.y);
        let i5 = self.p(i1 + grid.y + 1);

        let d = grad_dot(self.p(i2 + grid.z), delta);
        let d1 = grad_dot(self.p(i4 + grid.z), delta - Point::new(1.0, 0.0, 0.0));
        let d2 = grad_dot(self.p(i3 + grid.z), delta - Point::new(0.0, 1.0, 0.0));
        let d3 = grad_dot(self.p(i5 + grid.z), delta - Point::new(1.0, 1.0, 0.0));
        let d4 = grad_dot(self.p(i2 + grid.z + 1), delta - Point::new(0.0, 0.0, 1.0));
        let d5 = grad_dot(self.p(i4 + grid.z + 1), delta - Point::new(1.0, 0.0, 1.0));
        let d6 = grad_dot(self.p(i3 + grid.z + 1), delta - Point::new(0.0, 1.0, 1.0));
        let d7 = grad_dot(self.p(i5 + grid.z + 1), delta - Point::new(1.0, 1.0, 1.0));

        let smooth = delta.smoothstep();

        lerp3(smooth, d, d1, d2, d3, d4, d5, d6, d7)
    }

    fn p(&self, index: i32) -> i32 {
        self.p[(index & 0xFF) as usize].into()
    }
}

fn grad_dot(grad_index: i32, p: Point<f64>) -> f64 {
    const SIMPLEX_GRADIENT: [Point<f64>; 16] = [
        Point::new(1.0, 1.0, 0.0),
        Point::new(-1.0, 1.0, 0.0),
        Point::new(1.0, -1.0, 0.0),
        Point::new(-1.0, -1.0, 0.0),
        Point::new(1.0, 0.0, 1.0),
        Point::new(-1.0, 0.0, 1.0),
        Point::new(1.0, 0.0, -1.0),
        Point::new(-1.0, 0.0, -1.0),
        Point::new(0.0, 1.0, 1.0),
        Point::new(0.0, -1.0, 1.0),
        Point::new(0.0, 1.0, -1.0),
        Point::new(0.0, -1.0, -1.0),
        Point::new(1.0, 1.0, 0.0),
        Point::new(0.0, -1.0, 1.0),
        Point::new(-1.0, 1.0, 0.0),
        Point::new(0.0, -1.0, -1.0),
    ];
    p * SIMPLEX_GRADIENT[grad_index as usize & 15]
}

pub fn lerp(delta: f64, start: f64, end: f64) -> f64 {
    start + delta * (end - start)
}

pub fn lerp2(delta1: f64, delta2: f64, start1: f64, end1: f64, start2: f64, end2: f64) -> f64 {
    lerp(
        delta2,
        lerp(delta1, start1, end1),
        lerp(delta1, start2, end2),
    )
}

#[allow(clippy::too_many_arguments)]
pub fn lerp3(
    delta: Point<f64>,
    start1: f64,
    end1: f64,
    start2: f64,
    end2: f64,
    start3: f64,
    end3: f64,
    start4: f64,
    end4: f64,
) -> f64 {
    lerp(
        delta.z,
        lerp2(delta.x, delta.y, start1, end1, start2, end2),
        lerp2(delta.x, delta.y, start3, end3, start4, end4),
    )
}

#[test]
fn test_improved_noise() {
    let mut rng = Xoroshiro128PlusPlus::new(0, 0);
    let noise = ImprovedNoise::new(&mut rng);

    assert_eq!(
        noise.noise(Point::new(0.0, 0.0, 0.0)),
        -0.045044799854318,
        "Mismatch in noise at zero"
    );
    assert_eq!(
        noise.noise(Point::new(10000.123, 203.5, -20031.78)),
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
        perlin_noise.get_value(Point::new(0.0, 0.0, 0.0)),
        0.11030635847227427,
        "Mismatch in get_value at zero"
    );
    assert_eq!(
        perlin_noise.get_value(Point::new(10000.123, 203.5, -20031.78)),
        -0.005210683092268373,
        "Mismatch in get_value"
    );
}
