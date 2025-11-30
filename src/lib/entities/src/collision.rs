use bevy_math::bounding::Aabb3d;
use bevy_math::Vec3A;
use ferrumc_core::transform::position::Position;
use ferrumc_data::generated::entities::EntityType as EntityTypeData;
use ferrumc_state::GlobalState;

/// Pig bounding box using Bevy's Aabb3d
pub const PIG_AABB: Aabb3d = {
    let width = EntityTypeData::PIG.dimension[0];
    let height = EntityTypeData::PIG.dimension[1];
    Aabb3d {
        min: Vec3A::new(-width / 2.0, 0.0, -width / 2.0),
        max: Vec3A::new(width / 2.0, height, width / 2.0),
    }
};

pub fn is_water_block(state: &GlobalState, x: i32, y: i32, z: i32) -> bool {
    state
        .world
        .get_block_and_fetch(x, y, z, "overworld")
        .map(|block_state| (86..=101).contains(&block_state.0))
        .unwrap_or(false)
}

pub fn is_solid_block(state: &GlobalState, x: i32, y: i32, z: i32) -> bool {
    state
        .world
        .get_block_and_fetch(x, y, z, "overworld")
        .map(|block_state| {
            let id = block_state.0;
            id != 0 && !(86..=117).contains(&id)
        })
        .unwrap_or(false)
}

/// Checks collision by testing 8 points (4 corners at feet + 4 at head level)
pub fn check_collision(state: &GlobalState, pos: &Position, aabb: &Aabb3d) -> bool {
    let min = aabb.min.as_dvec3();
    let max = aabb.max.as_dvec3();

    let check_positions = [
        // Feet level (min.y)
        ((pos.x + min.x).floor() as i32, (pos.y + min.y).floor() as i32, (pos.z + min.z).floor() as i32),
        ((pos.x + max.x).floor() as i32, (pos.y + min.y).floor() as i32, (pos.z + min.z).floor() as i32),
        ((pos.x + min.x).floor() as i32, (pos.y + min.y).floor() as i32, (pos.z + max.z).floor() as i32),
        ((pos.x + max.x).floor() as i32, (pos.y + min.y).floor() as i32, (pos.z + max.z).floor() as i32),
        // Head level (max.y)
        ((pos.x + min.x).floor() as i32, (pos.y + max.y).floor() as i32, (pos.z + min.z).floor() as i32),
        ((pos.x + max.x).floor() as i32, (pos.y + max.y).floor() as i32, (pos.z + min.z).floor() as i32),
        ((pos.x + min.x).floor() as i32, (pos.y + max.y).floor() as i32, (pos.z + max.z).floor() as i32),
        ((pos.x + max.x).floor() as i32, (pos.y + max.y).floor() as i32, (pos.z + max.z).floor() as i32),
    ];

    for (check_x, check_y, check_z) in check_positions {
        if is_solid_block(state, check_x, check_y, check_z) {
            return true;
        }
    }

    false
}

pub fn is_in_water(state: &GlobalState, pos: &Position, aabb: &Aabb3d) -> bool {
    let center = pos.coords + (aabb.min.as_dvec3() + aabb.max.as_dvec3()) * 0.5;
    is_water_block(state, center.x.floor() as i32, center.y.floor() as i32, center.z.floor() as i32)
}
