use std::range::Range;

use crate::pos::{BlockPos, ChunkPos};

#[const_trait]
pub trait Rng {
    fn next_f32(&mut self) -> f32;
    fn next_f64(&mut self) -> f64;

    fn next_bounded(&mut self, bound: u32) -> u32;

    fn next_i32_range(&mut self, range: Range<i32>) -> i32 {
        self.next_bounded((range.end - range.start) as u32) as i32 + range.start
    }
    fn next_f32_range(&mut self, range: Range<f32>) -> f32 {
        self.next_f32() * (range.end - range.start) + range.start
    }
    fn next_trapezoid(&mut self, min: f32, max: f32, plateau: f32) -> f32 {
        let size = max - min;
        let height = (size - plateau) / 2.0;
        min + self.next_f32() * (size - height) + self.next_f32() * height
    }
    fn shuffle<T>(&mut self, array: &mut [T]) {
        let mut i = 0;
        while i < array.len() {
            array.swap(
                i,
                i + self.next_bounded(array.len() as u32 - i as u32) as usize,
            );
            i += 1;
        }
    }

    fn fork(&mut self) -> Self;
    fn with_hash(&self, s: &str) -> Self;
}

#[derive(Clone, Copy)]
pub struct Xoroshiro128PlusPlus {
    lo: u64,
    hi: u64,
}

/// Reference: net.minecraft.world.level.levelgen.Xoroshiro128PlusPlus
#[allow(dead_code)]
impl Xoroshiro128PlusPlus {
    const PHI: u64 = 0x9e3779b97f4a7c15;
    /// Reference: net.minecraft.world.level.levelgen.RandomSupport
    pub const fn from_seed(seed: u64) -> Self {
        const fn mix_stafford13(mut seed: u64) -> u64 {
            seed = (seed ^ (seed >> 30)).wrapping_mul(0xBF58476D1CE4E5B9u64);
            seed = (seed ^ (seed >> 27)).wrapping_mul(0x94D049BB133111EBu64);
            seed ^ (seed >> 31)
        }

        let low = seed ^ 0x6a09e667f3bcc909;
        Self {
            lo: mix_stafford13(low),
            hi: mix_stafford13(low.wrapping_add(Self::PHI)),
        }
    }

    pub const fn new(lo: u64, hi: u64) -> Self {
        if (lo | hi) == 0 {
            return Self {
                lo: Self::PHI,
                hi: 0x6a09e667f3bcc909,
            };
        }
        Self { lo, hi }
    }

    const fn next_u64(&mut self) -> u64 {
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

    pub const fn next_bool(&mut self) -> bool {
        self.next_u64() & 1 != 0
    }

    ///reference: net.minecraft.world.level.levelgen.XoroshiroRandomSource
    pub const fn next_bounded(&mut self, bound: u32) -> u32 {
        assert!(bound != 0, "Bound must be positive");
        loop {
            let res = (self.next_u64() & 0xFFFF_FFFF).wrapping_mul(bound as u64);
            let lo = res as u32;
            if lo >= bound || lo >= (!bound + 1) % bound {
                return (res >> 32) as u32;
            }
        }
    }

    pub const fn next_f64(&mut self) -> f64 {
        ((self.next_u64() >> 11) as f32 * 1.110223E-16f32) as f64
    }

    pub const fn next_f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 * 5.9604645E-8f32
    }

    pub const fn fork(&mut self) -> Self {
        Self {
            lo: self.next_u64(),
            hi: self.next_u64(),
        }
    }

    pub const fn with_hash(&self, s: &str) -> Self {
        let a = cthash::md5(s.as_bytes());
        Self::new(
            u64::from_be_bytes([a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]]) ^ self.lo,
            u64::from_be_bytes([a[8], a[9], a[10], a[11], a[12], a[13], a[14], a[15]]) ^ self.hi,
        )
    }

    pub const fn at(&self, pos: BlockPos) -> Self {
        Self::new(seed_at(pos) ^ self.lo, self.hi)
    }
}

impl const Rng for Xoroshiro128PlusPlus {
    fn next_bounded(&mut self, bound: u32) -> u32 {
        self.next_bounded(bound)
    }

    fn next_f64(&mut self) -> f64 {
        self.next_f64()
    }

    fn next_f32(&mut self) -> f32 {
        self.next_f32()
    }

    fn fork(&mut self) -> Self {
        self.fork()
    }

    fn with_hash(&self, s: &str) -> Self {
        self.with_hash(s)
    }
}

pub struct LegacyRandom {
    seed: u64,
}

impl LegacyRandom {
    pub fn large_features(seed: u64, chunk_pos: ChunkPos) -> Self {
        let mut random = Self::new(seed);
        Self::new(
            (i64::from(chunk_pos.pos.x) as u64 * random.next_u64())
                ^ (i64::from(chunk_pos.pos.y) as u64 * random.next_u64())
                ^ seed,
        )
    }
    pub const fn new(seed: u64) -> Self {
        Self {
            seed: (seed ^ 0x5DEECE66D) & ((1 << 48) - 1),
        }
    }
    const fn next(&mut self, bits: u32) -> i32 {
        self.seed = self.seed.wrapping_mul(0x5DEECE66D).wrapping_add(11) & ((1 << 48) - 1);
        (self.seed >> (48 - bits)) as i32
    }

    pub const fn next_f64(&mut self) -> f64 {
        ((((self.next(26) as u64) << 27) + self.next(27) as u64) as f32 * 1.110223E-16f32) as f64
    }

    pub const fn next_bounded(&mut self, bound: u32) -> u32 {
        if (bound & (bound - 1)) == 0 {
            ((bound as u64 * self.next(31) as u64) >> 31) as u32
        } else {
            self.next(31) as u32 % bound
        }
    }

    const fn next_u64(&mut self) -> u64 {
        (((self.next(32) as i64) << 32) + (self.next(32) as i64)) as u64
    }

    pub const fn next_random(&mut self) -> LegacyRandom {
        LegacyRandom::new(self.next_u64())
    }

    pub const fn next_f32(&mut self) -> f32 {
        self.next(24) as f32 * 5.9604645E-8f32
    }

    pub const fn fork(&mut self) -> Self {
        Self {
            seed: self.next_u64(),
        }
    }
    pub const fn with_hash(&self, s: &str) -> Self {
        Self::new(((java_string_hashcode(s) as i64) ^ self.seed as i64) as u64)
    }
    pub const fn at(&self, pos: BlockPos) -> Self {
        Self::new(seed_at(pos) ^ self.seed)
    }
}

impl const Rng for LegacyRandom {
    fn next_f32(&mut self) -> f32 {
        self.next_f32()
    }
    fn next_f64(&mut self) -> f64 {
        self.next_f64()
    }
    fn next_bounded(&mut self, bound: u32) -> u32 {
        self.next_bounded(bound)
    }
    fn fork(&mut self) -> Self {
        self.fork()
    }
    fn with_hash(&self, s: &str) -> Self {
        self.with_hash(s)
    }
}

const fn seed_at(pos: BlockPos) -> u64 {
    let composition = ((pos.x as i64).wrapping_mul(3129871)
        ^ (pos.z as i64).wrapping_mul(116129781)
        ^ (pos.y as i64)) as u64;
    let shuffle = composition
        .wrapping_mul(composition)
        .wrapping_mul(42317861)
        .wrapping_add(composition.wrapping_mul(11));
    shuffle >> 16
}

const fn java_string_hashcode(s: &str) -> i32 {
    let s = s.as_bytes();
    let mut hash: i32 = 0;
    let mut i = 0;
    while i < s.len() {
        hash = hash.wrapping_mul(31).wrapping_add(s[i] as i32);
        i += 1;
    }
    hash
}

#[test]
fn test_java_string_hashcode() {
    assert_eq!(java_string_hashcode("test"), 3556498);
    assert_eq!(java_string_hashcode("1234567890"), -2054162789);
}

#[test]
fn test_legacy_u64() {
    let mut rng = LegacyRandom::new(0);

    assert_eq!(rng.next_u64(), -4962768465676381896i64 as u64);
    assert_eq!(rng.next_u64(), 4437113781045784766);
}

#[test]
fn test_legacy_float() {
    let mut rng = LegacyRandom::new(0);

    assert_eq!(rng.next_f32(), 0.73096776);
    assert_eq!(rng.next_f64(), 0.8314409852027893);
}

#[test]
fn test_legacy_factory() {
    let mut rng = LegacyRandom::new(0);

    let factory = rng.fork();

    assert_eq!(factory.with_hash("test").seed, 198298808087495);
    assert_eq!(factory.with_hash("test").next_u64(), 1964728489694604786);
    assert_eq!(factory.at((1, 1, 1).into()).next_u64(), 6437814084537238339);
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
    let mut rng = rng.fork().with_hash("test");

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
