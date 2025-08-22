#[derive(Debug, Clone, Copy)]
pub struct Xoroshiro128PlusPlus {
    lo: u64,
    hi: u64,
}

/// Reference: net.minecraft.world.level.levelgen.Xoroshiro128PlusPlus
impl Xoroshiro128PlusPlus {
    /// Reference: net.minecraft.world.level.levelgen.RandomSupport
    pub fn from_seed(seed: u64) -> Self {
        fn mix_stafford13(mut seed: u64) -> u64 {
            seed = (seed ^ (seed >> 30)).wrapping_mul(0xBF58476D1CE4E5B9u64); // -4658895280553007687_i64 as u64
            seed = (seed ^ (seed >> 27)).wrapping_mul(0x94D049BB133111EBu64); // -7723592293110705685_i64 as u64
            seed ^ (seed >> 31)
        }

        let low = seed ^ 7640891576956012809u64; // 0x69C16F48A42B9D5
        Self {
            lo: mix_stafford13(low),
            hi: mix_stafford13(low.wrapping_add(0x9E3779B97F4A7C15u64)), // -7046029254386353131_i64 as u64
        }
    }

    pub fn new(lo: u64, hi: u64) -> Self {
        if (lo | hi) == 0 {
            return Self {
                lo: 0x9E3779B97F4A7C15, // -7046029254386353131_i64 as u64
                hi: 0x6a09e667f3bcc909, // 7640891576956012809
            };
        }
        Self { lo, hi }
    }

    pub fn next_u64(&mut self) -> u64 {
        let res = self
            .lo
            .wrapping_add(self.hi)
            .rotate_left(17)
            .wrapping_add(self.lo);

        let xor = self.lo ^ self.hi;
        self.lo = self.lo.rotate_left(49) ^ xor ^ (xor << 21);
        self.hi = xor.rotate_left(28);

        res
    }

    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    ///reference: net.minecraft.world.level.levelgen.XoroshiroRandomSource
    pub fn next_bounded(&mut self, bound: u32) -> u32 {
        assert_ne!(bound, 0, "Bound must be positive");
        loop {
            let res = u64::from(self.next_u32()).wrapping_mul(bound.into());
            let lo = res as u32;
            if lo >= bound || lo >= (!bound + 1) % bound {
                return (res >> 32) as u32;
            }
        }
    }

    pub fn next_f64(&mut self) -> f64 {
        ((self.next_u64() >> 11) as f32 * 1.110223E-16f32).into()
    }

    pub fn next_f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 * 5.9604645E-8f32
    }

    pub fn fork_positional(&mut self) -> PositionalFactory {
        PositionalFactory {
            lo: self.next_u64(),
            hi: self.next_u64(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PositionalFactory {
    lo: u64,
    hi: u64,
}

impl PositionalFactory {
    pub fn with_hash(&self, s: &str) -> Xoroshiro128PlusPlus {
        let digest = md5::compute(s.as_bytes());

        Xoroshiro128PlusPlus {
            lo: u64::from_be_bytes(digest[0..8].try_into().unwrap()) ^ self.lo,
            hi: u64::from_be_bytes(digest[8..16].try_into().unwrap()) ^ self.hi,
        }
    }
}

#[test]
fn test_zero() {
    let mut rng = Xoroshiro128PlusPlus::new(0, 0);

    // Expected outputs from running the Java version with the same seeds:
    let expected: [u64; 5] = [
        6807859099481836695,
        5275285228792843439,
        -1883134111310439721i64 as u64,
        -7481282880567689833i64 as u64,
        -7884262219761809303i64 as u64,
    ];

    for &exp in &expected {
        let got = rng.next_u64();
        assert_eq!(got, exp, "Mismatch in sequence");
    }
}

#[test]
fn test_from_seed() {
    let rng = Xoroshiro128PlusPlus::from_seed(3257840388504953787);

    assert_eq!(
        rng.lo, -6493781293903536373i64 as u64,
        "Mismatch in lo seed"
    );
    assert_eq!(
        rng.hi, -6828912693740136794i64 as u64,
        "Mismatch in hi seed"
    );
}

#[test]
fn test_fork_positional_with_hash() {
    let mut rng = Xoroshiro128PlusPlus::new(0, 0);
    let mut rng = rng.fork_positional().with_hash("test");

    assert_eq!(rng.next_u64(), 8856493334125025190, "Mismatch in next_u64");
}

#[test]
fn test_next_float() {
    let mut rng = Xoroshiro128PlusPlus::new(0, 0);

    assert_eq!(rng.next_f64(), 0.36905479431152344, "Mismatch in next_f64");
    assert_eq!(rng.next_f32(), 0.28597373, "Mismatch in next_f32");
}

#[test]
fn test_next_bounded() {
    let mut rng = Xoroshiro128PlusPlus::new(0, 0);

    assert_eq!(rng.next_bounded(123), 4, "Mismatch in next_bounded");
    assert_eq!(rng.next_bounded(100_000), 27758, "Mismatch in next_bounded");
}
