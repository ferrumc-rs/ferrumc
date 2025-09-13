use std::ops::Range;

use bevy_math::IVec3;

use crate::pos::ChunkPos;

const PHI: u64 = 0x9e3779b97f4a7c15;

pub trait Rng<RF> {
    fn next_u32(&mut self) -> u32;
    fn next_u64(&mut self) -> u64;

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

    fn fork_positional(&mut self) -> RF;
}

pub trait RngFactory<R> {
    fn with_hash(&self, s: &str) -> R;
    fn with_pos(&self, pos: IVec3) -> R;
}

#[derive(Clone, Copy)]
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
            seed = (seed ^ (seed >> 30)).wrapping_mul(0xBF58476D1CE4E5B9u64);
            seed = (seed ^ (seed >> 27)).wrapping_mul(0x94D049BB133111EBu64);
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
}
impl Rng<Xoroshiro128PlusPlusFactory> for Xoroshiro128PlusPlus {
    fn next_u64(&mut self) -> u64 {
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
    fn next_bounded(&mut self, bound: u32) -> u32 {
        assert_ne!(bound, 0, "Bound must be positive");
        loop {
            let res = u64::from(self.next_u32()).wrapping_mul(bound.into());
            let lo = res as u32;
            if lo >= bound || lo >= (!bound + 1) % bound {
                return (res >> 32) as u32;
            }
        }
    }

    fn next_f64(&mut self) -> f64 {
        ((self.next_u64() >> 11) as f32 * 1.110223E-16f32).into()
    }

    fn next_f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 * 5.9604645E-8f32
    }

    fn fork_positional(&mut self) -> Xoroshiro128PlusPlusFactory {
        Xoroshiro128PlusPlusFactory {
            lo: self.next_u64(),
            hi: self.next_u64(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Xoroshiro128PlusPlusFactory {
    lo: u64,
    hi: u64,
}
impl RngFactory<Xoroshiro128PlusPlus> for Xoroshiro128PlusPlusFactory {
    fn with_hash(&self, s: &str) -> Xoroshiro128PlusPlus {
        let digest = md5::compute(s.as_bytes());

        Xoroshiro128PlusPlus::new(
            u64::from_be_bytes(digest[0..8].try_into().unwrap()) ^ self.lo,
            u64::from_be_bytes(digest[8..16].try_into().unwrap()) ^ self.hi,
        )
    }

    fn with_pos(&self, pos: IVec3) -> Xoroshiro128PlusPlus {
        Xoroshiro128PlusPlus::new(seed_at(pos) as u64 ^ self.lo, self.hi)
    }
}

#[allow(dead_code)]
pub struct LegacyRandom {
    seed: u64,
}

#[allow(dead_code)]
impl LegacyRandom {
    pub fn large_features(seed: u64, chunk_pos: ChunkPos) -> Self {
        let mut random = Self::new(seed);
        Self::new(
            (i64::from(chunk_pos.pos.x) as u64 * random.next_u64())
                ^ (i64::from(chunk_pos.pos.y) as u64 * random.next_u64())
                ^ seed,
        )
    }
    pub fn new(seed: u64) -> Self {
        Self {
            seed: (seed ^ 0x5DEECE66D) & ((1 << 48) - 1),
        }
    }
    pub fn next(&mut self, bits: u32) -> i32 {
        self.seed = self.seed.wrapping_mul(0x5DEECE66D).wrapping_add(11) & ((1 << 48) - 1);
        (self.seed >> (48 - bits)) as i32
    }
}
impl Rng<LegacyPositionalFactory> for LegacyRandom {
    fn next_u32(&mut self) -> u32 {
        self.next(32) as u32
    }

    fn next_u64(&mut self) -> u64 {
        ((i64::from(self.next(32)) << 32) + i64::from(self.next(32))) as u64
    }

    fn next_f32(&mut self) -> f32 {
        self.next(24) as f32 * 5.9604645E-8f32
    }

    fn next_f64(&mut self) -> f64 {
        f64::from((((self.next(26) as u64) << 27) + self.next(27) as u64) as f32 * 1.110223E-16f32)
    }

    fn next_bounded(&mut self, bound: u32) -> u32 {
        if (bound & (bound - 1)) == 0 {
            ((u64::from(bound) * self.next(31) as u64) >> 31) as u32
        } else {
            self.next(31) as u32 % bound
        }
    }

    fn fork_positional(&mut self) -> LegacyPositionalFactory {
        LegacyPositionalFactory {
            seed: self.next_u64(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct LegacyPositionalFactory {
    seed: u64,
}

impl RngFactory<LegacyRandom> for LegacyPositionalFactory {
    fn with_hash(&self, s: &str) -> LegacyRandom {
        LegacyRandom::new((i64::from(java_string_hashcode(s)) ^ self.seed as i64) as u64)
    }

    fn with_pos(&self, pos: IVec3) -> LegacyRandom {
        LegacyRandom::new((seed_at(pos) ^ self.seed as i64) as u64)
    }
}

fn seed_at(pos: IVec3) -> i64 {
    let composition = i64::from(pos.x).wrapping_mul(3129871)
        ^ i64::from(pos.z).wrapping_mul(116129781)
        ^ i64::from(pos.y);
    let shuffle = composition
        .wrapping_mul(composition)
        .wrapping_mul(42317861)
        .wrapping_add(composition.wrapping_mul(11));
    shuffle >> 16
}

fn java_string_hashcode(s: &str) -> i32 {
    let mut hash: i32 = 0;
    for unit in s.encode_utf16() {
        hash = hash.wrapping_mul(31).wrapping_add(unit.into());
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

    let factory = rng.fork_positional();

    assert_eq!(factory.with_hash("test").seed, 198298808087495);
    assert_eq!(factory.with_hash("test").next_u64(), 1964728489694604786);
    assert_eq!(
        factory.with_pos((1, 1, 1).into()).next_u64(),
        6437814084537238339
    );
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
