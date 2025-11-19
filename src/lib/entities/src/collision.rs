use ferrumc_core::transform::position::Position;
use ferrumc_data::generated::entities::EntityType as EntityTypeData;
use ferrumc_state::GlobalState;

/// Bounding box dimensions for an entity
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub half_width: f64,
    pub height: f64,
}

impl BoundingBox {
    /// Pig hitbox from Minecraft vanilla data (0.9 × 0.9 blocks)
    pub const PIG: BoundingBox = BoundingBox {
        half_width: EntityTypeData::PIG.dimension[0] as f64 / 2.0,
        height: EntityTypeData::PIG.dimension[1] as f64,
    };
}

/// Check if a block is water
pub fn is_water_block(state: &GlobalState, x: i32, y: i32, z: i32) -> bool {
    state
        .world
        .get_block_and_fetch(x, y, z, "overworld")
        .map(|block_state| {
            let id = block_state.0;
            // Water is 86-101
            (86..=101).contains(&id)
        })
        .unwrap_or(false)
}

/// Check if there's a solid block at the given position
///
/// Excludes water (86-101) and lava (102-117) as they are not solid for collision purposes.
/// Entities should fall through these blocks.
pub fn is_solid_block(state: &GlobalState, x: i32, y: i32, z: i32) -> bool {
    state
        .world
        .get_block_and_fetch(x, y, z, "overworld")
        .map(|block_state| {
            let id = block_state.0;
            // Air is 0, water is 86-101, lava is 102-117
            id != 0 && !(86..=117).contains(&id)
        })
        .unwrap_or(false)
}

/// Check if an entity with the given bounding box would collide with blocks at the position
///
/// Checks 8 points: 4 corners at feet level and 4 corners at head level
pub fn check_collision(state: &GlobalState, x: f64, y: f64, z: f64, bbox: &BoundingBox) -> bool {
    // Check corners of the bounding box at feet and head level
    let check_positions = [
        // Feet level - 4 corners
        (
            (x - bbox.half_width).floor() as i32,
            y.floor() as i32,
            (z - bbox.half_width).floor() as i32,
        ),
        (
            (x + bbox.half_width).floor() as i32,
            y.floor() as i32,
            (z - bbox.half_width).floor() as i32,
        ),
        (
            (x - bbox.half_width).floor() as i32,
            y.floor() as i32,
            (z + bbox.half_width).floor() as i32,
        ),
        (
            (x + bbox.half_width).floor() as i32,
            y.floor() as i32,
            (z + bbox.half_width).floor() as i32,
        ),
        // Head level - 4 corners
        (
            (x - bbox.half_width).floor() as i32,
            (y + bbox.height).floor() as i32,
            (z - bbox.half_width).floor() as i32,
        ),
        (
            (x + bbox.half_width).floor() as i32,
            (y + bbox.height).floor() as i32,
            (z - bbox.half_width).floor() as i32,
        ),
        (
            (x - bbox.half_width).floor() as i32,
            (y + bbox.height).floor() as i32,
            (z + bbox.half_width).floor() as i32,
        ),
        (
            (x + bbox.half_width).floor() as i32,
            (y + bbox.height).floor() as i32,
            (z + bbox.half_width).floor() as i32,
        ),
    ];

    for (check_x, check_y, check_z) in check_positions {
        if is_solid_block(state, check_x, check_y, check_z) {
            return true;
        }
    }

    false
}

/// Check if there's an obstacle ahead in the movement direction
///
/// Used by AI to detect walls. Checks feet and head level.
pub fn check_obstacle_ahead(
    state: &GlobalState,
    pos: &Position,
    vel_x: f64,
    vel_z: f64,
    bbox: &BoundingBox,
) -> bool {
    let check_distance = 0.6; // Look slightly ahead
    let next_x = pos.x + vel_x.signum() * check_distance;
    let next_z = pos.z + vel_z.signum() * check_distance;

    // Check at feet level and head level
    let check_positions = [
        // Feet level
        (
            next_x.floor() as i32,
            pos.y.floor() as i32,
            next_z.floor() as i32,
        ),
        // Head level
        (
            next_x.floor() as i32,
            (pos.y + bbox.height * 0.5).floor() as i32,
            next_z.floor() as i32,
        ),
    ];

    for (check_x, check_y, check_z) in check_positions {
        if is_solid_block(state, check_x, check_y, check_z) {
            return true;
        }
    }

    false
}

/// Check if an entity is submerged in water
///
/// Checks if the entity's body (from feet to head) is in water
pub fn is_in_water(state: &GlobalState, x: f64, y: f64, z: f64, bbox: &BoundingBox) -> bool {
    // Check center of the entity at feet and mid-body level
    let block_x = x.floor() as i32;
    let block_z = z.floor() as i32;

    // Check feet level
    let feet_y = y.floor() as i32;
    if is_water_block(state, block_x, feet_y, block_z) {
        return true;
    }

    // Check mid-body level (waist height)
    let mid_y = (y + bbox.height * 0.5).floor() as i32;
    if is_water_block(state, block_x, mid_y, block_z) {
        return true;
    }

    false
}
