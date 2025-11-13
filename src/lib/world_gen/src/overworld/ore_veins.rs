use crate::common::math::clamped_map;
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use std::ops::RangeInclusive;

use bevy_math::IVec3;

use crate::{
    perlin_noise::{NormalNoise, ORE_GAP, ORE_VEIN_A, ORE_VEIN_B, ORE_VEININESS},
    random::Xoroshiro128PlusPlus,
};

pub struct Vein {
    vein_toggle: NormalNoise<1>,
    vein_a: NormalNoise<1>,
    vein_b: NormalNoise<1>,
    vein_gap: NormalNoise<1>,
    factory: Xoroshiro128PlusPlus,
}

impl Vein {
    pub fn new(factory: Xoroshiro128PlusPlus) -> Self {
        let factory = factory.with_hash("minecraft:ore").fork();
        Self {
            vein_toggle: ORE_VEININESS.init(factory),
            vein_a: ORE_VEIN_A.init(factory),
            vein_b: ORE_VEIN_B.init(factory),
            vein_gap: ORE_GAP.init(factory),
            factory,
        }
    }

    pub(crate) fn at(&self, pos: IVec3) -> Option<BlockStateId> {
        let copper: (
            BlockStateId,
            BlockStateId,
            BlockStateId,
            RangeInclusive<i32>,
        ) = (
            block!("copper_ore"),
            block!("raw_copper_block"),
            block!("granite"),
            (0..=50),
        );
        let iron: (
            BlockStateId,
            BlockStateId,
            BlockStateId,
            RangeInclusive<i32>,
        ) = (
            block!("deepslate_iron_ore"),
            block!("raw_iron_block"),
            block!("tuff"),
            (-60..=-8),
        );
        let vein_toggle = self.vein_toggle.at(pos.as_dvec3() * 1.5);
        let vein_type = if vein_toggle > 0.0 { copper } else { iron };

        let distance = distance(vein_type.3, pos.y);

        if distance < 0 {
            return None;
        }

        if vein_toggle.abs() < 0.6 - f64::from(distance).clamp(0., 20.) / 10. {
            return None;
        }
        let mut rand = self.factory.at(pos);
        let vein_pos = pos.as_dvec3() * 4.0;
        if rand.next_f32() > 0.7 || self.vein_a.at(vein_pos).max(self.vein_b.at(vein_pos)) >= 0.08 {
            return None;
        }

        if f64::from(rand.next_f32()) < clamped_map(vein_toggle.abs(), 0.4, 0.6, 0.1, 0.3)
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
