use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct Recipe {
    pub name: &'static str,
    pub recipe_type: &'static str,
    pub group: Option<&'static str>,
    pub category: Option<&'static str>,
    pub experience: Option<f32>,
    pub cookingtime: Option<u32>,
}
impl Recipe {
    pub const RECIPE_0: Recipe = Recipe {
        name: "recipe_0",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1: Recipe = Recipe {
        name: "recipe_1",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wooden_button"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_2: Recipe = Recipe {
        name: "recipe_2",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("chest_boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_3: Recipe = Recipe {
        name: "recipe_3",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_door"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_4: Recipe = Recipe {
        name: "recipe_4",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_5: Recipe = Recipe {
        name: "recipe_5",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence_gate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_6: Recipe = Recipe {
        name: "recipe_6",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("hanging_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_7: Recipe = Recipe {
        name: "recipe_7",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("planks"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_8: Recipe = Recipe {
        name: "recipe_8",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_pressure_plate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_9: Recipe = Recipe {
        name: "recipe_9",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_10: Recipe = Recipe {
        name: "recipe_10",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_11: Recipe = Recipe {
        name: "recipe_11",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_12: Recipe = Recipe {
        name: "recipe_12",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_trapdoor"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_13: Recipe = Recipe {
        name: "recipe_13",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_14: Recipe = Recipe {
        name: "recipe_14",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_15: Recipe = Recipe {
        name: "recipe_15",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_16: Recipe = Recipe {
        name: "recipe_16",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_17: Recipe = Recipe {
        name: "recipe_17",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_18: Recipe = Recipe {
        name: "recipe_18",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_19: Recipe = Recipe {
        name: "recipe_19",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_20: Recipe = Recipe {
        name: "recipe_20",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_21: Recipe = Recipe {
        name: "recipe_21",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_22: Recipe = Recipe {
        name: "recipe_22",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_23: Recipe = Recipe {
        name: "recipe_23",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_24: Recipe = Recipe {
        name: "recipe_24",
        recipe_type: "minecraft:crafting_special_armordye",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_25: Recipe = Recipe {
        name: "recipe_25",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_26: Recipe = Recipe {
        name: "recipe_26",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_27: Recipe = Recipe {
        name: "recipe_27",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(200),
    };
    pub const RECIPE_28: Recipe = Recipe {
        name: "recipe_28",
        recipe_type: "minecraft:campfire_cooking",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(600),
    };
    pub const RECIPE_29: Recipe = Recipe {
        name: "recipe_29",
        recipe_type: "minecraft:smoking",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(100),
    };
    pub const RECIPE_30: Recipe = Recipe {
        name: "recipe_30",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_31: Recipe = Recipe {
        name: "recipe_31",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wooden_button"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_32: Recipe = Recipe {
        name: "recipe_32",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("chest_boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_33: Recipe = Recipe {
        name: "recipe_33",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_door"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_34: Recipe = Recipe {
        name: "recipe_34",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_35: Recipe = Recipe {
        name: "recipe_35",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence_gate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_36: Recipe = Recipe {
        name: "recipe_36",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("hanging_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_37: Recipe = Recipe {
        name: "recipe_37",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_38: Recipe = Recipe {
        name: "recipe_38",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_39: Recipe = Recipe {
        name: "recipe_39",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_40: Recipe = Recipe {
        name: "recipe_40",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("planks"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_41: Recipe = Recipe {
        name: "recipe_41",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_pressure_plate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_42: Recipe = Recipe {
        name: "recipe_42",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_43: Recipe = Recipe {
        name: "recipe_43",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_44: Recipe = Recipe {
        name: "recipe_44",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_45: Recipe = Recipe {
        name: "recipe_45",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_46: Recipe = Recipe {
        name: "recipe_46",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_trapdoor"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_47: Recipe = Recipe {
        name: "recipe_47",
        recipe_type: "minecraft:crafting_special_bannerduplicate",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_48: Recipe = Recipe {
        name: "recipe_48",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_49: Recipe = Recipe {
        name: "recipe_49",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_50: Recipe = Recipe {
        name: "recipe_50",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_51: Recipe = Recipe {
        name: "recipe_51",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_52: Recipe = Recipe {
        name: "recipe_52",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_53: Recipe = Recipe {
        name: "recipe_53",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wooden_button"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_54: Recipe = Recipe {
        name: "recipe_54",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("chest_boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_55: Recipe = Recipe {
        name: "recipe_55",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_door"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_56: Recipe = Recipe {
        name: "recipe_56",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_57: Recipe = Recipe {
        name: "recipe_57",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence_gate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_58: Recipe = Recipe {
        name: "recipe_58",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("hanging_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_59: Recipe = Recipe {
        name: "recipe_59",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("planks"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_60: Recipe = Recipe {
        name: "recipe_60",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_pressure_plate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_61: Recipe = Recipe {
        name: "recipe_61",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_62: Recipe = Recipe {
        name: "recipe_62",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_63: Recipe = Recipe {
        name: "recipe_63",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_64: Recipe = Recipe {
        name: "recipe_64",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_trapdoor"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_65: Recipe = Recipe {
        name: "recipe_65",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_66: Recipe = Recipe {
        name: "recipe_66",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("banner"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_67: Recipe = Recipe {
        name: "recipe_67",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bed"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_68: Recipe = Recipe {
        name: "recipe_68",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("bundle_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_69: Recipe = Recipe {
        name: "recipe_69",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("dyed_candle"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_70: Recipe = Recipe {
        name: "recipe_70",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("carpet"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_71: Recipe = Recipe {
        name: "recipe_71",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("concrete_powder"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_72: Recipe = Recipe {
        name: "recipe_72",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("black_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_73: Recipe = Recipe {
        name: "recipe_73",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("black_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_74: Recipe = Recipe {
        name: "recipe_74",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_75: Recipe = Recipe {
        name: "recipe_75",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("harness"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_76: Recipe = Recipe {
        name: "recipe_76",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("shulker_box_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_77: Recipe = Recipe {
        name: "recipe_77",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_78: Recipe = Recipe {
        name: "recipe_78",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_79: Recipe = Recipe {
        name: "recipe_79",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_80: Recipe = Recipe {
        name: "recipe_80",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_terracotta"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_81: Recipe = Recipe {
        name: "recipe_81",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_82: Recipe = Recipe {
        name: "recipe_82",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_83: Recipe = Recipe {
        name: "recipe_83",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_84: Recipe = Recipe {
        name: "recipe_84",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_85: Recipe = Recipe {
        name: "recipe_85",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_86: Recipe = Recipe {
        name: "recipe_86",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_87: Recipe = Recipe {
        name: "recipe_87",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_88: Recipe = Recipe {
        name: "recipe_88",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_89: Recipe = Recipe {
        name: "recipe_89",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("banner"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_90: Recipe = Recipe {
        name: "recipe_90",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bed"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_91: Recipe = Recipe {
        name: "recipe_91",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("bundle_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_92: Recipe = Recipe {
        name: "recipe_92",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("dyed_candle"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_93: Recipe = Recipe {
        name: "recipe_93",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("carpet"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_94: Recipe = Recipe {
        name: "recipe_94",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("concrete_powder"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_95: Recipe = Recipe {
        name: "recipe_95",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("blue_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_96: Recipe = Recipe {
        name: "recipe_96",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("blue_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_97: Recipe = Recipe {
        name: "recipe_97",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_98: Recipe = Recipe {
        name: "recipe_98",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("harness"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_99: Recipe = Recipe {
        name: "recipe_99",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_100: Recipe = Recipe {
        name: "recipe_100",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("shulker_box_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_101: Recipe = Recipe {
        name: "recipe_101",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_102: Recipe = Recipe {
        name: "recipe_102",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_103: Recipe = Recipe {
        name: "recipe_103",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_104: Recipe = Recipe {
        name: "recipe_104",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_terracotta"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_105: Recipe = Recipe {
        name: "recipe_105",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_106: Recipe = Recipe {
        name: "recipe_106",
        recipe_type: "minecraft:smithing_trim",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_107: Recipe = Recipe {
        name: "recipe_107",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_108: Recipe = Recipe {
        name: "recipe_108",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("bonemeal"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_109: Recipe = Recipe {
        name: "recipe_109",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("bonemeal"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_110: Recipe = Recipe {
        name: "recipe_110",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_111: Recipe = Recipe {
        name: "recipe_111",
        recipe_type: "minecraft:crafting_special_bookcloning",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_112: Recipe = Recipe {
        name: "recipe_112",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_113: Recipe = Recipe {
        name: "recipe_113",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_114: Recipe = Recipe {
        name: "recipe_114",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_115: Recipe = Recipe {
        name: "recipe_115",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_116: Recipe = Recipe {
        name: "recipe_116",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_117: Recipe = Recipe {
        name: "recipe_117",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_118: Recipe = Recipe {
        name: "recipe_118",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("misc"),
        experience: Some(0.3),
        cookingtime: Some(200),
    };
    pub const RECIPE_119: Recipe = Recipe {
        name: "recipe_119",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_120: Recipe = Recipe {
        name: "recipe_120",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_121: Recipe = Recipe {
        name: "recipe_121",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_122: Recipe = Recipe {
        name: "recipe_122",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_123: Recipe = Recipe {
        name: "recipe_123",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_124: Recipe = Recipe {
        name: "recipe_124",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_125: Recipe = Recipe {
        name: "recipe_125",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_126: Recipe = Recipe {
        name: "recipe_126",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("banner"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_127: Recipe = Recipe {
        name: "recipe_127",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bed"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_128: Recipe = Recipe {
        name: "recipe_128",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("bundle_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_129: Recipe = Recipe {
        name: "recipe_129",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("dyed_candle"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_130: Recipe = Recipe {
        name: "recipe_130",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("carpet"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_131: Recipe = Recipe {
        name: "recipe_131",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("concrete_powder"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_132: Recipe = Recipe {
        name: "recipe_132",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("brown_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_133: Recipe = Recipe {
        name: "recipe_133",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_134: Recipe = Recipe {
        name: "recipe_134",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("harness"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_135: Recipe = Recipe {
        name: "recipe_135",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("shulker_box_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_136: Recipe = Recipe {
        name: "recipe_136",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_137: Recipe = Recipe {
        name: "recipe_137",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_138: Recipe = Recipe {
        name: "recipe_138",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_139: Recipe = Recipe {
        name: "recipe_139",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_terracotta"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_140: Recipe = Recipe {
        name: "recipe_140",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_141: Recipe = Recipe {
        name: "recipe_141",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_142: Recipe = Recipe {
        name: "recipe_142",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_143: Recipe = Recipe {
        name: "recipe_143",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_144: Recipe = Recipe {
        name: "recipe_144",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_145: Recipe = Recipe {
        name: "recipe_145",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_146: Recipe = Recipe {
        name: "recipe_146",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_147: Recipe = Recipe {
        name: "recipe_147",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_148: Recipe = Recipe {
        name: "recipe_148",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_149: Recipe = Recipe {
        name: "recipe_149",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_150: Recipe = Recipe {
        name: "recipe_150",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_151: Recipe = Recipe {
        name: "recipe_151",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("misc"),
        experience: Some(0.15),
        cookingtime: Some(200),
    };
    pub const RECIPE_152: Recipe = Recipe {
        name: "recipe_152",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_153: Recipe = Recipe {
        name: "recipe_153",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wooden_button"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_154: Recipe = Recipe {
        name: "recipe_154",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("chest_boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_155: Recipe = Recipe {
        name: "recipe_155",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_door"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_156: Recipe = Recipe {
        name: "recipe_156",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_157: Recipe = Recipe {
        name: "recipe_157",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence_gate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_158: Recipe = Recipe {
        name: "recipe_158",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("hanging_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_159: Recipe = Recipe {
        name: "recipe_159",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("planks"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_160: Recipe = Recipe {
        name: "recipe_160",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_pressure_plate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_161: Recipe = Recipe {
        name: "recipe_161",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_162: Recipe = Recipe {
        name: "recipe_162",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_163: Recipe = Recipe {
        name: "recipe_163",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_164: Recipe = Recipe {
        name: "recipe_164",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_trapdoor"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_165: Recipe = Recipe {
        name: "recipe_165",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_166: Recipe = Recipe {
        name: "recipe_166",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_167: Recipe = Recipe {
        name: "recipe_167",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_168: Recipe = Recipe {
        name: "recipe_168",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_169: Recipe = Recipe {
        name: "recipe_169",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_170: Recipe = Recipe {
        name: "recipe_170",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_171: Recipe = Recipe {
        name: "recipe_171",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_172: Recipe = Recipe {
        name: "recipe_172",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_173: Recipe = Recipe {
        name: "recipe_173",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_174: Recipe = Recipe {
        name: "recipe_174",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_175: Recipe = Recipe {
        name: "recipe_175",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_176: Recipe = Recipe {
        name: "recipe_176",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_177: Recipe = Recipe {
        name: "recipe_177",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_178: Recipe = Recipe {
        name: "recipe_178",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_179: Recipe = Recipe {
        name: "recipe_179",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_180: Recipe = Recipe {
        name: "recipe_180",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_181: Recipe = Recipe {
        name: "recipe_181",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_182: Recipe = Recipe {
        name: "recipe_182",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_183: Recipe = Recipe {
        name: "recipe_183",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_184: Recipe = Recipe {
        name: "recipe_184",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_185: Recipe = Recipe {
        name: "recipe_185",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_186: Recipe = Recipe {
        name: "recipe_186",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_187: Recipe = Recipe {
        name: "recipe_187",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_188: Recipe = Recipe {
        name: "recipe_188",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_189: Recipe = Recipe {
        name: "recipe_189",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_190: Recipe = Recipe {
        name: "recipe_190",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_191: Recipe = Recipe {
        name: "recipe_191",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_192: Recipe = Recipe {
        name: "recipe_192",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_193: Recipe = Recipe {
        name: "recipe_193",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_194: Recipe = Recipe {
        name: "recipe_194",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_195: Recipe = Recipe {
        name: "recipe_195",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_196: Recipe = Recipe {
        name: "recipe_196",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_197: Recipe = Recipe {
        name: "recipe_197",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_198: Recipe = Recipe {
        name: "recipe_198",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_199: Recipe = Recipe {
        name: "recipe_199",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_200: Recipe = Recipe {
        name: "recipe_200",
        recipe_type: "minecraft:blasting",
        group: Some("coal"),
        category: Some("misc"),
        experience: Some(0.1),
        cookingtime: Some(100),
    };
    pub const RECIPE_201: Recipe = Recipe {
        name: "recipe_201",
        recipe_type: "minecraft:blasting",
        group: Some("coal"),
        category: Some("misc"),
        experience: Some(0.1),
        cookingtime: Some(100),
    };
    pub const RECIPE_202: Recipe = Recipe {
        name: "recipe_202",
        recipe_type: "minecraft:smelting",
        group: Some("coal"),
        category: Some("misc"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_203: Recipe = Recipe {
        name: "recipe_203",
        recipe_type: "minecraft:smelting",
        group: Some("coal"),
        category: Some("misc"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_204: Recipe = Recipe {
        name: "recipe_204",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_205: Recipe = Recipe {
        name: "recipe_205",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_206: Recipe = Recipe {
        name: "recipe_206",
        recipe_type: "minecraft:smithing_trim",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_207: Recipe = Recipe {
        name: "recipe_207",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_208: Recipe = Recipe {
        name: "recipe_208",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_209: Recipe = Recipe {
        name: "recipe_209",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_210: Recipe = Recipe {
        name: "recipe_210",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_211: Recipe = Recipe {
        name: "recipe_211",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_212: Recipe = Recipe {
        name: "recipe_212",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_213: Recipe = Recipe {
        name: "recipe_213",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_214: Recipe = Recipe {
        name: "recipe_214",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_215: Recipe = Recipe {
        name: "recipe_215",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_216: Recipe = Recipe {
        name: "recipe_216",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_217: Recipe = Recipe {
        name: "recipe_217",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_218: Recipe = Recipe {
        name: "recipe_218",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_219: Recipe = Recipe {
        name: "recipe_219",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_220: Recipe = Recipe {
        name: "recipe_220",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_221: Recipe = Recipe {
        name: "recipe_221",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_222: Recipe = Recipe {
        name: "recipe_222",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_223: Recipe = Recipe {
        name: "recipe_223",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(200),
    };
    pub const RECIPE_224: Recipe = Recipe {
        name: "recipe_224",
        recipe_type: "minecraft:campfire_cooking",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(600),
    };
    pub const RECIPE_225: Recipe = Recipe {
        name: "recipe_225",
        recipe_type: "minecraft:smoking",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(100),
    };
    pub const RECIPE_226: Recipe = Recipe {
        name: "recipe_226",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(200),
    };
    pub const RECIPE_227: Recipe = Recipe {
        name: "recipe_227",
        recipe_type: "minecraft:campfire_cooking",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(600),
    };
    pub const RECIPE_228: Recipe = Recipe {
        name: "recipe_228",
        recipe_type: "minecraft:smoking",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(100),
    };
    pub const RECIPE_229: Recipe = Recipe {
        name: "recipe_229",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(200),
    };
    pub const RECIPE_230: Recipe = Recipe {
        name: "recipe_230",
        recipe_type: "minecraft:campfire_cooking",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(600),
    };
    pub const RECIPE_231: Recipe = Recipe {
        name: "recipe_231",
        recipe_type: "minecraft:smoking",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(100),
    };
    pub const RECIPE_232: Recipe = Recipe {
        name: "recipe_232",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(200),
    };
    pub const RECIPE_233: Recipe = Recipe {
        name: "recipe_233",
        recipe_type: "minecraft:campfire_cooking",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(600),
    };
    pub const RECIPE_234: Recipe = Recipe {
        name: "recipe_234",
        recipe_type: "minecraft:smoking",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(100),
    };
    pub const RECIPE_235: Recipe = Recipe {
        name: "recipe_235",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(200),
    };
    pub const RECIPE_236: Recipe = Recipe {
        name: "recipe_236",
        recipe_type: "minecraft:campfire_cooking",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(600),
    };
    pub const RECIPE_237: Recipe = Recipe {
        name: "recipe_237",
        recipe_type: "minecraft:smoking",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(100),
    };
    pub const RECIPE_238: Recipe = Recipe {
        name: "recipe_238",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(200),
    };
    pub const RECIPE_239: Recipe = Recipe {
        name: "recipe_239",
        recipe_type: "minecraft:campfire_cooking",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(600),
    };
    pub const RECIPE_240: Recipe = Recipe {
        name: "recipe_240",
        recipe_type: "minecraft:smoking",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(100),
    };
    pub const RECIPE_241: Recipe = Recipe {
        name: "recipe_241",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(200),
    };
    pub const RECIPE_242: Recipe = Recipe {
        name: "recipe_242",
        recipe_type: "minecraft:campfire_cooking",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(600),
    };
    pub const RECIPE_243: Recipe = Recipe {
        name: "recipe_243",
        recipe_type: "minecraft:smoking",
        group: None,
        category: Some("food"),
        experience: Some(0.35),
        cookingtime: Some(100),
    };
    pub const RECIPE_244: Recipe = Recipe {
        name: "recipe_244",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_245: Recipe = Recipe {
        name: "recipe_245",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_246: Recipe = Recipe {
        name: "recipe_246",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_247: Recipe = Recipe {
        name: "recipe_247",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_248: Recipe = Recipe {
        name: "recipe_248",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_249: Recipe = Recipe {
        name: "recipe_249",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_250: Recipe = Recipe {
        name: "recipe_250",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("copper_ingot"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_251: Recipe = Recipe {
        name: "recipe_251",
        recipe_type: "minecraft:blasting",
        group: Some("copper_ingot"),
        category: Some("misc"),
        experience: Some(0.7),
        cookingtime: Some(100),
    };
    pub const RECIPE_252: Recipe = Recipe {
        name: "recipe_252",
        recipe_type: "minecraft:blasting",
        group: Some("copper_ingot"),
        category: Some("misc"),
        experience: Some(0.7),
        cookingtime: Some(100),
    };
    pub const RECIPE_253: Recipe = Recipe {
        name: "recipe_253",
        recipe_type: "minecraft:blasting",
        group: Some("copper_ingot"),
        category: Some("misc"),
        experience: Some(0.7),
        cookingtime: Some(100),
    };
    pub const RECIPE_254: Recipe = Recipe {
        name: "recipe_254",
        recipe_type: "minecraft:smelting",
        group: Some("copper_ingot"),
        category: Some("misc"),
        experience: Some(0.7),
        cookingtime: Some(200),
    };
    pub const RECIPE_255: Recipe = Recipe {
        name: "recipe_255",
        recipe_type: "minecraft:smelting",
        group: Some("copper_ingot"),
        category: Some("misc"),
        experience: Some(0.7),
        cookingtime: Some(200),
    };
    pub const RECIPE_256: Recipe = Recipe {
        name: "recipe_256",
        recipe_type: "minecraft:smelting",
        group: Some("copper_ingot"),
        category: Some("misc"),
        experience: Some(0.7),
        cookingtime: Some(200),
    };
    pub const RECIPE_257: Recipe = Recipe {
        name: "recipe_257",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("copper_ingot"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_258: Recipe = Recipe {
        name: "recipe_258",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_259: Recipe = Recipe {
        name: "recipe_259",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_260: Recipe = Recipe {
        name: "recipe_260",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_261: Recipe = Recipe {
        name: "recipe_261",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_262: Recipe = Recipe {
        name: "recipe_262",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_263: Recipe = Recipe {
        name: "recipe_263",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_264: Recipe = Recipe {
        name: "recipe_264",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_265: Recipe = Recipe {
        name: "recipe_265",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_266: Recipe = Recipe {
        name: "recipe_266",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_267: Recipe = Recipe {
        name: "recipe_267",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_268: Recipe = Recipe {
        name: "recipe_268",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wooden_button"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_269: Recipe = Recipe {
        name: "recipe_269",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_door"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_270: Recipe = Recipe {
        name: "recipe_270",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_271: Recipe = Recipe {
        name: "recipe_271",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence_gate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_272: Recipe = Recipe {
        name: "recipe_272",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("hanging_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_273: Recipe = Recipe {
        name: "recipe_273",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_274: Recipe = Recipe {
        name: "recipe_274",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("planks"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_275: Recipe = Recipe {
        name: "recipe_275",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_pressure_plate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_276: Recipe = Recipe {
        name: "recipe_276",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_277: Recipe = Recipe {
        name: "recipe_277",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_278: Recipe = Recipe {
        name: "recipe_278",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_279: Recipe = Recipe {
        name: "recipe_279",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_trapdoor"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_280: Recipe = Recipe {
        name: "recipe_280",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_281: Recipe = Recipe {
        name: "recipe_281",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_282: Recipe = Recipe {
        name: "recipe_282",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_283: Recipe = Recipe {
        name: "recipe_283",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_284: Recipe = Recipe {
        name: "recipe_284",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_285: Recipe = Recipe {
        name: "recipe_285",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_286: Recipe = Recipe {
        name: "recipe_286",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_287: Recipe = Recipe {
        name: "recipe_287",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_288: Recipe = Recipe {
        name: "recipe_288",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_289: Recipe = Recipe {
        name: "recipe_289",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_290: Recipe = Recipe {
        name: "recipe_290",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_291: Recipe = Recipe {
        name: "recipe_291",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_292: Recipe = Recipe {
        name: "recipe_292",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_293: Recipe = Recipe {
        name: "recipe_293",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_294: Recipe = Recipe {
        name: "recipe_294",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_295: Recipe = Recipe {
        name: "recipe_295",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_296: Recipe = Recipe {
        name: "recipe_296",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_297: Recipe = Recipe {
        name: "recipe_297",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_298: Recipe = Recipe {
        name: "recipe_298",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_299: Recipe = Recipe {
        name: "recipe_299",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("banner"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_300: Recipe = Recipe {
        name: "recipe_300",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bed"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_301: Recipe = Recipe {
        name: "recipe_301",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("bundle_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_302: Recipe = Recipe {
        name: "recipe_302",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("dyed_candle"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_303: Recipe = Recipe {
        name: "recipe_303",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("carpet"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_304: Recipe = Recipe {
        name: "recipe_304",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("concrete_powder"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_305: Recipe = Recipe {
        name: "recipe_305",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("cyan_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_306: Recipe = Recipe {
        name: "recipe_306",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("cyan_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_307: Recipe = Recipe {
        name: "recipe_307",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_308: Recipe = Recipe {
        name: "recipe_308",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("harness"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_309: Recipe = Recipe {
        name: "recipe_309",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("shulker_box_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_310: Recipe = Recipe {
        name: "recipe_310",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_311: Recipe = Recipe {
        name: "recipe_311",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_312: Recipe = Recipe {
        name: "recipe_312",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_313: Recipe = Recipe {
        name: "recipe_313",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_terracotta"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_314: Recipe = Recipe {
        name: "recipe_314",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_315: Recipe = Recipe {
        name: "recipe_315",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wooden_button"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_316: Recipe = Recipe {
        name: "recipe_316",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("chest_boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_317: Recipe = Recipe {
        name: "recipe_317",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_door"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_318: Recipe = Recipe {
        name: "recipe_318",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_319: Recipe = Recipe {
        name: "recipe_319",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence_gate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_320: Recipe = Recipe {
        name: "recipe_320",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("hanging_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_321: Recipe = Recipe {
        name: "recipe_321",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("planks"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_322: Recipe = Recipe {
        name: "recipe_322",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_pressure_plate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_323: Recipe = Recipe {
        name: "recipe_323",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_324: Recipe = Recipe {
        name: "recipe_324",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_325: Recipe = Recipe {
        name: "recipe_325",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_326: Recipe = Recipe {
        name: "recipe_326",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_trapdoor"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_327: Recipe = Recipe {
        name: "recipe_327",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_328: Recipe = Recipe {
        name: "recipe_328",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_329: Recipe = Recipe {
        name: "recipe_329",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_330: Recipe = Recipe {
        name: "recipe_330",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_331: Recipe = Recipe {
        name: "recipe_331",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_332: Recipe = Recipe {
        name: "recipe_332",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_333: Recipe = Recipe {
        name: "recipe_333",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_334: Recipe = Recipe {
        name: "recipe_334",
        recipe_type: "minecraft:crafting_decorated_pot",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_335: Recipe = Recipe {
        name: "recipe_335",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_336: Recipe = Recipe {
        name: "recipe_336",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_337: Recipe = Recipe {
        name: "recipe_337",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_338: Recipe = Recipe {
        name: "recipe_338",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_339: Recipe = Recipe {
        name: "recipe_339",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_340: Recipe = Recipe {
        name: "recipe_340",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_341: Recipe = Recipe {
        name: "recipe_341",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_342: Recipe = Recipe {
        name: "recipe_342",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_343: Recipe = Recipe {
        name: "recipe_343",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_344: Recipe = Recipe {
        name: "recipe_344",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_345: Recipe = Recipe {
        name: "recipe_345",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_346: Recipe = Recipe {
        name: "recipe_346",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_347: Recipe = Recipe {
        name: "recipe_347",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_348: Recipe = Recipe {
        name: "recipe_348",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_349: Recipe = Recipe {
        name: "recipe_349",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_350: Recipe = Recipe {
        name: "recipe_350",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_351: Recipe = Recipe {
        name: "recipe_351",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_352: Recipe = Recipe {
        name: "recipe_352",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_353: Recipe = Recipe {
        name: "recipe_353",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_354: Recipe = Recipe {
        name: "recipe_354",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_355: Recipe = Recipe {
        name: "recipe_355",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_356: Recipe = Recipe {
        name: "recipe_356",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_357: Recipe = Recipe {
        name: "recipe_357",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_358: Recipe = Recipe {
        name: "recipe_358",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_359: Recipe = Recipe {
        name: "recipe_359",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_360: Recipe = Recipe {
        name: "recipe_360",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_361: Recipe = Recipe {
        name: "recipe_361",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_362: Recipe = Recipe {
        name: "recipe_362",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_363: Recipe = Recipe {
        name: "recipe_363",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_364: Recipe = Recipe {
        name: "recipe_364",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_365: Recipe = Recipe {
        name: "recipe_365",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_366: Recipe = Recipe {
        name: "recipe_366",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_367: Recipe = Recipe {
        name: "recipe_367",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_368: Recipe = Recipe {
        name: "recipe_368",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_369: Recipe = Recipe {
        name: "recipe_369",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_370: Recipe = Recipe {
        name: "recipe_370",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_371: Recipe = Recipe {
        name: "recipe_371",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_372: Recipe = Recipe {
        name: "recipe_372",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_373: Recipe = Recipe {
        name: "recipe_373",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_374: Recipe = Recipe {
        name: "recipe_374",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_375: Recipe = Recipe {
        name: "recipe_375",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_376: Recipe = Recipe {
        name: "recipe_376",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_377: Recipe = Recipe {
        name: "recipe_377",
        recipe_type: "minecraft:blasting",
        group: Some("diamond"),
        category: Some("misc"),
        experience: Some(1.0),
        cookingtime: Some(100),
    };
    pub const RECIPE_378: Recipe = Recipe {
        name: "recipe_378",
        recipe_type: "minecraft:blasting",
        group: Some("diamond"),
        category: Some("misc"),
        experience: Some(1.0),
        cookingtime: Some(100),
    };
    pub const RECIPE_379: Recipe = Recipe {
        name: "recipe_379",
        recipe_type: "minecraft:smelting",
        group: Some("diamond"),
        category: Some("misc"),
        experience: Some(1.0),
        cookingtime: Some(200),
    };
    pub const RECIPE_380: Recipe = Recipe {
        name: "recipe_380",
        recipe_type: "minecraft:smelting",
        group: Some("diamond"),
        category: Some("misc"),
        experience: Some(1.0),
        cookingtime: Some(200),
    };
    pub const RECIPE_381: Recipe = Recipe {
        name: "recipe_381",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_382: Recipe = Recipe {
        name: "recipe_382",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_383: Recipe = Recipe {
        name: "recipe_383",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_384: Recipe = Recipe {
        name: "recipe_384",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_385: Recipe = Recipe {
        name: "recipe_385",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_386: Recipe = Recipe {
        name: "recipe_386",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_387: Recipe = Recipe {
        name: "recipe_387",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_388: Recipe = Recipe {
        name: "recipe_388",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_389: Recipe = Recipe {
        name: "recipe_389",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_390: Recipe = Recipe {
        name: "recipe_390",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_391: Recipe = Recipe {
        name: "recipe_391",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_392: Recipe = Recipe {
        name: "recipe_392",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_393: Recipe = Recipe {
        name: "recipe_393",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_394: Recipe = Recipe {
        name: "recipe_394",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_395: Recipe = Recipe {
        name: "recipe_395",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("dry_ghast"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_396: Recipe = Recipe {
        name: "recipe_396",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_397: Recipe = Recipe {
        name: "recipe_397",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_398: Recipe = Recipe {
        name: "recipe_398",
        recipe_type: "minecraft:campfire_cooking",
        group: None,
        category: Some("food"),
        experience: Some(0.1),
        cookingtime: Some(600),
    };
    pub const RECIPE_399: Recipe = Recipe {
        name: "recipe_399",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("food"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_400: Recipe = Recipe {
        name: "recipe_400",
        recipe_type: "minecraft:smoking",
        group: None,
        category: Some("food"),
        experience: Some(0.1),
        cookingtime: Some(100),
    };
    pub const RECIPE_401: Recipe = Recipe {
        name: "recipe_401",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_402: Recipe = Recipe {
        name: "recipe_402",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_403: Recipe = Recipe {
        name: "recipe_403",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_404: Recipe = Recipe {
        name: "recipe_404",
        recipe_type: "minecraft:smithing_trim",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_405: Recipe = Recipe {
        name: "recipe_405",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("bed_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_406: Recipe = Recipe {
        name: "recipe_406",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("carpet_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_407: Recipe = Recipe {
        name: "recipe_407",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("harness_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_408: Recipe = Recipe {
        name: "recipe_408",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wool"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_409: Recipe = Recipe {
        name: "recipe_409",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("bed_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_410: Recipe = Recipe {
        name: "recipe_410",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("carpet_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_411: Recipe = Recipe {
        name: "recipe_411",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("harness_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_412: Recipe = Recipe {
        name: "recipe_412",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wool"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_413: Recipe = Recipe {
        name: "recipe_413",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("bed_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_414: Recipe = Recipe {
        name: "recipe_414",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("carpet_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_415: Recipe = Recipe {
        name: "recipe_415",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("harness_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_416: Recipe = Recipe {
        name: "recipe_416",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wool"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_417: Recipe = Recipe {
        name: "recipe_417",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("bed_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_418: Recipe = Recipe {
        name: "recipe_418",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("carpet_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_419: Recipe = Recipe {
        name: "recipe_419",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("harness_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_420: Recipe = Recipe {
        name: "recipe_420",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wool"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_421: Recipe = Recipe {
        name: "recipe_421",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("bed_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_422: Recipe = Recipe {
        name: "recipe_422",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("carpet_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_423: Recipe = Recipe {
        name: "recipe_423",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("harness_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_424: Recipe = Recipe {
        name: "recipe_424",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wool"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_425: Recipe = Recipe {
        name: "recipe_425",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("bed_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_426: Recipe = Recipe {
        name: "recipe_426",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("carpet_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_427: Recipe = Recipe {
        name: "recipe_427",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("harness_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_428: Recipe = Recipe {
        name: "recipe_428",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wool"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_429: Recipe = Recipe {
        name: "recipe_429",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("bed_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_430: Recipe = Recipe {
        name: "recipe_430",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("carpet_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_431: Recipe = Recipe {
        name: "recipe_431",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("harness_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_432: Recipe = Recipe {
        name: "recipe_432",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wool"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_433: Recipe = Recipe {
        name: "recipe_433",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("bed_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_434: Recipe = Recipe {
        name: "recipe_434",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("carpet_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_435: Recipe = Recipe {
        name: "recipe_435",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("harness_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_436: Recipe = Recipe {
        name: "recipe_436",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wool"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_437: Recipe = Recipe {
        name: "recipe_437",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("bed_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_438: Recipe = Recipe {
        name: "recipe_438",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("carpet_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_439: Recipe = Recipe {
        name: "recipe_439",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("harness_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_440: Recipe = Recipe {
        name: "recipe_440",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wool"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_441: Recipe = Recipe {
        name: "recipe_441",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("bed_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_442: Recipe = Recipe {
        name: "recipe_442",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("carpet_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_443: Recipe = Recipe {
        name: "recipe_443",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("harness_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_444: Recipe = Recipe {
        name: "recipe_444",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wool"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_445: Recipe = Recipe {
        name: "recipe_445",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("bed_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_446: Recipe = Recipe {
        name: "recipe_446",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("carpet_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_447: Recipe = Recipe {
        name: "recipe_447",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("harness_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_448: Recipe = Recipe {
        name: "recipe_448",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wool"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_449: Recipe = Recipe {
        name: "recipe_449",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("bed_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_450: Recipe = Recipe {
        name: "recipe_450",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("carpet_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_451: Recipe = Recipe {
        name: "recipe_451",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("harness_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_452: Recipe = Recipe {
        name: "recipe_452",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wool"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_453: Recipe = Recipe {
        name: "recipe_453",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("bed_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_454: Recipe = Recipe {
        name: "recipe_454",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("carpet_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_455: Recipe = Recipe {
        name: "recipe_455",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("harness_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_456: Recipe = Recipe {
        name: "recipe_456",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wool"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_457: Recipe = Recipe {
        name: "recipe_457",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("bed_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_458: Recipe = Recipe {
        name: "recipe_458",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("carpet_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_459: Recipe = Recipe {
        name: "recipe_459",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("harness_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_460: Recipe = Recipe {
        name: "recipe_460",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wool"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_461: Recipe = Recipe {
        name: "recipe_461",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("bed_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_462: Recipe = Recipe {
        name: "recipe_462",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("carpet_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_463: Recipe = Recipe {
        name: "recipe_463",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("harness_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_464: Recipe = Recipe {
        name: "recipe_464",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wool"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_465: Recipe = Recipe {
        name: "recipe_465",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("bed_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_466: Recipe = Recipe {
        name: "recipe_466",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("carpet_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_467: Recipe = Recipe {
        name: "recipe_467",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("harness_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_468: Recipe = Recipe {
        name: "recipe_468",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wool"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_469: Recipe = Recipe {
        name: "recipe_469",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_470: Recipe = Recipe {
        name: "recipe_470",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_471: Recipe = Recipe {
        name: "recipe_471",
        recipe_type: "minecraft:blasting",
        group: Some("emerald"),
        category: Some("misc"),
        experience: Some(1.0),
        cookingtime: Some(100),
    };
    pub const RECIPE_472: Recipe = Recipe {
        name: "recipe_472",
        recipe_type: "minecraft:blasting",
        group: Some("emerald"),
        category: Some("misc"),
        experience: Some(1.0),
        cookingtime: Some(100),
    };
    pub const RECIPE_473: Recipe = Recipe {
        name: "recipe_473",
        recipe_type: "minecraft:smelting",
        group: Some("emerald"),
        category: Some("misc"),
        experience: Some(1.0),
        cookingtime: Some(200),
    };
    pub const RECIPE_474: Recipe = Recipe {
        name: "recipe_474",
        recipe_type: "minecraft:smelting",
        group: Some("emerald"),
        category: Some("misc"),
        experience: Some(1.0),
        cookingtime: Some(200),
    };
    pub const RECIPE_475: Recipe = Recipe {
        name: "recipe_475",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_476: Recipe = Recipe {
        name: "recipe_476",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_477: Recipe = Recipe {
        name: "recipe_477",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_478: Recipe = Recipe {
        name: "recipe_478",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_479: Recipe = Recipe {
        name: "recipe_479",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_480: Recipe = Recipe {
        name: "recipe_480",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_481: Recipe = Recipe {
        name: "recipe_481",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_482: Recipe = Recipe {
        name: "recipe_482",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_483: Recipe = Recipe {
        name: "recipe_483",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_484: Recipe = Recipe {
        name: "recipe_484",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_485: Recipe = Recipe {
        name: "recipe_485",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_486: Recipe = Recipe {
        name: "recipe_486",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_487: Recipe = Recipe {
        name: "recipe_487",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_488: Recipe = Recipe {
        name: "recipe_488",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_489: Recipe = Recipe {
        name: "recipe_489",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_490: Recipe = Recipe {
        name: "recipe_490",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_491: Recipe = Recipe {
        name: "recipe_491",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_492: Recipe = Recipe {
        name: "recipe_492",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_493: Recipe = Recipe {
        name: "recipe_493",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_494: Recipe = Recipe {
        name: "recipe_494",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_495: Recipe = Recipe {
        name: "recipe_495",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_496: Recipe = Recipe {
        name: "recipe_496",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_497: Recipe = Recipe {
        name: "recipe_497",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_498: Recipe = Recipe {
        name: "recipe_498",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_499: Recipe = Recipe {
        name: "recipe_499",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_500: Recipe = Recipe {
        name: "recipe_500",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_501: Recipe = Recipe {
        name: "recipe_501",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_502: Recipe = Recipe {
        name: "recipe_502",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_503: Recipe = Recipe {
        name: "recipe_503",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_504: Recipe = Recipe {
        name: "recipe_504",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_505: Recipe = Recipe {
        name: "recipe_505",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_506: Recipe = Recipe {
        name: "recipe_506",
        recipe_type: "minecraft:smithing_trim",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_507: Recipe = Recipe {
        name: "recipe_507",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_508: Recipe = Recipe {
        name: "recipe_508",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_509: Recipe = Recipe {
        name: "recipe_509",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_510: Recipe = Recipe {
        name: "recipe_510",
        recipe_type: "minecraft:crafting_special_firework_rocket",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_511: Recipe = Recipe {
        name: "recipe_511",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_512: Recipe = Recipe {
        name: "recipe_512",
        recipe_type: "minecraft:crafting_special_firework_star",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_513: Recipe = Recipe {
        name: "recipe_513",
        recipe_type: "minecraft:crafting_special_firework_star_fade",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_514: Recipe = Recipe {
        name: "recipe_514",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_515: Recipe = Recipe {
        name: "recipe_515",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_516: Recipe = Recipe {
        name: "recipe_516",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_517: Recipe = Recipe {
        name: "recipe_517",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_518: Recipe = Recipe {
        name: "recipe_518",
        recipe_type: "minecraft:smithing_trim",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_519: Recipe = Recipe {
        name: "recipe_519",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_520: Recipe = Recipe {
        name: "recipe_520",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_521: Recipe = Recipe {
        name: "recipe_521",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_522: Recipe = Recipe {
        name: "recipe_522",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_523: Recipe = Recipe {
        name: "recipe_523",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_524: Recipe = Recipe {
        name: "recipe_524",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_525: Recipe = Recipe {
        name: "recipe_525",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_526: Recipe = Recipe {
        name: "recipe_526",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_527: Recipe = Recipe {
        name: "recipe_527",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_528: Recipe = Recipe {
        name: "recipe_528",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_529: Recipe = Recipe {
        name: "recipe_529",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_530: Recipe = Recipe {
        name: "recipe_530",
        recipe_type: "minecraft:blasting",
        group: Some("gold_ingot"),
        category: Some("misc"),
        experience: Some(1.0),
        cookingtime: Some(100),
    };
    pub const RECIPE_531: Recipe = Recipe {
        name: "recipe_531",
        recipe_type: "minecraft:blasting",
        group: Some("gold_ingot"),
        category: Some("misc"),
        experience: Some(1.0),
        cookingtime: Some(100),
    };
    pub const RECIPE_532: Recipe = Recipe {
        name: "recipe_532",
        recipe_type: "minecraft:blasting",
        group: Some("gold_ingot"),
        category: Some("misc"),
        experience: Some(1.0),
        cookingtime: Some(100),
    };
    pub const RECIPE_533: Recipe = Recipe {
        name: "recipe_533",
        recipe_type: "minecraft:blasting",
        group: Some("gold_ingot"),
        category: Some("misc"),
        experience: Some(1.0),
        cookingtime: Some(100),
    };
    pub const RECIPE_534: Recipe = Recipe {
        name: "recipe_534",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("gold_ingot"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_535: Recipe = Recipe {
        name: "recipe_535",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("gold_ingot"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_536: Recipe = Recipe {
        name: "recipe_536",
        recipe_type: "minecraft:smelting",
        group: Some("gold_ingot"),
        category: Some("misc"),
        experience: Some(1.0),
        cookingtime: Some(200),
    };
    pub const RECIPE_537: Recipe = Recipe {
        name: "recipe_537",
        recipe_type: "minecraft:smelting",
        group: Some("gold_ingot"),
        category: Some("misc"),
        experience: Some(1.0),
        cookingtime: Some(200),
    };
    pub const RECIPE_538: Recipe = Recipe {
        name: "recipe_538",
        recipe_type: "minecraft:smelting",
        group: Some("gold_ingot"),
        category: Some("misc"),
        experience: Some(1.0),
        cookingtime: Some(200),
    };
    pub const RECIPE_539: Recipe = Recipe {
        name: "recipe_539",
        recipe_type: "minecraft:smelting",
        group: Some("gold_ingot"),
        category: Some("misc"),
        experience: Some(1.0),
        cookingtime: Some(200),
    };
    pub const RECIPE_540: Recipe = Recipe {
        name: "recipe_540",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_541: Recipe = Recipe {
        name: "recipe_541",
        recipe_type: "minecraft:blasting",
        group: None,
        category: Some("misc"),
        experience: Some(0.1),
        cookingtime: Some(100),
    };
    pub const RECIPE_542: Recipe = Recipe {
        name: "recipe_542",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("misc"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_543: Recipe = Recipe {
        name: "recipe_543",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_544: Recipe = Recipe {
        name: "recipe_544",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_545: Recipe = Recipe {
        name: "recipe_545",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_546: Recipe = Recipe {
        name: "recipe_546",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_547: Recipe = Recipe {
        name: "recipe_547",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_548: Recipe = Recipe {
        name: "recipe_548",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_549: Recipe = Recipe {
        name: "recipe_549",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_550: Recipe = Recipe {
        name: "recipe_550",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_551: Recipe = Recipe {
        name: "recipe_551",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_552: Recipe = Recipe {
        name: "recipe_552",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_553: Recipe = Recipe {
        name: "recipe_553",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_554: Recipe = Recipe {
        name: "recipe_554",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_555: Recipe = Recipe {
        name: "recipe_555",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_556: Recipe = Recipe {
        name: "recipe_556",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_557: Recipe = Recipe {
        name: "recipe_557",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_558: Recipe = Recipe {
        name: "recipe_558",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_559: Recipe = Recipe {
        name: "recipe_559",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_560: Recipe = Recipe {
        name: "recipe_560",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_561: Recipe = Recipe {
        name: "recipe_561",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("banner"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_562: Recipe = Recipe {
        name: "recipe_562",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bed"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_563: Recipe = Recipe {
        name: "recipe_563",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("bundle_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_564: Recipe = Recipe {
        name: "recipe_564",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("dyed_candle"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_565: Recipe = Recipe {
        name: "recipe_565",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("carpet"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_566: Recipe = Recipe {
        name: "recipe_566",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("concrete_powder"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_567: Recipe = Recipe {
        name: "recipe_567",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("gray_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_568: Recipe = Recipe {
        name: "recipe_568",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("gray_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_569: Recipe = Recipe {
        name: "recipe_569",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_570: Recipe = Recipe {
        name: "recipe_570",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("harness"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_571: Recipe = Recipe {
        name: "recipe_571",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("shulker_box_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_572: Recipe = Recipe {
        name: "recipe_572",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_573: Recipe = Recipe {
        name: "recipe_573",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_574: Recipe = Recipe {
        name: "recipe_574",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_575: Recipe = Recipe {
        name: "recipe_575",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_terracotta"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_576: Recipe = Recipe {
        name: "recipe_576",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("banner"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_577: Recipe = Recipe {
        name: "recipe_577",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bed"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_578: Recipe = Recipe {
        name: "recipe_578",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("bundle_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_579: Recipe = Recipe {
        name: "recipe_579",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("dyed_candle"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_580: Recipe = Recipe {
        name: "recipe_580",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("carpet"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_581: Recipe = Recipe {
        name: "recipe_581",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("concrete_powder"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_582: Recipe = Recipe {
        name: "recipe_582",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("misc"),
        experience: Some(1.0),
        cookingtime: Some(200),
    };
    pub const RECIPE_583: Recipe = Recipe {
        name: "recipe_583",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_584: Recipe = Recipe {
        name: "recipe_584",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("harness"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_585: Recipe = Recipe {
        name: "recipe_585",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("shulker_box_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_586: Recipe = Recipe {
        name: "recipe_586",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_587: Recipe = Recipe {
        name: "recipe_587",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_588: Recipe = Recipe {
        name: "recipe_588",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_589: Recipe = Recipe {
        name: "recipe_589",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_terracotta"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_590: Recipe = Recipe {
        name: "recipe_590",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_591: Recipe = Recipe {
        name: "recipe_591",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_592: Recipe = Recipe {
        name: "recipe_592",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_593: Recipe = Recipe {
        name: "recipe_593",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_594: Recipe = Recipe {
        name: "recipe_594",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_595: Recipe = Recipe {
        name: "recipe_595",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_596: Recipe = Recipe {
        name: "recipe_596",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_597: Recipe = Recipe {
        name: "recipe_597",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_598: Recipe = Recipe {
        name: "recipe_598",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_599: Recipe = Recipe {
        name: "recipe_599",
        recipe_type: "minecraft:smithing_trim",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_600: Recipe = Recipe {
        name: "recipe_600",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_601: Recipe = Recipe {
        name: "recipe_601",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_602: Recipe = Recipe {
        name: "recipe_602",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_603: Recipe = Recipe {
        name: "recipe_603",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_604: Recipe = Recipe {
        name: "recipe_604",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_605: Recipe = Recipe {
        name: "recipe_605",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_606: Recipe = Recipe {
        name: "recipe_606",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_607: Recipe = Recipe {
        name: "recipe_607",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_608: Recipe = Recipe {
        name: "recipe_608",
        recipe_type: "minecraft:blasting",
        group: Some("iron_ingot"),
        category: Some("misc"),
        experience: Some(0.7),
        cookingtime: Some(100),
    };
    pub const RECIPE_609: Recipe = Recipe {
        name: "recipe_609",
        recipe_type: "minecraft:blasting",
        group: Some("iron_ingot"),
        category: Some("misc"),
        experience: Some(0.7),
        cookingtime: Some(100),
    };
    pub const RECIPE_610: Recipe = Recipe {
        name: "recipe_610",
        recipe_type: "minecraft:blasting",
        group: Some("iron_ingot"),
        category: Some("misc"),
        experience: Some(0.7),
        cookingtime: Some(100),
    };
    pub const RECIPE_611: Recipe = Recipe {
        name: "recipe_611",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("iron_ingot"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_612: Recipe = Recipe {
        name: "recipe_612",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("iron_ingot"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_613: Recipe = Recipe {
        name: "recipe_613",
        recipe_type: "minecraft:smelting",
        group: Some("iron_ingot"),
        category: Some("misc"),
        experience: Some(0.7),
        cookingtime: Some(200),
    };
    pub const RECIPE_614: Recipe = Recipe {
        name: "recipe_614",
        recipe_type: "minecraft:smelting",
        group: Some("iron_ingot"),
        category: Some("misc"),
        experience: Some(0.7),
        cookingtime: Some(200),
    };
    pub const RECIPE_615: Recipe = Recipe {
        name: "recipe_615",
        recipe_type: "minecraft:smelting",
        group: Some("iron_ingot"),
        category: Some("misc"),
        experience: Some(0.7),
        cookingtime: Some(200),
    };
    pub const RECIPE_616: Recipe = Recipe {
        name: "recipe_616",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_617: Recipe = Recipe {
        name: "recipe_617",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_618: Recipe = Recipe {
        name: "recipe_618",
        recipe_type: "minecraft:blasting",
        group: None,
        category: Some("misc"),
        experience: Some(0.1),
        cookingtime: Some(100),
    };
    pub const RECIPE_619: Recipe = Recipe {
        name: "recipe_619",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("misc"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_620: Recipe = Recipe {
        name: "recipe_620",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_621: Recipe = Recipe {
        name: "recipe_621",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_622: Recipe = Recipe {
        name: "recipe_622",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_623: Recipe = Recipe {
        name: "recipe_623",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_624: Recipe = Recipe {
        name: "recipe_624",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_625: Recipe = Recipe {
        name: "recipe_625",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_626: Recipe = Recipe {
        name: "recipe_626",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_627: Recipe = Recipe {
        name: "recipe_627",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_628: Recipe = Recipe {
        name: "recipe_628",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wooden_button"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_629: Recipe = Recipe {
        name: "recipe_629",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("chest_boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_630: Recipe = Recipe {
        name: "recipe_630",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_door"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_631: Recipe = Recipe {
        name: "recipe_631",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_632: Recipe = Recipe {
        name: "recipe_632",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence_gate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_633: Recipe = Recipe {
        name: "recipe_633",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("hanging_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_634: Recipe = Recipe {
        name: "recipe_634",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("planks"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_635: Recipe = Recipe {
        name: "recipe_635",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_pressure_plate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_636: Recipe = Recipe {
        name: "recipe_636",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_637: Recipe = Recipe {
        name: "recipe_637",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_638: Recipe = Recipe {
        name: "recipe_638",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_639: Recipe = Recipe {
        name: "recipe_639",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_trapdoor"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_640: Recipe = Recipe {
        name: "recipe_640",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_641: Recipe = Recipe {
        name: "recipe_641",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_642: Recipe = Recipe {
        name: "recipe_642",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_643: Recipe = Recipe {
        name: "recipe_643",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_644: Recipe = Recipe {
        name: "recipe_644",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_645: Recipe = Recipe {
        name: "recipe_645",
        recipe_type: "minecraft:blasting",
        group: Some("lapis_lazuli"),
        category: Some("misc"),
        experience: Some(0.2),
        cookingtime: Some(100),
    };
    pub const RECIPE_646: Recipe = Recipe {
        name: "recipe_646",
        recipe_type: "minecraft:blasting",
        group: Some("lapis_lazuli"),
        category: Some("misc"),
        experience: Some(0.2),
        cookingtime: Some(100),
    };
    pub const RECIPE_647: Recipe = Recipe {
        name: "recipe_647",
        recipe_type: "minecraft:smelting",
        group: Some("lapis_lazuli"),
        category: Some("misc"),
        experience: Some(0.2),
        cookingtime: Some(200),
    };
    pub const RECIPE_648: Recipe = Recipe {
        name: "recipe_648",
        recipe_type: "minecraft:smelting",
        group: Some("lapis_lazuli"),
        category: Some("misc"),
        experience: Some(0.2),
        cookingtime: Some(200),
    };
    pub const RECIPE_649: Recipe = Recipe {
        name: "recipe_649",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_650: Recipe = Recipe {
        name: "recipe_650",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_651: Recipe = Recipe {
        name: "recipe_651",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_652: Recipe = Recipe {
        name: "recipe_652",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_653: Recipe = Recipe {
        name: "recipe_653",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_654: Recipe = Recipe {
        name: "recipe_654",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_655: Recipe = Recipe {
        name: "recipe_655",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_656: Recipe = Recipe {
        name: "recipe_656",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_657: Recipe = Recipe {
        name: "recipe_657",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_658: Recipe = Recipe {
        name: "recipe_658",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_659: Recipe = Recipe {
        name: "recipe_659",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("banner"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_660: Recipe = Recipe {
        name: "recipe_660",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bed"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_661: Recipe = Recipe {
        name: "recipe_661",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("bundle_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_662: Recipe = Recipe {
        name: "recipe_662",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("dyed_candle"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_663: Recipe = Recipe {
        name: "recipe_663",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("carpet"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_664: Recipe = Recipe {
        name: "recipe_664",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("concrete_powder"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_665: Recipe = Recipe {
        name: "recipe_665",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("light_blue_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_666: Recipe = Recipe {
        name: "recipe_666",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("light_blue_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_667: Recipe = Recipe {
        name: "recipe_667",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_668: Recipe = Recipe {
        name: "recipe_668",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("harness"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_669: Recipe = Recipe {
        name: "recipe_669",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("shulker_box_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_670: Recipe = Recipe {
        name: "recipe_670",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_671: Recipe = Recipe {
        name: "recipe_671",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_672: Recipe = Recipe {
        name: "recipe_672",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_673: Recipe = Recipe {
        name: "recipe_673",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_terracotta"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_674: Recipe = Recipe {
        name: "recipe_674",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("banner"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_675: Recipe = Recipe {
        name: "recipe_675",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bed"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_676: Recipe = Recipe {
        name: "recipe_676",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("bundle_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_677: Recipe = Recipe {
        name: "recipe_677",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("dyed_candle"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_678: Recipe = Recipe {
        name: "recipe_678",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("carpet"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_679: Recipe = Recipe {
        name: "recipe_679",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("concrete_powder"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_680: Recipe = Recipe {
        name: "recipe_680",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("light_gray_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_681: Recipe = Recipe {
        name: "recipe_681",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("light_gray_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_682: Recipe = Recipe {
        name: "recipe_682",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("light_gray_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_683: Recipe = Recipe {
        name: "recipe_683",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("light_gray_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_684: Recipe = Recipe {
        name: "recipe_684",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("light_gray_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_685: Recipe = Recipe {
        name: "recipe_685",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_686: Recipe = Recipe {
        name: "recipe_686",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("harness"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_687: Recipe = Recipe {
        name: "recipe_687",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("shulker_box_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_688: Recipe = Recipe {
        name: "recipe_688",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_689: Recipe = Recipe {
        name: "recipe_689",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_690: Recipe = Recipe {
        name: "recipe_690",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_691: Recipe = Recipe {
        name: "recipe_691",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_terracotta"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_692: Recipe = Recipe {
        name: "recipe_692",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_693: Recipe = Recipe {
        name: "recipe_693",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_694: Recipe = Recipe {
        name: "recipe_694",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("banner"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_695: Recipe = Recipe {
        name: "recipe_695",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bed"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_696: Recipe = Recipe {
        name: "recipe_696",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("bundle_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_697: Recipe = Recipe {
        name: "recipe_697",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("dyed_candle"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_698: Recipe = Recipe {
        name: "recipe_698",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("carpet"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_699: Recipe = Recipe {
        name: "recipe_699",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("concrete_powder"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_700: Recipe = Recipe {
        name: "recipe_700",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_701: Recipe = Recipe {
        name: "recipe_701",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("misc"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_702: Recipe = Recipe {
        name: "recipe_702",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_703: Recipe = Recipe {
        name: "recipe_703",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("harness"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_704: Recipe = Recipe {
        name: "recipe_704",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("shulker_box_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_705: Recipe = Recipe {
        name: "recipe_705",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_706: Recipe = Recipe {
        name: "recipe_706",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_707: Recipe = Recipe {
        name: "recipe_707",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_708: Recipe = Recipe {
        name: "recipe_708",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_terracotta"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_709: Recipe = Recipe {
        name: "recipe_709",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_710: Recipe = Recipe {
        name: "recipe_710",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_711: Recipe = Recipe {
        name: "recipe_711",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_712: Recipe = Recipe {
        name: "recipe_712",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("banner"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_713: Recipe = Recipe {
        name: "recipe_713",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bed"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_714: Recipe = Recipe {
        name: "recipe_714",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("bundle_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_715: Recipe = Recipe {
        name: "recipe_715",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("dyed_candle"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_716: Recipe = Recipe {
        name: "recipe_716",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("carpet"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_717: Recipe = Recipe {
        name: "recipe_717",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("concrete_powder"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_718: Recipe = Recipe {
        name: "recipe_718",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("magenta_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_719: Recipe = Recipe {
        name: "recipe_719",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("magenta_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_720: Recipe = Recipe {
        name: "recipe_720",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("magenta_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_721: Recipe = Recipe {
        name: "recipe_721",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("magenta_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_722: Recipe = Recipe {
        name: "recipe_722",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("magenta_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_723: Recipe = Recipe {
        name: "recipe_723",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_724: Recipe = Recipe {
        name: "recipe_724",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("harness"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_725: Recipe = Recipe {
        name: "recipe_725",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("shulker_box_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_726: Recipe = Recipe {
        name: "recipe_726",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_727: Recipe = Recipe {
        name: "recipe_727",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_728: Recipe = Recipe {
        name: "recipe_728",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_729: Recipe = Recipe {
        name: "recipe_729",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_terracotta"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_730: Recipe = Recipe {
        name: "recipe_730",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_731: Recipe = Recipe {
        name: "recipe_731",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_732: Recipe = Recipe {
        name: "recipe_732",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_733: Recipe = Recipe {
        name: "recipe_733",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wooden_button"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_734: Recipe = Recipe {
        name: "recipe_734",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("chest_boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_735: Recipe = Recipe {
        name: "recipe_735",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_door"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_736: Recipe = Recipe {
        name: "recipe_736",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_737: Recipe = Recipe {
        name: "recipe_737",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence_gate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_738: Recipe = Recipe {
        name: "recipe_738",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("hanging_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_739: Recipe = Recipe {
        name: "recipe_739",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("planks"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_740: Recipe = Recipe {
        name: "recipe_740",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_pressure_plate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_741: Recipe = Recipe {
        name: "recipe_741",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_742: Recipe = Recipe {
        name: "recipe_742",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_743: Recipe = Recipe {
        name: "recipe_743",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_744: Recipe = Recipe {
        name: "recipe_744",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_trapdoor"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_745: Recipe = Recipe {
        name: "recipe_745",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_746: Recipe = Recipe {
        name: "recipe_746",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_747: Recipe = Recipe {
        name: "recipe_747",
        recipe_type: "minecraft:crafting_special_mapcloning",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_748: Recipe = Recipe {
        name: "recipe_748",
        recipe_type: "minecraft:crafting_special_mapextending",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_749: Recipe = Recipe {
        name: "recipe_749",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_750: Recipe = Recipe {
        name: "recipe_750",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_751: Recipe = Recipe {
        name: "recipe_751",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_752: Recipe = Recipe {
        name: "recipe_752",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_753: Recipe = Recipe {
        name: "recipe_753",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("carpet"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_754: Recipe = Recipe {
        name: "recipe_754",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("mossy_cobblestone"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_755: Recipe = Recipe {
        name: "recipe_755",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("mossy_cobblestone"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_756: Recipe = Recipe {
        name: "recipe_756",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_757: Recipe = Recipe {
        name: "recipe_757",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_758: Recipe = Recipe {
        name: "recipe_758",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_759: Recipe = Recipe {
        name: "recipe_759",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_760: Recipe = Recipe {
        name: "recipe_760",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_761: Recipe = Recipe {
        name: "recipe_761",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_762: Recipe = Recipe {
        name: "recipe_762",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_763: Recipe = Recipe {
        name: "recipe_763",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_764: Recipe = Recipe {
        name: "recipe_764",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_765: Recipe = Recipe {
        name: "recipe_765",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_766: Recipe = Recipe {
        name: "recipe_766",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_767: Recipe = Recipe {
        name: "recipe_767",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_768: Recipe = Recipe {
        name: "recipe_768",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("mossy_stone_bricks"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_769: Recipe = Recipe {
        name: "recipe_769",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("mossy_stone_bricks"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_770: Recipe = Recipe {
        name: "recipe_770",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_771: Recipe = Recipe {
        name: "recipe_771",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_772: Recipe = Recipe {
        name: "recipe_772",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_773: Recipe = Recipe {
        name: "recipe_773",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_774: Recipe = Recipe {
        name: "recipe_774",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_775: Recipe = Recipe {
        name: "recipe_775",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_776: Recipe = Recipe {
        name: "recipe_776",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_777: Recipe = Recipe {
        name: "recipe_777",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_778: Recipe = Recipe {
        name: "recipe_778",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_779: Recipe = Recipe {
        name: "recipe_779",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_780: Recipe = Recipe {
        name: "recipe_780",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("misc"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_781: Recipe = Recipe {
        name: "recipe_781",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_782: Recipe = Recipe {
        name: "recipe_782",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_783: Recipe = Recipe {
        name: "recipe_783",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_784: Recipe = Recipe {
        name: "recipe_784",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_785: Recipe = Recipe {
        name: "recipe_785",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_786: Recipe = Recipe {
        name: "recipe_786",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_787: Recipe = Recipe {
        name: "recipe_787",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_788: Recipe = Recipe {
        name: "recipe_788",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_789: Recipe = Recipe {
        name: "recipe_789",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_790: Recipe = Recipe {
        name: "recipe_790",
        recipe_type: "minecraft:smithing_transform",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_791: Recipe = Recipe {
        name: "recipe_791",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_792: Recipe = Recipe {
        name: "recipe_792",
        recipe_type: "minecraft:smithing_transform",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_793: Recipe = Recipe {
        name: "recipe_793",
        recipe_type: "minecraft:smithing_transform",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_794: Recipe = Recipe {
        name: "recipe_794",
        recipe_type: "minecraft:smithing_transform",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_795: Recipe = Recipe {
        name: "recipe_795",
        recipe_type: "minecraft:smithing_transform",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_796: Recipe = Recipe {
        name: "recipe_796",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("netherite_ingot"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_797: Recipe = Recipe {
        name: "recipe_797",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("netherite_ingot"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_798: Recipe = Recipe {
        name: "recipe_798",
        recipe_type: "minecraft:smithing_transform",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_799: Recipe = Recipe {
        name: "recipe_799",
        recipe_type: "minecraft:smithing_transform",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_800: Recipe = Recipe {
        name: "recipe_800",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("misc"),
        experience: Some(2.0),
        cookingtime: Some(200),
    };
    pub const RECIPE_801: Recipe = Recipe {
        name: "recipe_801",
        recipe_type: "minecraft:blasting",
        group: None,
        category: Some("misc"),
        experience: Some(2.0),
        cookingtime: Some(100),
    };
    pub const RECIPE_802: Recipe = Recipe {
        name: "recipe_802",
        recipe_type: "minecraft:smithing_transform",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_803: Recipe = Recipe {
        name: "recipe_803",
        recipe_type: "minecraft:smithing_transform",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_804: Recipe = Recipe {
        name: "recipe_804",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_805: Recipe = Recipe {
        name: "recipe_805",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_806: Recipe = Recipe {
        name: "recipe_806",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_807: Recipe = Recipe {
        name: "recipe_807",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wooden_button"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_808: Recipe = Recipe {
        name: "recipe_808",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("chest_boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_809: Recipe = Recipe {
        name: "recipe_809",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_door"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_810: Recipe = Recipe {
        name: "recipe_810",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_811: Recipe = Recipe {
        name: "recipe_811",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence_gate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_812: Recipe = Recipe {
        name: "recipe_812",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("hanging_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_813: Recipe = Recipe {
        name: "recipe_813",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("planks"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_814: Recipe = Recipe {
        name: "recipe_814",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_pressure_plate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_815: Recipe = Recipe {
        name: "recipe_815",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_816: Recipe = Recipe {
        name: "recipe_816",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_817: Recipe = Recipe {
        name: "recipe_817",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_818: Recipe = Recipe {
        name: "recipe_818",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_trapdoor"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_819: Recipe = Recipe {
        name: "recipe_819",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_820: Recipe = Recipe {
        name: "recipe_820",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_821: Recipe = Recipe {
        name: "recipe_821",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("banner"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_822: Recipe = Recipe {
        name: "recipe_822",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bed"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_823: Recipe = Recipe {
        name: "recipe_823",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("bundle_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_824: Recipe = Recipe {
        name: "recipe_824",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("dyed_candle"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_825: Recipe = Recipe {
        name: "recipe_825",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("carpet"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_826: Recipe = Recipe {
        name: "recipe_826",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("concrete_powder"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_827: Recipe = Recipe {
        name: "recipe_827",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("orange_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_828: Recipe = Recipe {
        name: "recipe_828",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("orange_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_829: Recipe = Recipe {
        name: "recipe_829",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("orange_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_830: Recipe = Recipe {
        name: "recipe_830",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("orange_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_831: Recipe = Recipe {
        name: "recipe_831",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_832: Recipe = Recipe {
        name: "recipe_832",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("harness"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_833: Recipe = Recipe {
        name: "recipe_833",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("shulker_box_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_834: Recipe = Recipe {
        name: "recipe_834",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_835: Recipe = Recipe {
        name: "recipe_835",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_836: Recipe = Recipe {
        name: "recipe_836",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_837: Recipe = Recipe {
        name: "recipe_837",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_terracotta"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_838: Recipe = Recipe {
        name: "recipe_838",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_839: Recipe = Recipe {
        name: "recipe_839",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_840: Recipe = Recipe {
        name: "recipe_840",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_841: Recipe = Recipe {
        name: "recipe_841",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_842: Recipe = Recipe {
        name: "recipe_842",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_843: Recipe = Recipe {
        name: "recipe_843",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_844: Recipe = Recipe {
        name: "recipe_844",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_845: Recipe = Recipe {
        name: "recipe_845",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_846: Recipe = Recipe {
        name: "recipe_846",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_847: Recipe = Recipe {
        name: "recipe_847",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_848: Recipe = Recipe {
        name: "recipe_848",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_849: Recipe = Recipe {
        name: "recipe_849",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_850: Recipe = Recipe {
        name: "recipe_850",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_851: Recipe = Recipe {
        name: "recipe_851",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_852: Recipe = Recipe {
        name: "recipe_852",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_853: Recipe = Recipe {
        name: "recipe_853",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_854: Recipe = Recipe {
        name: "recipe_854",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_855: Recipe = Recipe {
        name: "recipe_855",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("carpet"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_856: Recipe = Recipe {
        name: "recipe_856",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_857: Recipe = Recipe {
        name: "recipe_857",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wooden_button"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_858: Recipe = Recipe {
        name: "recipe_858",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("chest_boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_859: Recipe = Recipe {
        name: "recipe_859",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_door"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_860: Recipe = Recipe {
        name: "recipe_860",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_861: Recipe = Recipe {
        name: "recipe_861",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence_gate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_862: Recipe = Recipe {
        name: "recipe_862",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("hanging_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_863: Recipe = Recipe {
        name: "recipe_863",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("planks"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_864: Recipe = Recipe {
        name: "recipe_864",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_pressure_plate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_865: Recipe = Recipe {
        name: "recipe_865",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_866: Recipe = Recipe {
        name: "recipe_866",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_867: Recipe = Recipe {
        name: "recipe_867",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_868: Recipe = Recipe {
        name: "recipe_868",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_trapdoor"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_869: Recipe = Recipe {
        name: "recipe_869",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_870: Recipe = Recipe {
        name: "recipe_870",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_871: Recipe = Recipe {
        name: "recipe_871",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("banner"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_872: Recipe = Recipe {
        name: "recipe_872",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bed"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_873: Recipe = Recipe {
        name: "recipe_873",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("bundle_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_874: Recipe = Recipe {
        name: "recipe_874",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("dyed_candle"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_875: Recipe = Recipe {
        name: "recipe_875",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("carpet"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_876: Recipe = Recipe {
        name: "recipe_876",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("concrete_powder"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_877: Recipe = Recipe {
        name: "recipe_877",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("pink_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_878: Recipe = Recipe {
        name: "recipe_878",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("pink_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_879: Recipe = Recipe {
        name: "recipe_879",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("pink_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_880: Recipe = Recipe {
        name: "recipe_880",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("pink_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_881: Recipe = Recipe {
        name: "recipe_881",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("pink_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_882: Recipe = Recipe {
        name: "recipe_882",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_883: Recipe = Recipe {
        name: "recipe_883",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("harness"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_884: Recipe = Recipe {
        name: "recipe_884",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("shulker_box_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_885: Recipe = Recipe {
        name: "recipe_885",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_886: Recipe = Recipe {
        name: "recipe_886",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_887: Recipe = Recipe {
        name: "recipe_887",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_888: Recipe = Recipe {
        name: "recipe_888",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_terracotta"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_889: Recipe = Recipe {
        name: "recipe_889",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_890: Recipe = Recipe {
        name: "recipe_890",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_891: Recipe = Recipe {
        name: "recipe_891",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_892: Recipe = Recipe {
        name: "recipe_892",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_893: Recipe = Recipe {
        name: "recipe_893",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_894: Recipe = Recipe {
        name: "recipe_894",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_895: Recipe = Recipe {
        name: "recipe_895",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_896: Recipe = Recipe {
        name: "recipe_896",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_897: Recipe = Recipe {
        name: "recipe_897",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_898: Recipe = Recipe {
        name: "recipe_898",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_899: Recipe = Recipe {
        name: "recipe_899",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_900: Recipe = Recipe {
        name: "recipe_900",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_901: Recipe = Recipe {
        name: "recipe_901",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_902: Recipe = Recipe {
        name: "recipe_902",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_903: Recipe = Recipe {
        name: "recipe_903",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_904: Recipe = Recipe {
        name: "recipe_904",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_905: Recipe = Recipe {
        name: "recipe_905",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_906: Recipe = Recipe {
        name: "recipe_906",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_907: Recipe = Recipe {
        name: "recipe_907",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_908: Recipe = Recipe {
        name: "recipe_908",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_909: Recipe = Recipe {
        name: "recipe_909",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_910: Recipe = Recipe {
        name: "recipe_910",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_911: Recipe = Recipe {
        name: "recipe_911",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_912: Recipe = Recipe {
        name: "recipe_912",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_913: Recipe = Recipe {
        name: "recipe_913",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_914: Recipe = Recipe {
        name: "recipe_914",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_915: Recipe = Recipe {
        name: "recipe_915",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_916: Recipe = Recipe {
        name: "recipe_916",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_917: Recipe = Recipe {
        name: "recipe_917",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_918: Recipe = Recipe {
        name: "recipe_918",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_919: Recipe = Recipe {
        name: "recipe_919",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_920: Recipe = Recipe {
        name: "recipe_920",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_921: Recipe = Recipe {
        name: "recipe_921",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_922: Recipe = Recipe {
        name: "recipe_922",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_923: Recipe = Recipe {
        name: "recipe_923",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_924: Recipe = Recipe {
        name: "recipe_924",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_925: Recipe = Recipe {
        name: "recipe_925",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_926: Recipe = Recipe {
        name: "recipe_926",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_927: Recipe = Recipe {
        name: "recipe_927",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_928: Recipe = Recipe {
        name: "recipe_928",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_929: Recipe = Recipe {
        name: "recipe_929",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_930: Recipe = Recipe {
        name: "recipe_930",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_931: Recipe = Recipe {
        name: "recipe_931",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_932: Recipe = Recipe {
        name: "recipe_932",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_933: Recipe = Recipe {
        name: "recipe_933",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_934: Recipe = Recipe {
        name: "recipe_934",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_935: Recipe = Recipe {
        name: "recipe_935",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_936: Recipe = Recipe {
        name: "recipe_936",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_937: Recipe = Recipe {
        name: "recipe_937",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_938: Recipe = Recipe {
        name: "recipe_938",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_939: Recipe = Recipe {
        name: "recipe_939",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_940: Recipe = Recipe {
        name: "recipe_940",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_941: Recipe = Recipe {
        name: "recipe_941",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_942: Recipe = Recipe {
        name: "recipe_942",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_943: Recipe = Recipe {
        name: "recipe_943",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_944: Recipe = Recipe {
        name: "recipe_944",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_945: Recipe = Recipe {
        name: "recipe_945",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_946: Recipe = Recipe {
        name: "recipe_946",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_947: Recipe = Recipe {
        name: "recipe_947",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_948: Recipe = Recipe {
        name: "recipe_948",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_949: Recipe = Recipe {
        name: "recipe_949",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_950: Recipe = Recipe {
        name: "recipe_950",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_951: Recipe = Recipe {
        name: "recipe_951",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_952: Recipe = Recipe {
        name: "recipe_952",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_953: Recipe = Recipe {
        name: "recipe_953",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_954: Recipe = Recipe {
        name: "recipe_954",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_955: Recipe = Recipe {
        name: "recipe_955",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_956: Recipe = Recipe {
        name: "recipe_956",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_957: Recipe = Recipe {
        name: "recipe_957",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_958: Recipe = Recipe {
        name: "recipe_958",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_959: Recipe = Recipe {
        name: "recipe_959",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_960: Recipe = Recipe {
        name: "recipe_960",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_961: Recipe = Recipe {
        name: "recipe_961",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_962: Recipe = Recipe {
        name: "recipe_962",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_963: Recipe = Recipe {
        name: "recipe_963",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_964: Recipe = Recipe {
        name: "recipe_964",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_965: Recipe = Recipe {
        name: "recipe_965",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_966: Recipe = Recipe {
        name: "recipe_966",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("misc"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_967: Recipe = Recipe {
        name: "recipe_967",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_968: Recipe = Recipe {
        name: "recipe_968",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_969: Recipe = Recipe {
        name: "recipe_969",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_970: Recipe = Recipe {
        name: "recipe_970",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_971: Recipe = Recipe {
        name: "recipe_971",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_972: Recipe = Recipe {
        name: "recipe_972",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_973: Recipe = Recipe {
        name: "recipe_973",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_974: Recipe = Recipe {
        name: "recipe_974",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_975: Recipe = Recipe {
        name: "recipe_975",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_976: Recipe = Recipe {
        name: "recipe_976",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_977: Recipe = Recipe {
        name: "recipe_977",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_978: Recipe = Recipe {
        name: "recipe_978",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_979: Recipe = Recipe {
        name: "recipe_979",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_980: Recipe = Recipe {
        name: "recipe_980",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_981: Recipe = Recipe {
        name: "recipe_981",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_982: Recipe = Recipe {
        name: "recipe_982",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("banner"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_983: Recipe = Recipe {
        name: "recipe_983",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bed"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_984: Recipe = Recipe {
        name: "recipe_984",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("bundle_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_985: Recipe = Recipe {
        name: "recipe_985",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("dyed_candle"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_986: Recipe = Recipe {
        name: "recipe_986",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("carpet"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_987: Recipe = Recipe {
        name: "recipe_987",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("concrete_powder"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_988: Recipe = Recipe {
        name: "recipe_988",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_989: Recipe = Recipe {
        name: "recipe_989",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_990: Recipe = Recipe {
        name: "recipe_990",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("harness"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_991: Recipe = Recipe {
        name: "recipe_991",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("shulker_box_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_992: Recipe = Recipe {
        name: "recipe_992",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_993: Recipe = Recipe {
        name: "recipe_993",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_994: Recipe = Recipe {
        name: "recipe_994",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_995: Recipe = Recipe {
        name: "recipe_995",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_terracotta"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_996: Recipe = Recipe {
        name: "recipe_996",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_997: Recipe = Recipe {
        name: "recipe_997",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_998: Recipe = Recipe {
        name: "recipe_998",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_999: Recipe = Recipe {
        name: "recipe_999",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1000: Recipe = Recipe {
        name: "recipe_1000",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1001: Recipe = Recipe {
        name: "recipe_1001",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1002: Recipe = Recipe {
        name: "recipe_1002",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1003: Recipe = Recipe {
        name: "recipe_1003",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("misc"),
        experience: Some(0.2),
        cookingtime: Some(200),
    };
    pub const RECIPE_1004: Recipe = Recipe {
        name: "recipe_1004",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1005: Recipe = Recipe {
        name: "recipe_1005",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1006: Recipe = Recipe {
        name: "recipe_1006",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1007: Recipe = Recipe {
        name: "recipe_1007",
        recipe_type: "minecraft:blasting",
        group: None,
        category: Some("misc"),
        experience: Some(0.2),
        cookingtime: Some(100),
    };
    pub const RECIPE_1008: Recipe = Recipe {
        name: "recipe_1008",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1009: Recipe = Recipe {
        name: "recipe_1009",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1010: Recipe = Recipe {
        name: "recipe_1010",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1011: Recipe = Recipe {
        name: "recipe_1011",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1012: Recipe = Recipe {
        name: "recipe_1012",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1013: Recipe = Recipe {
        name: "recipe_1013",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1014: Recipe = Recipe {
        name: "recipe_1014",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("rabbit_stew"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1015: Recipe = Recipe {
        name: "recipe_1015",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("rabbit_stew"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1016: Recipe = Recipe {
        name: "recipe_1016",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1017: Recipe = Recipe {
        name: "recipe_1017",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1018: Recipe = Recipe {
        name: "recipe_1018",
        recipe_type: "minecraft:smithing_trim",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1019: Recipe = Recipe {
        name: "recipe_1019",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1020: Recipe = Recipe {
        name: "recipe_1020",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1021: Recipe = Recipe {
        name: "recipe_1021",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1022: Recipe = Recipe {
        name: "recipe_1022",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1023: Recipe = Recipe {
        name: "recipe_1023",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1024: Recipe = Recipe {
        name: "recipe_1024",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1025: Recipe = Recipe {
        name: "recipe_1025",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1026: Recipe = Recipe {
        name: "recipe_1026",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("banner"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1027: Recipe = Recipe {
        name: "recipe_1027",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bed"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1028: Recipe = Recipe {
        name: "recipe_1028",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("bundle_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1029: Recipe = Recipe {
        name: "recipe_1029",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("dyed_candle"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1030: Recipe = Recipe {
        name: "recipe_1030",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("carpet"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1031: Recipe = Recipe {
        name: "recipe_1031",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("concrete_powder"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1032: Recipe = Recipe {
        name: "recipe_1032",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("red_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1033: Recipe = Recipe {
        name: "recipe_1033",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("red_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1034: Recipe = Recipe {
        name: "recipe_1034",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("red_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1035: Recipe = Recipe {
        name: "recipe_1035",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("red_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1036: Recipe = Recipe {
        name: "recipe_1036",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_1037: Recipe = Recipe {
        name: "recipe_1037",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("harness"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1038: Recipe = Recipe {
        name: "recipe_1038",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1039: Recipe = Recipe {
        name: "recipe_1039",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1040: Recipe = Recipe {
        name: "recipe_1040",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1041: Recipe = Recipe {
        name: "recipe_1041",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1042: Recipe = Recipe {
        name: "recipe_1042",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1043: Recipe = Recipe {
        name: "recipe_1043",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1044: Recipe = Recipe {
        name: "recipe_1044",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1045: Recipe = Recipe {
        name: "recipe_1045",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1046: Recipe = Recipe {
        name: "recipe_1046",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1047: Recipe = Recipe {
        name: "recipe_1047",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1048: Recipe = Recipe {
        name: "recipe_1048",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1049: Recipe = Recipe {
        name: "recipe_1049",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1050: Recipe = Recipe {
        name: "recipe_1050",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1051: Recipe = Recipe {
        name: "recipe_1051",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1052: Recipe = Recipe {
        name: "recipe_1052",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("shulker_box_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1053: Recipe = Recipe {
        name: "recipe_1053",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1054: Recipe = Recipe {
        name: "recipe_1054",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1055: Recipe = Recipe {
        name: "recipe_1055",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1056: Recipe = Recipe {
        name: "recipe_1056",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_terracotta"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1057: Recipe = Recipe {
        name: "recipe_1057",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1058: Recipe = Recipe {
        name: "recipe_1058",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1059: Recipe = Recipe {
        name: "recipe_1059",
        recipe_type: "minecraft:blasting",
        group: Some("redstone"),
        category: Some("blocks"),
        experience: Some(0.7),
        cookingtime: Some(100),
    };
    pub const RECIPE_1060: Recipe = Recipe {
        name: "recipe_1060",
        recipe_type: "minecraft:blasting",
        group: Some("redstone"),
        category: Some("blocks"),
        experience: Some(0.7),
        cookingtime: Some(100),
    };
    pub const RECIPE_1061: Recipe = Recipe {
        name: "recipe_1061",
        recipe_type: "minecraft:smelting",
        group: Some("redstone"),
        category: Some("blocks"),
        experience: Some(0.7),
        cookingtime: Some(200),
    };
    pub const RECIPE_1062: Recipe = Recipe {
        name: "recipe_1062",
        recipe_type: "minecraft:smelting",
        group: Some("redstone"),
        category: Some("blocks"),
        experience: Some(0.7),
        cookingtime: Some(200),
    };
    pub const RECIPE_1063: Recipe = Recipe {
        name: "recipe_1063",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1064: Recipe = Recipe {
        name: "recipe_1064",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1065: Recipe = Recipe {
        name: "recipe_1065",
        recipe_type: "minecraft:crafting_special_repairitem",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1066: Recipe = Recipe {
        name: "recipe_1066",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1067: Recipe = Recipe {
        name: "recipe_1067",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1068: Recipe = Recipe {
        name: "recipe_1068",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("misc"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_1069: Recipe = Recipe {
        name: "recipe_1069",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1070: Recipe = Recipe {
        name: "recipe_1070",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1071: Recipe = Recipe {
        name: "recipe_1071",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1072: Recipe = Recipe {
        name: "recipe_1072",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1073: Recipe = Recipe {
        name: "recipe_1073",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1074: Recipe = Recipe {
        name: "recipe_1074",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1075: Recipe = Recipe {
        name: "recipe_1075",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1076: Recipe = Recipe {
        name: "recipe_1076",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1077: Recipe = Recipe {
        name: "recipe_1077",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1078: Recipe = Recipe {
        name: "recipe_1078",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1079: Recipe = Recipe {
        name: "recipe_1079",
        recipe_type: "minecraft:smithing_trim",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1080: Recipe = Recipe {
        name: "recipe_1080",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1081: Recipe = Recipe {
        name: "recipe_1081",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1082: Recipe = Recipe {
        name: "recipe_1082",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1083: Recipe = Recipe {
        name: "recipe_1083",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1084: Recipe = Recipe {
        name: "recipe_1084",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1085: Recipe = Recipe {
        name: "recipe_1085",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1086: Recipe = Recipe {
        name: "recipe_1086",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1087: Recipe = Recipe {
        name: "recipe_1087",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1088: Recipe = Recipe {
        name: "recipe_1088",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1089: Recipe = Recipe {
        name: "recipe_1089",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1090: Recipe = Recipe {
        name: "recipe_1090",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1091: Recipe = Recipe {
        name: "recipe_1091",
        recipe_type: "minecraft:smithing_trim",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1092: Recipe = Recipe {
        name: "recipe_1092",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1093: Recipe = Recipe {
        name: "recipe_1093",
        recipe_type: "minecraft:smithing_trim",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1094: Recipe = Recipe {
        name: "recipe_1094",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1095: Recipe = Recipe {
        name: "recipe_1095",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1096: Recipe = Recipe {
        name: "recipe_1096",
        recipe_type: "minecraft:crafting_special_shielddecoration",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1097: Recipe = Recipe {
        name: "recipe_1097",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1098: Recipe = Recipe {
        name: "recipe_1098",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1099: Recipe = Recipe {
        name: "recipe_1099",
        recipe_type: "minecraft:smithing_trim",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1100: Recipe = Recipe {
        name: "recipe_1100",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1101: Recipe = Recipe {
        name: "recipe_1101",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1102: Recipe = Recipe {
        name: "recipe_1102",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1103: Recipe = Recipe {
        name: "recipe_1103",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1104: Recipe = Recipe {
        name: "recipe_1104",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1105: Recipe = Recipe {
        name: "recipe_1105",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_1106: Recipe = Recipe {
        name: "recipe_1106",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_1107: Recipe = Recipe {
        name: "recipe_1107",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1108: Recipe = Recipe {
        name: "recipe_1108",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1109: Recipe = Recipe {
        name: "recipe_1109",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1110: Recipe = Recipe {
        name: "recipe_1110",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1111: Recipe = Recipe {
        name: "recipe_1111",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_1112: Recipe = Recipe {
        name: "recipe_1112",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1113: Recipe = Recipe {
        name: "recipe_1113",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1114: Recipe = Recipe {
        name: "recipe_1114",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1115: Recipe = Recipe {
        name: "recipe_1115",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1116: Recipe = Recipe {
        name: "recipe_1116",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_1117: Recipe = Recipe {
        name: "recipe_1117",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1118: Recipe = Recipe {
        name: "recipe_1118",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1119: Recipe = Recipe {
        name: "recipe_1119",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1120: Recipe = Recipe {
        name: "recipe_1120",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1121: Recipe = Recipe {
        name: "recipe_1121",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_1122: Recipe = Recipe {
        name: "recipe_1122",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1123: Recipe = Recipe {
        name: "recipe_1123",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1124: Recipe = Recipe {
        name: "recipe_1124",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1125: Recipe = Recipe {
        name: "recipe_1125",
        recipe_type: "minecraft:smithing_trim",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1126: Recipe = Recipe {
        name: "recipe_1126",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1127: Recipe = Recipe {
        name: "recipe_1127",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1128: Recipe = Recipe {
        name: "recipe_1128",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1129: Recipe = Recipe {
        name: "recipe_1129",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1130: Recipe = Recipe {
        name: "recipe_1130",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1131: Recipe = Recipe {
        name: "recipe_1131",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1132: Recipe = Recipe {
        name: "recipe_1132",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1133: Recipe = Recipe {
        name: "recipe_1133",
        recipe_type: "minecraft:smithing_trim",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1134: Recipe = Recipe {
        name: "recipe_1134",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.15),
        cookingtime: Some(200),
    };
    pub const RECIPE_1135: Recipe = Recipe {
        name: "recipe_1135",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1136: Recipe = Recipe {
        name: "recipe_1136",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wooden_button"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1137: Recipe = Recipe {
        name: "recipe_1137",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("chest_boat"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1138: Recipe = Recipe {
        name: "recipe_1138",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_door"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1139: Recipe = Recipe {
        name: "recipe_1139",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1140: Recipe = Recipe {
        name: "recipe_1140",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence_gate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1141: Recipe = Recipe {
        name: "recipe_1141",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("hanging_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1142: Recipe = Recipe {
        name: "recipe_1142",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("planks"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1143: Recipe = Recipe {
        name: "recipe_1143",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_pressure_plate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1144: Recipe = Recipe {
        name: "recipe_1144",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1145: Recipe = Recipe {
        name: "recipe_1145",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1146: Recipe = Recipe {
        name: "recipe_1146",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1147: Recipe = Recipe {
        name: "recipe_1147",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_trapdoor"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1148: Recipe = Recipe {
        name: "recipe_1148",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1149: Recipe = Recipe {
        name: "recipe_1149",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1150: Recipe = Recipe {
        name: "recipe_1150",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("sticks"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1151: Recipe = Recipe {
        name: "recipe_1151",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("sticks"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1152: Recipe = Recipe {
        name: "recipe_1152",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1153: Recipe = Recipe {
        name: "recipe_1153",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_1154: Recipe = Recipe {
        name: "recipe_1154",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1155: Recipe = Recipe {
        name: "recipe_1155",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1156: Recipe = Recipe {
        name: "recipe_1156",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1157: Recipe = Recipe {
        name: "recipe_1157",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1158: Recipe = Recipe {
        name: "recipe_1158",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1159: Recipe = Recipe {
        name: "recipe_1159",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1160: Recipe = Recipe {
        name: "recipe_1160",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1161: Recipe = Recipe {
        name: "recipe_1161",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1162: Recipe = Recipe {
        name: "recipe_1162",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1163: Recipe = Recipe {
        name: "recipe_1163",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1164: Recipe = Recipe {
        name: "recipe_1164",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1165: Recipe = Recipe {
        name: "recipe_1165",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1166: Recipe = Recipe {
        name: "recipe_1166",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1167: Recipe = Recipe {
        name: "recipe_1167",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1168: Recipe = Recipe {
        name: "recipe_1168",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1169: Recipe = Recipe {
        name: "recipe_1169",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1170: Recipe = Recipe {
        name: "recipe_1170",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1171: Recipe = Recipe {
        name: "recipe_1171",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1172: Recipe = Recipe {
        name: "recipe_1172",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1173: Recipe = Recipe {
        name: "recipe_1173",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1174: Recipe = Recipe {
        name: "recipe_1174",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1175: Recipe = Recipe {
        name: "recipe_1175",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1176: Recipe = Recipe {
        name: "recipe_1176",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1177: Recipe = Recipe {
        name: "recipe_1177",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1178: Recipe = Recipe {
        name: "recipe_1178",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1179: Recipe = Recipe {
        name: "recipe_1179",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1180: Recipe = Recipe {
        name: "recipe_1180",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1181: Recipe = Recipe {
        name: "recipe_1181",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1182: Recipe = Recipe {
        name: "recipe_1182",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1183: Recipe = Recipe {
        name: "recipe_1183",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1184: Recipe = Recipe {
        name: "recipe_1184",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1185: Recipe = Recipe {
        name: "recipe_1185",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1186: Recipe = Recipe {
        name: "recipe_1186",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1187: Recipe = Recipe {
        name: "recipe_1187",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1188: Recipe = Recipe {
        name: "recipe_1188",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("sugar"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1189: Recipe = Recipe {
        name: "recipe_1189",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("sugar"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1190: Recipe = Recipe {
        name: "recipe_1190",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("suspicious_stew"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1191: Recipe = Recipe {
        name: "recipe_1191",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("suspicious_stew"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1192: Recipe = Recipe {
        name: "recipe_1192",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("suspicious_stew"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1193: Recipe = Recipe {
        name: "recipe_1193",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("suspicious_stew"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1194: Recipe = Recipe {
        name: "recipe_1194",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("suspicious_stew"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1195: Recipe = Recipe {
        name: "recipe_1195",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("suspicious_stew"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1196: Recipe = Recipe {
        name: "recipe_1196",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("suspicious_stew"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1197: Recipe = Recipe {
        name: "recipe_1197",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("suspicious_stew"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1198: Recipe = Recipe {
        name: "recipe_1198",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("suspicious_stew"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1199: Recipe = Recipe {
        name: "recipe_1199",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("suspicious_stew"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1200: Recipe = Recipe {
        name: "recipe_1200",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("suspicious_stew"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1201: Recipe = Recipe {
        name: "recipe_1201",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("suspicious_stew"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1202: Recipe = Recipe {
        name: "recipe_1202",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("suspicious_stew"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1203: Recipe = Recipe {
        name: "recipe_1203",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("suspicious_stew"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1204: Recipe = Recipe {
        name: "recipe_1204",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("suspicious_stew"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1205: Recipe = Recipe {
        name: "recipe_1205",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("suspicious_stew"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1206: Recipe = Recipe {
        name: "recipe_1206",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1207: Recipe = Recipe {
        name: "recipe_1207",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.35),
        cookingtime: Some(200),
    };
    pub const RECIPE_1208: Recipe = Recipe {
        name: "recipe_1208",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1209: Recipe = Recipe {
        name: "recipe_1209",
        recipe_type: "minecraft:smithing_trim",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1210: Recipe = Recipe {
        name: "recipe_1210",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1211: Recipe = Recipe {
        name: "recipe_1211",
        recipe_type: "minecraft:crafting_special_tippedarrow",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1212: Recipe = Recipe {
        name: "recipe_1212",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1213: Recipe = Recipe {
        name: "recipe_1213",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1214: Recipe = Recipe {
        name: "recipe_1214",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1215: Recipe = Recipe {
        name: "recipe_1215",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1216: Recipe = Recipe {
        name: "recipe_1216",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1217: Recipe = Recipe {
        name: "recipe_1217",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1218: Recipe = Recipe {
        name: "recipe_1218",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1219: Recipe = Recipe {
        name: "recipe_1219",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1220: Recipe = Recipe {
        name: "recipe_1220",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1221: Recipe = Recipe {
        name: "recipe_1221",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1222: Recipe = Recipe {
        name: "recipe_1222",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1223: Recipe = Recipe {
        name: "recipe_1223",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1224: Recipe = Recipe {
        name: "recipe_1224",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1225: Recipe = Recipe {
        name: "recipe_1225",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1226: Recipe = Recipe {
        name: "recipe_1226",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1227: Recipe = Recipe {
        name: "recipe_1227",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1228: Recipe = Recipe {
        name: "recipe_1228",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1229: Recipe = Recipe {
        name: "recipe_1229",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1230: Recipe = Recipe {
        name: "recipe_1230",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1231: Recipe = Recipe {
        name: "recipe_1231",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1232: Recipe = Recipe {
        name: "recipe_1232",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1233: Recipe = Recipe {
        name: "recipe_1233",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1234: Recipe = Recipe {
        name: "recipe_1234",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1235: Recipe = Recipe {
        name: "recipe_1235",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1236: Recipe = Recipe {
        name: "recipe_1236",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1237: Recipe = Recipe {
        name: "recipe_1237",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1238: Recipe = Recipe {
        name: "recipe_1238",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1239: Recipe = Recipe {
        name: "recipe_1239",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1240: Recipe = Recipe {
        name: "recipe_1240",
        recipe_type: "minecraft:smithing_trim",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1241: Recipe = Recipe {
        name: "recipe_1241",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1242: Recipe = Recipe {
        name: "recipe_1242",
        recipe_type: "minecraft:smithing_trim",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1243: Recipe = Recipe {
        name: "recipe_1243",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("wooden_button"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1244: Recipe = Recipe {
        name: "recipe_1244",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_door"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1245: Recipe = Recipe {
        name: "recipe_1245",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1246: Recipe = Recipe {
        name: "recipe_1246",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_fence_gate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1247: Recipe = Recipe {
        name: "recipe_1247",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1248: Recipe = Recipe {
        name: "recipe_1248",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("hanging_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1249: Recipe = Recipe {
        name: "recipe_1249",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bark"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1250: Recipe = Recipe {
        name: "recipe_1250",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("planks"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1251: Recipe = Recipe {
        name: "recipe_1251",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_pressure_plate"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1252: Recipe = Recipe {
        name: "recipe_1252",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_sign"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1253: Recipe = Recipe {
        name: "recipe_1253",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1254: Recipe = Recipe {
        name: "recipe_1254",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1255: Recipe = Recipe {
        name: "recipe_1255",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("wooden_trapdoor"),
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1256: Recipe = Recipe {
        name: "recipe_1256",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("waxed_cut_copper_chiseled"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1257: Recipe = Recipe {
        name: "recipe_1257",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_chiseled_copper"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1258: Recipe = Recipe {
        name: "recipe_1258",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1259: Recipe = Recipe {
        name: "recipe_1259",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1260: Recipe = Recipe {
        name: "recipe_1260",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_copper_block"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1261: Recipe = Recipe {
        name: "recipe_1261",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1262: Recipe = Recipe {
        name: "recipe_1262",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_copper_bulb"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1263: Recipe = Recipe {
        name: "recipe_1263",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_copper_door"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1264: Recipe = Recipe {
        name: "recipe_1264",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1265: Recipe = Recipe {
        name: "recipe_1265",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_copper_grate"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1266: Recipe = Recipe {
        name: "recipe_1266",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1267: Recipe = Recipe {
        name: "recipe_1267",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_copper_trapdoor"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1268: Recipe = Recipe {
        name: "recipe_1268",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("waxed_cut_copper"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1269: Recipe = Recipe {
        name: "recipe_1269",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_cut_copper"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1270: Recipe = Recipe {
        name: "recipe_1270",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1271: Recipe = Recipe {
        name: "recipe_1271",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("waxed_cut_copper_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1272: Recipe = Recipe {
        name: "recipe_1272",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_cut_copper_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1273: Recipe = Recipe {
        name: "recipe_1273",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1274: Recipe = Recipe {
        name: "recipe_1274",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1275: Recipe = Recipe {
        name: "recipe_1275",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("waxed_cut_copper_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1276: Recipe = Recipe {
        name: "recipe_1276",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_cut_copper_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1277: Recipe = Recipe {
        name: "recipe_1277",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1278: Recipe = Recipe {
        name: "recipe_1278",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1279: Recipe = Recipe {
        name: "recipe_1279",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("waxed_exposed_cut_copper_chiseled"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1280: Recipe = Recipe {
        name: "recipe_1280",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_exposed_chiseled_copper"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1281: Recipe = Recipe {
        name: "recipe_1281",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1282: Recipe = Recipe {
        name: "recipe_1282",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1283: Recipe = Recipe {
        name: "recipe_1283",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1284: Recipe = Recipe {
        name: "recipe_1284",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_exposed_copper_bulb"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1285: Recipe = Recipe {
        name: "recipe_1285",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_exposed_copper_door"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1286: Recipe = Recipe {
        name: "recipe_1286",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_exposed_copper"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1287: Recipe = Recipe {
        name: "recipe_1287",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1288: Recipe = Recipe {
        name: "recipe_1288",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_exposed_copper_grate"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1289: Recipe = Recipe {
        name: "recipe_1289",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1290: Recipe = Recipe {
        name: "recipe_1290",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_exposed_copper_trapdoor"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1291: Recipe = Recipe {
        name: "recipe_1291",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("waxed_exposed_cut_copper"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1292: Recipe = Recipe {
        name: "recipe_1292",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_exposed_cut_copper"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1293: Recipe = Recipe {
        name: "recipe_1293",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1294: Recipe = Recipe {
        name: "recipe_1294",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("waxed_exposed_cut_copper_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1295: Recipe = Recipe {
        name: "recipe_1295",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_exposed_cut_copper_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1296: Recipe = Recipe {
        name: "recipe_1296",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1297: Recipe = Recipe {
        name: "recipe_1297",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1298: Recipe = Recipe {
        name: "recipe_1298",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("waxed_exposed_cut_copper_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1299: Recipe = Recipe {
        name: "recipe_1299",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_exposed_cut_copper_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1300: Recipe = Recipe {
        name: "recipe_1300",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1301: Recipe = Recipe {
        name: "recipe_1301",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1302: Recipe = Recipe {
        name: "recipe_1302",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("waxed_oxidized_cut_copper_chiseled"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1303: Recipe = Recipe {
        name: "recipe_1303",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_oxidized_chiseled_copper"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1304: Recipe = Recipe {
        name: "recipe_1304",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1305: Recipe = Recipe {
        name: "recipe_1305",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1306: Recipe = Recipe {
        name: "recipe_1306",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1307: Recipe = Recipe {
        name: "recipe_1307",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_oxidized_copper_bulb"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1308: Recipe = Recipe {
        name: "recipe_1308",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_oxidized_copper_door"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1309: Recipe = Recipe {
        name: "recipe_1309",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_oxidized_copper"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1310: Recipe = Recipe {
        name: "recipe_1310",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1311: Recipe = Recipe {
        name: "recipe_1311",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_oxidized_copper_grate"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1312: Recipe = Recipe {
        name: "recipe_1312",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1313: Recipe = Recipe {
        name: "recipe_1313",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_oxidized_copper_trapdoor"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1314: Recipe = Recipe {
        name: "recipe_1314",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("waxed_oxidized_cut_copper"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1315: Recipe = Recipe {
        name: "recipe_1315",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_oxidized_cut_copper"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1316: Recipe = Recipe {
        name: "recipe_1316",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1317: Recipe = Recipe {
        name: "recipe_1317",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("waxed_oxidized_cut_copper_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1318: Recipe = Recipe {
        name: "recipe_1318",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_oxidized_cut_copper_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1319: Recipe = Recipe {
        name: "recipe_1319",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1320: Recipe = Recipe {
        name: "recipe_1320",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1321: Recipe = Recipe {
        name: "recipe_1321",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("waxed_oxidized_cut_copper_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1322: Recipe = Recipe {
        name: "recipe_1322",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_oxidized_cut_copper_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1323: Recipe = Recipe {
        name: "recipe_1323",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1324: Recipe = Recipe {
        name: "recipe_1324",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1325: Recipe = Recipe {
        name: "recipe_1325",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("waxed_weathered_cut_copper_chiseled"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1326: Recipe = Recipe {
        name: "recipe_1326",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_weathered_chiseled_copper"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1327: Recipe = Recipe {
        name: "recipe_1327",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1328: Recipe = Recipe {
        name: "recipe_1328",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1329: Recipe = Recipe {
        name: "recipe_1329",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1330: Recipe = Recipe {
        name: "recipe_1330",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_weathered_copper_bulb"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1331: Recipe = Recipe {
        name: "recipe_1331",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_weathered_copper_door"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1332: Recipe = Recipe {
        name: "recipe_1332",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_weathered_copper"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1333: Recipe = Recipe {
        name: "recipe_1333",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1334: Recipe = Recipe {
        name: "recipe_1334",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_weathered_copper_grate"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1335: Recipe = Recipe {
        name: "recipe_1335",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1336: Recipe = Recipe {
        name: "recipe_1336",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_weathered_copper_trapdoor"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1337: Recipe = Recipe {
        name: "recipe_1337",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("waxed_weathered_cut_copper"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1338: Recipe = Recipe {
        name: "recipe_1338",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_weathered_cut_copper"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1339: Recipe = Recipe {
        name: "recipe_1339",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1340: Recipe = Recipe {
        name: "recipe_1340",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("waxed_weathered_cut_copper_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1341: Recipe = Recipe {
        name: "recipe_1341",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_weathered_cut_copper_slab"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1342: Recipe = Recipe {
        name: "recipe_1342",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1343: Recipe = Recipe {
        name: "recipe_1343",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1344: Recipe = Recipe {
        name: "recipe_1344",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("waxed_weathered_cut_copper_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1345: Recipe = Recipe {
        name: "recipe_1345",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("waxed_weathered_cut_copper_stairs"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1346: Recipe = Recipe {
        name: "recipe_1346",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1347: Recipe = Recipe {
        name: "recipe_1347",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1348: Recipe = Recipe {
        name: "recipe_1348",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1349: Recipe = Recipe {
        name: "recipe_1349",
        recipe_type: "minecraft:smithing_trim",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1350: Recipe = Recipe {
        name: "recipe_1350",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1351: Recipe = Recipe {
        name: "recipe_1351",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1352: Recipe = Recipe {
        name: "recipe_1352",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1353: Recipe = Recipe {
        name: "recipe_1353",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("redstone"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1354: Recipe = Recipe {
        name: "recipe_1354",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1355: Recipe = Recipe {
        name: "recipe_1355",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1356: Recipe = Recipe {
        name: "recipe_1356",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1357: Recipe = Recipe {
        name: "recipe_1357",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1358: Recipe = Recipe {
        name: "recipe_1358",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1359: Recipe = Recipe {
        name: "recipe_1359",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1360: Recipe = Recipe {
        name: "recipe_1360",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1361: Recipe = Recipe {
        name: "recipe_1361",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1362: Recipe = Recipe {
        name: "recipe_1362",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1363: Recipe = Recipe {
        name: "recipe_1363",
        recipe_type: "minecraft:stonecutting",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1364: Recipe = Recipe {
        name: "recipe_1364",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1365: Recipe = Recipe {
        name: "recipe_1365",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("banner"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1366: Recipe = Recipe {
        name: "recipe_1366",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bed"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1367: Recipe = Recipe {
        name: "recipe_1367",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("bundle_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1368: Recipe = Recipe {
        name: "recipe_1368",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("dyed_candle"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1369: Recipe = Recipe {
        name: "recipe_1369",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("carpet"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1370: Recipe = Recipe {
        name: "recipe_1370",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("concrete_powder"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1371: Recipe = Recipe {
        name: "recipe_1371",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("white_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1372: Recipe = Recipe {
        name: "recipe_1372",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("white_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1373: Recipe = Recipe {
        name: "recipe_1373",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_1374: Recipe = Recipe {
        name: "recipe_1374",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("harness"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1375: Recipe = Recipe {
        name: "recipe_1375",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("shulker_box_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1376: Recipe = Recipe {
        name: "recipe_1376",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1377: Recipe = Recipe {
        name: "recipe_1377",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1378: Recipe = Recipe {
        name: "recipe_1378",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1379: Recipe = Recipe {
        name: "recipe_1379",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_terracotta"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1380: Recipe = Recipe {
        name: "recipe_1380",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1381: Recipe = Recipe {
        name: "recipe_1381",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1382: Recipe = Recipe {
        name: "recipe_1382",
        recipe_type: "minecraft:smithing_trim",
        group: None,
        category: None,
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1383: Recipe = Recipe {
        name: "recipe_1383",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1384: Recipe = Recipe {
        name: "recipe_1384",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1385: Recipe = Recipe {
        name: "recipe_1385",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1386: Recipe = Recipe {
        name: "recipe_1386",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1387: Recipe = Recipe {
        name: "recipe_1387",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1388: Recipe = Recipe {
        name: "recipe_1388",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1389: Recipe = Recipe {
        name: "recipe_1389",
        recipe_type: "minecraft:crafting_shaped",
        group: None,
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1390: Recipe = Recipe {
        name: "recipe_1390",
        recipe_type: "minecraft:crafting_shapeless",
        group: None,
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1391: Recipe = Recipe {
        name: "recipe_1391",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("banner"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1392: Recipe = Recipe {
        name: "recipe_1392",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("bed"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1393: Recipe = Recipe {
        name: "recipe_1393",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("bundle_dye"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1394: Recipe = Recipe {
        name: "recipe_1394",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("dyed_candle"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1395: Recipe = Recipe {
        name: "recipe_1395",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("carpet"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1396: Recipe = Recipe {
        name: "recipe_1396",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("concrete_powder"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1397: Recipe = Recipe {
        name: "recipe_1397",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("yellow_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1398: Recipe = Recipe {
        name: "recipe_1398",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("yellow_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1399: Recipe = Recipe {
        name: "recipe_1399",
        recipe_type: "minecraft:crafting_shapeless",
        group: Some("yellow_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1400: Recipe = Recipe {
        name: "recipe_1400",
        recipe_type: "minecraft:smelting",
        group: None,
        category: Some("blocks"),
        experience: Some(0.1),
        cookingtime: Some(200),
    };
    pub const RECIPE_1401: Recipe = Recipe {
        name: "recipe_1401",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("harness"),
        category: Some("equipment"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1402: Recipe = Recipe {
        name: "recipe_1402",
        recipe_type: "minecraft:crafting_transmute",
        group: Some("shulker_box_dye"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1403: Recipe = Recipe {
        name: "recipe_1403",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1404: Recipe = Recipe {
        name: "recipe_1404",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1405: Recipe = Recipe {
        name: "recipe_1405",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_glass_pane"),
        category: Some("misc"),
        experience: None,
        cookingtime: None,
    };
    pub const RECIPE_1406: Recipe = Recipe {
        name: "recipe_1406",
        recipe_type: "minecraft:crafting_shaped",
        group: Some("stained_terracotta"),
        category: Some("building"),
        experience: None,
        cookingtime: None,
    };
    #[doc = r" Try to parse a `Recipe` from a resource location string."]
    pub fn from_name(name: &str) -> Option<&'static Self> {
        let name = name.strip_prefix("minecraft:").unwrap_or(name);
        match name {
            "recipe_0" => Some(&Self::RECIPE_0),
            "recipe_1" => Some(&Self::RECIPE_1),
            "recipe_2" => Some(&Self::RECIPE_2),
            "recipe_3" => Some(&Self::RECIPE_3),
            "recipe_4" => Some(&Self::RECIPE_4),
            "recipe_5" => Some(&Self::RECIPE_5),
            "recipe_6" => Some(&Self::RECIPE_6),
            "recipe_7" => Some(&Self::RECIPE_7),
            "recipe_8" => Some(&Self::RECIPE_8),
            "recipe_9" => Some(&Self::RECIPE_9),
            "recipe_10" => Some(&Self::RECIPE_10),
            "recipe_11" => Some(&Self::RECIPE_11),
            "recipe_12" => Some(&Self::RECIPE_12),
            "recipe_13" => Some(&Self::RECIPE_13),
            "recipe_14" => Some(&Self::RECIPE_14),
            "recipe_15" => Some(&Self::RECIPE_15),
            "recipe_16" => Some(&Self::RECIPE_16),
            "recipe_17" => Some(&Self::RECIPE_17),
            "recipe_18" => Some(&Self::RECIPE_18),
            "recipe_19" => Some(&Self::RECIPE_19),
            "recipe_20" => Some(&Self::RECIPE_20),
            "recipe_21" => Some(&Self::RECIPE_21),
            "recipe_22" => Some(&Self::RECIPE_22),
            "recipe_23" => Some(&Self::RECIPE_23),
            "recipe_24" => Some(&Self::RECIPE_24),
            "recipe_25" => Some(&Self::RECIPE_25),
            "recipe_26" => Some(&Self::RECIPE_26),
            "recipe_27" => Some(&Self::RECIPE_27),
            "recipe_28" => Some(&Self::RECIPE_28),
            "recipe_29" => Some(&Self::RECIPE_29),
            "recipe_30" => Some(&Self::RECIPE_30),
            "recipe_31" => Some(&Self::RECIPE_31),
            "recipe_32" => Some(&Self::RECIPE_32),
            "recipe_33" => Some(&Self::RECIPE_33),
            "recipe_34" => Some(&Self::RECIPE_34),
            "recipe_35" => Some(&Self::RECIPE_35),
            "recipe_36" => Some(&Self::RECIPE_36),
            "recipe_37" => Some(&Self::RECIPE_37),
            "recipe_38" => Some(&Self::RECIPE_38),
            "recipe_39" => Some(&Self::RECIPE_39),
            "recipe_40" => Some(&Self::RECIPE_40),
            "recipe_41" => Some(&Self::RECIPE_41),
            "recipe_42" => Some(&Self::RECIPE_42),
            "recipe_43" => Some(&Self::RECIPE_43),
            "recipe_44" => Some(&Self::RECIPE_44),
            "recipe_45" => Some(&Self::RECIPE_45),
            "recipe_46" => Some(&Self::RECIPE_46),
            "recipe_47" => Some(&Self::RECIPE_47),
            "recipe_48" => Some(&Self::RECIPE_48),
            "recipe_49" => Some(&Self::RECIPE_49),
            "recipe_50" => Some(&Self::RECIPE_50),
            "recipe_51" => Some(&Self::RECIPE_51),
            "recipe_52" => Some(&Self::RECIPE_52),
            "recipe_53" => Some(&Self::RECIPE_53),
            "recipe_54" => Some(&Self::RECIPE_54),
            "recipe_55" => Some(&Self::RECIPE_55),
            "recipe_56" => Some(&Self::RECIPE_56),
            "recipe_57" => Some(&Self::RECIPE_57),
            "recipe_58" => Some(&Self::RECIPE_58),
            "recipe_59" => Some(&Self::RECIPE_59),
            "recipe_60" => Some(&Self::RECIPE_60),
            "recipe_61" => Some(&Self::RECIPE_61),
            "recipe_62" => Some(&Self::RECIPE_62),
            "recipe_63" => Some(&Self::RECIPE_63),
            "recipe_64" => Some(&Self::RECIPE_64),
            "recipe_65" => Some(&Self::RECIPE_65),
            "recipe_66" => Some(&Self::RECIPE_66),
            "recipe_67" => Some(&Self::RECIPE_67),
            "recipe_68" => Some(&Self::RECIPE_68),
            "recipe_69" => Some(&Self::RECIPE_69),
            "recipe_70" => Some(&Self::RECIPE_70),
            "recipe_71" => Some(&Self::RECIPE_71),
            "recipe_72" => Some(&Self::RECIPE_72),
            "recipe_73" => Some(&Self::RECIPE_73),
            "recipe_74" => Some(&Self::RECIPE_74),
            "recipe_75" => Some(&Self::RECIPE_75),
            "recipe_76" => Some(&Self::RECIPE_76),
            "recipe_77" => Some(&Self::RECIPE_77),
            "recipe_78" => Some(&Self::RECIPE_78),
            "recipe_79" => Some(&Self::RECIPE_79),
            "recipe_80" => Some(&Self::RECIPE_80),
            "recipe_81" => Some(&Self::RECIPE_81),
            "recipe_82" => Some(&Self::RECIPE_82),
            "recipe_83" => Some(&Self::RECIPE_83),
            "recipe_84" => Some(&Self::RECIPE_84),
            "recipe_85" => Some(&Self::RECIPE_85),
            "recipe_86" => Some(&Self::RECIPE_86),
            "recipe_87" => Some(&Self::RECIPE_87),
            "recipe_88" => Some(&Self::RECIPE_88),
            "recipe_89" => Some(&Self::RECIPE_89),
            "recipe_90" => Some(&Self::RECIPE_90),
            "recipe_91" => Some(&Self::RECIPE_91),
            "recipe_92" => Some(&Self::RECIPE_92),
            "recipe_93" => Some(&Self::RECIPE_93),
            "recipe_94" => Some(&Self::RECIPE_94),
            "recipe_95" => Some(&Self::RECIPE_95),
            "recipe_96" => Some(&Self::RECIPE_96),
            "recipe_97" => Some(&Self::RECIPE_97),
            "recipe_98" => Some(&Self::RECIPE_98),
            "recipe_99" => Some(&Self::RECIPE_99),
            "recipe_100" => Some(&Self::RECIPE_100),
            "recipe_101" => Some(&Self::RECIPE_101),
            "recipe_102" => Some(&Self::RECIPE_102),
            "recipe_103" => Some(&Self::RECIPE_103),
            "recipe_104" => Some(&Self::RECIPE_104),
            "recipe_105" => Some(&Self::RECIPE_105),
            "recipe_106" => Some(&Self::RECIPE_106),
            "recipe_107" => Some(&Self::RECIPE_107),
            "recipe_108" => Some(&Self::RECIPE_108),
            "recipe_109" => Some(&Self::RECIPE_109),
            "recipe_110" => Some(&Self::RECIPE_110),
            "recipe_111" => Some(&Self::RECIPE_111),
            "recipe_112" => Some(&Self::RECIPE_112),
            "recipe_113" => Some(&Self::RECIPE_113),
            "recipe_114" => Some(&Self::RECIPE_114),
            "recipe_115" => Some(&Self::RECIPE_115),
            "recipe_116" => Some(&Self::RECIPE_116),
            "recipe_117" => Some(&Self::RECIPE_117),
            "recipe_118" => Some(&Self::RECIPE_118),
            "recipe_119" => Some(&Self::RECIPE_119),
            "recipe_120" => Some(&Self::RECIPE_120),
            "recipe_121" => Some(&Self::RECIPE_121),
            "recipe_122" => Some(&Self::RECIPE_122),
            "recipe_123" => Some(&Self::RECIPE_123),
            "recipe_124" => Some(&Self::RECIPE_124),
            "recipe_125" => Some(&Self::RECIPE_125),
            "recipe_126" => Some(&Self::RECIPE_126),
            "recipe_127" => Some(&Self::RECIPE_127),
            "recipe_128" => Some(&Self::RECIPE_128),
            "recipe_129" => Some(&Self::RECIPE_129),
            "recipe_130" => Some(&Self::RECIPE_130),
            "recipe_131" => Some(&Self::RECIPE_131),
            "recipe_132" => Some(&Self::RECIPE_132),
            "recipe_133" => Some(&Self::RECIPE_133),
            "recipe_134" => Some(&Self::RECIPE_134),
            "recipe_135" => Some(&Self::RECIPE_135),
            "recipe_136" => Some(&Self::RECIPE_136),
            "recipe_137" => Some(&Self::RECIPE_137),
            "recipe_138" => Some(&Self::RECIPE_138),
            "recipe_139" => Some(&Self::RECIPE_139),
            "recipe_140" => Some(&Self::RECIPE_140),
            "recipe_141" => Some(&Self::RECIPE_141),
            "recipe_142" => Some(&Self::RECIPE_142),
            "recipe_143" => Some(&Self::RECIPE_143),
            "recipe_144" => Some(&Self::RECIPE_144),
            "recipe_145" => Some(&Self::RECIPE_145),
            "recipe_146" => Some(&Self::RECIPE_146),
            "recipe_147" => Some(&Self::RECIPE_147),
            "recipe_148" => Some(&Self::RECIPE_148),
            "recipe_149" => Some(&Self::RECIPE_149),
            "recipe_150" => Some(&Self::RECIPE_150),
            "recipe_151" => Some(&Self::RECIPE_151),
            "recipe_152" => Some(&Self::RECIPE_152),
            "recipe_153" => Some(&Self::RECIPE_153),
            "recipe_154" => Some(&Self::RECIPE_154),
            "recipe_155" => Some(&Self::RECIPE_155),
            "recipe_156" => Some(&Self::RECIPE_156),
            "recipe_157" => Some(&Self::RECIPE_157),
            "recipe_158" => Some(&Self::RECIPE_158),
            "recipe_159" => Some(&Self::RECIPE_159),
            "recipe_160" => Some(&Self::RECIPE_160),
            "recipe_161" => Some(&Self::RECIPE_161),
            "recipe_162" => Some(&Self::RECIPE_162),
            "recipe_163" => Some(&Self::RECIPE_163),
            "recipe_164" => Some(&Self::RECIPE_164),
            "recipe_165" => Some(&Self::RECIPE_165),
            "recipe_166" => Some(&Self::RECIPE_166),
            "recipe_167" => Some(&Self::RECIPE_167),
            "recipe_168" => Some(&Self::RECIPE_168),
            "recipe_169" => Some(&Self::RECIPE_169),
            "recipe_170" => Some(&Self::RECIPE_170),
            "recipe_171" => Some(&Self::RECIPE_171),
            "recipe_172" => Some(&Self::RECIPE_172),
            "recipe_173" => Some(&Self::RECIPE_173),
            "recipe_174" => Some(&Self::RECIPE_174),
            "recipe_175" => Some(&Self::RECIPE_175),
            "recipe_176" => Some(&Self::RECIPE_176),
            "recipe_177" => Some(&Self::RECIPE_177),
            "recipe_178" => Some(&Self::RECIPE_178),
            "recipe_179" => Some(&Self::RECIPE_179),
            "recipe_180" => Some(&Self::RECIPE_180),
            "recipe_181" => Some(&Self::RECIPE_181),
            "recipe_182" => Some(&Self::RECIPE_182),
            "recipe_183" => Some(&Self::RECIPE_183),
            "recipe_184" => Some(&Self::RECIPE_184),
            "recipe_185" => Some(&Self::RECIPE_185),
            "recipe_186" => Some(&Self::RECIPE_186),
            "recipe_187" => Some(&Self::RECIPE_187),
            "recipe_188" => Some(&Self::RECIPE_188),
            "recipe_189" => Some(&Self::RECIPE_189),
            "recipe_190" => Some(&Self::RECIPE_190),
            "recipe_191" => Some(&Self::RECIPE_191),
            "recipe_192" => Some(&Self::RECIPE_192),
            "recipe_193" => Some(&Self::RECIPE_193),
            "recipe_194" => Some(&Self::RECIPE_194),
            "recipe_195" => Some(&Self::RECIPE_195),
            "recipe_196" => Some(&Self::RECIPE_196),
            "recipe_197" => Some(&Self::RECIPE_197),
            "recipe_198" => Some(&Self::RECIPE_198),
            "recipe_199" => Some(&Self::RECIPE_199),
            "recipe_200" => Some(&Self::RECIPE_200),
            "recipe_201" => Some(&Self::RECIPE_201),
            "recipe_202" => Some(&Self::RECIPE_202),
            "recipe_203" => Some(&Self::RECIPE_203),
            "recipe_204" => Some(&Self::RECIPE_204),
            "recipe_205" => Some(&Self::RECIPE_205),
            "recipe_206" => Some(&Self::RECIPE_206),
            "recipe_207" => Some(&Self::RECIPE_207),
            "recipe_208" => Some(&Self::RECIPE_208),
            "recipe_209" => Some(&Self::RECIPE_209),
            "recipe_210" => Some(&Self::RECIPE_210),
            "recipe_211" => Some(&Self::RECIPE_211),
            "recipe_212" => Some(&Self::RECIPE_212),
            "recipe_213" => Some(&Self::RECIPE_213),
            "recipe_214" => Some(&Self::RECIPE_214),
            "recipe_215" => Some(&Self::RECIPE_215),
            "recipe_216" => Some(&Self::RECIPE_216),
            "recipe_217" => Some(&Self::RECIPE_217),
            "recipe_218" => Some(&Self::RECIPE_218),
            "recipe_219" => Some(&Self::RECIPE_219),
            "recipe_220" => Some(&Self::RECIPE_220),
            "recipe_221" => Some(&Self::RECIPE_221),
            "recipe_222" => Some(&Self::RECIPE_222),
            "recipe_223" => Some(&Self::RECIPE_223),
            "recipe_224" => Some(&Self::RECIPE_224),
            "recipe_225" => Some(&Self::RECIPE_225),
            "recipe_226" => Some(&Self::RECIPE_226),
            "recipe_227" => Some(&Self::RECIPE_227),
            "recipe_228" => Some(&Self::RECIPE_228),
            "recipe_229" => Some(&Self::RECIPE_229),
            "recipe_230" => Some(&Self::RECIPE_230),
            "recipe_231" => Some(&Self::RECIPE_231),
            "recipe_232" => Some(&Self::RECIPE_232),
            "recipe_233" => Some(&Self::RECIPE_233),
            "recipe_234" => Some(&Self::RECIPE_234),
            "recipe_235" => Some(&Self::RECIPE_235),
            "recipe_236" => Some(&Self::RECIPE_236),
            "recipe_237" => Some(&Self::RECIPE_237),
            "recipe_238" => Some(&Self::RECIPE_238),
            "recipe_239" => Some(&Self::RECIPE_239),
            "recipe_240" => Some(&Self::RECIPE_240),
            "recipe_241" => Some(&Self::RECIPE_241),
            "recipe_242" => Some(&Self::RECIPE_242),
            "recipe_243" => Some(&Self::RECIPE_243),
            "recipe_244" => Some(&Self::RECIPE_244),
            "recipe_245" => Some(&Self::RECIPE_245),
            "recipe_246" => Some(&Self::RECIPE_246),
            "recipe_247" => Some(&Self::RECIPE_247),
            "recipe_248" => Some(&Self::RECIPE_248),
            "recipe_249" => Some(&Self::RECIPE_249),
            "recipe_250" => Some(&Self::RECIPE_250),
            "recipe_251" => Some(&Self::RECIPE_251),
            "recipe_252" => Some(&Self::RECIPE_252),
            "recipe_253" => Some(&Self::RECIPE_253),
            "recipe_254" => Some(&Self::RECIPE_254),
            "recipe_255" => Some(&Self::RECIPE_255),
            "recipe_256" => Some(&Self::RECIPE_256),
            "recipe_257" => Some(&Self::RECIPE_257),
            "recipe_258" => Some(&Self::RECIPE_258),
            "recipe_259" => Some(&Self::RECIPE_259),
            "recipe_260" => Some(&Self::RECIPE_260),
            "recipe_261" => Some(&Self::RECIPE_261),
            "recipe_262" => Some(&Self::RECIPE_262),
            "recipe_263" => Some(&Self::RECIPE_263),
            "recipe_264" => Some(&Self::RECIPE_264),
            "recipe_265" => Some(&Self::RECIPE_265),
            "recipe_266" => Some(&Self::RECIPE_266),
            "recipe_267" => Some(&Self::RECIPE_267),
            "recipe_268" => Some(&Self::RECIPE_268),
            "recipe_269" => Some(&Self::RECIPE_269),
            "recipe_270" => Some(&Self::RECIPE_270),
            "recipe_271" => Some(&Self::RECIPE_271),
            "recipe_272" => Some(&Self::RECIPE_272),
            "recipe_273" => Some(&Self::RECIPE_273),
            "recipe_274" => Some(&Self::RECIPE_274),
            "recipe_275" => Some(&Self::RECIPE_275),
            "recipe_276" => Some(&Self::RECIPE_276),
            "recipe_277" => Some(&Self::RECIPE_277),
            "recipe_278" => Some(&Self::RECIPE_278),
            "recipe_279" => Some(&Self::RECIPE_279),
            "recipe_280" => Some(&Self::RECIPE_280),
            "recipe_281" => Some(&Self::RECIPE_281),
            "recipe_282" => Some(&Self::RECIPE_282),
            "recipe_283" => Some(&Self::RECIPE_283),
            "recipe_284" => Some(&Self::RECIPE_284),
            "recipe_285" => Some(&Self::RECIPE_285),
            "recipe_286" => Some(&Self::RECIPE_286),
            "recipe_287" => Some(&Self::RECIPE_287),
            "recipe_288" => Some(&Self::RECIPE_288),
            "recipe_289" => Some(&Self::RECIPE_289),
            "recipe_290" => Some(&Self::RECIPE_290),
            "recipe_291" => Some(&Self::RECIPE_291),
            "recipe_292" => Some(&Self::RECIPE_292),
            "recipe_293" => Some(&Self::RECIPE_293),
            "recipe_294" => Some(&Self::RECIPE_294),
            "recipe_295" => Some(&Self::RECIPE_295),
            "recipe_296" => Some(&Self::RECIPE_296),
            "recipe_297" => Some(&Self::RECIPE_297),
            "recipe_298" => Some(&Self::RECIPE_298),
            "recipe_299" => Some(&Self::RECIPE_299),
            "recipe_300" => Some(&Self::RECIPE_300),
            "recipe_301" => Some(&Self::RECIPE_301),
            "recipe_302" => Some(&Self::RECIPE_302),
            "recipe_303" => Some(&Self::RECIPE_303),
            "recipe_304" => Some(&Self::RECIPE_304),
            "recipe_305" => Some(&Self::RECIPE_305),
            "recipe_306" => Some(&Self::RECIPE_306),
            "recipe_307" => Some(&Self::RECIPE_307),
            "recipe_308" => Some(&Self::RECIPE_308),
            "recipe_309" => Some(&Self::RECIPE_309),
            "recipe_310" => Some(&Self::RECIPE_310),
            "recipe_311" => Some(&Self::RECIPE_311),
            "recipe_312" => Some(&Self::RECIPE_312),
            "recipe_313" => Some(&Self::RECIPE_313),
            "recipe_314" => Some(&Self::RECIPE_314),
            "recipe_315" => Some(&Self::RECIPE_315),
            "recipe_316" => Some(&Self::RECIPE_316),
            "recipe_317" => Some(&Self::RECIPE_317),
            "recipe_318" => Some(&Self::RECIPE_318),
            "recipe_319" => Some(&Self::RECIPE_319),
            "recipe_320" => Some(&Self::RECIPE_320),
            "recipe_321" => Some(&Self::RECIPE_321),
            "recipe_322" => Some(&Self::RECIPE_322),
            "recipe_323" => Some(&Self::RECIPE_323),
            "recipe_324" => Some(&Self::RECIPE_324),
            "recipe_325" => Some(&Self::RECIPE_325),
            "recipe_326" => Some(&Self::RECIPE_326),
            "recipe_327" => Some(&Self::RECIPE_327),
            "recipe_328" => Some(&Self::RECIPE_328),
            "recipe_329" => Some(&Self::RECIPE_329),
            "recipe_330" => Some(&Self::RECIPE_330),
            "recipe_331" => Some(&Self::RECIPE_331),
            "recipe_332" => Some(&Self::RECIPE_332),
            "recipe_333" => Some(&Self::RECIPE_333),
            "recipe_334" => Some(&Self::RECIPE_334),
            "recipe_335" => Some(&Self::RECIPE_335),
            "recipe_336" => Some(&Self::RECIPE_336),
            "recipe_337" => Some(&Self::RECIPE_337),
            "recipe_338" => Some(&Self::RECIPE_338),
            "recipe_339" => Some(&Self::RECIPE_339),
            "recipe_340" => Some(&Self::RECIPE_340),
            "recipe_341" => Some(&Self::RECIPE_341),
            "recipe_342" => Some(&Self::RECIPE_342),
            "recipe_343" => Some(&Self::RECIPE_343),
            "recipe_344" => Some(&Self::RECIPE_344),
            "recipe_345" => Some(&Self::RECIPE_345),
            "recipe_346" => Some(&Self::RECIPE_346),
            "recipe_347" => Some(&Self::RECIPE_347),
            "recipe_348" => Some(&Self::RECIPE_348),
            "recipe_349" => Some(&Self::RECIPE_349),
            "recipe_350" => Some(&Self::RECIPE_350),
            "recipe_351" => Some(&Self::RECIPE_351),
            "recipe_352" => Some(&Self::RECIPE_352),
            "recipe_353" => Some(&Self::RECIPE_353),
            "recipe_354" => Some(&Self::RECIPE_354),
            "recipe_355" => Some(&Self::RECIPE_355),
            "recipe_356" => Some(&Self::RECIPE_356),
            "recipe_357" => Some(&Self::RECIPE_357),
            "recipe_358" => Some(&Self::RECIPE_358),
            "recipe_359" => Some(&Self::RECIPE_359),
            "recipe_360" => Some(&Self::RECIPE_360),
            "recipe_361" => Some(&Self::RECIPE_361),
            "recipe_362" => Some(&Self::RECIPE_362),
            "recipe_363" => Some(&Self::RECIPE_363),
            "recipe_364" => Some(&Self::RECIPE_364),
            "recipe_365" => Some(&Self::RECIPE_365),
            "recipe_366" => Some(&Self::RECIPE_366),
            "recipe_367" => Some(&Self::RECIPE_367),
            "recipe_368" => Some(&Self::RECIPE_368),
            "recipe_369" => Some(&Self::RECIPE_369),
            "recipe_370" => Some(&Self::RECIPE_370),
            "recipe_371" => Some(&Self::RECIPE_371),
            "recipe_372" => Some(&Self::RECIPE_372),
            "recipe_373" => Some(&Self::RECIPE_373),
            "recipe_374" => Some(&Self::RECIPE_374),
            "recipe_375" => Some(&Self::RECIPE_375),
            "recipe_376" => Some(&Self::RECIPE_376),
            "recipe_377" => Some(&Self::RECIPE_377),
            "recipe_378" => Some(&Self::RECIPE_378),
            "recipe_379" => Some(&Self::RECIPE_379),
            "recipe_380" => Some(&Self::RECIPE_380),
            "recipe_381" => Some(&Self::RECIPE_381),
            "recipe_382" => Some(&Self::RECIPE_382),
            "recipe_383" => Some(&Self::RECIPE_383),
            "recipe_384" => Some(&Self::RECIPE_384),
            "recipe_385" => Some(&Self::RECIPE_385),
            "recipe_386" => Some(&Self::RECIPE_386),
            "recipe_387" => Some(&Self::RECIPE_387),
            "recipe_388" => Some(&Self::RECIPE_388),
            "recipe_389" => Some(&Self::RECIPE_389),
            "recipe_390" => Some(&Self::RECIPE_390),
            "recipe_391" => Some(&Self::RECIPE_391),
            "recipe_392" => Some(&Self::RECIPE_392),
            "recipe_393" => Some(&Self::RECIPE_393),
            "recipe_394" => Some(&Self::RECIPE_394),
            "recipe_395" => Some(&Self::RECIPE_395),
            "recipe_396" => Some(&Self::RECIPE_396),
            "recipe_397" => Some(&Self::RECIPE_397),
            "recipe_398" => Some(&Self::RECIPE_398),
            "recipe_399" => Some(&Self::RECIPE_399),
            "recipe_400" => Some(&Self::RECIPE_400),
            "recipe_401" => Some(&Self::RECIPE_401),
            "recipe_402" => Some(&Self::RECIPE_402),
            "recipe_403" => Some(&Self::RECIPE_403),
            "recipe_404" => Some(&Self::RECIPE_404),
            "recipe_405" => Some(&Self::RECIPE_405),
            "recipe_406" => Some(&Self::RECIPE_406),
            "recipe_407" => Some(&Self::RECIPE_407),
            "recipe_408" => Some(&Self::RECIPE_408),
            "recipe_409" => Some(&Self::RECIPE_409),
            "recipe_410" => Some(&Self::RECIPE_410),
            "recipe_411" => Some(&Self::RECIPE_411),
            "recipe_412" => Some(&Self::RECIPE_412),
            "recipe_413" => Some(&Self::RECIPE_413),
            "recipe_414" => Some(&Self::RECIPE_414),
            "recipe_415" => Some(&Self::RECIPE_415),
            "recipe_416" => Some(&Self::RECIPE_416),
            "recipe_417" => Some(&Self::RECIPE_417),
            "recipe_418" => Some(&Self::RECIPE_418),
            "recipe_419" => Some(&Self::RECIPE_419),
            "recipe_420" => Some(&Self::RECIPE_420),
            "recipe_421" => Some(&Self::RECIPE_421),
            "recipe_422" => Some(&Self::RECIPE_422),
            "recipe_423" => Some(&Self::RECIPE_423),
            "recipe_424" => Some(&Self::RECIPE_424),
            "recipe_425" => Some(&Self::RECIPE_425),
            "recipe_426" => Some(&Self::RECIPE_426),
            "recipe_427" => Some(&Self::RECIPE_427),
            "recipe_428" => Some(&Self::RECIPE_428),
            "recipe_429" => Some(&Self::RECIPE_429),
            "recipe_430" => Some(&Self::RECIPE_430),
            "recipe_431" => Some(&Self::RECIPE_431),
            "recipe_432" => Some(&Self::RECIPE_432),
            "recipe_433" => Some(&Self::RECIPE_433),
            "recipe_434" => Some(&Self::RECIPE_434),
            "recipe_435" => Some(&Self::RECIPE_435),
            "recipe_436" => Some(&Self::RECIPE_436),
            "recipe_437" => Some(&Self::RECIPE_437),
            "recipe_438" => Some(&Self::RECIPE_438),
            "recipe_439" => Some(&Self::RECIPE_439),
            "recipe_440" => Some(&Self::RECIPE_440),
            "recipe_441" => Some(&Self::RECIPE_441),
            "recipe_442" => Some(&Self::RECIPE_442),
            "recipe_443" => Some(&Self::RECIPE_443),
            "recipe_444" => Some(&Self::RECIPE_444),
            "recipe_445" => Some(&Self::RECIPE_445),
            "recipe_446" => Some(&Self::RECIPE_446),
            "recipe_447" => Some(&Self::RECIPE_447),
            "recipe_448" => Some(&Self::RECIPE_448),
            "recipe_449" => Some(&Self::RECIPE_449),
            "recipe_450" => Some(&Self::RECIPE_450),
            "recipe_451" => Some(&Self::RECIPE_451),
            "recipe_452" => Some(&Self::RECIPE_452),
            "recipe_453" => Some(&Self::RECIPE_453),
            "recipe_454" => Some(&Self::RECIPE_454),
            "recipe_455" => Some(&Self::RECIPE_455),
            "recipe_456" => Some(&Self::RECIPE_456),
            "recipe_457" => Some(&Self::RECIPE_457),
            "recipe_458" => Some(&Self::RECIPE_458),
            "recipe_459" => Some(&Self::RECIPE_459),
            "recipe_460" => Some(&Self::RECIPE_460),
            "recipe_461" => Some(&Self::RECIPE_461),
            "recipe_462" => Some(&Self::RECIPE_462),
            "recipe_463" => Some(&Self::RECIPE_463),
            "recipe_464" => Some(&Self::RECIPE_464),
            "recipe_465" => Some(&Self::RECIPE_465),
            "recipe_466" => Some(&Self::RECIPE_466),
            "recipe_467" => Some(&Self::RECIPE_467),
            "recipe_468" => Some(&Self::RECIPE_468),
            "recipe_469" => Some(&Self::RECIPE_469),
            "recipe_470" => Some(&Self::RECIPE_470),
            "recipe_471" => Some(&Self::RECIPE_471),
            "recipe_472" => Some(&Self::RECIPE_472),
            "recipe_473" => Some(&Self::RECIPE_473),
            "recipe_474" => Some(&Self::RECIPE_474),
            "recipe_475" => Some(&Self::RECIPE_475),
            "recipe_476" => Some(&Self::RECIPE_476),
            "recipe_477" => Some(&Self::RECIPE_477),
            "recipe_478" => Some(&Self::RECIPE_478),
            "recipe_479" => Some(&Self::RECIPE_479),
            "recipe_480" => Some(&Self::RECIPE_480),
            "recipe_481" => Some(&Self::RECIPE_481),
            "recipe_482" => Some(&Self::RECIPE_482),
            "recipe_483" => Some(&Self::RECIPE_483),
            "recipe_484" => Some(&Self::RECIPE_484),
            "recipe_485" => Some(&Self::RECIPE_485),
            "recipe_486" => Some(&Self::RECIPE_486),
            "recipe_487" => Some(&Self::RECIPE_487),
            "recipe_488" => Some(&Self::RECIPE_488),
            "recipe_489" => Some(&Self::RECIPE_489),
            "recipe_490" => Some(&Self::RECIPE_490),
            "recipe_491" => Some(&Self::RECIPE_491),
            "recipe_492" => Some(&Self::RECIPE_492),
            "recipe_493" => Some(&Self::RECIPE_493),
            "recipe_494" => Some(&Self::RECIPE_494),
            "recipe_495" => Some(&Self::RECIPE_495),
            "recipe_496" => Some(&Self::RECIPE_496),
            "recipe_497" => Some(&Self::RECIPE_497),
            "recipe_498" => Some(&Self::RECIPE_498),
            "recipe_499" => Some(&Self::RECIPE_499),
            "recipe_500" => Some(&Self::RECIPE_500),
            "recipe_501" => Some(&Self::RECIPE_501),
            "recipe_502" => Some(&Self::RECIPE_502),
            "recipe_503" => Some(&Self::RECIPE_503),
            "recipe_504" => Some(&Self::RECIPE_504),
            "recipe_505" => Some(&Self::RECIPE_505),
            "recipe_506" => Some(&Self::RECIPE_506),
            "recipe_507" => Some(&Self::RECIPE_507),
            "recipe_508" => Some(&Self::RECIPE_508),
            "recipe_509" => Some(&Self::RECIPE_509),
            "recipe_510" => Some(&Self::RECIPE_510),
            "recipe_511" => Some(&Self::RECIPE_511),
            "recipe_512" => Some(&Self::RECIPE_512),
            "recipe_513" => Some(&Self::RECIPE_513),
            "recipe_514" => Some(&Self::RECIPE_514),
            "recipe_515" => Some(&Self::RECIPE_515),
            "recipe_516" => Some(&Self::RECIPE_516),
            "recipe_517" => Some(&Self::RECIPE_517),
            "recipe_518" => Some(&Self::RECIPE_518),
            "recipe_519" => Some(&Self::RECIPE_519),
            "recipe_520" => Some(&Self::RECIPE_520),
            "recipe_521" => Some(&Self::RECIPE_521),
            "recipe_522" => Some(&Self::RECIPE_522),
            "recipe_523" => Some(&Self::RECIPE_523),
            "recipe_524" => Some(&Self::RECIPE_524),
            "recipe_525" => Some(&Self::RECIPE_525),
            "recipe_526" => Some(&Self::RECIPE_526),
            "recipe_527" => Some(&Self::RECIPE_527),
            "recipe_528" => Some(&Self::RECIPE_528),
            "recipe_529" => Some(&Self::RECIPE_529),
            "recipe_530" => Some(&Self::RECIPE_530),
            "recipe_531" => Some(&Self::RECIPE_531),
            "recipe_532" => Some(&Self::RECIPE_532),
            "recipe_533" => Some(&Self::RECIPE_533),
            "recipe_534" => Some(&Self::RECIPE_534),
            "recipe_535" => Some(&Self::RECIPE_535),
            "recipe_536" => Some(&Self::RECIPE_536),
            "recipe_537" => Some(&Self::RECIPE_537),
            "recipe_538" => Some(&Self::RECIPE_538),
            "recipe_539" => Some(&Self::RECIPE_539),
            "recipe_540" => Some(&Self::RECIPE_540),
            "recipe_541" => Some(&Self::RECIPE_541),
            "recipe_542" => Some(&Self::RECIPE_542),
            "recipe_543" => Some(&Self::RECIPE_543),
            "recipe_544" => Some(&Self::RECIPE_544),
            "recipe_545" => Some(&Self::RECIPE_545),
            "recipe_546" => Some(&Self::RECIPE_546),
            "recipe_547" => Some(&Self::RECIPE_547),
            "recipe_548" => Some(&Self::RECIPE_548),
            "recipe_549" => Some(&Self::RECIPE_549),
            "recipe_550" => Some(&Self::RECIPE_550),
            "recipe_551" => Some(&Self::RECIPE_551),
            "recipe_552" => Some(&Self::RECIPE_552),
            "recipe_553" => Some(&Self::RECIPE_553),
            "recipe_554" => Some(&Self::RECIPE_554),
            "recipe_555" => Some(&Self::RECIPE_555),
            "recipe_556" => Some(&Self::RECIPE_556),
            "recipe_557" => Some(&Self::RECIPE_557),
            "recipe_558" => Some(&Self::RECIPE_558),
            "recipe_559" => Some(&Self::RECIPE_559),
            "recipe_560" => Some(&Self::RECIPE_560),
            "recipe_561" => Some(&Self::RECIPE_561),
            "recipe_562" => Some(&Self::RECIPE_562),
            "recipe_563" => Some(&Self::RECIPE_563),
            "recipe_564" => Some(&Self::RECIPE_564),
            "recipe_565" => Some(&Self::RECIPE_565),
            "recipe_566" => Some(&Self::RECIPE_566),
            "recipe_567" => Some(&Self::RECIPE_567),
            "recipe_568" => Some(&Self::RECIPE_568),
            "recipe_569" => Some(&Self::RECIPE_569),
            "recipe_570" => Some(&Self::RECIPE_570),
            "recipe_571" => Some(&Self::RECIPE_571),
            "recipe_572" => Some(&Self::RECIPE_572),
            "recipe_573" => Some(&Self::RECIPE_573),
            "recipe_574" => Some(&Self::RECIPE_574),
            "recipe_575" => Some(&Self::RECIPE_575),
            "recipe_576" => Some(&Self::RECIPE_576),
            "recipe_577" => Some(&Self::RECIPE_577),
            "recipe_578" => Some(&Self::RECIPE_578),
            "recipe_579" => Some(&Self::RECIPE_579),
            "recipe_580" => Some(&Self::RECIPE_580),
            "recipe_581" => Some(&Self::RECIPE_581),
            "recipe_582" => Some(&Self::RECIPE_582),
            "recipe_583" => Some(&Self::RECIPE_583),
            "recipe_584" => Some(&Self::RECIPE_584),
            "recipe_585" => Some(&Self::RECIPE_585),
            "recipe_586" => Some(&Self::RECIPE_586),
            "recipe_587" => Some(&Self::RECIPE_587),
            "recipe_588" => Some(&Self::RECIPE_588),
            "recipe_589" => Some(&Self::RECIPE_589),
            "recipe_590" => Some(&Self::RECIPE_590),
            "recipe_591" => Some(&Self::RECIPE_591),
            "recipe_592" => Some(&Self::RECIPE_592),
            "recipe_593" => Some(&Self::RECIPE_593),
            "recipe_594" => Some(&Self::RECIPE_594),
            "recipe_595" => Some(&Self::RECIPE_595),
            "recipe_596" => Some(&Self::RECIPE_596),
            "recipe_597" => Some(&Self::RECIPE_597),
            "recipe_598" => Some(&Self::RECIPE_598),
            "recipe_599" => Some(&Self::RECIPE_599),
            "recipe_600" => Some(&Self::RECIPE_600),
            "recipe_601" => Some(&Self::RECIPE_601),
            "recipe_602" => Some(&Self::RECIPE_602),
            "recipe_603" => Some(&Self::RECIPE_603),
            "recipe_604" => Some(&Self::RECIPE_604),
            "recipe_605" => Some(&Self::RECIPE_605),
            "recipe_606" => Some(&Self::RECIPE_606),
            "recipe_607" => Some(&Self::RECIPE_607),
            "recipe_608" => Some(&Self::RECIPE_608),
            "recipe_609" => Some(&Self::RECIPE_609),
            "recipe_610" => Some(&Self::RECIPE_610),
            "recipe_611" => Some(&Self::RECIPE_611),
            "recipe_612" => Some(&Self::RECIPE_612),
            "recipe_613" => Some(&Self::RECIPE_613),
            "recipe_614" => Some(&Self::RECIPE_614),
            "recipe_615" => Some(&Self::RECIPE_615),
            "recipe_616" => Some(&Self::RECIPE_616),
            "recipe_617" => Some(&Self::RECIPE_617),
            "recipe_618" => Some(&Self::RECIPE_618),
            "recipe_619" => Some(&Self::RECIPE_619),
            "recipe_620" => Some(&Self::RECIPE_620),
            "recipe_621" => Some(&Self::RECIPE_621),
            "recipe_622" => Some(&Self::RECIPE_622),
            "recipe_623" => Some(&Self::RECIPE_623),
            "recipe_624" => Some(&Self::RECIPE_624),
            "recipe_625" => Some(&Self::RECIPE_625),
            "recipe_626" => Some(&Self::RECIPE_626),
            "recipe_627" => Some(&Self::RECIPE_627),
            "recipe_628" => Some(&Self::RECIPE_628),
            "recipe_629" => Some(&Self::RECIPE_629),
            "recipe_630" => Some(&Self::RECIPE_630),
            "recipe_631" => Some(&Self::RECIPE_631),
            "recipe_632" => Some(&Self::RECIPE_632),
            "recipe_633" => Some(&Self::RECIPE_633),
            "recipe_634" => Some(&Self::RECIPE_634),
            "recipe_635" => Some(&Self::RECIPE_635),
            "recipe_636" => Some(&Self::RECIPE_636),
            "recipe_637" => Some(&Self::RECIPE_637),
            "recipe_638" => Some(&Self::RECIPE_638),
            "recipe_639" => Some(&Self::RECIPE_639),
            "recipe_640" => Some(&Self::RECIPE_640),
            "recipe_641" => Some(&Self::RECIPE_641),
            "recipe_642" => Some(&Self::RECIPE_642),
            "recipe_643" => Some(&Self::RECIPE_643),
            "recipe_644" => Some(&Self::RECIPE_644),
            "recipe_645" => Some(&Self::RECIPE_645),
            "recipe_646" => Some(&Self::RECIPE_646),
            "recipe_647" => Some(&Self::RECIPE_647),
            "recipe_648" => Some(&Self::RECIPE_648),
            "recipe_649" => Some(&Self::RECIPE_649),
            "recipe_650" => Some(&Self::RECIPE_650),
            "recipe_651" => Some(&Self::RECIPE_651),
            "recipe_652" => Some(&Self::RECIPE_652),
            "recipe_653" => Some(&Self::RECIPE_653),
            "recipe_654" => Some(&Self::RECIPE_654),
            "recipe_655" => Some(&Self::RECIPE_655),
            "recipe_656" => Some(&Self::RECIPE_656),
            "recipe_657" => Some(&Self::RECIPE_657),
            "recipe_658" => Some(&Self::RECIPE_658),
            "recipe_659" => Some(&Self::RECIPE_659),
            "recipe_660" => Some(&Self::RECIPE_660),
            "recipe_661" => Some(&Self::RECIPE_661),
            "recipe_662" => Some(&Self::RECIPE_662),
            "recipe_663" => Some(&Self::RECIPE_663),
            "recipe_664" => Some(&Self::RECIPE_664),
            "recipe_665" => Some(&Self::RECIPE_665),
            "recipe_666" => Some(&Self::RECIPE_666),
            "recipe_667" => Some(&Self::RECIPE_667),
            "recipe_668" => Some(&Self::RECIPE_668),
            "recipe_669" => Some(&Self::RECIPE_669),
            "recipe_670" => Some(&Self::RECIPE_670),
            "recipe_671" => Some(&Self::RECIPE_671),
            "recipe_672" => Some(&Self::RECIPE_672),
            "recipe_673" => Some(&Self::RECIPE_673),
            "recipe_674" => Some(&Self::RECIPE_674),
            "recipe_675" => Some(&Self::RECIPE_675),
            "recipe_676" => Some(&Self::RECIPE_676),
            "recipe_677" => Some(&Self::RECIPE_677),
            "recipe_678" => Some(&Self::RECIPE_678),
            "recipe_679" => Some(&Self::RECIPE_679),
            "recipe_680" => Some(&Self::RECIPE_680),
            "recipe_681" => Some(&Self::RECIPE_681),
            "recipe_682" => Some(&Self::RECIPE_682),
            "recipe_683" => Some(&Self::RECIPE_683),
            "recipe_684" => Some(&Self::RECIPE_684),
            "recipe_685" => Some(&Self::RECIPE_685),
            "recipe_686" => Some(&Self::RECIPE_686),
            "recipe_687" => Some(&Self::RECIPE_687),
            "recipe_688" => Some(&Self::RECIPE_688),
            "recipe_689" => Some(&Self::RECIPE_689),
            "recipe_690" => Some(&Self::RECIPE_690),
            "recipe_691" => Some(&Self::RECIPE_691),
            "recipe_692" => Some(&Self::RECIPE_692),
            "recipe_693" => Some(&Self::RECIPE_693),
            "recipe_694" => Some(&Self::RECIPE_694),
            "recipe_695" => Some(&Self::RECIPE_695),
            "recipe_696" => Some(&Self::RECIPE_696),
            "recipe_697" => Some(&Self::RECIPE_697),
            "recipe_698" => Some(&Self::RECIPE_698),
            "recipe_699" => Some(&Self::RECIPE_699),
            "recipe_700" => Some(&Self::RECIPE_700),
            "recipe_701" => Some(&Self::RECIPE_701),
            "recipe_702" => Some(&Self::RECIPE_702),
            "recipe_703" => Some(&Self::RECIPE_703),
            "recipe_704" => Some(&Self::RECIPE_704),
            "recipe_705" => Some(&Self::RECIPE_705),
            "recipe_706" => Some(&Self::RECIPE_706),
            "recipe_707" => Some(&Self::RECIPE_707),
            "recipe_708" => Some(&Self::RECIPE_708),
            "recipe_709" => Some(&Self::RECIPE_709),
            "recipe_710" => Some(&Self::RECIPE_710),
            "recipe_711" => Some(&Self::RECIPE_711),
            "recipe_712" => Some(&Self::RECIPE_712),
            "recipe_713" => Some(&Self::RECIPE_713),
            "recipe_714" => Some(&Self::RECIPE_714),
            "recipe_715" => Some(&Self::RECIPE_715),
            "recipe_716" => Some(&Self::RECIPE_716),
            "recipe_717" => Some(&Self::RECIPE_717),
            "recipe_718" => Some(&Self::RECIPE_718),
            "recipe_719" => Some(&Self::RECIPE_719),
            "recipe_720" => Some(&Self::RECIPE_720),
            "recipe_721" => Some(&Self::RECIPE_721),
            "recipe_722" => Some(&Self::RECIPE_722),
            "recipe_723" => Some(&Self::RECIPE_723),
            "recipe_724" => Some(&Self::RECIPE_724),
            "recipe_725" => Some(&Self::RECIPE_725),
            "recipe_726" => Some(&Self::RECIPE_726),
            "recipe_727" => Some(&Self::RECIPE_727),
            "recipe_728" => Some(&Self::RECIPE_728),
            "recipe_729" => Some(&Self::RECIPE_729),
            "recipe_730" => Some(&Self::RECIPE_730),
            "recipe_731" => Some(&Self::RECIPE_731),
            "recipe_732" => Some(&Self::RECIPE_732),
            "recipe_733" => Some(&Self::RECIPE_733),
            "recipe_734" => Some(&Self::RECIPE_734),
            "recipe_735" => Some(&Self::RECIPE_735),
            "recipe_736" => Some(&Self::RECIPE_736),
            "recipe_737" => Some(&Self::RECIPE_737),
            "recipe_738" => Some(&Self::RECIPE_738),
            "recipe_739" => Some(&Self::RECIPE_739),
            "recipe_740" => Some(&Self::RECIPE_740),
            "recipe_741" => Some(&Self::RECIPE_741),
            "recipe_742" => Some(&Self::RECIPE_742),
            "recipe_743" => Some(&Self::RECIPE_743),
            "recipe_744" => Some(&Self::RECIPE_744),
            "recipe_745" => Some(&Self::RECIPE_745),
            "recipe_746" => Some(&Self::RECIPE_746),
            "recipe_747" => Some(&Self::RECIPE_747),
            "recipe_748" => Some(&Self::RECIPE_748),
            "recipe_749" => Some(&Self::RECIPE_749),
            "recipe_750" => Some(&Self::RECIPE_750),
            "recipe_751" => Some(&Self::RECIPE_751),
            "recipe_752" => Some(&Self::RECIPE_752),
            "recipe_753" => Some(&Self::RECIPE_753),
            "recipe_754" => Some(&Self::RECIPE_754),
            "recipe_755" => Some(&Self::RECIPE_755),
            "recipe_756" => Some(&Self::RECIPE_756),
            "recipe_757" => Some(&Self::RECIPE_757),
            "recipe_758" => Some(&Self::RECIPE_758),
            "recipe_759" => Some(&Self::RECIPE_759),
            "recipe_760" => Some(&Self::RECIPE_760),
            "recipe_761" => Some(&Self::RECIPE_761),
            "recipe_762" => Some(&Self::RECIPE_762),
            "recipe_763" => Some(&Self::RECIPE_763),
            "recipe_764" => Some(&Self::RECIPE_764),
            "recipe_765" => Some(&Self::RECIPE_765),
            "recipe_766" => Some(&Self::RECIPE_766),
            "recipe_767" => Some(&Self::RECIPE_767),
            "recipe_768" => Some(&Self::RECIPE_768),
            "recipe_769" => Some(&Self::RECIPE_769),
            "recipe_770" => Some(&Self::RECIPE_770),
            "recipe_771" => Some(&Self::RECIPE_771),
            "recipe_772" => Some(&Self::RECIPE_772),
            "recipe_773" => Some(&Self::RECIPE_773),
            "recipe_774" => Some(&Self::RECIPE_774),
            "recipe_775" => Some(&Self::RECIPE_775),
            "recipe_776" => Some(&Self::RECIPE_776),
            "recipe_777" => Some(&Self::RECIPE_777),
            "recipe_778" => Some(&Self::RECIPE_778),
            "recipe_779" => Some(&Self::RECIPE_779),
            "recipe_780" => Some(&Self::RECIPE_780),
            "recipe_781" => Some(&Self::RECIPE_781),
            "recipe_782" => Some(&Self::RECIPE_782),
            "recipe_783" => Some(&Self::RECIPE_783),
            "recipe_784" => Some(&Self::RECIPE_784),
            "recipe_785" => Some(&Self::RECIPE_785),
            "recipe_786" => Some(&Self::RECIPE_786),
            "recipe_787" => Some(&Self::RECIPE_787),
            "recipe_788" => Some(&Self::RECIPE_788),
            "recipe_789" => Some(&Self::RECIPE_789),
            "recipe_790" => Some(&Self::RECIPE_790),
            "recipe_791" => Some(&Self::RECIPE_791),
            "recipe_792" => Some(&Self::RECIPE_792),
            "recipe_793" => Some(&Self::RECIPE_793),
            "recipe_794" => Some(&Self::RECIPE_794),
            "recipe_795" => Some(&Self::RECIPE_795),
            "recipe_796" => Some(&Self::RECIPE_796),
            "recipe_797" => Some(&Self::RECIPE_797),
            "recipe_798" => Some(&Self::RECIPE_798),
            "recipe_799" => Some(&Self::RECIPE_799),
            "recipe_800" => Some(&Self::RECIPE_800),
            "recipe_801" => Some(&Self::RECIPE_801),
            "recipe_802" => Some(&Self::RECIPE_802),
            "recipe_803" => Some(&Self::RECIPE_803),
            "recipe_804" => Some(&Self::RECIPE_804),
            "recipe_805" => Some(&Self::RECIPE_805),
            "recipe_806" => Some(&Self::RECIPE_806),
            "recipe_807" => Some(&Self::RECIPE_807),
            "recipe_808" => Some(&Self::RECIPE_808),
            "recipe_809" => Some(&Self::RECIPE_809),
            "recipe_810" => Some(&Self::RECIPE_810),
            "recipe_811" => Some(&Self::RECIPE_811),
            "recipe_812" => Some(&Self::RECIPE_812),
            "recipe_813" => Some(&Self::RECIPE_813),
            "recipe_814" => Some(&Self::RECIPE_814),
            "recipe_815" => Some(&Self::RECIPE_815),
            "recipe_816" => Some(&Self::RECIPE_816),
            "recipe_817" => Some(&Self::RECIPE_817),
            "recipe_818" => Some(&Self::RECIPE_818),
            "recipe_819" => Some(&Self::RECIPE_819),
            "recipe_820" => Some(&Self::RECIPE_820),
            "recipe_821" => Some(&Self::RECIPE_821),
            "recipe_822" => Some(&Self::RECIPE_822),
            "recipe_823" => Some(&Self::RECIPE_823),
            "recipe_824" => Some(&Self::RECIPE_824),
            "recipe_825" => Some(&Self::RECIPE_825),
            "recipe_826" => Some(&Self::RECIPE_826),
            "recipe_827" => Some(&Self::RECIPE_827),
            "recipe_828" => Some(&Self::RECIPE_828),
            "recipe_829" => Some(&Self::RECIPE_829),
            "recipe_830" => Some(&Self::RECIPE_830),
            "recipe_831" => Some(&Self::RECIPE_831),
            "recipe_832" => Some(&Self::RECIPE_832),
            "recipe_833" => Some(&Self::RECIPE_833),
            "recipe_834" => Some(&Self::RECIPE_834),
            "recipe_835" => Some(&Self::RECIPE_835),
            "recipe_836" => Some(&Self::RECIPE_836),
            "recipe_837" => Some(&Self::RECIPE_837),
            "recipe_838" => Some(&Self::RECIPE_838),
            "recipe_839" => Some(&Self::RECIPE_839),
            "recipe_840" => Some(&Self::RECIPE_840),
            "recipe_841" => Some(&Self::RECIPE_841),
            "recipe_842" => Some(&Self::RECIPE_842),
            "recipe_843" => Some(&Self::RECIPE_843),
            "recipe_844" => Some(&Self::RECIPE_844),
            "recipe_845" => Some(&Self::RECIPE_845),
            "recipe_846" => Some(&Self::RECIPE_846),
            "recipe_847" => Some(&Self::RECIPE_847),
            "recipe_848" => Some(&Self::RECIPE_848),
            "recipe_849" => Some(&Self::RECIPE_849),
            "recipe_850" => Some(&Self::RECIPE_850),
            "recipe_851" => Some(&Self::RECIPE_851),
            "recipe_852" => Some(&Self::RECIPE_852),
            "recipe_853" => Some(&Self::RECIPE_853),
            "recipe_854" => Some(&Self::RECIPE_854),
            "recipe_855" => Some(&Self::RECIPE_855),
            "recipe_856" => Some(&Self::RECIPE_856),
            "recipe_857" => Some(&Self::RECIPE_857),
            "recipe_858" => Some(&Self::RECIPE_858),
            "recipe_859" => Some(&Self::RECIPE_859),
            "recipe_860" => Some(&Self::RECIPE_860),
            "recipe_861" => Some(&Self::RECIPE_861),
            "recipe_862" => Some(&Self::RECIPE_862),
            "recipe_863" => Some(&Self::RECIPE_863),
            "recipe_864" => Some(&Self::RECIPE_864),
            "recipe_865" => Some(&Self::RECIPE_865),
            "recipe_866" => Some(&Self::RECIPE_866),
            "recipe_867" => Some(&Self::RECIPE_867),
            "recipe_868" => Some(&Self::RECIPE_868),
            "recipe_869" => Some(&Self::RECIPE_869),
            "recipe_870" => Some(&Self::RECIPE_870),
            "recipe_871" => Some(&Self::RECIPE_871),
            "recipe_872" => Some(&Self::RECIPE_872),
            "recipe_873" => Some(&Self::RECIPE_873),
            "recipe_874" => Some(&Self::RECIPE_874),
            "recipe_875" => Some(&Self::RECIPE_875),
            "recipe_876" => Some(&Self::RECIPE_876),
            "recipe_877" => Some(&Self::RECIPE_877),
            "recipe_878" => Some(&Self::RECIPE_878),
            "recipe_879" => Some(&Self::RECIPE_879),
            "recipe_880" => Some(&Self::RECIPE_880),
            "recipe_881" => Some(&Self::RECIPE_881),
            "recipe_882" => Some(&Self::RECIPE_882),
            "recipe_883" => Some(&Self::RECIPE_883),
            "recipe_884" => Some(&Self::RECIPE_884),
            "recipe_885" => Some(&Self::RECIPE_885),
            "recipe_886" => Some(&Self::RECIPE_886),
            "recipe_887" => Some(&Self::RECIPE_887),
            "recipe_888" => Some(&Self::RECIPE_888),
            "recipe_889" => Some(&Self::RECIPE_889),
            "recipe_890" => Some(&Self::RECIPE_890),
            "recipe_891" => Some(&Self::RECIPE_891),
            "recipe_892" => Some(&Self::RECIPE_892),
            "recipe_893" => Some(&Self::RECIPE_893),
            "recipe_894" => Some(&Self::RECIPE_894),
            "recipe_895" => Some(&Self::RECIPE_895),
            "recipe_896" => Some(&Self::RECIPE_896),
            "recipe_897" => Some(&Self::RECIPE_897),
            "recipe_898" => Some(&Self::RECIPE_898),
            "recipe_899" => Some(&Self::RECIPE_899),
            "recipe_900" => Some(&Self::RECIPE_900),
            "recipe_901" => Some(&Self::RECIPE_901),
            "recipe_902" => Some(&Self::RECIPE_902),
            "recipe_903" => Some(&Self::RECIPE_903),
            "recipe_904" => Some(&Self::RECIPE_904),
            "recipe_905" => Some(&Self::RECIPE_905),
            "recipe_906" => Some(&Self::RECIPE_906),
            "recipe_907" => Some(&Self::RECIPE_907),
            "recipe_908" => Some(&Self::RECIPE_908),
            "recipe_909" => Some(&Self::RECIPE_909),
            "recipe_910" => Some(&Self::RECIPE_910),
            "recipe_911" => Some(&Self::RECIPE_911),
            "recipe_912" => Some(&Self::RECIPE_912),
            "recipe_913" => Some(&Self::RECIPE_913),
            "recipe_914" => Some(&Self::RECIPE_914),
            "recipe_915" => Some(&Self::RECIPE_915),
            "recipe_916" => Some(&Self::RECIPE_916),
            "recipe_917" => Some(&Self::RECIPE_917),
            "recipe_918" => Some(&Self::RECIPE_918),
            "recipe_919" => Some(&Self::RECIPE_919),
            "recipe_920" => Some(&Self::RECIPE_920),
            "recipe_921" => Some(&Self::RECIPE_921),
            "recipe_922" => Some(&Self::RECIPE_922),
            "recipe_923" => Some(&Self::RECIPE_923),
            "recipe_924" => Some(&Self::RECIPE_924),
            "recipe_925" => Some(&Self::RECIPE_925),
            "recipe_926" => Some(&Self::RECIPE_926),
            "recipe_927" => Some(&Self::RECIPE_927),
            "recipe_928" => Some(&Self::RECIPE_928),
            "recipe_929" => Some(&Self::RECIPE_929),
            "recipe_930" => Some(&Self::RECIPE_930),
            "recipe_931" => Some(&Self::RECIPE_931),
            "recipe_932" => Some(&Self::RECIPE_932),
            "recipe_933" => Some(&Self::RECIPE_933),
            "recipe_934" => Some(&Self::RECIPE_934),
            "recipe_935" => Some(&Self::RECIPE_935),
            "recipe_936" => Some(&Self::RECIPE_936),
            "recipe_937" => Some(&Self::RECIPE_937),
            "recipe_938" => Some(&Self::RECIPE_938),
            "recipe_939" => Some(&Self::RECIPE_939),
            "recipe_940" => Some(&Self::RECIPE_940),
            "recipe_941" => Some(&Self::RECIPE_941),
            "recipe_942" => Some(&Self::RECIPE_942),
            "recipe_943" => Some(&Self::RECIPE_943),
            "recipe_944" => Some(&Self::RECIPE_944),
            "recipe_945" => Some(&Self::RECIPE_945),
            "recipe_946" => Some(&Self::RECIPE_946),
            "recipe_947" => Some(&Self::RECIPE_947),
            "recipe_948" => Some(&Self::RECIPE_948),
            "recipe_949" => Some(&Self::RECIPE_949),
            "recipe_950" => Some(&Self::RECIPE_950),
            "recipe_951" => Some(&Self::RECIPE_951),
            "recipe_952" => Some(&Self::RECIPE_952),
            "recipe_953" => Some(&Self::RECIPE_953),
            "recipe_954" => Some(&Self::RECIPE_954),
            "recipe_955" => Some(&Self::RECIPE_955),
            "recipe_956" => Some(&Self::RECIPE_956),
            "recipe_957" => Some(&Self::RECIPE_957),
            "recipe_958" => Some(&Self::RECIPE_958),
            "recipe_959" => Some(&Self::RECIPE_959),
            "recipe_960" => Some(&Self::RECIPE_960),
            "recipe_961" => Some(&Self::RECIPE_961),
            "recipe_962" => Some(&Self::RECIPE_962),
            "recipe_963" => Some(&Self::RECIPE_963),
            "recipe_964" => Some(&Self::RECIPE_964),
            "recipe_965" => Some(&Self::RECIPE_965),
            "recipe_966" => Some(&Self::RECIPE_966),
            "recipe_967" => Some(&Self::RECIPE_967),
            "recipe_968" => Some(&Self::RECIPE_968),
            "recipe_969" => Some(&Self::RECIPE_969),
            "recipe_970" => Some(&Self::RECIPE_970),
            "recipe_971" => Some(&Self::RECIPE_971),
            "recipe_972" => Some(&Self::RECIPE_972),
            "recipe_973" => Some(&Self::RECIPE_973),
            "recipe_974" => Some(&Self::RECIPE_974),
            "recipe_975" => Some(&Self::RECIPE_975),
            "recipe_976" => Some(&Self::RECIPE_976),
            "recipe_977" => Some(&Self::RECIPE_977),
            "recipe_978" => Some(&Self::RECIPE_978),
            "recipe_979" => Some(&Self::RECIPE_979),
            "recipe_980" => Some(&Self::RECIPE_980),
            "recipe_981" => Some(&Self::RECIPE_981),
            "recipe_982" => Some(&Self::RECIPE_982),
            "recipe_983" => Some(&Self::RECIPE_983),
            "recipe_984" => Some(&Self::RECIPE_984),
            "recipe_985" => Some(&Self::RECIPE_985),
            "recipe_986" => Some(&Self::RECIPE_986),
            "recipe_987" => Some(&Self::RECIPE_987),
            "recipe_988" => Some(&Self::RECIPE_988),
            "recipe_989" => Some(&Self::RECIPE_989),
            "recipe_990" => Some(&Self::RECIPE_990),
            "recipe_991" => Some(&Self::RECIPE_991),
            "recipe_992" => Some(&Self::RECIPE_992),
            "recipe_993" => Some(&Self::RECIPE_993),
            "recipe_994" => Some(&Self::RECIPE_994),
            "recipe_995" => Some(&Self::RECIPE_995),
            "recipe_996" => Some(&Self::RECIPE_996),
            "recipe_997" => Some(&Self::RECIPE_997),
            "recipe_998" => Some(&Self::RECIPE_998),
            "recipe_999" => Some(&Self::RECIPE_999),
            "recipe_1000" => Some(&Self::RECIPE_1000),
            "recipe_1001" => Some(&Self::RECIPE_1001),
            "recipe_1002" => Some(&Self::RECIPE_1002),
            "recipe_1003" => Some(&Self::RECIPE_1003),
            "recipe_1004" => Some(&Self::RECIPE_1004),
            "recipe_1005" => Some(&Self::RECIPE_1005),
            "recipe_1006" => Some(&Self::RECIPE_1006),
            "recipe_1007" => Some(&Self::RECIPE_1007),
            "recipe_1008" => Some(&Self::RECIPE_1008),
            "recipe_1009" => Some(&Self::RECIPE_1009),
            "recipe_1010" => Some(&Self::RECIPE_1010),
            "recipe_1011" => Some(&Self::RECIPE_1011),
            "recipe_1012" => Some(&Self::RECIPE_1012),
            "recipe_1013" => Some(&Self::RECIPE_1013),
            "recipe_1014" => Some(&Self::RECIPE_1014),
            "recipe_1015" => Some(&Self::RECIPE_1015),
            "recipe_1016" => Some(&Self::RECIPE_1016),
            "recipe_1017" => Some(&Self::RECIPE_1017),
            "recipe_1018" => Some(&Self::RECIPE_1018),
            "recipe_1019" => Some(&Self::RECIPE_1019),
            "recipe_1020" => Some(&Self::RECIPE_1020),
            "recipe_1021" => Some(&Self::RECIPE_1021),
            "recipe_1022" => Some(&Self::RECIPE_1022),
            "recipe_1023" => Some(&Self::RECIPE_1023),
            "recipe_1024" => Some(&Self::RECIPE_1024),
            "recipe_1025" => Some(&Self::RECIPE_1025),
            "recipe_1026" => Some(&Self::RECIPE_1026),
            "recipe_1027" => Some(&Self::RECIPE_1027),
            "recipe_1028" => Some(&Self::RECIPE_1028),
            "recipe_1029" => Some(&Self::RECIPE_1029),
            "recipe_1030" => Some(&Self::RECIPE_1030),
            "recipe_1031" => Some(&Self::RECIPE_1031),
            "recipe_1032" => Some(&Self::RECIPE_1032),
            "recipe_1033" => Some(&Self::RECIPE_1033),
            "recipe_1034" => Some(&Self::RECIPE_1034),
            "recipe_1035" => Some(&Self::RECIPE_1035),
            "recipe_1036" => Some(&Self::RECIPE_1036),
            "recipe_1037" => Some(&Self::RECIPE_1037),
            "recipe_1038" => Some(&Self::RECIPE_1038),
            "recipe_1039" => Some(&Self::RECIPE_1039),
            "recipe_1040" => Some(&Self::RECIPE_1040),
            "recipe_1041" => Some(&Self::RECIPE_1041),
            "recipe_1042" => Some(&Self::RECIPE_1042),
            "recipe_1043" => Some(&Self::RECIPE_1043),
            "recipe_1044" => Some(&Self::RECIPE_1044),
            "recipe_1045" => Some(&Self::RECIPE_1045),
            "recipe_1046" => Some(&Self::RECIPE_1046),
            "recipe_1047" => Some(&Self::RECIPE_1047),
            "recipe_1048" => Some(&Self::RECIPE_1048),
            "recipe_1049" => Some(&Self::RECIPE_1049),
            "recipe_1050" => Some(&Self::RECIPE_1050),
            "recipe_1051" => Some(&Self::RECIPE_1051),
            "recipe_1052" => Some(&Self::RECIPE_1052),
            "recipe_1053" => Some(&Self::RECIPE_1053),
            "recipe_1054" => Some(&Self::RECIPE_1054),
            "recipe_1055" => Some(&Self::RECIPE_1055),
            "recipe_1056" => Some(&Self::RECIPE_1056),
            "recipe_1057" => Some(&Self::RECIPE_1057),
            "recipe_1058" => Some(&Self::RECIPE_1058),
            "recipe_1059" => Some(&Self::RECIPE_1059),
            "recipe_1060" => Some(&Self::RECIPE_1060),
            "recipe_1061" => Some(&Self::RECIPE_1061),
            "recipe_1062" => Some(&Self::RECIPE_1062),
            "recipe_1063" => Some(&Self::RECIPE_1063),
            "recipe_1064" => Some(&Self::RECIPE_1064),
            "recipe_1065" => Some(&Self::RECIPE_1065),
            "recipe_1066" => Some(&Self::RECIPE_1066),
            "recipe_1067" => Some(&Self::RECIPE_1067),
            "recipe_1068" => Some(&Self::RECIPE_1068),
            "recipe_1069" => Some(&Self::RECIPE_1069),
            "recipe_1070" => Some(&Self::RECIPE_1070),
            "recipe_1071" => Some(&Self::RECIPE_1071),
            "recipe_1072" => Some(&Self::RECIPE_1072),
            "recipe_1073" => Some(&Self::RECIPE_1073),
            "recipe_1074" => Some(&Self::RECIPE_1074),
            "recipe_1075" => Some(&Self::RECIPE_1075),
            "recipe_1076" => Some(&Self::RECIPE_1076),
            "recipe_1077" => Some(&Self::RECIPE_1077),
            "recipe_1078" => Some(&Self::RECIPE_1078),
            "recipe_1079" => Some(&Self::RECIPE_1079),
            "recipe_1080" => Some(&Self::RECIPE_1080),
            "recipe_1081" => Some(&Self::RECIPE_1081),
            "recipe_1082" => Some(&Self::RECIPE_1082),
            "recipe_1083" => Some(&Self::RECIPE_1083),
            "recipe_1084" => Some(&Self::RECIPE_1084),
            "recipe_1085" => Some(&Self::RECIPE_1085),
            "recipe_1086" => Some(&Self::RECIPE_1086),
            "recipe_1087" => Some(&Self::RECIPE_1087),
            "recipe_1088" => Some(&Self::RECIPE_1088),
            "recipe_1089" => Some(&Self::RECIPE_1089),
            "recipe_1090" => Some(&Self::RECIPE_1090),
            "recipe_1091" => Some(&Self::RECIPE_1091),
            "recipe_1092" => Some(&Self::RECIPE_1092),
            "recipe_1093" => Some(&Self::RECIPE_1093),
            "recipe_1094" => Some(&Self::RECIPE_1094),
            "recipe_1095" => Some(&Self::RECIPE_1095),
            "recipe_1096" => Some(&Self::RECIPE_1096),
            "recipe_1097" => Some(&Self::RECIPE_1097),
            "recipe_1098" => Some(&Self::RECIPE_1098),
            "recipe_1099" => Some(&Self::RECIPE_1099),
            "recipe_1100" => Some(&Self::RECIPE_1100),
            "recipe_1101" => Some(&Self::RECIPE_1101),
            "recipe_1102" => Some(&Self::RECIPE_1102),
            "recipe_1103" => Some(&Self::RECIPE_1103),
            "recipe_1104" => Some(&Self::RECIPE_1104),
            "recipe_1105" => Some(&Self::RECIPE_1105),
            "recipe_1106" => Some(&Self::RECIPE_1106),
            "recipe_1107" => Some(&Self::RECIPE_1107),
            "recipe_1108" => Some(&Self::RECIPE_1108),
            "recipe_1109" => Some(&Self::RECIPE_1109),
            "recipe_1110" => Some(&Self::RECIPE_1110),
            "recipe_1111" => Some(&Self::RECIPE_1111),
            "recipe_1112" => Some(&Self::RECIPE_1112),
            "recipe_1113" => Some(&Self::RECIPE_1113),
            "recipe_1114" => Some(&Self::RECIPE_1114),
            "recipe_1115" => Some(&Self::RECIPE_1115),
            "recipe_1116" => Some(&Self::RECIPE_1116),
            "recipe_1117" => Some(&Self::RECIPE_1117),
            "recipe_1118" => Some(&Self::RECIPE_1118),
            "recipe_1119" => Some(&Self::RECIPE_1119),
            "recipe_1120" => Some(&Self::RECIPE_1120),
            "recipe_1121" => Some(&Self::RECIPE_1121),
            "recipe_1122" => Some(&Self::RECIPE_1122),
            "recipe_1123" => Some(&Self::RECIPE_1123),
            "recipe_1124" => Some(&Self::RECIPE_1124),
            "recipe_1125" => Some(&Self::RECIPE_1125),
            "recipe_1126" => Some(&Self::RECIPE_1126),
            "recipe_1127" => Some(&Self::RECIPE_1127),
            "recipe_1128" => Some(&Self::RECIPE_1128),
            "recipe_1129" => Some(&Self::RECIPE_1129),
            "recipe_1130" => Some(&Self::RECIPE_1130),
            "recipe_1131" => Some(&Self::RECIPE_1131),
            "recipe_1132" => Some(&Self::RECIPE_1132),
            "recipe_1133" => Some(&Self::RECIPE_1133),
            "recipe_1134" => Some(&Self::RECIPE_1134),
            "recipe_1135" => Some(&Self::RECIPE_1135),
            "recipe_1136" => Some(&Self::RECIPE_1136),
            "recipe_1137" => Some(&Self::RECIPE_1137),
            "recipe_1138" => Some(&Self::RECIPE_1138),
            "recipe_1139" => Some(&Self::RECIPE_1139),
            "recipe_1140" => Some(&Self::RECIPE_1140),
            "recipe_1141" => Some(&Self::RECIPE_1141),
            "recipe_1142" => Some(&Self::RECIPE_1142),
            "recipe_1143" => Some(&Self::RECIPE_1143),
            "recipe_1144" => Some(&Self::RECIPE_1144),
            "recipe_1145" => Some(&Self::RECIPE_1145),
            "recipe_1146" => Some(&Self::RECIPE_1146),
            "recipe_1147" => Some(&Self::RECIPE_1147),
            "recipe_1148" => Some(&Self::RECIPE_1148),
            "recipe_1149" => Some(&Self::RECIPE_1149),
            "recipe_1150" => Some(&Self::RECIPE_1150),
            "recipe_1151" => Some(&Self::RECIPE_1151),
            "recipe_1152" => Some(&Self::RECIPE_1152),
            "recipe_1153" => Some(&Self::RECIPE_1153),
            "recipe_1154" => Some(&Self::RECIPE_1154),
            "recipe_1155" => Some(&Self::RECIPE_1155),
            "recipe_1156" => Some(&Self::RECIPE_1156),
            "recipe_1157" => Some(&Self::RECIPE_1157),
            "recipe_1158" => Some(&Self::RECIPE_1158),
            "recipe_1159" => Some(&Self::RECIPE_1159),
            "recipe_1160" => Some(&Self::RECIPE_1160),
            "recipe_1161" => Some(&Self::RECIPE_1161),
            "recipe_1162" => Some(&Self::RECIPE_1162),
            "recipe_1163" => Some(&Self::RECIPE_1163),
            "recipe_1164" => Some(&Self::RECIPE_1164),
            "recipe_1165" => Some(&Self::RECIPE_1165),
            "recipe_1166" => Some(&Self::RECIPE_1166),
            "recipe_1167" => Some(&Self::RECIPE_1167),
            "recipe_1168" => Some(&Self::RECIPE_1168),
            "recipe_1169" => Some(&Self::RECIPE_1169),
            "recipe_1170" => Some(&Self::RECIPE_1170),
            "recipe_1171" => Some(&Self::RECIPE_1171),
            "recipe_1172" => Some(&Self::RECIPE_1172),
            "recipe_1173" => Some(&Self::RECIPE_1173),
            "recipe_1174" => Some(&Self::RECIPE_1174),
            "recipe_1175" => Some(&Self::RECIPE_1175),
            "recipe_1176" => Some(&Self::RECIPE_1176),
            "recipe_1177" => Some(&Self::RECIPE_1177),
            "recipe_1178" => Some(&Self::RECIPE_1178),
            "recipe_1179" => Some(&Self::RECIPE_1179),
            "recipe_1180" => Some(&Self::RECIPE_1180),
            "recipe_1181" => Some(&Self::RECIPE_1181),
            "recipe_1182" => Some(&Self::RECIPE_1182),
            "recipe_1183" => Some(&Self::RECIPE_1183),
            "recipe_1184" => Some(&Self::RECIPE_1184),
            "recipe_1185" => Some(&Self::RECIPE_1185),
            "recipe_1186" => Some(&Self::RECIPE_1186),
            "recipe_1187" => Some(&Self::RECIPE_1187),
            "recipe_1188" => Some(&Self::RECIPE_1188),
            "recipe_1189" => Some(&Self::RECIPE_1189),
            "recipe_1190" => Some(&Self::RECIPE_1190),
            "recipe_1191" => Some(&Self::RECIPE_1191),
            "recipe_1192" => Some(&Self::RECIPE_1192),
            "recipe_1193" => Some(&Self::RECIPE_1193),
            "recipe_1194" => Some(&Self::RECIPE_1194),
            "recipe_1195" => Some(&Self::RECIPE_1195),
            "recipe_1196" => Some(&Self::RECIPE_1196),
            "recipe_1197" => Some(&Self::RECIPE_1197),
            "recipe_1198" => Some(&Self::RECIPE_1198),
            "recipe_1199" => Some(&Self::RECIPE_1199),
            "recipe_1200" => Some(&Self::RECIPE_1200),
            "recipe_1201" => Some(&Self::RECIPE_1201),
            "recipe_1202" => Some(&Self::RECIPE_1202),
            "recipe_1203" => Some(&Self::RECIPE_1203),
            "recipe_1204" => Some(&Self::RECIPE_1204),
            "recipe_1205" => Some(&Self::RECIPE_1205),
            "recipe_1206" => Some(&Self::RECIPE_1206),
            "recipe_1207" => Some(&Self::RECIPE_1207),
            "recipe_1208" => Some(&Self::RECIPE_1208),
            "recipe_1209" => Some(&Self::RECIPE_1209),
            "recipe_1210" => Some(&Self::RECIPE_1210),
            "recipe_1211" => Some(&Self::RECIPE_1211),
            "recipe_1212" => Some(&Self::RECIPE_1212),
            "recipe_1213" => Some(&Self::RECIPE_1213),
            "recipe_1214" => Some(&Self::RECIPE_1214),
            "recipe_1215" => Some(&Self::RECIPE_1215),
            "recipe_1216" => Some(&Self::RECIPE_1216),
            "recipe_1217" => Some(&Self::RECIPE_1217),
            "recipe_1218" => Some(&Self::RECIPE_1218),
            "recipe_1219" => Some(&Self::RECIPE_1219),
            "recipe_1220" => Some(&Self::RECIPE_1220),
            "recipe_1221" => Some(&Self::RECIPE_1221),
            "recipe_1222" => Some(&Self::RECIPE_1222),
            "recipe_1223" => Some(&Self::RECIPE_1223),
            "recipe_1224" => Some(&Self::RECIPE_1224),
            "recipe_1225" => Some(&Self::RECIPE_1225),
            "recipe_1226" => Some(&Self::RECIPE_1226),
            "recipe_1227" => Some(&Self::RECIPE_1227),
            "recipe_1228" => Some(&Self::RECIPE_1228),
            "recipe_1229" => Some(&Self::RECIPE_1229),
            "recipe_1230" => Some(&Self::RECIPE_1230),
            "recipe_1231" => Some(&Self::RECIPE_1231),
            "recipe_1232" => Some(&Self::RECIPE_1232),
            "recipe_1233" => Some(&Self::RECIPE_1233),
            "recipe_1234" => Some(&Self::RECIPE_1234),
            "recipe_1235" => Some(&Self::RECIPE_1235),
            "recipe_1236" => Some(&Self::RECIPE_1236),
            "recipe_1237" => Some(&Self::RECIPE_1237),
            "recipe_1238" => Some(&Self::RECIPE_1238),
            "recipe_1239" => Some(&Self::RECIPE_1239),
            "recipe_1240" => Some(&Self::RECIPE_1240),
            "recipe_1241" => Some(&Self::RECIPE_1241),
            "recipe_1242" => Some(&Self::RECIPE_1242),
            "recipe_1243" => Some(&Self::RECIPE_1243),
            "recipe_1244" => Some(&Self::RECIPE_1244),
            "recipe_1245" => Some(&Self::RECIPE_1245),
            "recipe_1246" => Some(&Self::RECIPE_1246),
            "recipe_1247" => Some(&Self::RECIPE_1247),
            "recipe_1248" => Some(&Self::RECIPE_1248),
            "recipe_1249" => Some(&Self::RECIPE_1249),
            "recipe_1250" => Some(&Self::RECIPE_1250),
            "recipe_1251" => Some(&Self::RECIPE_1251),
            "recipe_1252" => Some(&Self::RECIPE_1252),
            "recipe_1253" => Some(&Self::RECIPE_1253),
            "recipe_1254" => Some(&Self::RECIPE_1254),
            "recipe_1255" => Some(&Self::RECIPE_1255),
            "recipe_1256" => Some(&Self::RECIPE_1256),
            "recipe_1257" => Some(&Self::RECIPE_1257),
            "recipe_1258" => Some(&Self::RECIPE_1258),
            "recipe_1259" => Some(&Self::RECIPE_1259),
            "recipe_1260" => Some(&Self::RECIPE_1260),
            "recipe_1261" => Some(&Self::RECIPE_1261),
            "recipe_1262" => Some(&Self::RECIPE_1262),
            "recipe_1263" => Some(&Self::RECIPE_1263),
            "recipe_1264" => Some(&Self::RECIPE_1264),
            "recipe_1265" => Some(&Self::RECIPE_1265),
            "recipe_1266" => Some(&Self::RECIPE_1266),
            "recipe_1267" => Some(&Self::RECIPE_1267),
            "recipe_1268" => Some(&Self::RECIPE_1268),
            "recipe_1269" => Some(&Self::RECIPE_1269),
            "recipe_1270" => Some(&Self::RECIPE_1270),
            "recipe_1271" => Some(&Self::RECIPE_1271),
            "recipe_1272" => Some(&Self::RECIPE_1272),
            "recipe_1273" => Some(&Self::RECIPE_1273),
            "recipe_1274" => Some(&Self::RECIPE_1274),
            "recipe_1275" => Some(&Self::RECIPE_1275),
            "recipe_1276" => Some(&Self::RECIPE_1276),
            "recipe_1277" => Some(&Self::RECIPE_1277),
            "recipe_1278" => Some(&Self::RECIPE_1278),
            "recipe_1279" => Some(&Self::RECIPE_1279),
            "recipe_1280" => Some(&Self::RECIPE_1280),
            "recipe_1281" => Some(&Self::RECIPE_1281),
            "recipe_1282" => Some(&Self::RECIPE_1282),
            "recipe_1283" => Some(&Self::RECIPE_1283),
            "recipe_1284" => Some(&Self::RECIPE_1284),
            "recipe_1285" => Some(&Self::RECIPE_1285),
            "recipe_1286" => Some(&Self::RECIPE_1286),
            "recipe_1287" => Some(&Self::RECIPE_1287),
            "recipe_1288" => Some(&Self::RECIPE_1288),
            "recipe_1289" => Some(&Self::RECIPE_1289),
            "recipe_1290" => Some(&Self::RECIPE_1290),
            "recipe_1291" => Some(&Self::RECIPE_1291),
            "recipe_1292" => Some(&Self::RECIPE_1292),
            "recipe_1293" => Some(&Self::RECIPE_1293),
            "recipe_1294" => Some(&Self::RECIPE_1294),
            "recipe_1295" => Some(&Self::RECIPE_1295),
            "recipe_1296" => Some(&Self::RECIPE_1296),
            "recipe_1297" => Some(&Self::RECIPE_1297),
            "recipe_1298" => Some(&Self::RECIPE_1298),
            "recipe_1299" => Some(&Self::RECIPE_1299),
            "recipe_1300" => Some(&Self::RECIPE_1300),
            "recipe_1301" => Some(&Self::RECIPE_1301),
            "recipe_1302" => Some(&Self::RECIPE_1302),
            "recipe_1303" => Some(&Self::RECIPE_1303),
            "recipe_1304" => Some(&Self::RECIPE_1304),
            "recipe_1305" => Some(&Self::RECIPE_1305),
            "recipe_1306" => Some(&Self::RECIPE_1306),
            "recipe_1307" => Some(&Self::RECIPE_1307),
            "recipe_1308" => Some(&Self::RECIPE_1308),
            "recipe_1309" => Some(&Self::RECIPE_1309),
            "recipe_1310" => Some(&Self::RECIPE_1310),
            "recipe_1311" => Some(&Self::RECIPE_1311),
            "recipe_1312" => Some(&Self::RECIPE_1312),
            "recipe_1313" => Some(&Self::RECIPE_1313),
            "recipe_1314" => Some(&Self::RECIPE_1314),
            "recipe_1315" => Some(&Self::RECIPE_1315),
            "recipe_1316" => Some(&Self::RECIPE_1316),
            "recipe_1317" => Some(&Self::RECIPE_1317),
            "recipe_1318" => Some(&Self::RECIPE_1318),
            "recipe_1319" => Some(&Self::RECIPE_1319),
            "recipe_1320" => Some(&Self::RECIPE_1320),
            "recipe_1321" => Some(&Self::RECIPE_1321),
            "recipe_1322" => Some(&Self::RECIPE_1322),
            "recipe_1323" => Some(&Self::RECIPE_1323),
            "recipe_1324" => Some(&Self::RECIPE_1324),
            "recipe_1325" => Some(&Self::RECIPE_1325),
            "recipe_1326" => Some(&Self::RECIPE_1326),
            "recipe_1327" => Some(&Self::RECIPE_1327),
            "recipe_1328" => Some(&Self::RECIPE_1328),
            "recipe_1329" => Some(&Self::RECIPE_1329),
            "recipe_1330" => Some(&Self::RECIPE_1330),
            "recipe_1331" => Some(&Self::RECIPE_1331),
            "recipe_1332" => Some(&Self::RECIPE_1332),
            "recipe_1333" => Some(&Self::RECIPE_1333),
            "recipe_1334" => Some(&Self::RECIPE_1334),
            "recipe_1335" => Some(&Self::RECIPE_1335),
            "recipe_1336" => Some(&Self::RECIPE_1336),
            "recipe_1337" => Some(&Self::RECIPE_1337),
            "recipe_1338" => Some(&Self::RECIPE_1338),
            "recipe_1339" => Some(&Self::RECIPE_1339),
            "recipe_1340" => Some(&Self::RECIPE_1340),
            "recipe_1341" => Some(&Self::RECIPE_1341),
            "recipe_1342" => Some(&Self::RECIPE_1342),
            "recipe_1343" => Some(&Self::RECIPE_1343),
            "recipe_1344" => Some(&Self::RECIPE_1344),
            "recipe_1345" => Some(&Self::RECIPE_1345),
            "recipe_1346" => Some(&Self::RECIPE_1346),
            "recipe_1347" => Some(&Self::RECIPE_1347),
            "recipe_1348" => Some(&Self::RECIPE_1348),
            "recipe_1349" => Some(&Self::RECIPE_1349),
            "recipe_1350" => Some(&Self::RECIPE_1350),
            "recipe_1351" => Some(&Self::RECIPE_1351),
            "recipe_1352" => Some(&Self::RECIPE_1352),
            "recipe_1353" => Some(&Self::RECIPE_1353),
            "recipe_1354" => Some(&Self::RECIPE_1354),
            "recipe_1355" => Some(&Self::RECIPE_1355),
            "recipe_1356" => Some(&Self::RECIPE_1356),
            "recipe_1357" => Some(&Self::RECIPE_1357),
            "recipe_1358" => Some(&Self::RECIPE_1358),
            "recipe_1359" => Some(&Self::RECIPE_1359),
            "recipe_1360" => Some(&Self::RECIPE_1360),
            "recipe_1361" => Some(&Self::RECIPE_1361),
            "recipe_1362" => Some(&Self::RECIPE_1362),
            "recipe_1363" => Some(&Self::RECIPE_1363),
            "recipe_1364" => Some(&Self::RECIPE_1364),
            "recipe_1365" => Some(&Self::RECIPE_1365),
            "recipe_1366" => Some(&Self::RECIPE_1366),
            "recipe_1367" => Some(&Self::RECIPE_1367),
            "recipe_1368" => Some(&Self::RECIPE_1368),
            "recipe_1369" => Some(&Self::RECIPE_1369),
            "recipe_1370" => Some(&Self::RECIPE_1370),
            "recipe_1371" => Some(&Self::RECIPE_1371),
            "recipe_1372" => Some(&Self::RECIPE_1372),
            "recipe_1373" => Some(&Self::RECIPE_1373),
            "recipe_1374" => Some(&Self::RECIPE_1374),
            "recipe_1375" => Some(&Self::RECIPE_1375),
            "recipe_1376" => Some(&Self::RECIPE_1376),
            "recipe_1377" => Some(&Self::RECIPE_1377),
            "recipe_1378" => Some(&Self::RECIPE_1378),
            "recipe_1379" => Some(&Self::RECIPE_1379),
            "recipe_1380" => Some(&Self::RECIPE_1380),
            "recipe_1381" => Some(&Self::RECIPE_1381),
            "recipe_1382" => Some(&Self::RECIPE_1382),
            "recipe_1383" => Some(&Self::RECIPE_1383),
            "recipe_1384" => Some(&Self::RECIPE_1384),
            "recipe_1385" => Some(&Self::RECIPE_1385),
            "recipe_1386" => Some(&Self::RECIPE_1386),
            "recipe_1387" => Some(&Self::RECIPE_1387),
            "recipe_1388" => Some(&Self::RECIPE_1388),
            "recipe_1389" => Some(&Self::RECIPE_1389),
            "recipe_1390" => Some(&Self::RECIPE_1390),
            "recipe_1391" => Some(&Self::RECIPE_1391),
            "recipe_1392" => Some(&Self::RECIPE_1392),
            "recipe_1393" => Some(&Self::RECIPE_1393),
            "recipe_1394" => Some(&Self::RECIPE_1394),
            "recipe_1395" => Some(&Self::RECIPE_1395),
            "recipe_1396" => Some(&Self::RECIPE_1396),
            "recipe_1397" => Some(&Self::RECIPE_1397),
            "recipe_1398" => Some(&Self::RECIPE_1398),
            "recipe_1399" => Some(&Self::RECIPE_1399),
            "recipe_1400" => Some(&Self::RECIPE_1400),
            "recipe_1401" => Some(&Self::RECIPE_1401),
            "recipe_1402" => Some(&Self::RECIPE_1402),
            "recipe_1403" => Some(&Self::RECIPE_1403),
            "recipe_1404" => Some(&Self::RECIPE_1404),
            "recipe_1405" => Some(&Self::RECIPE_1405),
            "recipe_1406" => Some(&Self::RECIPE_1406),
            _ => None,
        }
    }
    #[doc = r" Check if this is a crafting recipe."]
    pub const fn is_crafting(&self) -> bool {
        matches!(
            self.recipe_type,
            "minecraft:crafting_shaped" | "minecraft:crafting_shapeless"
        )
    }
    #[doc = r" Check if this is a smelting recipe."]
    pub const fn is_smelting(&self) -> bool {
        matches!(
            self.recipe_type,
            "minecraft:smelting"
                | "minecraft:blasting"
                | "minecraft:smoking"
                | "minecraft:campfire_cooking"
        )
    }
    #[doc = r" Check if this is a stonecutting recipe."]
    pub const fn is_stonecutting(&self) -> bool {
        self.recipe_type == "minecraft:stonecutting"
    }
    #[doc = r" Check if this is a smithing recipe."]
    pub const fn is_smithing(&self) -> bool {
        self.recipe_type == "minecraft:smithing"
    }
}
