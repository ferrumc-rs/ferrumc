use std::ops::RangeInclusive;

use bevy_math::IVec3;
use ferrumc_world::vanilla_chunk_format::BlockData;

use crate::{
    overworld::aquifer::clamped_map,
    perlin_noise::{NormalNoise, ORE_GAP, ORE_VEIN_A, ORE_VEIN_B, ORE_VEININESS},
    random::{Rng, RngFactory, Xoroshiro128PlusPlusFactory},
};

pub struct Vein {
    vein_toggle: NormalNoise<1>,
    vein_a: NormalNoise<1>,
    vein_b: NormalNoise<1>,
    vein_gap: NormalNoise<1>,
    random: Xoroshiro128PlusPlusFactory,
}

#[allow(dead_code)]
impl Vein {
    pub fn new(random: Xoroshiro128PlusPlusFactory) -> Self {
        let random = random.with_hash("minecraft:ore").fork_positional();
        Self {
            vein_toggle: ORE_VEININESS.init(random),
            vein_a: ORE_VEIN_A.init(random),
            vein_b: ORE_VEIN_B.init(random),
            vein_gap: ORE_GAP.init(random),
            random,
        }
    }

    pub(crate) fn at(&self, pos: IVec3) -> Option<BlockData> {
        let copper: (BlockData, BlockData, BlockData, RangeInclusive<i32>) = (
            BlockData {
                name: "minecraft:copper_ore".to_string(),
                properties: None,
            },
            BlockData {
                name: "minecraft:raw_copper_block".to_string(),
                properties: None,
            },
            BlockData {
                name: "minecraft:granine".to_string(),
                properties: None,
            },
            (0..=50),
        );
        let iron: (BlockData, BlockData, BlockData, RangeInclusive<i32>) = (
            BlockData {
                name: "minecraft:deepslate_iron_ore".to_string(),
                properties: None,
            },
            BlockData {
                name: "minecraft:raw_iron_block".to_string(),
                properties: None,
            },
            BlockData {
                name: "minecraft:tuff".to_string(),
                properties: None,
            },
            (-60..=-8),
        );
        let vein_toggle = self.vein_toggle.at(pos.as_dvec3() * 1.5);
        let vein_type = if vein_toggle > 0.0 { copper } else { iron };

        let distance = distance(vein_type.3, pos.y);

        if distance <= 0 {
            return None;
        }
        let d1 = clamped_map(f64::from(distance), 0.0, 20.0, -0.2, 0.0);

        let vein_toggle_abs = vein_toggle.abs();
        if vein_toggle_abs + d1 < 0.4 {
            return None;
        }
        let mut rand = self.random.at(pos);
        let vein_pos = pos.as_dvec3() * 4.0;
        if rand.next_f32() > 0.7 || self.vein_a.at(vein_pos).max(self.vein_b.at(vein_pos)) >= 0.08 {
            return None;
        }

        if f64::from(rand.next_f32()) < clamped_map(vein_toggle_abs, 0.4, 0.6, 0.1, 0.3)
            && self.vein_gap.at(pos.into()) > -0.3
        {
            if rand.next_f32() < 0.02 {
                Some(vein_type.1)
            } else {
                Some(vein_type.0)
            }
        } else {
            Some(vein_type.2)
        }
    }
}
fn distance(range: RangeInclusive<i32>, i: i32) -> i32 {
    let dist_to_upper = range.end() - i;
    let dist_to_lower = i - range.start();
    dist_to_lower.min(dist_to_upper)
}
