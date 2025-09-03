use std::ops::RangeInclusive;

use bevy_math::IVec3;

use crate::{
    NoiseRouter,
    aquifier::clamped_map,
    random::{RandomState, Rng, RngFactory},
};

#[allow(dead_code)]
fn compute_vein_block(
    random: &RandomState,
    settings: &NoiseRouter,
    pos: IVec3,
) -> Option<VeinBlockType> {
    let vein_toggle = settings.vein_toggle.compute(pos);
    let vein_type = if vein_toggle > 0.0 { COPPER } else { IRON };

    let dist_to_upper = vein_type.3.end() - pos.y;
    let dist_to_lower = pos.y - vein_type.3.start();

    if !vein_type.3.contains(&pos.y) {
        return None;
    }
    let d1 = clamped_map(
        f64::from(dist_to_upper.min(dist_to_lower)),
        0.0,
        20.0,
        -0.2,
        0.0,
    );

    let vein_toggle_abs = vein_toggle.abs();
    if vein_toggle_abs + d1 < 0.4 {
        return None;
    }
    let mut rand = random.ore_random.with_pos(pos);
    if rand.next_f32() > 0.7 || settings.vein_ridged.compute(pos) >= 0.0 {
        return None;
    }

    if f64::from(rand.next_f32()) < clamped_map(vein_toggle_abs, 0.4, 0.6, 0.1, 0.3)
        && settings.vein_gap.compute(pos) > -0.3
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

const COPPER: (
    VeinBlockType,
    VeinBlockType,
    VeinBlockType,
    RangeInclusive<i32>,
) = (
    VeinBlockType::CopperOre,
    VeinBlockType::RawCopperBlock,
    VeinBlockType::Granite,
    (0..=50),
);
const IRON: (
    VeinBlockType,
    VeinBlockType,
    VeinBlockType,
    RangeInclusive<i32>,
) = (
    VeinBlockType::DeepslateIronOre,
    VeinBlockType::RawIronBlock,
    VeinBlockType::Tuff,
    (-60..=-8),
);
enum VeinBlockType {
    CopperOre,
    RawCopperBlock,
    Granite,
    DeepslateIronOre,
    RawIronBlock,
    Tuff,
}
