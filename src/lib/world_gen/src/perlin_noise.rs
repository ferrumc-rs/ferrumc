use crate::random::Xoroshiro128PlusPlus;

///reference: net.minecraft.world.level.levelgen.synth.PerlinNoise
pub struct PerlinNoise {
    first_octave: i32,
    amplitudes: Vec<f64>,
    noise_levels: Vec<ImprovedNoise>,
    lowest_freq_input_factor: f64,
    lowest_freq_value_factor: f64,
    max: f64,
}

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

    pub fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        fn wrap(value: f64) -> f64 {
            value - ((value / 33_554_432.0 + 0.5).floor() * 33_554_432.0)
        }

        let mut res = 0.0;
        let mut freq_input_factor = self.lowest_freq_input_factor;
        let mut freq_value_factor = self.lowest_freq_value_factor;

        for (noise, amp) in self.noise_levels.iter().zip(self.amplitudes.iter()) {
            res +=
                amp * noise.noise(
                    wrap(x * freq_input_factor),
                    wrap(y * freq_input_factor),
                    wrap(z * freq_input_factor),
                ) * freq_value_factor;

            freq_input_factor *= 2.0;
            freq_value_factor /= 2.0;
        }

        res
    }
}

/// reference: net.minecraft.world.level.levelgen.synth.ImprovedNoise
pub struct ImprovedNoise {
    p: [u8; 256],
    xo: f64,
    yo: f64,
    zo: f64,
}

impl ImprovedNoise {
    pub fn new(random: &mut Xoroshiro128PlusPlus) -> Self {
        let xo = random.next_f64() * 256.0;
        let yo = random.next_f64() * 256.0;
        let zo = random.next_f64() * 256.0;

        let mut p = [0u8; 256];
        for (i, p) in p.iter_mut().enumerate() {
            *p = i as u8;
        }

        for i in 0..256 {
            p.swap(i, i + random.next_bounded(256 - i as u32) as usize);
        }

        Self { p, xo, yo, zo }
    }

    pub fn noise(&self, x: f64, y: f64, z: f64) -> f64 {
        let dx = x + self.xo;
        let dy = y + self.yo;
        let dz = z + self.zo;

        let floor_x = dx.floor() as i32;
        let floor_y = dy.floor() as i32;
        let floor_z = dz.floor() as i32;

        let delta_x = dx - f64::from(floor_x);
        let delta_y = dy - f64::from(floor_y);
        let delta_z = dz - f64::from(floor_z);

        self.sample_and_lerp(
            floor_x, floor_y, floor_z, delta_x, delta_y, delta_z, delta_y,
        )
    }

    fn p(&self, index: i32) -> i32 {
        self.p[(index & 0xFF) as usize].into()
    }

    fn sample_and_lerp(
        &self,
        grid_x: i32,
        grid_y: i32,
        grid_z: i32,
        delta_x: f64,
        weird_delta_y: f64, //TODO: rename and remove delta_y
        delta_z: f64,
        delta_y: f64,
    ) -> f64 {
        let i = self.p(grid_x);
        let i1 = self.p(grid_x + 1);
        let i2 = self.p(i + grid_y);
        let i3 = self.p(i + grid_y + 1);
        let i4 = self.p(i1 + grid_y);
        let i5 = self.p(i1 + grid_y + 1);

        let d = grad_dot(self.p(i2 + grid_z), delta_x, weird_delta_y, delta_z);
        let d1 = grad_dot(self.p(i4 + grid_z), delta_x - 1.0, weird_delta_y, delta_z);
        let d2 = grad_dot(self.p(i3 + grid_z), delta_x, weird_delta_y - 1.0, delta_z);
        let d3 = grad_dot(
            self.p(i5 + grid_z),
            delta_x - 1.0,
            weird_delta_y - 1.0,
            delta_z,
        );
        let d4 = grad_dot(
            self.p(i2 + grid_z + 1),
            delta_x,
            weird_delta_y,
            delta_z - 1.0,
        );
        let d5 = grad_dot(
            self.p(i4 + grid_z + 1),
            delta_x - 1.0,
            weird_delta_y,
            delta_z - 1.0,
        );
        let d6 = grad_dot(
            self.p(i3 + grid_z + 1),
            delta_x,
            weird_delta_y - 1.0,
            delta_z - 1.0,
        );
        let d7 = grad_dot(
            self.p(i5 + grid_z + 1),
            delta_x - 1.0,
            weird_delta_y - 1.0,
            delta_z - 1.0,
        );

        let sx = smoothstep(delta_x);
        let sy = smoothstep(delta_y);
        let sz = smoothstep(delta_z);

        lerp3(sx, sy, sz, d, d1, d2, d3, d4, d5, d6, d7)
    }
}

fn grad_dot(grad_index: i32, x: f64, y: f64, z: f64) -> f64 {
    const SIMPLEX_GRADIENT: [[f64; 3]; 16] = [
        [1.0, 1.0, 0.0],
        [-1.0, 1.0, 0.0],
        [1.0, -1.0, 0.0],
        [-1.0, -1.0, 0.0],
        [1.0, 0.0, 1.0],
        [-1.0, 0.0, 1.0],
        [1.0, 0.0, -1.0],
        [-1.0, 0.0, -1.0],
        [0.0, 1.0, 1.0],
        [0.0, -1.0, 1.0],
        [0.0, 1.0, -1.0],
        [0.0, -1.0, -1.0],
        [1.0, 1.0, 0.0],
        [0.0, -1.0, 1.0],
        [-1.0, 1.0, 0.0],
        [0.0, -1.0, -1.0],
    ];
    let grad = SIMPLEX_GRADIENT[grad_index as usize & 15];
    grad[0] * x + grad[1] * y + grad[2] * z
}

fn smoothstep(input: f64) -> f64 {
    input * input * input * (input * (input * 6.0 - 15.0) + 10.0)
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

pub fn lerp3(
    delta1: f64,
    delta2: f64,
    delta3: f64,
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
        delta3,
        lerp2(delta1, delta2, start1, end1, start2, end2),
        lerp2(delta1, delta2, start3, end3, start4, end4),
    )
}

#[test]
fn test_sample_and_lerp() {
    let mut rng = Xoroshiro128PlusPlus::new(0, 0);
    let noise = ImprovedNoise::new(&mut rng);

    assert_eq!(
        noise.sample_and_lerp(0, 0, 0, 0.0, 0.0, 0.0, 0.0),
        0.0,
        "Mismatch in sample_and_lerp with zeros"
    );
    assert_eq!(
        noise.sample_and_lerp(123, 456, 789, 0.123, 0.456, 0.789, 0.456),
        -0.6187674359192081,
        "Mismatch in noise at zero"
    );
}

#[test]
fn test_improved_noise() {
    let mut rng = Xoroshiro128PlusPlus::new(0, 0);
    let noise = ImprovedNoise::new(&mut rng);

    assert_eq!(
        noise.noise(0.0, 0.0, 0.0),
        -0.045044799854318,
        "Mismatch in noise at zero"
    );
    assert_eq!(
        noise.noise(10000.123, 203.5, -20031.78),
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
        perlin_noise.get_value(0.0, 0.0, 0.0),
        0.11030635847227427,
        "Mismatch in get_value at zero"
    );
    assert_eq!(
        perlin_noise.get_value(10000.123, 203.5, -20031.78),
        -0.005210683092268373,
        "Mismatch in get_value"
    );
}
