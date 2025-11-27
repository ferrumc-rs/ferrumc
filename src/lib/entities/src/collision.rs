use ferrumc_data::generated::entities::EntityType as EntityTypeData;
use ferrumc_state::GlobalState;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub half_width: f64,
    pub height: f64,
}

impl BoundingBox {
    pub const PIG: BoundingBox = BoundingBox {
        half_width: EntityTypeData::PIG.dimension[0] as f64 / 2.0,
        height: EntityTypeData::PIG.dimension[1] as f64,
    };
}

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
pub fn check_collision(state: &GlobalState, x: f64, y: f64, z: f64, bbox: &BoundingBox) -> bool {
    let check_positions = [
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

pub fn is_in_water(state: &GlobalState, x: f64, y: f64, z: f64, bbox: &BoundingBox) -> bool {
    let center_x = x.floor() as i32;
    let center_y = (y + bbox.height / 2.0).floor() as i32;
    let center_z = z.floor() as i32;

    is_water_block(state, center_x, center_y, center_z)
}
