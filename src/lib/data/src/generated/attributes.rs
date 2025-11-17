
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Attribute {
    pub id: u16,
    pub name: &'static str,
    pub default_value: f64,
}
impl Attribute {
    pub const ARMOR: Attribute = Attribute {
        id: 0,
        name: "armor",
        default_value: 0.0,
    };
    pub const ARMOR_TOUGHNESS: Attribute = Attribute {
        id: 1,
        name: "armor_toughness",
        default_value: 0.0,
    };
    pub const ATTACK_DAMAGE: Attribute = Attribute {
        id: 2,
        name: "attack_damage",
        default_value: 2.0,
    };
    pub const ATTACK_KNOCKBACK: Attribute = Attribute {
        id: 3,
        name: "attack_knockback",
        default_value: 0.0,
    };
    pub const ATTACK_SPEED: Attribute = Attribute {
        id: 4,
        name: "attack_speed",
        default_value: 4.0,
    };
    pub const BLOCK_BREAK_SPEED: Attribute = Attribute {
        id: 5,
        name: "block_break_speed",
        default_value: 1.0,
    };
    pub const BLOCK_INTERACTION_RANGE: Attribute = Attribute {
        id: 6,
        name: "block_interaction_range",
        default_value: 4.5,
    };
    pub const BURNING_TIME: Attribute = Attribute {
        id: 7,
        name: "burning_time",
        default_value: 1.0,
    };
    pub const CAMERA_DISTANCE: Attribute = Attribute {
        id: 8,
        name: "camera_distance",
        default_value: 4.0,
    };
    pub const ENTITY_INTERACTION_RANGE: Attribute = Attribute {
        id: 10,
        name: "entity_interaction_range",
        default_value: 3.0,
    };
    pub const EXPLOSION_KNOCKBACK_RESISTANCE: Attribute = Attribute {
        id: 9,
        name: "explosion_knockback_resistance",
        default_value: 0.0,
    };
    pub const FALL_DAMAGE_MULTIPLIER: Attribute = Attribute {
        id: 11,
        name: "fall_damage_multiplier",
        default_value: 1.0,
    };
    pub const FLYING_SPEED: Attribute = Attribute {
        id: 12,
        name: "flying_speed",
        default_value: 0.4,
    };
    pub const FOLLOW_RANGE: Attribute = Attribute {
        id: 13,
        name: "follow_range",
        default_value: 32.0,
    };
    pub const GRAVITY: Attribute = Attribute {
        id: 14,
        name: "gravity",
        default_value: 0.1,
    };
    pub const JUMP_STRENGTH: Attribute = Attribute {
        id: 15,
        name: "jump_strength",
        default_value: 0.4,
    };
    pub const KNOCKBACK_RESISTANCE: Attribute = Attribute {
        id: 16,
        name: "knockback_resistance",
        default_value: 0.0,
    };
    pub const LUCK: Attribute = Attribute {
        id: 17,
        name: "luck",
        default_value: 0.0,
    };
    pub const MAX_ABSORPTION: Attribute = Attribute {
        id: 18,
        name: "max_absorption",
        default_value: 0.0,
    };
    pub const MAX_HEALTH: Attribute = Attribute {
        id: 19,
        name: "max_health",
        default_value: 20.0,
    };
    pub const MINING_EFFICIENCY: Attribute = Attribute {
        id: 20,
        name: "mining_efficiency",
        default_value: 0.0,
    };
    pub const MOVEMENT_EFFICIENCY: Attribute = Attribute {
        id: 21,
        name: "movement_efficiency",
        default_value: 0.0,
    };
    pub const MOVEMENT_SPEED: Attribute = Attribute {
        id: 22,
        name: "movement_speed",
        default_value: 0.7,
    };
    pub const OXYGEN_BONUS: Attribute = Attribute {
        id: 23,
        name: "oxygen_bonus",
        default_value: 0.0,
    };
    pub const SAFE_FALL_DISTANCE: Attribute = Attribute {
        id: 24,
        name: "safe_fall_distance",
        default_value: 3.0,
    };
    pub const SCALE: Attribute = Attribute {
        id: 25,
        name: "scale",
        default_value: 1.0,
    };
    pub const SNEAKING_SPEED: Attribute = Attribute {
        id: 26,
        name: "sneaking_speed",
        default_value: 0.3,
    };
    pub const SPAWN_REINFORCEMENTS: Attribute = Attribute {
        id: 27,
        name: "spawn_reinforcements",
        default_value: 0.0,
    };
    pub const STEP_HEIGHT: Attribute = Attribute {
        id: 28,
        name: "step_height",
        default_value: 0.6,
    };
    pub const SUBMERGED_MINING_SPEED: Attribute = Attribute {
        id: 29,
        name: "submerged_mining_speed",
        default_value: 0.2,
    };
    pub const SWEEPING_DAMAGE_RATIO: Attribute = Attribute {
        id: 30,
        name: "sweeping_damage_ratio",
        default_value: 0.0,
    };
    pub const TEMPT_RANGE: Attribute = Attribute {
        id: 31,
        name: "tempt_range",
        default_value: 10.0,
    };
    pub const WATER_MOVEMENT_EFFICIENCY: Attribute = Attribute {
        id: 32,
        name: "water_movement_efficiency",
        default_value: 0.0,
    };
    pub const WAYPOINT_RECEIVE_RANGE: Attribute = Attribute {
        id: 34,
        name: "waypoint_receive_range",
        default_value: 0.0,
    };
    pub const WAYPOINT_TRANSMIT_RANGE: Attribute = Attribute {
        id: 33,
        name: "waypoint_transmit_range",
        default_value: 0.0,
    };
    #[doc = r" Try to parse an `Attribute` from a resource location string."]
    pub fn from_name(name: &str) -> Option<&'static Self> {
        let name = name.strip_prefix("minecraft:").unwrap_or(name);
        match name {
            "armor" => Some(&Self::ARMOR),
            "armor_toughness" => Some(&Self::ARMOR_TOUGHNESS),
            "attack_damage" => Some(&Self::ATTACK_DAMAGE),
            "attack_knockback" => Some(&Self::ATTACK_KNOCKBACK),
            "attack_speed" => Some(&Self::ATTACK_SPEED),
            "block_break_speed" => Some(&Self::BLOCK_BREAK_SPEED),
            "block_interaction_range" => Some(&Self::BLOCK_INTERACTION_RANGE),
            "burning_time" => Some(&Self::BURNING_TIME),
            "camera_distance" => Some(&Self::CAMERA_DISTANCE),
            "entity_interaction_range" => Some(&Self::ENTITY_INTERACTION_RANGE),
            "explosion_knockback_resistance" => Some(&Self::EXPLOSION_KNOCKBACK_RESISTANCE),
            "fall_damage_multiplier" => Some(&Self::FALL_DAMAGE_MULTIPLIER),
            "flying_speed" => Some(&Self::FLYING_SPEED),
            "follow_range" => Some(&Self::FOLLOW_RANGE),
            "gravity" => Some(&Self::GRAVITY),
            "jump_strength" => Some(&Self::JUMP_STRENGTH),
            "knockback_resistance" => Some(&Self::KNOCKBACK_RESISTANCE),
            "luck" => Some(&Self::LUCK),
            "max_absorption" => Some(&Self::MAX_ABSORPTION),
            "max_health" => Some(&Self::MAX_HEALTH),
            "mining_efficiency" => Some(&Self::MINING_EFFICIENCY),
            "movement_efficiency" => Some(&Self::MOVEMENT_EFFICIENCY),
            "movement_speed" => Some(&Self::MOVEMENT_SPEED),
            "oxygen_bonus" => Some(&Self::OXYGEN_BONUS),
            "safe_fall_distance" => Some(&Self::SAFE_FALL_DISTANCE),
            "scale" => Some(&Self::SCALE),
            "sneaking_speed" => Some(&Self::SNEAKING_SPEED),
            "spawn_reinforcements" => Some(&Self::SPAWN_REINFORCEMENTS),
            "step_height" => Some(&Self::STEP_HEIGHT),
            "submerged_mining_speed" => Some(&Self::SUBMERGED_MINING_SPEED),
            "sweeping_damage_ratio" => Some(&Self::SWEEPING_DAMAGE_RATIO),
            "tempt_range" => Some(&Self::TEMPT_RANGE),
            "water_movement_efficiency" => Some(&Self::WATER_MOVEMENT_EFFICIENCY),
            "waypoint_receive_range" => Some(&Self::WAYPOINT_RECEIVE_RANGE),
            "waypoint_transmit_range" => Some(&Self::WAYPOINT_TRANSMIT_RANGE),
            _ => None,
        }
    }
    #[doc = r" Try to get an `Attribute` from its ID."]
    pub const fn from_id(id: u16) -> Option<&'static Self> {
        match id {
            0 => Some(&Self::ARMOR),
            1 => Some(&Self::ARMOR_TOUGHNESS),
            2 => Some(&Self::ATTACK_DAMAGE),
            3 => Some(&Self::ATTACK_KNOCKBACK),
            4 => Some(&Self::ATTACK_SPEED),
            5 => Some(&Self::BLOCK_BREAK_SPEED),
            6 => Some(&Self::BLOCK_INTERACTION_RANGE),
            7 => Some(&Self::BURNING_TIME),
            8 => Some(&Self::CAMERA_DISTANCE),
            10 => Some(&Self::ENTITY_INTERACTION_RANGE),
            9 => Some(&Self::EXPLOSION_KNOCKBACK_RESISTANCE),
            11 => Some(&Self::FALL_DAMAGE_MULTIPLIER),
            12 => Some(&Self::FLYING_SPEED),
            13 => Some(&Self::FOLLOW_RANGE),
            14 => Some(&Self::GRAVITY),
            15 => Some(&Self::JUMP_STRENGTH),
            16 => Some(&Self::KNOCKBACK_RESISTANCE),
            17 => Some(&Self::LUCK),
            18 => Some(&Self::MAX_ABSORPTION),
            19 => Some(&Self::MAX_HEALTH),
            20 => Some(&Self::MINING_EFFICIENCY),
            21 => Some(&Self::MOVEMENT_EFFICIENCY),
            22 => Some(&Self::MOVEMENT_SPEED),
            23 => Some(&Self::OXYGEN_BONUS),
            24 => Some(&Self::SAFE_FALL_DISTANCE),
            25 => Some(&Self::SCALE),
            26 => Some(&Self::SNEAKING_SPEED),
            27 => Some(&Self::SPAWN_REINFORCEMENTS),
            28 => Some(&Self::STEP_HEIGHT),
            29 => Some(&Self::SUBMERGED_MINING_SPEED),
            30 => Some(&Self::SWEEPING_DAMAGE_RATIO),
            31 => Some(&Self::TEMPT_RANGE),
            32 => Some(&Self::WATER_MOVEMENT_EFFICIENCY),
            34 => Some(&Self::WAYPOINT_RECEIVE_RANGE),
            33 => Some(&Self::WAYPOINT_TRANSMIT_RANGE),
            _ => None,
        }
    }
    #[doc = r" Get all attributes as a slice."]
    pub fn all() -> &'static [&'static Self] {
        &[
            &Self::ARMOR,
            &Self::ARMOR_TOUGHNESS,
            &Self::ATTACK_DAMAGE,
            &Self::ATTACK_KNOCKBACK,
            &Self::ATTACK_SPEED,
            &Self::BLOCK_BREAK_SPEED,
            &Self::BLOCK_INTERACTION_RANGE,
            &Self::BURNING_TIME,
            &Self::CAMERA_DISTANCE,
            &Self::ENTITY_INTERACTION_RANGE,
            &Self::EXPLOSION_KNOCKBACK_RESISTANCE,
            &Self::FALL_DAMAGE_MULTIPLIER,
            &Self::FLYING_SPEED,
            &Self::FOLLOW_RANGE,
            &Self::GRAVITY,
            &Self::JUMP_STRENGTH,
            &Self::KNOCKBACK_RESISTANCE,
            &Self::LUCK,
            &Self::MAX_ABSORPTION,
            &Self::MAX_HEALTH,
            &Self::MINING_EFFICIENCY,
            &Self::MOVEMENT_EFFICIENCY,
            &Self::MOVEMENT_SPEED,
            &Self::OXYGEN_BONUS,
            &Self::SAFE_FALL_DISTANCE,
            &Self::SCALE,
            &Self::SNEAKING_SPEED,
            &Self::SPAWN_REINFORCEMENTS,
            &Self::STEP_HEIGHT,
            &Self::SUBMERGED_MINING_SPEED,
            &Self::SWEEPING_DAMAGE_RATIO,
            &Self::TEMPT_RANGE,
            &Self::WATER_MOVEMENT_EFFICIENCY,
            &Self::WAYPOINT_RECEIVE_RANGE,
            &Self::WAYPOINT_TRANSMIT_RANGE,
        ]
    }
}
