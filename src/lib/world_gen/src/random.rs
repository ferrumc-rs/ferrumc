#[derive(Debug, Clone)]
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

    pub fn next_long(&mut self) -> u64 {
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
        let got = rng.next_long();
        assert_eq!(got, exp, "Mismatch in sequence");
    }
}

#[test]
fn test_from_seed() {
    let mut rng = Xoroshiro128PlusPlus::from_seed(3257840388504953787);

    assert_eq!(
        rng.lo, -6493781293903536373i64 as u64,
        "Mismatch in lo seed"
    );
    assert_eq!(
        rng.hi, -6828912693740136794i64 as u64,
        "Mismatch in hi seed"
    );
}
