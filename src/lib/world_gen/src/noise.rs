use simdnoise::Settings;
use splines::Spline;

pub struct NoiseGenerator {
    seed: u64,
    frequency: f32,
    _octaves: u8,
    spline: Option<Spline<f32, f32>>,
}

impl NoiseGenerator {
    pub fn new(seed: u64, frequency: f32, octaves: u8, spline: Option<Spline<f32, f32>>) -> Self {
        NoiseGenerator {
            seed,
            frequency,
            _octaves: octaves,
            spline,
        }
    }

    pub fn get(&self, x: f32, z: f32) -> f32 {
        // Wrap coordinates to avoid precision issues since simdnoise taps out at 2^26
        let x = x % 67108864f32;
        let z = z % 67108864f32;
        let mut noise_builder = simdnoise::NoiseBuilder::gradient_2d_offset(x, 4, z, 4);
        let noise_val = *noise_builder
            .with_freq(self.frequency)
            .with_seed(self.seed as i32)
            // .with_octaves(self.octaves)
            .generate_scaled(0.0, 1.0)
            .first()
            .expect("Failed to generate noise");
        if let Some(spline) = &self.spline {
            spline.clamped_sample(noise_val).unwrap()
        } else {
            noise_val
        }
    }
}
