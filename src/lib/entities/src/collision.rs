use bevy_math::bounding::Aabb3d;
use bevy_math::{IVec3, Vec3A};
use ferrumc_core::transform::position::Position;
use ferrumc_data::generated::entities::EntityType as EntityTypeData;
use ferrumc_state::GlobalState;

/// Pig bounding box from vanilla data
///
/// Uses Bevy's `Aabb3d` with dimensions from `ferrumc-data`.
/// The box is centered on the X/Z axes with min.y at 0 (feet).
pub const PIG_AABB: Aabb3d = {
    let width = EntityTypeData::PIG.dimension[0];
    let height = EntityTypeData::PIG.dimension[1];
    Aabb3d {
        min: Vec3A::new(-width / 2.0, 0.0, -width / 2.0),
        max: Vec3A::new(width / 2.0, height, width / 2.0),
    }
};

pub fn is_water_block(state: &GlobalState, pos: IVec3) -> bool {
    state
        .world
        .get_block_and_fetch(pos.x, pos.y, pos.z, "overworld")
        .map(|block_state| (86..=101).contains(&block_state.0))
        .unwrap_or(false)
}

pub fn is_solid_block(state: &GlobalState, pos: IVec3) -> bool {
    state
        .world
        .get_block_and_fetch(pos.x, pos.y, pos.z, "overworld")
        .map(|block_state| {
            let id = block_state.0;
            id != 0 && !(86..=117).contains(&id)
        })
        .unwrap_or(false)
}

/// Checks collision by testing 8 points (4 corners at feet + 4 at head level)
pub fn check_collision(state: &GlobalState, pos: &Position, aabb: &Aabb3d) -> bool {
    let (min, max) = (aabb.min.as_dvec3(), aabb.max.as_dvec3());

    let to_block =
        |x: f64, y: f64, z: f64| IVec3::new(x.floor() as i32, y.floor() as i32, z.floor() as i32);

    let check_positions = [
        // Feet level
        to_block(pos.x + min.x, pos.y + min.y, pos.z + min.z),
        to_block(pos.x + max.x, pos.y + min.y, pos.z + min.z),
        to_block(pos.x + min.x, pos.y + min.y, pos.z + max.z),
        to_block(pos.x + max.x, pos.y + min.y, pos.z + max.z),
        // Head level
        to_block(pos.x + min.x, pos.y + max.y, pos.z + min.z),
        to_block(pos.x + max.x, pos.y + max.y, pos.z + min.z),
        to_block(pos.x + min.x, pos.y + max.y, pos.z + max.z),
        to_block(pos.x + max.x, pos.y + max.y, pos.z + max.z),
    ];

    check_positions
        .iter()
        .any(|&pos| is_solid_block(state, pos))
}

pub fn is_in_water(state: &GlobalState, pos: &Position, aabb: &Aabb3d) -> bool {
    let center = pos.coords + (aabb.min.as_dvec3() + aabb.max.as_dvec3()) * 0.5;
    is_water_block(state, center.as_ivec3())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pig_aabb_dimensions() {
        // Pig dimensions from vanilla: 0.9 x 0.9 x 0.9 blocks
        let expected_width = EntityTypeData::PIG.dimension[0];
        let expected_height = EntityTypeData::PIG.dimension[1];

        assert_eq!(PIG_AABB.min.x, -expected_width / 2.0);
        assert_eq!(PIG_AABB.max.x, expected_width / 2.0);
        assert_eq!(PIG_AABB.min.y, 0.0); // Feet at y=0
        assert_eq!(PIG_AABB.max.y, expected_height);
        assert_eq!(PIG_AABB.min.z, -expected_width / 2.0);
        assert_eq!(PIG_AABB.max.z, expected_width / 2.0);
    }

    #[test]
    fn test_pig_aabb_centered() {
        // The box should be centered on X/Z axes
        assert_eq!(PIG_AABB.min.x, -PIG_AABB.max.x);
        assert_eq!(PIG_AABB.min.z, -PIG_AABB.max.z);
    }

    #[test]
    fn test_aabb_conversion_to_dvec3() {
        let min_dvec = PIG_AABB.min.as_dvec3();
        let max_dvec = PIG_AABB.max.as_dvec3();

        // Verify conversion works correctly
        assert_eq!(min_dvec.x as f32, PIG_AABB.min.x);
        assert_eq!(min_dvec.y as f32, PIG_AABB.min.y);
        assert_eq!(min_dvec.z as f32, PIG_AABB.min.z);
        assert_eq!(max_dvec.x as f32, PIG_AABB.max.x);
        assert_eq!(max_dvec.y as f32, PIG_AABB.max.y);
        assert_eq!(max_dvec.z as f32, PIG_AABB.max.z);
    }
}
