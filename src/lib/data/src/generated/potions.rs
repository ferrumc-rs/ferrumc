#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Potion {
    pub id: u16,
    pub name: &'static str,
    pub base_name: &'static str,
    pub effects: &'static [PotionEffect],
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PotionEffect {
    pub effect_type: &'static str,
    pub duration: i32,
    pub amplifier: u8,
    pub ambient: bool,
    pub show_particles: bool,
    pub show_icon: bool,
}
impl Potion {
    pub const AWKWARD: Potion = Potion {
        id: 3,
        name: "awkward",
        base_name: "awkward",
        effects: &[],
    };
    pub const FIRE_RESISTANCE: Potion = Potion {
        id: 11,
        name: "fire_resistance",
        base_name: "fire_resistance",
        effects: &[PotionEffect {
            effect_type: "minecraft:fire_resistance",
            duration: 3600,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const HARMING: Potion = Potion {
        id: 26,
        name: "harming",
        base_name: "harming",
        effects: &[PotionEffect {
            effect_type: "minecraft:instant_damage",
            duration: 1,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const HEALING: Potion = Potion {
        id: 24,
        name: "healing",
        base_name: "healing",
        effects: &[PotionEffect {
            effect_type: "minecraft:instant_health",
            duration: 1,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const INFESTED: Potion = Potion {
        id: 45,
        name: "infested",
        base_name: "infested",
        effects: &[PotionEffect {
            effect_type: "minecraft:infested",
            duration: 3600,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const INVISIBILITY: Potion = Potion {
        id: 6,
        name: "invisibility",
        base_name: "invisibility",
        effects: &[PotionEffect {
            effect_type: "minecraft:invisibility",
            duration: 3600,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const LEAPING: Potion = Potion {
        id: 8,
        name: "leaping",
        base_name: "leaping",
        effects: &[PotionEffect {
            effect_type: "minecraft:jump_boost",
            duration: 3600,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const LONG_FIRE_RESISTANCE: Potion = Potion {
        id: 12,
        name: "long_fire_resistance",
        base_name: "fire_resistance",
        effects: &[PotionEffect {
            effect_type: "minecraft:fire_resistance",
            duration: 9600,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const LONG_INVISIBILITY: Potion = Potion {
        id: 7,
        name: "long_invisibility",
        base_name: "invisibility",
        effects: &[PotionEffect {
            effect_type: "minecraft:invisibility",
            duration: 9600,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const LONG_LEAPING: Potion = Potion {
        id: 9,
        name: "long_leaping",
        base_name: "leaping",
        effects: &[PotionEffect {
            effect_type: "minecraft:jump_boost",
            duration: 9600,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const LONG_NIGHT_VISION: Potion = Potion {
        id: 5,
        name: "long_night_vision",
        base_name: "night_vision",
        effects: &[PotionEffect {
            effect_type: "minecraft:night_vision",
            duration: 9600,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const LONG_POISON: Potion = Potion {
        id: 29,
        name: "long_poison",
        base_name: "poison",
        effects: &[PotionEffect {
            effect_type: "minecraft:poison",
            duration: 1800,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const LONG_REGENERATION: Potion = Potion {
        id: 32,
        name: "long_regeneration",
        base_name: "regeneration",
        effects: &[PotionEffect {
            effect_type: "minecraft:regeneration",
            duration: 1800,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const LONG_SLOW_FALLING: Potion = Potion {
        id: 41,
        name: "long_slow_falling",
        base_name: "slow_falling",
        effects: &[PotionEffect {
            effect_type: "minecraft:slow_falling",
            duration: 4800,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const LONG_SLOWNESS: Potion = Potion {
        id: 17,
        name: "long_slowness",
        base_name: "slowness",
        effects: &[PotionEffect {
            effect_type: "minecraft:slowness",
            duration: 4800,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const LONG_STRENGTH: Potion = Potion {
        id: 35,
        name: "long_strength",
        base_name: "strength",
        effects: &[PotionEffect {
            effect_type: "minecraft:strength",
            duration: 9600,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const LONG_SWIFTNESS: Potion = Potion {
        id: 14,
        name: "long_swiftness",
        base_name: "swiftness",
        effects: &[PotionEffect {
            effect_type: "minecraft:speed",
            duration: 9600,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const LONG_TURTLE_MASTER: Potion = Potion {
        id: 20,
        name: "long_turtle_master",
        base_name: "turtle_master",
        effects: &[
            PotionEffect {
                effect_type: "minecraft:slowness",
                duration: 800,
                amplifier: 3,
                ambient: false,
                show_particles: true,
                show_icon: true,
            },
            PotionEffect {
                effect_type: "minecraft:resistance",
                duration: 800,
                amplifier: 2,
                ambient: false,
                show_particles: true,
                show_icon: true,
            },
        ],
    };
    pub const LONG_WATER_BREATHING: Potion = Potion {
        id: 23,
        name: "long_water_breathing",
        base_name: "water_breathing",
        effects: &[PotionEffect {
            effect_type: "minecraft:water_breathing",
            duration: 9600,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const LONG_WEAKNESS: Potion = Potion {
        id: 38,
        name: "long_weakness",
        base_name: "weakness",
        effects: &[PotionEffect {
            effect_type: "minecraft:weakness",
            duration: 4800,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const LUCK: Potion = Potion {
        id: 39,
        name: "luck",
        base_name: "luck",
        effects: &[PotionEffect {
            effect_type: "minecraft:luck",
            duration: 6000,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const MUNDANE: Potion = Potion {
        id: 1,
        name: "mundane",
        base_name: "mundane",
        effects: &[],
    };
    pub const NIGHT_VISION: Potion = Potion {
        id: 4,
        name: "night_vision",
        base_name: "night_vision",
        effects: &[PotionEffect {
            effect_type: "minecraft:night_vision",
            duration: 3600,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const OOZING: Potion = Potion {
        id: 44,
        name: "oozing",
        base_name: "oozing",
        effects: &[PotionEffect {
            effect_type: "minecraft:oozing",
            duration: 3600,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const POISON: Potion = Potion {
        id: 28,
        name: "poison",
        base_name: "poison",
        effects: &[PotionEffect {
            effect_type: "minecraft:poison",
            duration: 900,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const REGENERATION: Potion = Potion {
        id: 31,
        name: "regeneration",
        base_name: "regeneration",
        effects: &[PotionEffect {
            effect_type: "minecraft:regeneration",
            duration: 900,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const SLOW_FALLING: Potion = Potion {
        id: 40,
        name: "slow_falling",
        base_name: "slow_falling",
        effects: &[PotionEffect {
            effect_type: "minecraft:slow_falling",
            duration: 1800,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const SLOWNESS: Potion = Potion {
        id: 16,
        name: "slowness",
        base_name: "slowness",
        effects: &[PotionEffect {
            effect_type: "minecraft:slowness",
            duration: 1800,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const STRENGTH: Potion = Potion {
        id: 34,
        name: "strength",
        base_name: "strength",
        effects: &[PotionEffect {
            effect_type: "minecraft:strength",
            duration: 3600,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const STRONG_HARMING: Potion = Potion {
        id: 27,
        name: "strong_harming",
        base_name: "harming",
        effects: &[PotionEffect {
            effect_type: "minecraft:instant_damage",
            duration: 1,
            amplifier: 1,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const STRONG_HEALING: Potion = Potion {
        id: 25,
        name: "strong_healing",
        base_name: "healing",
        effects: &[PotionEffect {
            effect_type: "minecraft:instant_health",
            duration: 1,
            amplifier: 1,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const STRONG_LEAPING: Potion = Potion {
        id: 10,
        name: "strong_leaping",
        base_name: "leaping",
        effects: &[PotionEffect {
            effect_type: "minecraft:jump_boost",
            duration: 1800,
            amplifier: 1,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const STRONG_POISON: Potion = Potion {
        id: 30,
        name: "strong_poison",
        base_name: "poison",
        effects: &[PotionEffect {
            effect_type: "minecraft:poison",
            duration: 432,
            amplifier: 1,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const STRONG_REGENERATION: Potion = Potion {
        id: 33,
        name: "strong_regeneration",
        base_name: "regeneration",
        effects: &[PotionEffect {
            effect_type: "minecraft:regeneration",
            duration: 450,
            amplifier: 1,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const STRONG_SLOWNESS: Potion = Potion {
        id: 18,
        name: "strong_slowness",
        base_name: "slowness",
        effects: &[PotionEffect {
            effect_type: "minecraft:slowness",
            duration: 400,
            amplifier: 3,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const STRONG_STRENGTH: Potion = Potion {
        id: 36,
        name: "strong_strength",
        base_name: "strength",
        effects: &[PotionEffect {
            effect_type: "minecraft:strength",
            duration: 1800,
            amplifier: 1,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const STRONG_SWIFTNESS: Potion = Potion {
        id: 15,
        name: "strong_swiftness",
        base_name: "swiftness",
        effects: &[PotionEffect {
            effect_type: "minecraft:speed",
            duration: 1800,
            amplifier: 1,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const STRONG_TURTLE_MASTER: Potion = Potion {
        id: 21,
        name: "strong_turtle_master",
        base_name: "turtle_master",
        effects: &[
            PotionEffect {
                effect_type: "minecraft:slowness",
                duration: 400,
                amplifier: 5,
                ambient: false,
                show_particles: true,
                show_icon: true,
            },
            PotionEffect {
                effect_type: "minecraft:resistance",
                duration: 400,
                amplifier: 3,
                ambient: false,
                show_particles: true,
                show_icon: true,
            },
        ],
    };
    pub const SWIFTNESS: Potion = Potion {
        id: 13,
        name: "swiftness",
        base_name: "swiftness",
        effects: &[PotionEffect {
            effect_type: "minecraft:speed",
            duration: 3600,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const THICK: Potion = Potion {
        id: 2,
        name: "thick",
        base_name: "thick",
        effects: &[],
    };
    pub const TURTLE_MASTER: Potion = Potion {
        id: 19,
        name: "turtle_master",
        base_name: "turtle_master",
        effects: &[
            PotionEffect {
                effect_type: "minecraft:slowness",
                duration: 400,
                amplifier: 3,
                ambient: false,
                show_particles: true,
                show_icon: true,
            },
            PotionEffect {
                effect_type: "minecraft:resistance",
                duration: 400,
                amplifier: 2,
                ambient: false,
                show_particles: true,
                show_icon: true,
            },
        ],
    };
    pub const WATER: Potion = Potion {
        id: 0,
        name: "water",
        base_name: "water",
        effects: &[],
    };
    pub const WATER_BREATHING: Potion = Potion {
        id: 22,
        name: "water_breathing",
        base_name: "water_breathing",
        effects: &[PotionEffect {
            effect_type: "minecraft:water_breathing",
            duration: 3600,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const WEAKNESS: Potion = Potion {
        id: 37,
        name: "weakness",
        base_name: "weakness",
        effects: &[PotionEffect {
            effect_type: "minecraft:weakness",
            duration: 1800,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const WEAVING: Potion = Potion {
        id: 43,
        name: "weaving",
        base_name: "weaving",
        effects: &[PotionEffect {
            effect_type: "minecraft:weaving",
            duration: 3600,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    pub const WIND_CHARGED: Potion = Potion {
        id: 42,
        name: "wind_charged",
        base_name: "wind_charged",
        effects: &[PotionEffect {
            effect_type: "minecraft:wind_charged",
            duration: 3600,
            amplifier: 0,
            ambient: false,
            show_particles: true,
            show_icon: true,
        }],
    };
    #[doc = r" Try to parse a `Potion` from a resource location string."]
    pub const fn try_from_name(name: &str) -> Option<&'static Self> {
        let name = crate::helpers::strip_prefix_or_self(name, "minecraft:");
        match name {
            "awkward" => Some(&Self::AWKWARD),
            "fire_resistance" => Some(&Self::FIRE_RESISTANCE),
            "harming" => Some(&Self::HARMING),
            "healing" => Some(&Self::HEALING),
            "infested" => Some(&Self::INFESTED),
            "invisibility" => Some(&Self::INVISIBILITY),
            "leaping" => Some(&Self::LEAPING),
            "long_fire_resistance" => Some(&Self::LONG_FIRE_RESISTANCE),
            "long_invisibility" => Some(&Self::LONG_INVISIBILITY),
            "long_leaping" => Some(&Self::LONG_LEAPING),
            "long_night_vision" => Some(&Self::LONG_NIGHT_VISION),
            "long_poison" => Some(&Self::LONG_POISON),
            "long_regeneration" => Some(&Self::LONG_REGENERATION),
            "long_slow_falling" => Some(&Self::LONG_SLOW_FALLING),
            "long_slowness" => Some(&Self::LONG_SLOWNESS),
            "long_strength" => Some(&Self::LONG_STRENGTH),
            "long_swiftness" => Some(&Self::LONG_SWIFTNESS),
            "long_turtle_master" => Some(&Self::LONG_TURTLE_MASTER),
            "long_water_breathing" => Some(&Self::LONG_WATER_BREATHING),
            "long_weakness" => Some(&Self::LONG_WEAKNESS),
            "luck" => Some(&Self::LUCK),
            "mundane" => Some(&Self::MUNDANE),
            "night_vision" => Some(&Self::NIGHT_VISION),
            "oozing" => Some(&Self::OOZING),
            "poison" => Some(&Self::POISON),
            "regeneration" => Some(&Self::REGENERATION),
            "slow_falling" => Some(&Self::SLOW_FALLING),
            "slowness" => Some(&Self::SLOWNESS),
            "strength" => Some(&Self::STRENGTH),
            "strong_harming" => Some(&Self::STRONG_HARMING),
            "strong_healing" => Some(&Self::STRONG_HEALING),
            "strong_leaping" => Some(&Self::STRONG_LEAPING),
            "strong_poison" => Some(&Self::STRONG_POISON),
            "strong_regeneration" => Some(&Self::STRONG_REGENERATION),
            "strong_slowness" => Some(&Self::STRONG_SLOWNESS),
            "strong_strength" => Some(&Self::STRONG_STRENGTH),
            "strong_swiftness" => Some(&Self::STRONG_SWIFTNESS),
            "strong_turtle_master" => Some(&Self::STRONG_TURTLE_MASTER),
            "swiftness" => Some(&Self::SWIFTNESS),
            "thick" => Some(&Self::THICK),
            "turtle_master" => Some(&Self::TURTLE_MASTER),
            "water" => Some(&Self::WATER),
            "water_breathing" => Some(&Self::WATER_BREATHING),
            "weakness" => Some(&Self::WEAKNESS),
            "weaving" => Some(&Self::WEAVING),
            "wind_charged" => Some(&Self::WIND_CHARGED),
            _ => None,
        }
    }
    #[doc = r" Try to get a `Potion` from its ID."]
    pub const fn try_from_id(id: u16) -> Option<&'static Self> {
        match id {
            3 => Some(&Self::AWKWARD),
            11 => Some(&Self::FIRE_RESISTANCE),
            26 => Some(&Self::HARMING),
            24 => Some(&Self::HEALING),
            45 => Some(&Self::INFESTED),
            6 => Some(&Self::INVISIBILITY),
            8 => Some(&Self::LEAPING),
            12 => Some(&Self::LONG_FIRE_RESISTANCE),
            7 => Some(&Self::LONG_INVISIBILITY),
            9 => Some(&Self::LONG_LEAPING),
            5 => Some(&Self::LONG_NIGHT_VISION),
            29 => Some(&Self::LONG_POISON),
            32 => Some(&Self::LONG_REGENERATION),
            41 => Some(&Self::LONG_SLOW_FALLING),
            17 => Some(&Self::LONG_SLOWNESS),
            35 => Some(&Self::LONG_STRENGTH),
            14 => Some(&Self::LONG_SWIFTNESS),
            20 => Some(&Self::LONG_TURTLE_MASTER),
            23 => Some(&Self::LONG_WATER_BREATHING),
            38 => Some(&Self::LONG_WEAKNESS),
            39 => Some(&Self::LUCK),
            1 => Some(&Self::MUNDANE),
            4 => Some(&Self::NIGHT_VISION),
            44 => Some(&Self::OOZING),
            28 => Some(&Self::POISON),
            31 => Some(&Self::REGENERATION),
            40 => Some(&Self::SLOW_FALLING),
            16 => Some(&Self::SLOWNESS),
            34 => Some(&Self::STRENGTH),
            27 => Some(&Self::STRONG_HARMING),
            25 => Some(&Self::STRONG_HEALING),
            10 => Some(&Self::STRONG_LEAPING),
            30 => Some(&Self::STRONG_POISON),
            33 => Some(&Self::STRONG_REGENERATION),
            18 => Some(&Self::STRONG_SLOWNESS),
            36 => Some(&Self::STRONG_STRENGTH),
            15 => Some(&Self::STRONG_SWIFTNESS),
            21 => Some(&Self::STRONG_TURTLE_MASTER),
            13 => Some(&Self::SWIFTNESS),
            2 => Some(&Self::THICK),
            19 => Some(&Self::TURTLE_MASTER),
            0 => Some(&Self::WATER),
            22 => Some(&Self::WATER_BREATHING),
            37 => Some(&Self::WEAKNESS),
            43 => Some(&Self::WEAVING),
            42 => Some(&Self::WIND_CHARGED),
            _ => None,
        }
    }
    #[doc = r" Check if this potion has any effects."]
    pub const fn has_effects(&self) -> bool {
        !self.effects.is_empty()
    }
    #[doc = r" Get the number of effects this potion has."]
    pub const fn effect_count(&self) -> usize {
        self.effects.len()
    }
}
