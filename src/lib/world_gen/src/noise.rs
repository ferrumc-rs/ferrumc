pub struct NoiseGenerator {
    seed: u64,
    scale: (f32, f32),
    frequency: f32,
    octaves: u8,
}

impl NoiseGenerator {
    pub fn new(seed: u64, min: f32, max: f32, frequency: f32, octaves: u8) -> Self {
        NoiseGenerator {
            seed,
            scale: (min, max),
            frequency,
            octaves,
        }
    }

    pub fn get(&self, x: f32, z: f32) -> f32 {
        let mut noise_builder = simdnoise::NoiseBuilder::gradient_2d_offset(
            x,
            4,
            z,
            4,
        );
        *noise_builder.with_freq(
            self.frequency
        ).with_seed(
            self.seed as i32
        ).generate_scaled(self.scale.0, self.scale.1).first().expect(
            "Failed to generate noise"
        )
    }
}