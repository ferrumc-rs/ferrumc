use super::*;
pub const POWDER_SNOW_CAULDRON: Block = Block {
    id: 359,
    name: "powder_snow_cauldron",
    translation_key: "block.minecraft.powder_snow_cauldron",
    hardness: 2f32,
    blast_resistance: 2f32,
    slipperiness: 0.6f32,
    velocity_multiplier: 1f32,
    jump_velocity_multiplier: 1f32,
    item_id: 1082,
    default_state: &BlockState {
        id: 8187,
        state_flags: 68,
        side_flags: 0,
        instrument: Instrument::HARP,
        luminance: 0,
        piston_behavior: PistonBehavior::Normal,
        hardness: 2f32,
        collision_shapes: &[
            289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303,
        ],
        outline_shapes: &[
            289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303,
        ],
        opacity: u8::MAX,
        block_entity_type: u16::MAX,
    },
    states: &[
        BlockState {
            id: 8187,
            state_flags: 68,
            side_flags: 0,
            instrument: Instrument::HARP,
            luminance: 0,
            piston_behavior: PistonBehavior::Normal,
            hardness: 2f32,
            collision_shapes: &[
                289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303,
            ],
            outline_shapes: &[
                289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303,
            ],
            opacity: u8::MAX,
            block_entity_type: u16::MAX,
        },
        BlockState {
            id: 8188,
            state_flags: 68,
            side_flags: 0,
            instrument: Instrument::HARP,
            luminance: 0,
            piston_behavior: PistonBehavior::Normal,
            hardness: 2f32,
            collision_shapes: &[
                289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303,
            ],
            outline_shapes: &[
                289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303,
            ],
            opacity: u8::MAX,
            block_entity_type: u16::MAX,
        },
        BlockState {
            id: 8189,
            state_flags: 68,
            side_flags: 0,
            instrument: Instrument::HARP,
            luminance: 0,
            piston_behavior: PistonBehavior::Normal,
            hardness: 2f32,
            collision_shapes: &[
                289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303,
            ],
            outline_shapes: &[
                289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303,
            ],
            opacity: u8::MAX,
            block_entity_type: u16::MAX,
        },
    ],
    flammable: None,
};
