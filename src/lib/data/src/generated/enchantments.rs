#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Enchantment {
    pub id: u16,
    pub name: &'static str,
    pub description: &'static str,
    pub min_cost: Cost,
    pub max_cost: Cost,
    pub anvil_cost: u8,
    pub slots: &'static [EnchantmentSlot],
    pub supported_items: &'static str,
    pub weight: u8,
    pub max_level: u8,
    pub exclusive_set: Option<&'static str>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cost {
    pub base: f32,
    pub per_level_above_first: f32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnchantmentSlot {
    MAINHAND,
    OFFHAND,
    HEAD,
    CHEST,
    LEGS,
    FEET,
    ARMOR,
    ANY,
    HAND,
}
impl Enchantment {
    pub const AQUA_AFFINITY: Enchantment = Enchantment {
        id: 0,
        name: "minecraft:aqua_affinity",
        description: "enchantment.minecraft.aqua_affinity",
        min_cost: Cost {
            base: 1.0,
            per_level_above_first: 0.0,
        },
        max_cost: Cost {
            base: 41.0,
            per_level_above_first: 0.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::HEAD],
        supported_items: "#minecraft:enchantable/head_armor",
        weight: 2,
        max_level: 1,
        exclusive_set: None,
    };
    pub const BANE_OF_ARTHROPODS: Enchantment = Enchantment {
        id: 1,
        name: "minecraft:bane_of_arthropods",
        description: "enchantment.minecraft.bane_of_arthropods",
        min_cost: Cost {
            base: 5.0,
            per_level_above_first: 8.0,
        },
        max_cost: Cost {
            base: 25.0,
            per_level_above_first: 8.0,
        },
        anvil_cost: 2,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/weapon",
        weight: 5,
        max_level: 5,
        exclusive_set: Some("#minecraft:exclusive_set/damage"),
    };
    pub const BINDING_CURSE: Enchantment = Enchantment {
        id: 2,
        name: "minecraft:binding_curse",
        description: "enchantment.minecraft.binding_curse",
        min_cost: Cost {
            base: 25.0,
            per_level_above_first: 0.0,
        },
        max_cost: Cost {
            base: 50.0,
            per_level_above_first: 0.0,
        },
        anvil_cost: 8,
        slots: &[EnchantmentSlot::ARMOR],
        supported_items: "#minecraft:enchantable/equippable",
        weight: 1,
        max_level: 1,
        exclusive_set: None,
    };
    pub const BLAST_PROTECTION: Enchantment = Enchantment {
        id: 3,
        name: "minecraft:blast_protection",
        description: "enchantment.minecraft.blast_protection",
        min_cost: Cost {
            base: 5.0,
            per_level_above_first: 8.0,
        },
        max_cost: Cost {
            base: 13.0,
            per_level_above_first: 8.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::ARMOR],
        supported_items: "#minecraft:enchantable/armor",
        weight: 2,
        max_level: 4,
        exclusive_set: Some("#minecraft:exclusive_set/armor"),
    };
    pub const BREACH: Enchantment = Enchantment {
        id: 4,
        name: "minecraft:breach",
        description: "enchantment.minecraft.breach",
        min_cost: Cost {
            base: 15.0,
            per_level_above_first: 9.0,
        },
        max_cost: Cost {
            base: 65.0,
            per_level_above_first: 9.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/mace",
        weight: 2,
        max_level: 4,
        exclusive_set: Some("#minecraft:exclusive_set/damage"),
    };
    pub const CHANNELING: Enchantment = Enchantment {
        id: 5,
        name: "minecraft:channeling",
        description: "enchantment.minecraft.channeling",
        min_cost: Cost {
            base: 25.0,
            per_level_above_first: 0.0,
        },
        max_cost: Cost {
            base: 50.0,
            per_level_above_first: 0.0,
        },
        anvil_cost: 8,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/trident",
        weight: 1,
        max_level: 1,
        exclusive_set: None,
    };
    pub const DENSITY: Enchantment = Enchantment {
        id: 6,
        name: "minecraft:density",
        description: "enchantment.minecraft.density",
        min_cost: Cost {
            base: 5.0,
            per_level_above_first: 8.0,
        },
        max_cost: Cost {
            base: 25.0,
            per_level_above_first: 8.0,
        },
        anvil_cost: 2,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/mace",
        weight: 5,
        max_level: 5,
        exclusive_set: Some("#minecraft:exclusive_set/damage"),
    };
    pub const DEPTH_STRIDER: Enchantment = Enchantment {
        id: 7,
        name: "minecraft:depth_strider",
        description: "enchantment.minecraft.depth_strider",
        min_cost: Cost {
            base: 10.0,
            per_level_above_first: 10.0,
        },
        max_cost: Cost {
            base: 25.0,
            per_level_above_first: 10.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::FEET],
        supported_items: "#minecraft:enchantable/foot_armor",
        weight: 2,
        max_level: 3,
        exclusive_set: Some("#minecraft:exclusive_set/boots"),
    };
    pub const EFFICIENCY: Enchantment = Enchantment {
        id: 8,
        name: "minecraft:efficiency",
        description: "enchantment.minecraft.efficiency",
        min_cost: Cost {
            base: 1.0,
            per_level_above_first: 10.0,
        },
        max_cost: Cost {
            base: 51.0,
            per_level_above_first: 10.0,
        },
        anvil_cost: 1,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/mining",
        weight: 10,
        max_level: 5,
        exclusive_set: None,
    };
    pub const FEATHER_FALLING: Enchantment = Enchantment {
        id: 9,
        name: "minecraft:feather_falling",
        description: "enchantment.minecraft.feather_falling",
        min_cost: Cost {
            base: 5.0,
            per_level_above_first: 6.0,
        },
        max_cost: Cost {
            base: 11.0,
            per_level_above_first: 6.0,
        },
        anvil_cost: 2,
        slots: &[EnchantmentSlot::ARMOR],
        supported_items: "#minecraft:enchantable/foot_armor",
        weight: 5,
        max_level: 4,
        exclusive_set: None,
    };
    pub const FIRE_ASPECT: Enchantment = Enchantment {
        id: 10,
        name: "minecraft:fire_aspect",
        description: "enchantment.minecraft.fire_aspect",
        min_cost: Cost {
            base: 10.0,
            per_level_above_first: 20.0,
        },
        max_cost: Cost {
            base: 60.0,
            per_level_above_first: 20.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/fire_aspect",
        weight: 2,
        max_level: 2,
        exclusive_set: None,
    };
    pub const FIRE_PROTECTION: Enchantment = Enchantment {
        id: 11,
        name: "minecraft:fire_protection",
        description: "enchantment.minecraft.fire_protection",
        min_cost: Cost {
            base: 10.0,
            per_level_above_first: 8.0,
        },
        max_cost: Cost {
            base: 18.0,
            per_level_above_first: 8.0,
        },
        anvil_cost: 2,
        slots: &[EnchantmentSlot::ARMOR],
        supported_items: "#minecraft:enchantable/armor",
        weight: 5,
        max_level: 4,
        exclusive_set: Some("#minecraft:exclusive_set/armor"),
    };
    pub const FLAME: Enchantment = Enchantment {
        id: 12,
        name: "minecraft:flame",
        description: "enchantment.minecraft.flame",
        min_cost: Cost {
            base: 20.0,
            per_level_above_first: 0.0,
        },
        max_cost: Cost {
            base: 50.0,
            per_level_above_first: 0.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/bow",
        weight: 2,
        max_level: 1,
        exclusive_set: None,
    };
    pub const FORTUNE: Enchantment = Enchantment {
        id: 13,
        name: "minecraft:fortune",
        description: "enchantment.minecraft.fortune",
        min_cost: Cost {
            base: 15.0,
            per_level_above_first: 9.0,
        },
        max_cost: Cost {
            base: 65.0,
            per_level_above_first: 9.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/mining_loot",
        weight: 2,
        max_level: 3,
        exclusive_set: Some("#minecraft:exclusive_set/mining"),
    };
    pub const FROST_WALKER: Enchantment = Enchantment {
        id: 14,
        name: "minecraft:frost_walker",
        description: "enchantment.minecraft.frost_walker",
        min_cost: Cost {
            base: 10.0,
            per_level_above_first: 10.0,
        },
        max_cost: Cost {
            base: 25.0,
            per_level_above_first: 10.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::FEET],
        supported_items: "#minecraft:enchantable/foot_armor",
        weight: 2,
        max_level: 2,
        exclusive_set: Some("#minecraft:exclusive_set/boots"),
    };
    pub const IMPALING: Enchantment = Enchantment {
        id: 15,
        name: "minecraft:impaling",
        description: "enchantment.minecraft.impaling",
        min_cost: Cost {
            base: 1.0,
            per_level_above_first: 8.0,
        },
        max_cost: Cost {
            base: 21.0,
            per_level_above_first: 8.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/trident",
        weight: 2,
        max_level: 5,
        exclusive_set: Some("#minecraft:exclusive_set/damage"),
    };
    pub const INFINITY: Enchantment = Enchantment {
        id: 16,
        name: "minecraft:infinity",
        description: "enchantment.minecraft.infinity",
        min_cost: Cost {
            base: 20.0,
            per_level_above_first: 0.0,
        },
        max_cost: Cost {
            base: 50.0,
            per_level_above_first: 0.0,
        },
        anvil_cost: 8,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/bow",
        weight: 1,
        max_level: 1,
        exclusive_set: Some("#minecraft:exclusive_set/bow"),
    };
    pub const KNOCKBACK: Enchantment = Enchantment {
        id: 17,
        name: "minecraft:knockback",
        description: "enchantment.minecraft.knockback",
        min_cost: Cost {
            base: 5.0,
            per_level_above_first: 20.0,
        },
        max_cost: Cost {
            base: 55.0,
            per_level_above_first: 20.0,
        },
        anvil_cost: 2,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/sword",
        weight: 5,
        max_level: 2,
        exclusive_set: None,
    };
    pub const LOOTING: Enchantment = Enchantment {
        id: 18,
        name: "minecraft:looting",
        description: "enchantment.minecraft.looting",
        min_cost: Cost {
            base: 15.0,
            per_level_above_first: 9.0,
        },
        max_cost: Cost {
            base: 65.0,
            per_level_above_first: 9.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/sword",
        weight: 2,
        max_level: 3,
        exclusive_set: None,
    };
    pub const LOYALTY: Enchantment = Enchantment {
        id: 19,
        name: "minecraft:loyalty",
        description: "enchantment.minecraft.loyalty",
        min_cost: Cost {
            base: 12.0,
            per_level_above_first: 7.0,
        },
        max_cost: Cost {
            base: 50.0,
            per_level_above_first: 0.0,
        },
        anvil_cost: 2,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/trident",
        weight: 5,
        max_level: 3,
        exclusive_set: None,
    };
    pub const LUCK_OF_THE_SEA: Enchantment = Enchantment {
        id: 20,
        name: "minecraft:luck_of_the_sea",
        description: "enchantment.minecraft.luck_of_the_sea",
        min_cost: Cost {
            base: 15.0,
            per_level_above_first: 9.0,
        },
        max_cost: Cost {
            base: 65.0,
            per_level_above_first: 9.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/fishing",
        weight: 2,
        max_level: 3,
        exclusive_set: None,
    };
    pub const LURE: Enchantment = Enchantment {
        id: 21,
        name: "minecraft:lure",
        description: "enchantment.minecraft.lure",
        min_cost: Cost {
            base: 15.0,
            per_level_above_first: 9.0,
        },
        max_cost: Cost {
            base: 65.0,
            per_level_above_first: 9.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/fishing",
        weight: 2,
        max_level: 3,
        exclusive_set: None,
    };
    pub const MENDING: Enchantment = Enchantment {
        id: 22,
        name: "minecraft:mending",
        description: "enchantment.minecraft.mending",
        min_cost: Cost {
            base: 25.0,
            per_level_above_first: 25.0,
        },
        max_cost: Cost {
            base: 75.0,
            per_level_above_first: 25.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::ANY],
        supported_items: "#minecraft:enchantable/durability",
        weight: 2,
        max_level: 1,
        exclusive_set: None,
    };
    pub const MULTISHOT: Enchantment = Enchantment {
        id: 23,
        name: "minecraft:multishot",
        description: "enchantment.minecraft.multishot",
        min_cost: Cost {
            base: 20.0,
            per_level_above_first: 0.0,
        },
        max_cost: Cost {
            base: 50.0,
            per_level_above_first: 0.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/crossbow",
        weight: 2,
        max_level: 1,
        exclusive_set: Some("#minecraft:exclusive_set/crossbow"),
    };
    pub const PIERCING: Enchantment = Enchantment {
        id: 24,
        name: "minecraft:piercing",
        description: "enchantment.minecraft.piercing",
        min_cost: Cost {
            base: 1.0,
            per_level_above_first: 10.0,
        },
        max_cost: Cost {
            base: 50.0,
            per_level_above_first: 0.0,
        },
        anvil_cost: 1,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/crossbow",
        weight: 10,
        max_level: 4,
        exclusive_set: Some("#minecraft:exclusive_set/crossbow"),
    };
    pub const POWER: Enchantment = Enchantment {
        id: 25,
        name: "minecraft:power",
        description: "enchantment.minecraft.power",
        min_cost: Cost {
            base: 1.0,
            per_level_above_first: 10.0,
        },
        max_cost: Cost {
            base: 16.0,
            per_level_above_first: 10.0,
        },
        anvil_cost: 1,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/bow",
        weight: 10,
        max_level: 5,
        exclusive_set: None,
    };
    pub const PROJECTILE_PROTECTION: Enchantment = Enchantment {
        id: 26,
        name: "minecraft:projectile_protection",
        description: "enchantment.minecraft.projectile_protection",
        min_cost: Cost {
            base: 3.0,
            per_level_above_first: 6.0,
        },
        max_cost: Cost {
            base: 9.0,
            per_level_above_first: 6.0,
        },
        anvil_cost: 2,
        slots: &[EnchantmentSlot::ARMOR],
        supported_items: "#minecraft:enchantable/armor",
        weight: 5,
        max_level: 4,
        exclusive_set: Some("#minecraft:exclusive_set/armor"),
    };
    pub const PROTECTION: Enchantment = Enchantment {
        id: 27,
        name: "minecraft:protection",
        description: "enchantment.minecraft.protection",
        min_cost: Cost {
            base: 1.0,
            per_level_above_first: 11.0,
        },
        max_cost: Cost {
            base: 12.0,
            per_level_above_first: 11.0,
        },
        anvil_cost: 1,
        slots: &[EnchantmentSlot::ARMOR],
        supported_items: "#minecraft:enchantable/armor",
        weight: 10,
        max_level: 4,
        exclusive_set: Some("#minecraft:exclusive_set/armor"),
    };
    pub const PUNCH: Enchantment = Enchantment {
        id: 28,
        name: "minecraft:punch",
        description: "enchantment.minecraft.punch",
        min_cost: Cost {
            base: 12.0,
            per_level_above_first: 20.0,
        },
        max_cost: Cost {
            base: 37.0,
            per_level_above_first: 20.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/bow",
        weight: 2,
        max_level: 2,
        exclusive_set: None,
    };
    pub const QUICK_CHARGE: Enchantment = Enchantment {
        id: 29,
        name: "minecraft:quick_charge",
        description: "enchantment.minecraft.quick_charge",
        min_cost: Cost {
            base: 12.0,
            per_level_above_first: 20.0,
        },
        max_cost: Cost {
            base: 50.0,
            per_level_above_first: 0.0,
        },
        anvil_cost: 2,
        slots: &[EnchantmentSlot::MAINHAND, EnchantmentSlot::OFFHAND],
        supported_items: "#minecraft:enchantable/crossbow",
        weight: 5,
        max_level: 3,
        exclusive_set: None,
    };
    pub const RESPIRATION: Enchantment = Enchantment {
        id: 30,
        name: "minecraft:respiration",
        description: "enchantment.minecraft.respiration",
        min_cost: Cost {
            base: 10.0,
            per_level_above_first: 10.0,
        },
        max_cost: Cost {
            base: 40.0,
            per_level_above_first: 10.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::HEAD],
        supported_items: "#minecraft:enchantable/head_armor",
        weight: 2,
        max_level: 3,
        exclusive_set: None,
    };
    pub const RIPTIDE: Enchantment = Enchantment {
        id: 31,
        name: "minecraft:riptide",
        description: "enchantment.minecraft.riptide",
        min_cost: Cost {
            base: 17.0,
            per_level_above_first: 7.0,
        },
        max_cost: Cost {
            base: 50.0,
            per_level_above_first: 0.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::HAND],
        supported_items: "#minecraft:enchantable/trident",
        weight: 2,
        max_level: 3,
        exclusive_set: Some("#minecraft:exclusive_set/riptide"),
    };
    pub const SHARPNESS: Enchantment = Enchantment {
        id: 32,
        name: "minecraft:sharpness",
        description: "enchantment.minecraft.sharpness",
        min_cost: Cost {
            base: 1.0,
            per_level_above_first: 11.0,
        },
        max_cost: Cost {
            base: 21.0,
            per_level_above_first: 11.0,
        },
        anvil_cost: 1,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/sharp_weapon",
        weight: 10,
        max_level: 5,
        exclusive_set: Some("#minecraft:exclusive_set/damage"),
    };
    pub const SILK_TOUCH: Enchantment = Enchantment {
        id: 33,
        name: "minecraft:silk_touch",
        description: "enchantment.minecraft.silk_touch",
        min_cost: Cost {
            base: 15.0,
            per_level_above_first: 0.0,
        },
        max_cost: Cost {
            base: 65.0,
            per_level_above_first: 0.0,
        },
        anvil_cost: 8,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/mining_loot",
        weight: 1,
        max_level: 1,
        exclusive_set: Some("#minecraft:exclusive_set/mining"),
    };
    pub const SMITE: Enchantment = Enchantment {
        id: 34,
        name: "minecraft:smite",
        description: "enchantment.minecraft.smite",
        min_cost: Cost {
            base: 5.0,
            per_level_above_first: 8.0,
        },
        max_cost: Cost {
            base: 25.0,
            per_level_above_first: 8.0,
        },
        anvil_cost: 2,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/weapon",
        weight: 5,
        max_level: 5,
        exclusive_set: Some("#minecraft:exclusive_set/damage"),
    };
    pub const SOUL_SPEED: Enchantment = Enchantment {
        id: 35,
        name: "minecraft:soul_speed",
        description: "enchantment.minecraft.soul_speed",
        min_cost: Cost {
            base: 10.0,
            per_level_above_first: 10.0,
        },
        max_cost: Cost {
            base: 25.0,
            per_level_above_first: 10.0,
        },
        anvil_cost: 8,
        slots: &[EnchantmentSlot::FEET],
        supported_items: "#minecraft:enchantable/foot_armor",
        weight: 1,
        max_level: 3,
        exclusive_set: None,
    };
    pub const SWEEPING_EDGE: Enchantment = Enchantment {
        id: 36,
        name: "minecraft:sweeping_edge",
        description: "enchantment.minecraft.sweeping_edge",
        min_cost: Cost {
            base: 5.0,
            per_level_above_first: 9.0,
        },
        max_cost: Cost {
            base: 20.0,
            per_level_above_first: 9.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/sword",
        weight: 2,
        max_level: 3,
        exclusive_set: None,
    };
    pub const SWIFT_SNEAK: Enchantment = Enchantment {
        id: 37,
        name: "minecraft:swift_sneak",
        description: "enchantment.minecraft.swift_sneak",
        min_cost: Cost {
            base: 25.0,
            per_level_above_first: 25.0,
        },
        max_cost: Cost {
            base: 75.0,
            per_level_above_first: 25.0,
        },
        anvil_cost: 8,
        slots: &[EnchantmentSlot::LEGS],
        supported_items: "#minecraft:enchantable/leg_armor",
        weight: 1,
        max_level: 3,
        exclusive_set: None,
    };
    pub const THORNS: Enchantment = Enchantment {
        id: 38,
        name: "minecraft:thorns",
        description: "enchantment.minecraft.thorns",
        min_cost: Cost {
            base: 10.0,
            per_level_above_first: 20.0,
        },
        max_cost: Cost {
            base: 60.0,
            per_level_above_first: 20.0,
        },
        anvil_cost: 8,
        slots: &[EnchantmentSlot::ANY],
        supported_items: "#minecraft:enchantable/armor",
        weight: 1,
        max_level: 3,
        exclusive_set: None,
    };
    pub const UNBREAKING: Enchantment = Enchantment {
        id: 39,
        name: "minecraft:unbreaking",
        description: "enchantment.minecraft.unbreaking",
        min_cost: Cost {
            base: 5.0,
            per_level_above_first: 8.0,
        },
        max_cost: Cost {
            base: 55.0,
            per_level_above_first: 8.0,
        },
        anvil_cost: 2,
        slots: &[EnchantmentSlot::ANY],
        supported_items: "#minecraft:enchantable/durability",
        weight: 5,
        max_level: 3,
        exclusive_set: None,
    };
    pub const VANISHING_CURSE: Enchantment = Enchantment {
        id: 40,
        name: "minecraft:vanishing_curse",
        description: "enchantment.minecraft.vanishing_curse",
        min_cost: Cost {
            base: 25.0,
            per_level_above_first: 0.0,
        },
        max_cost: Cost {
            base: 50.0,
            per_level_above_first: 0.0,
        },
        anvil_cost: 8,
        slots: &[EnchantmentSlot::ANY],
        supported_items: "#minecraft:enchantable/vanishing",
        weight: 1,
        max_level: 1,
        exclusive_set: None,
    };
    pub const WIND_BURST: Enchantment = Enchantment {
        id: 41,
        name: "minecraft:wind_burst",
        description: "enchantment.minecraft.wind_burst",
        min_cost: Cost {
            base: 15.0,
            per_level_above_first: 9.0,
        },
        max_cost: Cost {
            base: 65.0,
            per_level_above_first: 9.0,
        },
        anvil_cost: 4,
        slots: &[EnchantmentSlot::MAINHAND],
        supported_items: "#minecraft:enchantable/mace",
        weight: 2,
        max_level: 3,
        exclusive_set: None,
    };
    #[doc = r" Try to parse an `Enchantment` from a resource location string."]
    pub fn from_name(name: &str) -> Option<&'static Self> {
        let name = name.strip_prefix("minecraft:").unwrap_or(name);
        match name {
            "minecraft:aqua_affinity" => Some(&Self::AQUA_AFFINITY),
            "minecraft:bane_of_arthropods" => Some(&Self::BANE_OF_ARTHROPODS),
            "minecraft:binding_curse" => Some(&Self::BINDING_CURSE),
            "minecraft:blast_protection" => Some(&Self::BLAST_PROTECTION),
            "minecraft:breach" => Some(&Self::BREACH),
            "minecraft:channeling" => Some(&Self::CHANNELING),
            "minecraft:density" => Some(&Self::DENSITY),
            "minecraft:depth_strider" => Some(&Self::DEPTH_STRIDER),
            "minecraft:efficiency" => Some(&Self::EFFICIENCY),
            "minecraft:feather_falling" => Some(&Self::FEATHER_FALLING),
            "minecraft:fire_aspect" => Some(&Self::FIRE_ASPECT),
            "minecraft:fire_protection" => Some(&Self::FIRE_PROTECTION),
            "minecraft:flame" => Some(&Self::FLAME),
            "minecraft:fortune" => Some(&Self::FORTUNE),
            "minecraft:frost_walker" => Some(&Self::FROST_WALKER),
            "minecraft:impaling" => Some(&Self::IMPALING),
            "minecraft:infinity" => Some(&Self::INFINITY),
            "minecraft:knockback" => Some(&Self::KNOCKBACK),
            "minecraft:looting" => Some(&Self::LOOTING),
            "minecraft:loyalty" => Some(&Self::LOYALTY),
            "minecraft:luck_of_the_sea" => Some(&Self::LUCK_OF_THE_SEA),
            "minecraft:lure" => Some(&Self::LURE),
            "minecraft:mending" => Some(&Self::MENDING),
            "minecraft:multishot" => Some(&Self::MULTISHOT),
            "minecraft:piercing" => Some(&Self::PIERCING),
            "minecraft:power" => Some(&Self::POWER),
            "minecraft:projectile_protection" => Some(&Self::PROJECTILE_PROTECTION),
            "minecraft:protection" => Some(&Self::PROTECTION),
            "minecraft:punch" => Some(&Self::PUNCH),
            "minecraft:quick_charge" => Some(&Self::QUICK_CHARGE),
            "minecraft:respiration" => Some(&Self::RESPIRATION),
            "minecraft:riptide" => Some(&Self::RIPTIDE),
            "minecraft:sharpness" => Some(&Self::SHARPNESS),
            "minecraft:silk_touch" => Some(&Self::SILK_TOUCH),
            "minecraft:smite" => Some(&Self::SMITE),
            "minecraft:soul_speed" => Some(&Self::SOUL_SPEED),
            "minecraft:sweeping_edge" => Some(&Self::SWEEPING_EDGE),
            "minecraft:swift_sneak" => Some(&Self::SWIFT_SNEAK),
            "minecraft:thorns" => Some(&Self::THORNS),
            "minecraft:unbreaking" => Some(&Self::UNBREAKING),
            "minecraft:vanishing_curse" => Some(&Self::VANISHING_CURSE),
            "minecraft:wind_burst" => Some(&Self::WIND_BURST),
            _ => None,
        }
    }
    #[doc = r" Try to get an `Enchantment` from its ID."]
    pub const fn from_id(id: u16) -> Option<&'static Self> {
        match id {
            0 => Some(&Self::AQUA_AFFINITY),
            1 => Some(&Self::BANE_OF_ARTHROPODS),
            2 => Some(&Self::BINDING_CURSE),
            3 => Some(&Self::BLAST_PROTECTION),
            4 => Some(&Self::BREACH),
            5 => Some(&Self::CHANNELING),
            6 => Some(&Self::DENSITY),
            7 => Some(&Self::DEPTH_STRIDER),
            8 => Some(&Self::EFFICIENCY),
            9 => Some(&Self::FEATHER_FALLING),
            10 => Some(&Self::FIRE_ASPECT),
            11 => Some(&Self::FIRE_PROTECTION),
            12 => Some(&Self::FLAME),
            13 => Some(&Self::FORTUNE),
            14 => Some(&Self::FROST_WALKER),
            15 => Some(&Self::IMPALING),
            16 => Some(&Self::INFINITY),
            17 => Some(&Self::KNOCKBACK),
            18 => Some(&Self::LOOTING),
            19 => Some(&Self::LOYALTY),
            20 => Some(&Self::LUCK_OF_THE_SEA),
            21 => Some(&Self::LURE),
            22 => Some(&Self::MENDING),
            23 => Some(&Self::MULTISHOT),
            24 => Some(&Self::PIERCING),
            25 => Some(&Self::POWER),
            26 => Some(&Self::PROJECTILE_PROTECTION),
            27 => Some(&Self::PROTECTION),
            28 => Some(&Self::PUNCH),
            29 => Some(&Self::QUICK_CHARGE),
            30 => Some(&Self::RESPIRATION),
            31 => Some(&Self::RIPTIDE),
            32 => Some(&Self::SHARPNESS),
            33 => Some(&Self::SILK_TOUCH),
            34 => Some(&Self::SMITE),
            35 => Some(&Self::SOUL_SPEED),
            36 => Some(&Self::SWEEPING_EDGE),
            37 => Some(&Self::SWIFT_SNEAK),
            38 => Some(&Self::THORNS),
            39 => Some(&Self::UNBREAKING),
            40 => Some(&Self::VANISHING_CURSE),
            41 => Some(&Self::WIND_BURST),
            _ => None,
        }
    }
    #[doc = r" Calculate the minimum cost for this enchantment at the given level."]
    pub const fn min_cost(&self, level: u8) -> f32 {
        self.min_cost.base + self.min_cost.per_level_above_first * (level - 1) as f32
    }
    #[doc = r" Calculate the maximum cost for this enchantment at the given level."]
    pub const fn max_cost(&self, level: u8) -> f32 {
        self.max_cost.base + self.max_cost.per_level_above_first * (level - 1) as f32
    }
}
