const PHI: u64 = 0x9e3779b97f4a7c15;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Xoroshiro128PlusPlus {
    lo: u64,
    hi: u64,
}

/// Reference: net.minecraft.world.level.levelgen.Xoroshiro128PlusPlus
#[allow(dead_code)]
impl Xoroshiro128PlusPlus {
    /// Reference: net.minecraft.world.level.levelgen.RandomSupport
    pub fn from_seed(seed: u64) -> Self {
        fn mix_stafford13(mut seed: u64) -> u64 {
            seed = (seed ^ (seed >> 30)).wrapping_mul(0xBF58476D1CE4E5B9u64); // -4658895280553007687_i64 as u64
            seed = (seed ^ (seed >> 27)).wrapping_mul(0x94D049BB133111EBu64); // -7723592293110705685_i64 as u64
            seed ^ (seed >> 31)
        }

        let low = seed ^ 0x6a09e667f3bcc909;
        Self {
            lo: mix_stafford13(low),
            hi: mix_stafford13(low.wrapping_add(PHI)),
        }
    }

    pub fn new(lo: u64, hi: u64) -> Self {
        if (lo | hi) == 0 {
            return Self {
                lo: PHI,
                hi: 0x6a09e667f3bcc909,
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

        self.hi ^= self.lo;
        self.lo = self.lo.rotate_left(49) ^ self.hi ^ (self.hi << 21);
        self.hi = self.hi.rotate_left(28);
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

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct PositionalFactory {
    lo: u64,
    hi: u64,
}

#[allow(dead_code)]
impl PositionalFactory {
    pub fn with_hash(&self, s: &str) -> Xoroshiro128PlusPlus {
        let digest = md5::compute(s.as_bytes());

        Xoroshiro128PlusPlus {
            lo: u64::from_be_bytes(digest[0..8].try_into().unwrap()) ^ self.lo,
            hi: u64::from_be_bytes(digest[8..16].try_into().unwrap()) ^ self.hi,
        }
    }
}

#[allow(dead_code)]
pub struct LegacyRandom {
    seed: u64,
}

#[allow(dead_code)]
impl LegacyRandom {
    pub fn new(seed: u64) -> Self {
        Self {
            seed: (seed ^ 0x5DEECE66D) & ((1 << 48) - 1),
        }
    }

    pub fn next(&mut self, bits: u32) -> i32 {
        self.seed = self.seed.wrapping_mul(0x5DEECE66D).wrapping_add(11) & ((1 << 48) - 1);
        (self.seed >> (48 - bits)) as i32
    }

    pub fn next_u64(&mut self) -> u64 {
        ((self.next(32) as u64) << 32) + self.next(32) as u64
    }

    pub fn next_bounded(&mut self, bound: u32) -> u32 {
        if (bound & (bound - 1)) == 0 {
            ((u64::from(bound) * self.next(31) as u64) >> 31) as u32
        } else {
            self.next(31) as u32 % bound
        }
    }
}

#[test]
fn test_legacy() {
    let mut rng = LegacyRandom::new(0);

    let expected: [i32; 5] = [-1268774284, 1362668399, -881149874, 1891536193, -906589512];

    for &exp in &expected {
        let got = rng.next(48);
        assert_eq!(got, exp, "Mismatch in sequence");
    }
}

#[test]
fn test_legacy_bounded() {
    let mut rng = LegacyRandom::new(0);

    let expected: [u32; 5] = [41360, 5948, 48029, 16447, 43515];

    for &exp in &expected {
        let got = rng.next_bounded(100000);
        assert_eq!(got, exp, "Mismatch in sequence");
    }

    let mut rng = LegacyRandom::new(0);

    let expected: [u32; 5] = [748, 851, 246, 620, 652];

    for &exp in &expected {
        let got = rng.next_bounded(1024);
        assert_eq!(got, exp, "Mismatch in sequence");
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
